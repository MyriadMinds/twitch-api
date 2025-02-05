use serde::{Deserialize, Serialize};

use super::{MessageSimple, User, broadcaster, maybe_string};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Subscribe {
  #[serde(flatten)]
  pub user:        User,
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster: User,
  pub tier:        String,
  #[serde(deserialize_with = "maybe_string")]
  pub is_gift:     bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscriptionEnd {
  #[serde(flatten)]
  pub user:        User,
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster: User,
  pub tier:        String,
  #[serde(deserialize_with = "maybe_string")]
  pub is_gift:     bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscriptionGift {
  #[serde(flatten)]
  pub user:             User,
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster:      User,
  #[serde(deserialize_with = "maybe_string")]
  pub total:            u32,
  pub tier:             String,
  pub cumulative_total: Option<u32>,
  #[serde(deserialize_with = "maybe_string")]
  pub is_anonymous:     bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscriptionMessage {
  #[serde(flatten)]
  pub user:              User,
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster:       User,
  pub tier:              String,
  pub message:           MessageSimple,
  #[serde(deserialize_with = "maybe_string")]
  pub cumulative_months: u32,
  pub streak_months:     Option<u32>,
  #[serde(deserialize_with = "maybe_string")]
  pub duration_months:   u32,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn channel_subscribe() {
    let event = r#"
    {
      "user_id": "1234",
      "user_login": "cool_user",
      "user_name": "Cool_User",
      "broadcaster_user_id": "1337",
      "broadcaster_user_login": "cooler_user",
      "broadcaster_user_name": "Cooler_User",
      "tier": "1000",
      "is_gift": false
    }"#;
    serde_json::from_str::<Subscribe>(event).unwrap();
  }

  #[test]
  fn channel_subscribtion_end() {
    let event = r#"
    {
      "user_id": "1234",
      "user_login": "cool_user",
      "user_name": "Cool_User",
      "broadcaster_user_id": "1337",
      "broadcaster_user_login": "cooler_user",
      "broadcaster_user_name": "Cooler_User",
      "tier": "1000",
      "is_gift": false
    }"#;
    serde_json::from_str::<SubscriptionEnd>(event).unwrap();
  }

  #[test]
  fn channel_subscribtion_gift() {
    let event = r#"
    {
      "user_id": "1234",
      "user_login": "cool_user",
      "user_name": "Cool_User",
      "broadcaster_user_id": "1337",
      "broadcaster_user_login": "cooler_user",
      "broadcaster_user_name": "Cooler_User",
      "total": 2,
      "tier": "1000",
      "cumulative_total": 284,
      "is_anonymous": false
    }"#;
    serde_json::from_str::<SubscriptionGift>(event).unwrap();
  }

  #[test]
  fn channel_subscribtion_message() {
    let event = r#"
    {
      "user_id": "1234",
      "user_login": "cool_user",
      "user_name": "Cool_User",
      "broadcaster_user_id": "1337",
      "broadcaster_user_login": "cooler_user",
      "broadcaster_user_name": "Cooler_User",
      "tier": "1000",
      "message": {
        "text": "Love the stream! FevziGG",
        "emotes": [
          {
            "begin": 23,
            "end": 30,
            "id": "302976485"
          }
        ]
      },
      "cumulative_months": 15,
      "streak_months": 1,
      "duration_months": 6
    }"#;
    serde_json::from_str::<SubscriptionMessage>(event).unwrap();
  }
}
