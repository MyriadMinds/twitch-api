use serde::de::Error;
use serde::{Deserialize, Serialize};

use super::{
  DonationAmount, Message, UpdateStatus, User, broadcaster, chatter, maybe_string, parent, target,
  thread,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatClear {
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster: User,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatClearUserMessages {
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster: User,
  #[serde(flatten, with = "target")]
  pub target:      User,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessage {
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster: User,
  #[serde(flatten, with = "chatter")]
  pub chatter: User,
  pub message_id: String,
  pub message: Message,
  pub message_type: MessageType,
  pub badges: Vec<Badge>,
  pub cheer: Option<CheerInfo>,
  pub color: String,
  pub reply: Option<Reply>,
  pub channel_points_custom_reward_id: Option<String>,
  #[serde(flatten)]
  pub source: SharedMessage,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessageDelete {
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster: User,
  #[serde(flatten, with = "target")]
  pub target:      User,
  pub message_id:  String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatNotification {
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster:    User,
  #[serde(flatten, with = "chatter")]
  pub chatter:        User,
  pub color:          String,
  pub badges:         Vec<Badge>,
  pub system_message: String,
  pub message_id:     String,
  pub message:        Message,
  #[serde(flatten)]
  pub notice_type:    NoticeType,
  #[serde(flatten)]
  pub source:         SharedMessage,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatSettingsUpdate {
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster:                    User,
  #[serde(deserialize_with = "maybe_string")]
  pub emote_mode:                     bool,
  pub follower_mode_duration_minutes: Option<u32>,
  pub slow_mode_wait_time_seconds:    Option<u32>,
  #[serde(deserialize_with = "maybe_string")]
  pub subscriber_mode:                bool,
  #[serde(deserialize_with = "maybe_string")]
  pub unique_chat_mode:               bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatUserMessageHold {
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster: User,
  #[serde(flatten)]
  pub user:        User,
  pub message_id:  String,
  pub message:     Message,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatUserMessageUpdate {
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster: User,
  #[serde(flatten)]
  pub user:        User,
  pub status:      UpdateStatus,
  pub message_id:  String,
  pub message:     Message,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
  Text,
  ChannelPointsHighlighted,
  ChannelPointsSubOnly,
  UserInto,
  PowerUpsMessageEffect,
  PowerUpsGigantifiedEmote,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CheerInfo {
  #[serde(deserialize_with = "maybe_string")]
  pub bits: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reply {
  #[serde(flatten, with = "parent")]
  pub parent:              User,
  pub parent_message_id:   String,
  pub parent_message_body: String,
  #[serde(flatten, with = "thread")]
  pub thread:              User,
  pub thread_message_id:   String,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Badge {
  pub set_id: String,
  pub id:     String,
  pub info:   String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "notice_type", rename_all = "snake_case")]
pub enum NoticeType {
  Sub { sub: SubNotice },
  Resub { resub: ResubNotice },
  SubGift { sub_gift: SubGiftNotice },
  CommunitySubGift { community_sub_gift: CommunitySubGiftNotice },
  GiftPaidUpgrade { gift_paid_upgrade: GiftUpgradeNotice },
  PrimePaidUpgrade { prime_paid_upgrade: PrimePaidUpgradeNotice },
  Raid { raid: RaidNotice },
  Unraid { unraid: UnraidNotice },
  PayItForward { pay_it_forward: GiftUpgradeNotice },
  Announcement { announcement: AnnouncementNotice },
  BitsBandTier { bits_badge_tier: BitsBadgeTierNotice },
  CharityDonation { charity_donation: CharityDonationNotice },
  SharedChatSub { shared_chat_sub: SubNotice },
  SharedChatResub { shared_chat_resub: ResubNotice },
  SharedChatSubGift { shared_chat_sub_gift: SubGiftNotice },
  SharedChatCommunitySubGift { shared_community_sub_gift: CommunitySubGiftNotice },
  SharedChatGiftPaidUpgrade { shared_chat_gift_paid_upgrade: GiftUpgradeNotice },
  SharedChatPrimePaidUpgrade { shared_chat_prime_paid_upgrade: PrimePaidUpgradeNotice },
  SharedChatRaid { shared_chat_raid: RaidNotice },
  SharedChatPayItForward { shared_chat_pay_it_forward: GiftUpgradeNotice },
  SharedChatAnnouncement { shared_chat_announcement: AnnouncementNotice },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubNotice {
  pub sub_tier:         String,
  #[serde(deserialize_with = "maybe_string")]
  pub is_prime:         bool,
  #[serde(deserialize_with = "maybe_string")]
  pub duriation_months: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResubNotice {
  #[serde(deserialize_with = "maybe_string")]
  pub cumulative_months: u32,
  #[serde(deserialize_with = "maybe_string")]
  pub duration_months:   u32,
  pub streak_months:     Option<u32>,
  pub sub_tier:          String,
  #[serde(default)]
  pub is_prime:          bool,
  #[serde(deserialize_with = "maybe_string")]
  pub is_gift:           bool,
  #[serde(flatten)]
  pub gifter:            Gifter,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubGiftNotice {
  #[serde(deserialize_with = "maybe_string")]
  pub cumulative_months: u32,
  #[serde(deserialize_with = "maybe_string")]
  pub duriation_months:  u32,
  pub sub_tier:          String,
  pub community_gift_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommunitySubGiftNotice {
  pub id:               String,
  #[serde(deserialize_with = "maybe_string")]
  pub total:            u32,
  pub sub_tier:         String,
  pub cumulative_total: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GiftUpgradeNotice {
  #[serde(flatten)]
  pub gifter: Gifter,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PrimePaidUpgradeNotice {
  pub sub_tier: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RaidNotice {
  #[serde(flatten)]
  pub user:              User,
  #[serde(deserialize_with = "maybe_string")]
  pub viewer_count:      u32,
  pub profile_image_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnraidNotice {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnnouncementNotice {
  pub color: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BitsBadgeTierNotice {
  #[serde(deserialize_with = "maybe_string")]
  pub tier: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharityDonationNotice {
  pub charity_name: String,
  pub amount:       DonationAmount,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum SharedMessage {
  Message { source_broadcaster: User, source_message_id: String, source_badges: Vec<Badge> },
  None,
}

impl<'de> Deserialize<'de> for SharedMessage {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    #[derive(Deserialize)]
    struct InnerMessage {
      #[serde(rename = "source_broadcaster_user_id")]
      user_id:    Option<String>,
      #[serde(rename = "source_broadcaster_user_name")]
      user_name:  Option<String>,
      #[serde(rename = "source_broadcaster_user_login")]
      user_login: Option<String>,
      #[serde(rename = "source_message_id")]
      message_id: Option<String>,
      #[serde(rename = "source_badges")]
      badges:     Option<Vec<Badge>>,
    }
    let message = InnerMessage::deserialize(deserializer)?;

    if message.user_id.is_none()
      || message.user_name.is_none()
      || message.user_login.is_none()
      || message.message_id.is_none()
      || message.badges.is_none()
    {
      return Ok(SharedMessage::None);
    }

    let error = |field: &'static str| move || D::Error::missing_field(field);
    let user = User {
      user_id:    message.user_id.ok_or_else(error("source_broadcaster_user_id"))?,
      user_name:  message.user_name.ok_or_else(error("source_broadcaster_user_name"))?,
      user_login: message.user_login.ok_or_else(error("source_broadcaster_user_login"))?,
    };

    Ok(SharedMessage::Message {
      source_broadcaster: user,
      source_message_id:  message.message_id.ok_or_else(error("source_message_id"))?,
      source_badges:      message.badges.ok_or_else(error("source_badges"))?,
    })
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub enum Gifter {
  None,
  Anonymous,
  Gifter { gifter: User },
}

impl Serialize for Gifter {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    #[derive(Default, Serialize)]
    struct InnerGifter<'a> {
      gifter_user_id:      Option<&'a str>,
      gifter_user_name:    Option<&'a str>,
      gifter_user_login:   Option<&'a str>,
      gifter_is_anonymous: Option<bool>,
    }

    match self {
      Self::Gifter { gifter } => InnerGifter {
        gifter_user_id:      Some(&gifter.user_id),
        gifter_user_name:    Some(&gifter.user_name),
        gifter_user_login:   Some(&gifter.user_login),
        gifter_is_anonymous: Some(false),
      },
      Self::Anonymous => InnerGifter { gifter_is_anonymous: Some(true), ..Default::default() },
      Self::None => Default::default(),
    }
    .serialize(serializer)
  }
}

impl<'de> Deserialize<'de> for Gifter {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    #[derive(Deserialize)]
    struct InnerGifter {
      #[serde(rename = "gifter_user_id")]
      user_id:      Option<String>,
      #[serde(rename = "gifter_user_name")]
      user_name:    Option<String>,
      #[serde(rename = "gifter_user_login")]
      user_login:   Option<String>,
      #[serde(rename = "gifter_is_anonymous")]
      is_anonymous: Option<bool>,
    }
    let gifter = InnerGifter::deserialize(deserializer)?;

    let error = |field: &'static str| move || D::Error::missing_field(field);
    if let Some(true) = gifter.is_anonymous {
      return Ok(Self::Anonymous);
    }

    if gifter.is_anonymous.is_none()
      || gifter.user_id.is_none()
      || gifter.user_name.is_none()
      || gifter.user_login.is_none()
    {
      return Ok(Self::None);
    }

    Ok(Self::Gifter {
      gifter: User {
        user_id:    gifter.user_id.ok_or_else(error("gifter_user_id"))?,
        user_name:  gifter.user_name.ok_or_else(error("gifter_user_name"))?,
        user_login: gifter.user_login.ok_or_else(error("gifter_user_login"))?,
      },
    })
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn channel_chat_clear() {
    let event = r#"
    {
      "broadcaster_user_id": "1337",
      "broadcaster_user_name": "Cool_User",
      "broadcaster_user_login": "cool_user"
    }"#;
    serde_json::from_str::<ChatClear>(event).unwrap();
  }

  #[test]
  fn channel_chat_clear_user_messages() {
    let event = r#"
    {
      "broadcaster_user_id": "1337",
      "broadcaster_user_name": "Cool_User",
      "broadcaster_user_login": "cool_user",
      "target_user_id": "7734",
      "target_user_name": "Uncool_viewer",
      "target_user_login": "uncool_viewer"
    }"#;
    serde_json::from_str::<ChatClear>(event).unwrap();
  }

  #[test]
  fn channel_chat_message() {
    let event = r##"
    {
      "broadcaster_user_id": "1971641",
      "broadcaster_user_login": "streamer",
      "broadcaster_user_name": "streamer",
      "chatter_user_id": "4145994",
      "chatter_user_login": "viewer32",
      "chatter_user_name": "viewer32",
      "message_id": "cc106a89-1814-919d-454c-f4f2f970aae7",
      "message": {
        "text": "Hi chat",
        "fragments": [
          {
            "type": "text",
            "text": "Hi chat",
            "cheermote": null,
            "emote": null,
            "mention": null
          }
        ]
      },
      "color": "#00FF7F",
      "badges": [
        {
          "set_id": "moderator",
          "id": "1",
          "info": ""
        },
        {
          "set_id": "subscriber",
          "id": "12",
          "info": "16"
        },
        {
          "set_id": "sub-gifter",
          "id": "1",
          "info": ""
        }
      ],
      "message_type": "text",
      "cheer": null,
      "reply": null,
      "channel_points_custom_reward_id": null,
      "source_broadcaster_user_id": "112233",
      "source_broadcaster_user_login": "streamer33",
      "source_broadcaster_user_name": "streamer33",
      "source_message_id": "e03f6d5d-8ec8-4c63-b473-9e5fe61e289b",
      "source_badges": [
        {
          "set_id": "subscriber",
          "id": "3",
          "info": "3"
        }
      ]
    }"##;
    serde_json::from_str::<ChatMessage>(event).unwrap();

    let event = r##"
    {
      "broadcaster_user_id": "1971641",
      "broadcaster_user_login": "streamer",
      "broadcaster_user_name": "streamer",
      "chatter_user_id": "4145994",
      "chatter_user_login": "viewer32",
      "chatter_user_name": "viewer32",
      "message_id": "cc106a89-1814-919d-454c-f4f2f970aae7",
      "message": {
        "text": "Hi chat",
        "fragments": [
          {
            "type": "text",
            "text": "Hi chat",
            "cheermote": null,
            "emote": null,
            "mention": null
          }
        ]
      },
      "color": "#00FF7F",
      "badges": [
        {
          "set_id": "moderator",
          "id": "1",
          "info": ""
        },
        {
          "set_id": "subscriber",
          "id": "12",
          "info": "16"
        },
        {
          "set_id": "sub-gifter",
          "id": "1",
          "info": ""
        }
      ],
      "message_type": "text",
      "cheer": null,
      "reply": null,
      "channel_points_custom_reward_id": null,
      "source_broadcaster_user_id": null,
      "source_broadcaster_user_login": null,
      "source_broadcaster_user_name": null,
      "source_message_id": null,
      "source_badges": null
    }"##;
    serde_json::from_str::<ChatMessage>(event).unwrap();
  }

  #[test]
  fn channel_chat_message_delete() {
    let event = r#"
    {
      "broadcaster_user_id": "1337",
      "broadcaster_user_name": "Cool_User",
      "broadcaster_user_login": "cool_user",
      "target_user_id": "7734",
      "target_user_name": "Uncool_viewer",
      "target_user_login": "uncool_viewer",
      "message_id": "ab24e0b0-2260-4bac-94e4-05eedd4ecd0e"
    }"#;
    serde_json::from_str::<ChatClear>(event).unwrap();
  }

  #[test]
  fn channel_chat_notification() {
    let event = r#"
    {
      "broadcaster_user_id": "1971641",
      "broadcaster_user_login": "streamer",
      "broadcaster_user_name": "streamer",
      "chatter_user_id": "49912639",
      "chatter_user_login": "viewer23",
      "chatter_user_name": "viewer23",
      "chatter_is_anonymous": false,
      "color": "",
      "badges": [],
      "system_message": "viewer23 subscribed at Tier 1. They've subscribed for 10 months!",
      "message_id": "d62235c8-47ff-a4f4--84e8-5a29a65a9c03",
      "message": {
        "text": "",
        "fragments": []
      },
      "notice_type": "shared_chat_resub",
      "sub": null,
      "resub": null,
      "sub_gift": null,
      "community_sub_gift": null,
      "gift_paid_upgrade": null,
      "prime_paid_upgrade": null,
      "pay_it_forward": null,
      "raid": null,
      "unraid": null,
      "announcement": null,
      "bits_badge_tier": null,
      "charity_donation": null,
      "shared_chat_sub": null,
      "shared_chat_resub": {
        "cumulative_months": 10,
        "duration_months": 0,
        "streak_months": null,
        "sub_tier": "1000",
        "is_gift": false,
        "gifter_is_anonymous": null,
        "gifter_user_id": null,
        "gifter_user_name": null,
        "gifter_user_login": null
      },
      "shared_chat_sub_gift": null,
      "shared_chat_community_sub_gift": null,
      "shared_chat_gift_paid_upgrade": null,
      "shared_chat_prime_paid_upgrade": null,
      "shared_chat_pay_it_forward": null,
      "shared_chat_raid": null,
      "shared_chat_unraid": null,
      "shared_chat_announcement": null,
      "shared_chat_bits_badge_tier": null,
      "shared_chat_charity_donation": null,
      "source_broadcaster_user_id": "112233",
      "source_broadcaster_user_login": "streamer33",
      "source_broadcaster_user_name": "streamer33",
      "source_message_id": "2be7193d-0366-4453-b6ec-b288ce9f2c39",
      "source_badges": [{
        "set_id": "subscriber",
        "id": "3",
        "info": "3"
      }]
    }"#;
    serde_json::from_str::<ChatNotification>(event).unwrap();
  }

  #[test]
  fn channel_chat_settings_update() {
    let event = r#"
    {
      "broadcaster_user_id": "1337",
      "broadcaster_user_login": "cool_user",
      "broadcaster_user_name": "Cool_User",
      "emote_mode": true,
      "follower_mode": false,
      "follower_mode_duration_minutes": null,
      "slow_mode": true,
      "slow_mode_wait_time_seconds": 10,
      "subscriber_mode": false,
      "unique_chat_mode": false
    }"#;
    serde_json::from_str::<ChatSettingsUpdate>(event).unwrap();
  }

  #[test]
  fn channel_chat_user_message_hold() {
    let event = r#"
    {
      "broadcaster_user_id": "123",
      "broadcaster_user_login": "bob",
      "broadcaster_user_name": "Bob",
      "user_id": "456",
      "user_login": "tom",
      "user_name": "Tommy",
      "message_id": "789",
      "message": {
        "text": "hey world",
        "fragments": [
          {
            "type": "emote",
            "text": "hey world",
            "cheermote": null,
            "emote": {
              "id": "foo",
              "emote_set_id": "7"
            }
          },
          {
            "type": "cheermote",
            "text": "bye world",
            "cheermote": {
              "prefix": "prefix",
              "bits": 100,
              "tier": 1
            },
            "emote": null
          },
          {
            "type": "text",
            "text": "surprise",
            "cheermote": null,
            "emote": null
          }
        ]
      }
    }"#;
    serde_json::from_str::<ChatUserMessageHold>(event).unwrap();
  }

  #[test]
  fn channel_chat_user_message_update() {
    let event = r#"
    {
      "broadcaster_user_id": "123",
      "broadcaster_user_login": "bob",
      "broadcaster_user_name": "Bob",
      "user_id": "456",
      "user_login": "tom",
      "user_name": "Tommy",
      "status": "approved",
      "message_id": "789",
      "message": {
        "text": "hey world",
        "fragments": [
          {
            "type": "emote",
            "text": "hey world",
            "cheermote": null,
            "emote": {
                "id": "foo",
                "emote_set_id": "7"
            }
          },
          {
            "type": "cheermote",
            "text": "bye world",
            "cheermote": {
                "prefix": "prefix",
                "bits": 100,
                "tier": 1
            },
            "emote": null
          },
          {
            "type": "text",
            "text": "surprise",
            "cheermote": null,
            "emote": null
          }
        ]
      }
    }"#;
    serde_json::from_str::<ChatUserMessageUpdate>(event).unwrap();
  }
}
