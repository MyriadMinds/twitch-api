use serde::{Deserialize, Serialize};

use super::{User, broadcaster, host_broadcaster};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SharedChatSessionBegin {
  pub session_id:       String,
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster:      User,
  #[serde(flatten, with = "host_broadcaster")]
  pub host_broadcaster: User,
  pub participants:     Vec<Participant>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SharedChatSessionUpdate {
  pub session_id:       String,
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster:      User,
  #[serde(flatten, with = "host_broadcaster")]
  pub host_broadcaster: User,
  pub participants:     Vec<Participant>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SharedChatSessionEnd {
  pub session_id:       String,
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster:      User,
  #[serde(flatten, with = "host_broadcaster")]
  pub host_broadcaster: User,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Participant {
  pub broadcaster_user_id:    String,
  pub broadcaster_user_name:  String,
  pub broadcaster_user_login: String,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn channel_shared_chat_session_begin() {
    let event = r#"
    {
      "session_id": "2b64a92a-dbb8-424e-b1c3-304423ba1b6f",
      "broadcaster_user_id": "1971641",
      "broadcaster_user_login": "streamer",
      "broadcaster_user_name": "streamer",
      "host_broadcaster_user_id": "1971641",
      "host_broadcaster_user_login": "streamer",
      "host_broadcaster_user_name": "streamer",
      "participants": [
        {
          "broadcaster_user_id": "1971641",
          "broadcaster_user_name": "streamer",
          "broadcaster_user_login": "streamer"
        },
        {
          "broadcaster_user_id": "112233",
          "broadcaster_user_name": "streamer33",
          "broadcaster_user_login": "streamer33"
        }
      ]
    }"#;
    serde_json::from_str::<SharedChatSessionBegin>(event).unwrap();
  }

  #[test]
  fn channel_shared_chat_session_update() {
    let event = r#"
    {
      "session_id": "2b64a92a-dbb8-424e-b1c3-304423ba1b6f",
      "broadcaster_user_id": "1971641",
      "broadcaster_user_login": "streamer",
      "broadcaster_user_name": "streamer",
      "host_broadcaster_user_id": "1971641",
      "host_broadcaster_user_login": "streamer",
      "host_broadcaster_user_name": "streamer",
      "participants": [
        {
          "broadcaster_user_id": "1971641",
          "broadcaster_user_name": "streamer",
          "broadcaster_user_login": "streamer"
        },
        {
          "broadcaster_user_id": "112233",
          "broadcaster_user_name": "streamer33",
          "broadcaster_user_login": "streamer33"
        },
        {
          "broadcaster_user_id": "332211",
          "broadcaster_user_name": "streamer11",
          "broadcaster_user_login": "streamer11"
        }
      ]
    }"#;
    serde_json::from_str::<SharedChatSessionUpdate>(event).unwrap();
  }

  #[test]
  fn channel_shared_chat_session_end() {
    let event = r#"
    {
      "session_id": "2b64a92a-dbb8-424e-b1c3-304423ba1b6f",
      "broadcaster_user_id": "1971641",
      "broadcaster_user_login": "streamer",
      "broadcaster_user_name": "streamer",
      "host_broadcaster_user_id": "1971641",
      "host_broadcaster_user_login": "streamer",
      "host_broadcaster_user_name": "streamer"
    }"#;
    serde_json::from_str::<SharedChatSessionEnd>(event).unwrap();
  }
}
