mod api;
mod eventsub;

#[cfg(feature = "token-helpers")]
mod token;

use api::{APIEndpoint, APIError, TwitchAPI};
pub use eventsub::{Conditions, Eventsub, EventsubError, Subscription, SubscriptionType};
use thiserror::Error;
#[cfg(feature = "token-helpers")]
pub use token::{Scope, get_access_token, get_refresh_token};
use twitch_eventsub_structs::{EventSubscription, NewAccessTokenResponse};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Error, Debug)]
pub enum TwitchError {
  #[error("failed to parse response: {0}")]
  ParseError(serde_json::Error),
  #[error("failed to establish eventsub connection: {0}")]
  EventsubError(EventsubError),
  #[error("failed to make a request: {0}")]
  APIError(APIError),
}

impl From<APIError> for TwitchError {
  fn from(v: APIError) -> Self {
    Self::APIError(v)
  }
}

impl From<EventsubError> for TwitchError {
  fn from(v: EventsubError) -> Self {
    Self::EventsubError(v)
  }
}

pub struct Twitch {
  api: TwitchAPI,
}

impl Twitch {
  pub fn new(client_id: String, access_token: String) -> Self {
    Self { api: TwitchAPI::new(client_id, access_token) }
  }

  pub fn authenticate(
    client_id: &str,
    client_secret: &str,
    refresh_token: &str,
  ) -> Result<(String, String), TwitchError> {
    let request_body = format!(
      "client_id={}&client_secret={}&grant_type=refresh_token&refresh_token={}",
      client_id, client_secret, refresh_token
    );

    let response = ureq::post(APIEndpoint::Token.endpoint())
      .set("Content-Type", "application/x-www-form-urlencoded")
      .send_string(&request_body)
      .map_err(|e| APIError::from(e))?
      .into_json::<NewAccessTokenResponse>()
      .map_err(|e| APIError::from(e))?;

    Ok((
      response.access_token,
      response
        .refresh_token
        .ok_or(APIError::ConnectionError(format!("refresh token missing from response")))?,
    ))
  }

  pub fn connect_eventsub() -> Result<Eventsub, TwitchError> {
    Ok(Eventsub::new()?)
  }

  pub fn create_eventsub_subscription(
    &self,
    subscription: Subscription,
  ) -> Result<EventSubscription, TwitchError> {
    Ok(self.api.post::<EventSubscription, _>(APIEndpoint::Subscriptions, subscription)?)
  }
}

// #[cfg(test)]
// mod tests {
//   use super::*;

//   #[test]
//   fn test() {
//     let scopes = Scope::UserReadChat | Scope::BitsRead | SubscriptionType::ChatClear.into();

//     let client_id = std::env::var("CLIENT_ID").unwrap();

//     let tokens = get_access_token(client_id, scopes);
//     println!("{tokens:#?}");
//   }
// }
