use log::error;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use thiserror::Error;

////////////////////////////////////////////////////////////////////////////////////////////////////

type Result<T> = std::result::Result<T, APIError>;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum APIError {
  #[error("failed to parse response to request: {0}")]
  ParseError(String),
  #[error("failed to connect to twitch services: {0}")]
  ConnectionError(String),
  #[error("failed to authenticate with the twitch service")]
  Unauthorized,
  #[error("the provided token is missing required scopes")]
  Forbidden,
  #[error("request is missing pagination information")]
  NoPagination,
}

impl From<ureq::Error> for APIError {
  fn from(value: ureq::Error) -> Self {
    use ureq::Error as E;
    match value {
      E::Status(401, _) => APIError::Unauthorized,
      E::Status(403, _) => APIError::Forbidden,
      E::Status(code, _) => APIError::ConnectionError(format!("status code: {code}")),
      E::Transport(_) => APIError::ConnectionError(format!("failed to reach twitch")),
    }
  }
}

impl From<std::io::Error> for APIError {
  fn from(e: std::io::Error) -> Self {
    APIError::ParseError(format!("{e}"))
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum APIEndpoint {
  Token,
  Subscriptions,
  Websocket,
}

impl APIEndpoint {
  pub fn endpoint(&self) -> &'static str {
    match self {
      APIEndpoint::Token => "https://id.twitch.tv/oauth2/token",
      APIEndpoint::Subscriptions => "https://api.twitch.tv/helix/eventsub/subscriptions",
      APIEndpoint::Websocket => "wss://eventsub.wss.twitch.tv/ws",
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Deserialize, Debug)]
struct DataList<T> {
  data:       Vec<T>,
  pagination: Option<Pagination>,
}

#[derive(Deserialize, Debug)]
struct Pagination {
  cursor: Option<String>,
}

pub(super) struct TwitchAPI {
  client_id: String,
  token:     String,
}

impl TwitchAPI {
  pub(super) fn new(client_id: String, token: String) -> Self {
    Self { client_id, token }
  }

  pub(super) fn get<T: DeserializeOwned>(&self, endpoint: APIEndpoint) -> Result<Vec<T>> {
    let mut parsed_subscriptions = Vec::new();
    let mut cursor = String::new();

    loop {
      let mut subscriptions = ureq::get(endpoint.endpoint())
        .set("Authorization", &format!("Bearer {}", self.token))
        .set("Client-Id", &self.client_id)
        .query("after", &cursor)
        .call()?
        .into_json::<DataList<T>>()?;

      parsed_subscriptions.append(&mut subscriptions.data);

      match subscriptions.pagination.ok_or(APIError::NoPagination)?.cursor {
        Some(new_cursor) => cursor = new_cursor,
        None => break,
      }
    }

    Ok(parsed_subscriptions)
  }

  pub(super) fn post<R, T>(&self, endpoint: APIEndpoint, data: T) -> Result<R>
  where
    R: DeserializeOwned,
    T: Serialize,
  {
    let mut response = ureq::post(endpoint.endpoint())
      .set("Authorization", &format!("Bearer {}", self.token))
      .set("Client-Id", &self.client_id)
      .set("Content-Type", "application/json")
      .send_json(data)?
      .into_json::<DataList<R>>()?;

    match response.data.pop() {
      Some(object) => Ok(object),
      None => Err(APIError::ParseError("data missing in response".to_owned())),
    }
  }

  pub(super) fn delete(&self, endpoint: APIEndpoint, id: &str) {
    let delete = ureq::delete(&endpoint.endpoint())
      .set("Authorization", &format!("Bearer {}", self.token))
      .set("Client-Id", &self.client_id)
      .query("id", id)
      .call();

    match delete {
      Ok(_) => (),
      Err(e) => error!("failed to delete object: {e}"),
    }
  }
}
