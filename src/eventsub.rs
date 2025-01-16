mod subscriptions;

use std::net::TcpStream;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;

use log::{error, info, warn};
pub use subscriptions::{Conditions, Subscription, SubscriptionType};
use thiserror::Error;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{Message, WebSocket};
use twitch_eventsub_structs::GenericMessage;

use super::api::APIEndpoint;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Error, Debug)]
pub enum EventsubError {
  #[error("failed to establish websocket connection: {0}")]
  WebsocketError(tungstenite::Error),
  #[error("message received from twitch was not a session welcome")]
  IncorrectMessage,
  #[error("message from twitch did not contain session ID")]
  NoSessionID,
  #[error("failed to parse incoming notification: {0}")]
  ParseError(serde_json::Error),
}

impl From<serde_json::Error> for EventsubError {
  fn from(v: serde_json::Error) -> Self {
    Self::ParseError(v)
  }
}

impl From<tungstenite::Error> for EventsubError {
  fn from(e: tungstenite::Error) -> Self {
    Self::WebsocketError(e)
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Eventsub {
  _thread:        JoinHandle<()>,
  pub session_id: String,
  receiver:       Receiver<GenericMessage>,
}

impl Eventsub {
  pub(super) fn new() -> Result<Self, EventsubError> {
    let (mut websocket, _) = tungstenite::connect(APIEndpoint::Websocket.endpoint())?;

    let message = websocket.read()?;
    let Message::Text(message) = message else { return Err(EventsubError::IncorrectMessage) };
    let message = serde_json::from_str::<GenericMessage>(&message)?;

    let session_id =
      message.payload.and_then(|p| p.session).map(|s| s.id).ok_or(EventsubError::NoSessionID)?;

    let (sender, receiver) = std::sync::mpsc::channel::<GenericMessage>();
    let mut eventsub = EventsubConnection { websocket, sender };

    let _thread = std::thread::spawn(move || {
      eventsub.run();
    });

    Ok(Self { _thread, session_id, receiver })
  }

  pub fn iter(&self) -> std::sync::mpsc::Iter<'_, GenericMessage> {
    self.receiver.iter()
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

struct EventsubConnection {
  websocket: WebSocket<MaybeTlsStream<TcpStream>>,
  sender:    Sender<GenericMessage>,
}

impl EventsubConnection {
  pub(super) fn run(&mut self) {
    loop {
      use tungstenite::Error as TE;
      match self.websocket.read() {
        Ok(message) => self.handle_message(message),
        Err(TE::ConnectionClosed | TE::AlreadyClosed) =>
          break info!("Websocket connection closing"),
        Err(e) => break error!("Websocket connection error: {}", e),
      };
    }
  }

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  fn handle_message(&mut self, message: Message) {
    use tungstenite::Message as TM;
    match message {
      TM::Text(text) => self.handle_notification(text),
      TM::Ping(_) => (),
      TM::Close(Some(message)) => info!("Close request received, reason: {}", message.reason),
      TM::Close(None) => info!("Close request received"),
      TM::Pong(_) | TM::Binary(_) | TM::Frame(_) =>
        info!("Received invalid message type over websocket"),
    };
  }

  fn handle_notification(&mut self, message: String) {
    let Ok(message) = serde_json::from_str::<GenericMessage>(&message) else {
      return error!("Failed to parse notification {}", message);
    };

    match message.metadata.message_type.as_str() {
      "notification" => self.sender.send(message).unwrap_or(()),
      "session_reconnect" => self.reconnect(message),
      "session_keepalive" => (),
      "revocation" => self
        .websocket
        .send(tungstenite::Message::Close(None))
        .unwrap_or(error!("Failed to close websocket on revocation")),
      _ => warn!("Received unknown notification on websocket: {:?}", message),
    }
  }

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  fn reconnect(&mut self, message: GenericMessage) {
    info!("Received request to reconnect websocket!");
    let Some(url) = message.payload.and_then(|p| p.session).and_then(|s| s.reconnect_url) else {
      return error!("Request doesn't contain reconnection info!");
    };

    match tungstenite::connect(url) {
      Ok((websocket, _)) => {
        // Process the rest of messages on the old socket before swap
        self.run();
        self.websocket = websocket;
      }
      Err(e) => error!("Failed to reconnect websocket to twitch: {e}"),
    }
  }
}
