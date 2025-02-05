use serde::{Deserialize, Serialize};

use super::{DonationAmount, User, broadcaster};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharityDonation {
  pub id:                  String,
  pub campaign_id:         String,
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster:         User,
  #[serde(flatten)]
  pub user:                User,
  pub charity_name:        String,
  pub charity_description: String,
  pub charity_logo:        String,
  pub charity_website:     String,
  pub amount:              DonationAmount,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharityCampaignStart {
  pub id:                  String,
  pub broadcaster_id:      String,
  pub broadcaster_login:   String,
  pub broadcaster_name:    String,
  pub charity_name:        String,
  pub charity_description: String,
  pub charity_logo:        String,
  pub charity_website:     String,
  pub current_amount:      DonationAmount,
  pub target_amount:       DonationAmount,
  pub started_at:          String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharityCampaignProgress {
  pub id:                  String,
  pub broadcaster_id:      String,
  pub broadcaster_login:   String,
  pub broadcaster_name:    String,
  pub charity_name:        String,
  pub charity_description: String,
  pub charity_logo:        String,
  pub charity_website:     String,
  pub current_amount:      DonationAmount,
  pub target_amount:       DonationAmount,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharityCampaignStop {
  pub id:                  String,
  pub broadcaster_id:      String,
  pub broadcaster_login:   String,
  pub broadcaster_name:    String,
  pub charity_name:        String,
  pub charity_description: String,
  pub charity_logo:        String,
  pub charity_website:     String,
  pub current_amount:      DonationAmount,
  pub target_amount:       DonationAmount,
  pub stopped_at:          String,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn channel_charity_donation() {
    let event = r##"
    {
      "id": "a1b2c3-aabb-4455-d1e2f3",
      "campaign_id": "123-abc-456-def",
      "broadcaster_user_id": "123456",
      "broadcaster_user_name": "SunnySideUp",
      "broadcaster_user_login": "sunnysideup",
      "user_id": "654321",
      "user_login": "generoususer1",
      "user_name": "GenerousUser1",
      "charity_name": "Example name",
      "charity_description": "Example description",
      "charity_logo": "https://abc.cloudfront.net/ppgf/1000/100.png",
      "charity_website": "https://www.example.com",
      "amount": {
        "value": 10000,
        "decimal_places": 2,
        "currency": "USD"
      }
    }"##;
    serde_json::from_str::<CharityDonation>(event).unwrap();
  }

  #[test]
  fn channel_charity_campaign_start() {
    let event = r##"
    {
      "id": "123-abc-456-def",
      "broadcaster_id": "123456",
      "broadcaster_name": "SunnySideUp",
      "broadcaster_login": "sunnysideup",
      "charity_name": "Example name",
      "charity_description": "Example description",
      "charity_logo": "https://abc.cloudfront.net/ppgf/1000/100.png",
      "charity_website": "https://www.example.com",
      "current_amount": {
        "value": 0,
        "decimal_places": 2,
        "currency": "USD"
      },
      "target_amount": {
        "value": 1500000,
        "decimal_places": 2,
        "currency": "USD"
      },
      "started_at": "2022-07-26T17:00:03.17106713Z"
    }"##;
    serde_json::from_str::<CharityCampaignStart>(event).unwrap();
  }

  #[test]
  fn channel_charity_campaign_progress() {
    let event = r##"
    {
      "id": "123-abc-456-def",
      "broadcaster_id": "123456",
      "broadcaster_name": "SunnySideUp",
      "broadcaster_login": "sunnysideup",
      "charity_name": "Example name",
      "charity_description": "Example description",
      "charity_logo": "https://abc.cloudfront.net/ppgf/1000/100.png",
      "charity_website": "https://www.example.com",
      "current_amount": {
        "value": 260000,
        "decimal_places": 2,
        "currency": "USD"
      },
      "target_amount": {
        "value": 1500000,
        "decimal_places": 2,
        "currency": "USD"
      }
    }"##;
    serde_json::from_str::<CharityCampaignProgress>(event).unwrap();
  }

  #[test]
  fn channel_charity_campaign_end() {
    let event = r##"
    {
      "id": "123-abc-456-def",
      "broadcaster_id": "123456",
      "broadcaster_name": "SunnySideUp",
      "broadcaster_login": "sunnysideup",
      "charity_name": "Example name",
      "charity_description": "Example description",
      "charity_logo": "https://abc.cloudfront.net/ppgf/1000/100.png",
      "charity_website": "https://www.example.com",
      "current_amount": {
        "value": 1450000,
        "decimal_places": 2,
        "currency": "USD"
      },
      "target_amount": {
        "value": 1500000,
        "decimal_places": 2,
        "currency": "USD"
      },
      "stopped_at": "2022-07-26T22:00:03.17106713Z"
    }"##;
    serde_json::from_str::<CharityCampaignStop>(event).unwrap();
  }
}
