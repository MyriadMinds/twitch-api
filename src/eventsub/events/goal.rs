use serde::{Deserialize, Serialize};

use super::{User, broadcaster, maybe_string};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GoalBegin {
  pub id:             String,
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster:    User,
  #[serde(rename = "type")]
  pub goal_type:      GoalType,
  pub description:    String,
  #[serde(deserialize_with = "maybe_string")]
  pub current_amount: u64,
  #[serde(deserialize_with = "maybe_string")]
  pub target_amount:  u64,
  pub started_at:     String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GoalProgress {
  pub id:             String,
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster:    User,
  #[serde(rename = "type")]
  pub goal_type:      GoalType,
  pub description:    String,
  #[serde(deserialize_with = "maybe_string")]
  pub current_amount: u64,
  #[serde(deserialize_with = "maybe_string")]
  pub target_amount:  u64,
  pub started_at:     String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GoalEnd {
  pub id:             String,
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster:    User,
  #[serde(rename = "type")]
  pub goal_type:      GoalType,
  pub description:    String,
  #[serde(deserialize_with = "maybe_string")]
  pub is_achieved:    bool,
  #[serde(deserialize_with = "maybe_string")]
  pub current_amount: u64,
  #[serde(deserialize_with = "maybe_string")]
  pub target_amount:  u64,
  pub started_at:     String,
  pub ended_at:       String,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum GoalType {
  Follow,
  Subscription,
  SubscriptionCount,
  NewSubscription,
  NewSubscriptionCount,
  NewBit,
  NewCheerer,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn channel_goal_begin() {
    let event = r##"
    {
      "id": "12345-cool-event",
      "broadcaster_user_id": "141981764",
      "broadcaster_user_name": "TwitchDev",
      "broadcaster_user_login": "twitchdev",
      "type": "subscription",
      "description": "Help me get partner!",
      "current_amount": 100,
      "target_amount": 220,
      "started_at": "2021-07-15T17:16:03.17106713Z"
    }"##;
    serde_json::from_str::<GoalBegin>(event).unwrap();
  }

  #[test]
  fn channel_goal_progress() {
    let event = r##"
    {
      "id": "12345-cool-event",
      "broadcaster_user_id": "141981764",
      "broadcaster_user_name": "TwitchDev",
      "broadcaster_user_login": "twitchdev",
      "type": "subscription",
      "description": "Help me get partner!",
      "current_amount": 120,
      "target_amount": 220,
      "started_at": "2021-07-15T17:16:03.17106713Z"
    }"##;
    serde_json::from_str::<GoalProgress>(event).unwrap();
  }

  #[test]
  fn channel_goal_end() {
    let event = r##"
    {
      "id": "12345-abc-678-defgh",
      "broadcaster_user_id": "141981764",
      "broadcaster_user_name": "TwitchDev",
      "broadcaster_user_login": "twitchdev",
      "type": "subscription",
      "description": "Help me get partner!",
      "is_achieved": false,
      "current_amount": 180,
      "target_amount": 220,
      "started_at": "2021-07-15T17:16:03.17106713Z",
      "ended_at": "2020-07-16T17:16:03.17106713Z"
    }"##;
    serde_json::from_str::<GoalEnd>(event).unwrap();
  }
}
