use serde::{Deserialize, Serialize};

use super::{MessageSimple, User, broadcaster, maybe_string};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PointsAutomaticRewardRedemption {
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster: User,
  #[serde(flatten)]
  pub user:        User,
  pub id:          String,
  pub reward:      AutomaticReward,
  pub message:     MessageSimple,
  pub user_input:  String,
  pub redeemed_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PointsCustomRewardAdd {
  pub id: String,
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster: User,
  #[serde(deserialize_with = "maybe_string")]
  pub is_enabled: bool,
  #[serde(deserialize_with = "maybe_string")]
  pub is_paused: bool,
  #[serde(deserialize_with = "maybe_string")]
  pub is_in_stock: bool,
  pub title: String,
  #[serde(deserialize_with = "maybe_string")]
  pub cost: u64,
  pub prompt: String,
  #[serde(deserialize_with = "maybe_string")]
  pub is_user_input_required: bool,
  #[serde(deserialize_with = "maybe_string")]
  pub should_redemptions_skip_request_queue: bool,
  pub max_per_stream: MaxPerStream,
  pub max_per_user_per_stream: MaxPerStream,
  pub background_color: String,
  pub image: Option<Image>,
  pub default_image: Image,
  pub global_cooldown: GlobalCooldown,
  pub cooldown_expires_at: Option<String>,
  pub redemptions_redeemed_current_stream: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PointsCustomRewardUpdate {
  pub id: String,
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster: User,
  #[serde(deserialize_with = "maybe_string")]
  pub is_enabled: bool,
  #[serde(deserialize_with = "maybe_string")]
  pub is_paused: bool,
  #[serde(deserialize_with = "maybe_string")]
  pub is_in_stock: bool,
  pub title: String,
  #[serde(deserialize_with = "maybe_string")]
  pub cost: u64,
  pub prompt: String,
  #[serde(deserialize_with = "maybe_string")]
  pub is_user_input_required: bool,
  #[serde(deserialize_with = "maybe_string")]
  pub should_redemptions_skip_request_queue: bool,
  pub max_per_stream: MaxPerStream,
  pub max_per_user_per_stream: MaxPerStream,
  pub background_color: String,
  pub image: Option<Image>,
  pub default_image: Image,
  pub global_cooldown: GlobalCooldown,
  pub cooldown_expires_at: Option<String>,
  pub redemptions_redeemed_current_stream: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PointsCustomRewardRemove {
  pub id: String,
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster: User,
  #[serde(deserialize_with = "maybe_string")]
  pub is_enabled: bool,
  #[serde(deserialize_with = "maybe_string")]
  pub is_paused: bool,
  #[serde(deserialize_with = "maybe_string")]
  pub is_in_stock: bool,
  pub title: String,
  #[serde(deserialize_with = "maybe_string")]
  pub cost: u64,
  pub prompt: String,
  #[serde(deserialize_with = "maybe_string")]
  pub is_user_input_required: bool,
  #[serde(deserialize_with = "maybe_string")]
  pub should_redemptions_skip_request_queue: bool,
  pub max_per_stream: MaxPerStream,
  pub max_per_user_per_stream: MaxPerStream,
  pub background_color: String,
  pub image: Option<Image>,
  pub default_image: Image,
  pub global_cooldown: GlobalCooldown,
  pub cooldown_expires_at: Option<String>,
  pub redemptions_redeemed_current_stream: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PointsCustomRewardRedemptionAdd {
  pub id:          String,
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster: User,
  #[serde(flatten)]
  pub user:        User,
  pub user_input:  String,
  pub status:      RedeemStatus,
  pub reward:      CustomReward,
  pub redeemed_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PointsCustomRewardRedemptionUpdate {
  pub id:          String,
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster: User,
  #[serde(flatten)]
  pub user:        User,
  pub user_input:  String,
  pub status:      RedeemStatus,
  pub reward:      CustomReward,
  pub redeemed_at: String,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutomaticReward {
  #[serde(rename = "type")]
  pub reward_type:    AutomaticRewardType,
  #[serde(deserialize_with = "maybe_string")]
  pub cost:           u64,
  pub unlocked_emote: Option<UnlockedEmote>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum AutomaticRewardType {
  SingleMessageBypassSubMode,
  SendHighlightedMessage,
  RandomSubEmoteUnlock,
  ChosenSubEmoteUnlock,
  ChosenModifiedSubEmoteUnlock,
  MessageEffect,
  GigantifyAnEmote,
  Celebration,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnlockedEmote {
  pub id:   String,
  pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MaxPerStream {
  #[serde(deserialize_with = "maybe_string")]
  pub is_enabled: bool,
  #[serde(deserialize_with = "maybe_string")]
  pub value:      u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Image {
  pub url_1x: String,
  pub url_2x: String,
  pub url_4x: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GlobalCooldown {
  #[serde(deserialize_with = "maybe_string")]
  pub is_enabled: bool,
  #[serde(deserialize_with = "maybe_string")]
  pub seconds:    u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RedeemStatus {
  Unfulfilled,
  Fulfilled,
  Canceled,
  Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomReward {
  pub id:     String,
  pub title:  String,
  #[serde(deserialize_with = "maybe_string")]
  pub cost:   u64,
  pub prompt: String,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn channel_points_automatic_reward_redemption() {
    let event = r#"
    {
      "broadcaster_user_id": "12826",
      "broadcaster_user_name": "Twitch",
      "broadcaster_user_login": "twitch",
      "user_id": "141981764",
      "user_name": "TwitchDev",
      "user_login": "twitchdev",
      "id": "f024099a-e0fe-4339-9a0a-a706fb59f353",
      "reward": {
        "type": "send_highlighted_message",
        "cost": 100,
        "unlocked_emote": null
      },
      "message": {
        "text": "Hello world! VoHiYo",
        "emotes": [
          {
            "id": "81274",
            "begin": 13,
            "end": 18
          }
        ]
      },
      "user_input": "Hello world! VoHiYo ",
      "redeemed_at": "2024-02-23T21:14:34.260398045Z"
    }"#;
    serde_json::from_str::<PointsAutomaticRewardRedemption>(event).unwrap();
  }

  #[test]
  fn channel_points_custom_reward_add() {
    let event = r##"
    {
      "id": "9001",
      "broadcaster_user_id": "1337",
      "broadcaster_user_login": "cool_user",
      "broadcaster_user_name": "Cool_User",
      "is_enabled": true,
      "is_paused": false,
      "is_in_stock": true,
      "title": "Cool Reward",
      "cost": 100,
      "prompt": "reward prompt",
      "is_user_input_required": true,
      "should_redemptions_skip_request_queue": false,
      "cooldown_expires_at": null,
      "redemptions_redeemed_current_stream": null,
      "max_per_stream": {
          "is_enabled": true,
          "value": 1000
      },
      "max_per_user_per_stream": {
          "is_enabled": true,
          "value": 1000
      },
      "global_cooldown": {
          "is_enabled": true,
          "seconds": 1000
      },
      "background_color": "#FA1ED2",
      "image": {
          "url_1x": "https://static-cdn.jtvnw.net/image-1.png",
          "url_2x": "https://static-cdn.jtvnw.net/image-2.png",
          "url_4x": "https://static-cdn.jtvnw.net/image-4.png"
      },
      "default_image": {
          "url_1x": "https://static-cdn.jtvnw.net/default-1.png",
          "url_2x": "https://static-cdn.jtvnw.net/default-2.png",
          "url_4x": "https://static-cdn.jtvnw.net/default-4.png"
      }
    }"##;
    serde_json::from_str::<PointsCustomRewardAdd>(event).unwrap();
  }

  #[test]
  fn channel_points_custom_reward_update() {
    let event = r##"
    {
      "id": "9001",
      "broadcaster_user_id": "1337",
      "broadcaster_user_login": "cool_user",
      "broadcaster_user_name": "Cool_User",
      "is_enabled": true,
      "is_paused": false,
      "is_in_stock": true,
      "title": "Cool Reward",
      "cost": 100,
      "prompt": "reward prompt",
      "is_user_input_required": true,
      "should_redemptions_skip_request_queue": false,
      "cooldown_expires_at": "2019-11-16T10:11:12.634234626Z",
      "redemptions_redeemed_current_stream": 123,
      "max_per_stream": {
          "is_enabled": true,
          "value": 1000
      },
      "max_per_user_per_stream": {
          "is_enabled": true,
          "value": 1000
      },
      "global_cooldown": {
          "is_enabled": true,
          "seconds": 1000
      },
      "background_color": "#FA1ED2",
      "image": {
          "url_1x": "https://static-cdn.jtvnw.net/image-1.png",
          "url_2x": "https://static-cdn.jtvnw.net/image-2.png",
          "url_4x": "https://static-cdn.jtvnw.net/image-4.png"
      },
      "default_image": {
          "url_1x": "https://static-cdn.jtvnw.net/default-1.png",
          "url_2x": "https://static-cdn.jtvnw.net/default-2.png",
          "url_4x": "https://static-cdn.jtvnw.net/default-4.png"
      }
    }"##;
    serde_json::from_str::<PointsCustomRewardUpdate>(event).unwrap();
  }

  #[test]
  fn channel_points_custom_reward_remove() {
    let event = r##"
    {
      "id": "9001",
      "broadcaster_user_id": "1337",
      "broadcaster_user_login": "cool_user",
      "broadcaster_user_name": "Cool_User",
      "is_enabled": true,
      "is_paused": false,
      "is_in_stock": true,
      "title": "Cool Reward",
      "cost": 100,
      "prompt": "reward prompt",
      "is_user_input_required": true,
      "should_redemptions_skip_request_queue": false,
      "cooldown_expires_at": "2019-11-16T10:11:12.634234626Z",
      "redemptions_redeemed_current_stream": 123,
      "max_per_stream": {
          "is_enabled": true,
          "value": 1000
      },
      "max_per_user_per_stream": {
          "is_enabled": true,
          "value": 1000
      },
      "global_cooldown": {
          "is_enabled": true,
          "seconds": 1000
      },
      "background_color": "#FA1ED2",
      "image": {
          "url_1x": "https://static-cdn.jtvnw.net/image-1.png",
          "url_2x": "https://static-cdn.jtvnw.net/image-2.png",
          "url_4x": "https://static-cdn.jtvnw.net/image-4.png"
      },
      "default_image": {
          "url_1x": "https://static-cdn.jtvnw.net/default-1.png",
          "url_2x": "https://static-cdn.jtvnw.net/default-2.png",
          "url_4x": "https://static-cdn.jtvnw.net/default-4.png"
      }
    }"##;
    serde_json::from_str::<PointsCustomRewardRemove>(event).unwrap();
  }

  #[test]
  fn channel_points_custom_reward_redemption_add() {
    let event = r##"
    {
      "id": "17fa2df1-ad76-4804-bfa5-a40ef63efe63",
      "broadcaster_user_id": "1337",
      "broadcaster_user_login": "cool_user",
      "broadcaster_user_name": "Cool_User",
      "user_id": "9001",
      "user_login": "cooler_user",
      "user_name": "Cooler_User",
      "user_input": "pogchamp",
      "status": "unfulfilled",
      "reward": {
          "id": "92af127c-7326-4483-a52b-b0da0be61c01",
          "title": "title",
          "cost": 100,
          "prompt": "reward prompt"
      },
      "redeemed_at": "2020-07-15T17:16:03.17106713Z"
    }"##;
    serde_json::from_str::<PointsCustomRewardRedemptionAdd>(event).unwrap();
  }

  #[test]
  fn channel_points_custom_reward_redemption_update() {
    let event = r##"
    {
      "id": "17fa2df1-ad76-4804-bfa5-a40ef63efe63",
      "broadcaster_user_id": "1337",
      "broadcaster_user_login": "cool_user",
      "broadcaster_user_name": "Cool_User",
      "user_id": "9001",
      "user_login": "cooler_user",
      "user_name": "Cooler_User",
      "user_input": "pogchamp",
      "status": "fulfilled",
      "reward": {
          "id": "92af127c-7326-4483-a52b-b0da0be61c01",
          "title": "title",
          "cost": 100,
          "prompt": "reward prompt"
      },
      "redeemed_at": "2020-07-15T17:16:03.17106713Z"
    }"##;
    serde_json::from_str::<PointsCustomRewardRedemptionUpdate>(event).unwrap();
  }
}
