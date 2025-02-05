pub mod events;
mod subscriptions;

use std::net::TcpStream;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;

use events::{Event, Payload, Reconnect};
use log::{error, info, warn};
pub use subscriptions::{Conditions, Raid, Subscription, SubscriptionType};
use thiserror::Error;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{Message, WebSocket};

use super::api::APIEndpoint;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Error, Debug)]
pub enum EventsubError {
  #[error("failed to establish websocket connection: {0}")]
  WebsocketError(tungstenite::Error),
  #[error("message received from twitch was not a session welcome")]
  IncorrectMessage,
  #[error("failed to acquire a welcome message from twitch")]
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
  receiver:       Receiver<Event>,
}

impl Eventsub {
  pub(super) fn new() -> Result<Self, EventsubError> {
    let (mut websocket, _) = tungstenite::connect(APIEndpoint::Websocket.endpoint())?;

    let message = websocket.read()?;
    let Message::Text(message) = message else { return Err(EventsubError::IncorrectMessage) };
    let message = serde_json::from_str::<events::EventsubMessage>(&message)?;
    let Payload::Welcome { session } = message.payload else {
      return Err(EventsubError::IncorrectMessage);
    };

    let session_id = session.id;

    let (sender, receiver) = std::sync::mpsc::channel::<Event>();
    let mut eventsub = EventsubConnection { websocket, sender };

    let _thread = std::thread::spawn(move || {
      eventsub.run();
    });

    Ok(Self { _thread, session_id, receiver })
  }

  pub fn iter(&self) -> std::sync::mpsc::Iter<'_, Event> {
    self.receiver.iter()
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

struct EventsubConnection {
  websocket: WebSocket<MaybeTlsStream<TcpStream>>,
  sender:    Sender<Event>,
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
        warn!("Received invalid websocket frame from eventsub"),
    };
  }

  fn handle_notification(&mut self, message: String) {
    let Ok(message) = serde_json::from_str::<events::EventsubMessage>(&message) else {
      return error!("Failed to parse notification {}", message);
    };

    match message.payload {
      Payload::Notification { subscription: _, event } => self.sender.send(event).unwrap_or(()),
      Payload::Reconnect { session } => self.reconnect(session),
      Payload::Revocation { subscription: _ } => self
        .websocket
        .send(tungstenite::Message::Close(None))
        .unwrap_or(error!("Failed to close websocket on revocation")),
      _ => warn!("Received unknown notification on websocket: {:?}", message),
    }
  }

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  fn reconnect(&mut self, session: Reconnect) {
    info!("Received request to reconnect websocket!");

    match tungstenite::connect(session.reconnect_url) {
      Ok((websocket, _)) => {
        // Process the rest of messages on the old socket before swap
        self.run();
        self.websocket = websocket;
      }
      Err(e) => error!("Failed to reconnect websocket to twitch: {e}"),
    }
  }
}
