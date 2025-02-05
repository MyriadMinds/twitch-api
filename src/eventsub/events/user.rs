use serde::{Deserialize, Serialize};

use super::{User, from, maybe_string, to};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserUpdate {
  #[serde(flatten)]
  pub user:           User,
  pub email:          String,
  #[serde(deserialize_with = "maybe_string")]
  pub email_verified: bool,
  pub description:    String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WhisperReceived {
  #[serde(flatten, with = "from")]
  pub from_user:  User,
  #[serde(flatten, with = "to")]
  pub to_user:    User,
  pub whisper_id: String,
  pub whisper:    Whisper,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Whisper {
  pub text: String,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn user_update() {
    let event = r##"
    {
      "user_id": "1337",
      "user_login": "cool_user",
      "user_name": "Cool_User",
      "email": "user@email.com",
      "email_verified": true,
      "description": "cool description"
    }"##;
    serde_json::from_str::<UserUpdate>(event).unwrap();
  }

  #[test]
  fn whisper_received() {
    let event = r##"
    {
      "from_user_id": "423374343",
      "from_user_login": "glowillig",
      "from_user_name": "glowillig",
      "to_user_id": "424596340",
      "to_user_login": "quotrok",
      "to_user_name": "quotrok",
      "whisper_id": "some-whisper-id",
      "whisper": {
        "text": "a secret"
      }
    }"##;
    serde_json::from_str::<WhisperReceived>(event).unwrap();
  }
}
