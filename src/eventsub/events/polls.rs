use serde::{Deserialize, Serialize};

use super::{User, broadcaster, maybe_string};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PollBegin {
  pub id:                    String,
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster:           User,
  pub title:                 String,
  pub choices:               Vec<Choice>,
  pub bits_voting:           ExtraVotes,
  pub channel_points_voting: ExtraVotes,
  pub started_at:            String,
  pub ends_at:               String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PollProgress {
  pub id:                    String,
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster:           User,
  pub title:                 String,
  pub choices:               Vec<Choice>,
  pub bits_voting:           ExtraVotes,
  pub channel_points_voting: ExtraVotes,
  pub started_at:            String,
  pub ends_at:               String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PollEnd {
  pub id:                    String,
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster:           User,
  pub title:                 String,
  pub choices:               Vec<Choice>,
  pub bits_voting:           ExtraVotes,
  pub channel_points_voting: ExtraVotes,
  pub status:                PollStatus,
  pub started_at:            String,
  pub ended_at:              String,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Choice {
  pub id:                   String,
  pub title:                String,
  #[serde(deserialize_with = "maybe_string", default)]
  pub bits_votes:           u64,
  #[serde(deserialize_with = "maybe_string", default)]
  pub channel_points_votes: u64,
  #[serde(deserialize_with = "maybe_string", default)]
  pub votes:                u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtraVotes {
  #[serde(deserialize_with = "maybe_string")]
  pub is_enabled:      bool,
  #[serde(deserialize_with = "maybe_string")]
  pub amount_per_vote: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum PollStatus {
  Completed,
  Archived,
  Terminated,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn channel_poll_begin() {
    let event = r##"
    {
      "id": "1243456",
      "broadcaster_user_id": "1337",
      "broadcaster_user_login": "cool_user",
      "broadcaster_user_name": "Cool_User",
      "title": "Aren’t shoes just really hard socks?",
      "choices": [
          {"id": "123", "title": "Yeah!"},
          {"id": "124", "title": "No!"},
          {"id": "125", "title": "Maybe!"}
      ],
      "bits_voting": {
          "is_enabled": true,
          "amount_per_vote": 10
      },
      "channel_points_voting": {
          "is_enabled": true,
          "amount_per_vote": 10
      },
      "started_at": "2020-07-15T17:16:03.17106713Z",
      "ends_at": "2020-07-15T17:16:08.17106713Z"
    }"##;
    serde_json::from_str::<PollBegin>(event).unwrap();
  }

  #[test]
  fn channel_poll_progress() {
    let event = r##"
    {
      "id": "1243456",
      "broadcaster_user_id": "1337",
      "broadcaster_user_login": "cool_user",
      "broadcaster_user_name": "Cool_User",
      "title": "Aren’t shoes just really hard socks?",
      "choices": [
          {"id": "123", "title": "Yeah!", "bits_votes": 5, "channel_points_votes": 7, "votes": 12},
          {"id": "124", "title": "No!", "bits_votes": 10, "channel_points_votes": 4, "votes": 14},
          {"id": "125", "title": "Maybe!", "bits_votes": 0, "channel_points_votes": 7, "votes": 7}
      ],
      "bits_voting": {
          "is_enabled": true,
          "amount_per_vote": 10
      },
      "channel_points_voting": {
          "is_enabled": true,
          "amount_per_vote": 10
      },
      "started_at": "2020-07-15T17:16:03.17106713Z",
      "ends_at": "2020-07-15T17:16:08.17106713Z"
    }"##;
    serde_json::from_str::<PollProgress>(event).unwrap();
  }

  #[test]
  fn channel_poll_end() {
    let event = r##"
    {
      "id": "1243456",
      "broadcaster_user_id": "1337",
      "broadcaster_user_login": "cool_user",
      "broadcaster_user_name": "Cool_User",
      "title": "Aren’t shoes just really hard socks?",
      "choices": [
          {"id": "123", "title": "Blue", "bits_votes": 50, "channel_points_votes": 70, "votes": 120},
          {"id": "124", "title": "Yellow", "bits_votes": 100, "channel_points_votes": 40, "votes": 140},
          {"id": "125", "title": "Green", "bits_votes": 10, "channel_points_votes": 70, "votes": 80}
      ],
      "bits_voting": {
          "is_enabled": true,
          "amount_per_vote": 10
      },
      "channel_points_voting": {
          "is_enabled": true,
          "amount_per_vote": 10
      },
      "status": "completed",
      "started_at": "2020-07-15T17:16:03.17106713Z",
      "ended_at": "2020-07-15T17:16:11.17106713Z"
    }"##;
    serde_json::from_str::<PollEnd>(event).unwrap();
  }
}
