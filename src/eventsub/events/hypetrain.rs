use serde::{Deserialize, Serialize};

use super::{User, broadcaster, maybe_string};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HypeTrainBegin {
  pub id:                    String,
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster:           User,
  #[serde(deserialize_with = "maybe_string")]
  pub total:                 u64,
  #[serde(deserialize_with = "maybe_string")]
  pub progress:              u64,
  #[serde(deserialize_with = "maybe_string")]
  pub goal:                  u64,
  pub top_contributions:     Vec<Contribution>,
  pub last_contribution:     Contribution,
  #[serde(deserialize_with = "maybe_string")]
  pub level:                 u64,
  pub started_at:            String,
  pub expires_at:            String,
  #[serde(deserialize_with = "maybe_string")]
  pub is_golden_kappa_train: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HypeTrainProgress {
  pub id:                    String,
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster:           User,
  #[serde(deserialize_with = "maybe_string")]
  pub total:                 u64,
  #[serde(deserialize_with = "maybe_string")]
  pub progress:              u64,
  #[serde(deserialize_with = "maybe_string")]
  pub goal:                  u64,
  pub top_contributions:     Vec<Contribution>,
  pub last_contribution:     Contribution,
  #[serde(deserialize_with = "maybe_string")]
  pub level:                 u64,
  pub started_at:            String,
  pub expires_at:            String,
  #[serde(deserialize_with = "maybe_string")]
  pub is_golden_kappa_train: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HypeTrainEnd {
  pub id:                    String,
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster:           User,
  #[serde(deserialize_with = "maybe_string")]
  pub total:                 u64,
  pub top_contributions:     Vec<Contribution>,
  pub level:                 u64,
  pub started_at:            String,
  pub ended_at:              String,
  pub cooldown_ends_at:      String,
  #[serde(deserialize_with = "maybe_string")]
  pub is_golden_kappa_train: bool,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Contribution {
  #[serde(flatten)]
  pub user:              User,
  #[serde(rename = "type")]
  pub contribution_type: ContributionType,
  #[serde(deserialize_with = "maybe_string")]
  pub total:             u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ContributionType {
  Bits,
  Subscription,
  Other,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn channel_hypetrain_begin() {
    let event = r##"
    {
      "id": "1b0AsbInCHZW2SQFQkCzqN07Ib2",
      "broadcaster_user_id": "1337",
      "broadcaster_user_login": "cool_user",
      "broadcaster_user_name": "Cool_User",
      "total": 137,
      "progress": 137,
      "goal": 500,
      "top_contributions": [
        { "user_id": "123", "user_login": "pogchamp", "user_name": "PogChamp", "type": "bits", "total": 50 },
        { "user_id": "456", "user_login": "kappa", "user_name": "Kappa", "type": "subscription", "total": 45 }
      ],
      "last_contribution": { "user_id": "123", "user_login": "pogchamp", "user_name": "PogChamp", "type": "bits", "total": 50 },
      "level": 2,
      "started_at": "2020-07-15T17:16:03.17106713Z",
      "expires_at": "2020-07-15T17:16:11.17106713Z",
      "is_golden_kappa_train": false
    }"##;
    serde_json::from_str::<HypeTrainBegin>(event).unwrap();
  }

  #[test]
  fn channel_hypetrain_progress() {
    let event = r##"
    {
      "id": "1b0AsbInCHZW2SQFQkCzqN07Ib2",
      "broadcaster_user_id": "1337",
      "broadcaster_user_login": "cool_user",
      "broadcaster_user_name": "Cool_User",
      "level": 2,
      "total": 700,
      "progress": 200,
      "goal": 1000,
      "top_contributions": [
        { "user_id": "123", "user_login": "pogchamp", "user_name": "PogChamp", "type": "bits", "total": 50 },
        { "user_id": "456", "user_login": "kappa", "user_name": "Kappa", "type": "subscription", "total": 45 }
      ],
      "last_contribution": { "user_id": "123", "user_login": "pogchamp", "user_name": "PogChamp", "type": "bits", "total": 50 },
      "started_at": "2020-07-15T17:16:03.17106713Z",
      "expires_at": "2020-07-15T17:16:11.17106713Z",
      "is_golden_kappa_train": false
    }"##;
    serde_json::from_str::<HypeTrainProgress>(event).unwrap();
  }

  #[test]
  fn channel_hypetrain_end() {
    let event = r##"
    {
      "id": "1b0AsbInCHZW2SQFQkCzqN07Ib2",
      "broadcaster_user_id": "1337",
      "broadcaster_user_login": "cool_user",
      "broadcaster_user_name": "Cool_User",
      "level": 2,
      "total": 137,
      "top_contributions": [
        { "user_id": "123", "user_login": "pogchamp", "user_name": "PogChamp", "type": "bits", "total": 50 },
        { "user_id": "456", "user_login": "kappa", "user_name": "Kappa", "type": "subscription", "total": 45 }
      ],
      "started_at": "2020-07-15T17:16:03.17106713Z",
      "ended_at": "2020-07-15T17:16:11.17106713Z",
      "cooldown_ends_at": "2020-07-15T18:16:11.17106713Z",
      "is_golden_kappa_train": false
    }"##;
    serde_json::from_str::<HypeTrainEnd>(event).unwrap();
  }
}
