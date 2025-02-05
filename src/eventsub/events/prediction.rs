use serde::{Deserialize, Serialize};

use super::{User, broadcaster, maybe_string};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PredictionBegin {
  pub id:          String,
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster: User,
  pub title:       String,
  pub outcomes:    Vec<OutcomeInfo>,
  pub started_at:  String,
  pub locks_at:    String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PredictionProgress {
  pub id:          String,
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster: User,
  pub title:       String,
  pub outcomes:    Vec<Outcome>,
  pub started_at:  String,
  pub locks_at:    String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PredictionLock {
  pub id:          String,
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster: User,
  pub title:       String,
  pub outcomes:    Vec<Outcome>,
  pub started_at:  String,
  pub locked_at:   String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PredictionEnd {
  pub id:                 String,
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster:        User,
  pub title:              String,
  #[serde(deserialize_with = "maybe_string", default)]
  pub winning_outcome_id: u64,
  pub outcomes:           Vec<Outcome>,
  pub status:             PredictionStatus,
  pub started_at:         String,
  pub ended_at:           String,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutcomeInfo {
  pub id:    String,
  pub title: String,
  pub color: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Outcome {
  pub id:             String,
  pub title:          String,
  pub color:          String,
  #[serde(deserialize_with = "maybe_string", default)]
  pub users:          u64,
  #[serde(deserialize_with = "maybe_string", default)]
  pub channel_points: u64,
  pub top_predictors: Vec<Predictor>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Predictor {
  #[serde(flatten)]
  pub user:                User,
  pub channel_points_won:  Option<u64>,
  #[serde(deserialize_with = "maybe_string")]
  pub channel_points_used: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum PredictionStatus {
  Resolved,
  Canceled,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn channel_prediction_begin() {
    let event = r##"
    {
      "id": "1243456",
      "broadcaster_user_id": "1337",
      "broadcaster_user_login": "cool_user",
      "broadcaster_user_name": "Cool_User",
      "title": "Aren’t shoes just really hard socks?",
      "outcomes": [
        {"id": "1243456", "title": "Yeah!", "color": "blue"},
        {"id": "2243456", "title": "No!", "color": "pink"}
      ],
      "started_at": "2020-07-15T17:16:03.17106713Z",
      "locks_at": "2020-07-15T17:21:03.17106713Z"
    }"##;
    serde_json::from_str::<PredictionBegin>(event).unwrap();
  }

  #[test]
  fn channel_prediction_progress() {
    let event = r##"
    {
      "id": "1243456",
      "broadcaster_user_id": "1337",
      "broadcaster_user_login": "cool_user",
      "broadcaster_user_name": "Cool_User",
      "title": "Aren’t shoes just really hard socks?",
      "outcomes": [
          {
          "id": "1243456",
          "title": "Yeah!",
          "color": "blue",
          "users": 10,
          "channel_points": 15000,
          "top_predictors": [
            {
              "user_name": "Cool_User",
              "user_login": "cool_user",
              "user_id": "1234",
              "channel_points_won": null,
              "channel_points_used": 500
            },
            {
              "user_name": "Coolest_User",
              "user_login": "coolest_user",
              "user_id": "1236",
              "channel_points_won": null,
              "channel_points_used": 200
            }
          ]
        },
        {
          "id": "2243456",
          "title": "No!",
          "color": "pink",
          "top_predictors": [
            {
              "user_name": "Cooler_User",
              "user_login": "cooler_user",
              "user_id": "12345",
              "channel_points_won": null,
              "channel_points_used": 5000
            }
          ]
        }
      ],
      "started_at": "2020-07-15T17:16:03.17106713Z",
      "locks_at": "2020-07-15T17:21:03.17106713Z"
    }"##;
    serde_json::from_str::<PredictionProgress>(event).unwrap();
  }

  #[test]
  fn channel_prediction_lock() {
    let event = r##"
    {
      "id": "1243456",
      "broadcaster_user_id": "1337",
      "broadcaster_user_login": "cool_user",
      "broadcaster_user_name": "Cool_User",
      "title": "Aren’t shoes just really hard socks?",
      "outcomes": [
        {
          "id": "1243456",
          "title": "Yeah!",
          "color": "blue",
          "users": 10,
          "channel_points": 15000,
          "top_predictors": [
            {
              "user_name": "Cool_User",
              "user_login": "cool_user",
              "user_id": "1234",
              "channel_points_won": null,
              "channel_points_used": 500
            },
            {
              "user_name": "Coolest_User",
              "user_login": "coolest_user",
              "user_id": "1236",
              "channel_points_won": null,
              "channel_points_used": 200
            }
          ]
        },
        {
          "id": "2243456",
          "title": "No!",
          "color": "pink",
          "top_predictors": [
            {
              "user_name": "Cooler_User",
              "user_login": "cooler_user",
              "user_id": "12345",
              "channel_points_won": null,
              "channel_points_used": 5000
            }
          ]
        }
      ],
      "started_at": "2020-07-15T17:16:03.17106713Z",
      "locked_at": "2020-07-15T17:21:03.17106713Z"
    }"##;
    serde_json::from_str::<PredictionLock>(event).unwrap();
  }

  #[test]
  fn channel_prediction_end() {
    let event = r##"
    {
      "id": "1243456",
      "broadcaster_user_id": "1337",
      "broadcaster_user_login": "cool_user",
      "broadcaster_user_name": "Cool_User",
      "title": "Aren’t shoes just really hard socks?",
      "winning_outcome_id": "12345",
      "outcomes": [
        {
          "id": "12345",
          "title": "Yeah!",
          "color": "blue",
          "users": 2,
          "channel_points": 15000,
          "top_predictors": [
            {
              "user_name": "Cool_User",
              "user_login": "cool_user",
              "user_id": "1234",
              "channel_points_won": 10000,
              "channel_points_used": 500
            },
            {
              "user_name": "Coolest_User",
              "user_login": "coolest_user",
              "user_id": "1236",
              "channel_points_won": 5000,
              "channel_points_used": 100
            }
          ]
        },
        {
          "id": "22435",
          "title": "No!",
          "users": 2,
          "channel_points": 200,
          "color": "pink",
          "top_predictors": [
            {
              "user_name": "Cooler_User",
              "user_login": "cooler_user",
              "user_id": "12345",
              "channel_points_won": null,
              "channel_points_used": 100
            },
            {
              "user_name": "Elite_User",
              "user_login": "elite_user",
              "user_id": "1337",
              "channel_points_won": null,
              "channel_points_used": 100
            }
          ]
        }
      ],
      "status": "resolved",
      "started_at": "2020-07-15T17:16:03.17106713Z",
      "ended_at": "2020-07-15T17:16:11.17106713Z"
    }"##;
    serde_json::from_str::<PredictionEnd>(event).unwrap();
  }
}
