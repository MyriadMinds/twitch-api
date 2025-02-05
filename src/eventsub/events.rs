mod automod;
mod channel;
mod charity;
mod chat;
mod goal;
mod hypetrain;
mod moderation;
mod polls;
mod prediction;
mod rewards;
mod shared_chat;
mod subscription;
mod user;

use std::fmt::Display;
use std::str::FromStr;

pub use automod::*;
pub use channel::*;
pub use charity::*;
pub use chat::*;
pub use goal::*;
pub use hypetrain::*;
pub use moderation::*;
pub use polls::*;
pub use prediction::*;
pub use rewards::*;
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::with_prefix;
pub use shared_chat::*;
pub use subscription::*;
pub use user::*;

use super::Subscription;

// Eventsub communication
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(super) struct EventsubMessage {
  pub(super) metadata: Metadata,
  pub(super) payload:  Payload,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata {
  pub(super) message_id:           String,
  pub(super) message_type:         EventsubMessageType,
  pub(super) message_timestamp:    String,
  pub(super) subscription_type:    Option<String>,
  pub(super) subscription_version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub(super) enum EventsubMessageType {
  SessionWelcome,
  SessionKeepalive,
  Notification,
  SessionReconnect,
  Revocation,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub(super) enum Payload {
  Notification { subscription: Subscription, event: Event },
  Revocation { subscription: Subscription },
  Reconnect { session: Reconnect },
  Welcome { session: Welcome },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(super) struct Reconnect {
  pub(super) id: String,
  pub(super) status: String,
  pub(super) connected_at: String,
  #[serde(deserialize_with = "maybe_string")]
  pub(super) keepalive_timeout_seconds: u32,
  pub(super) reconnect_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(super) struct Welcome {
  pub(super) id: String,
  pub(super) status: String,
  pub(super) connected_at: String,
  #[serde(deserialize_with = "maybe_string")]
  pub(super) keepalive_timeout_seconds: u32,
}

// Events
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Event {
  AutomodMessageHold(AutomodMessageHold),
  AutomodMessageUpdate(AutomodMessageUpdate),
  AutomodSettingsUpdate(AutomodSettingsUpdate),
  AutomodTermsUpdate(AutomodTermsUpdate),
  ChannelUpdate(ChannelUpdate),
  Follow(Follow),
  AdBreakBegin(AdBreakBegin),
  ChatClear(ChatClear),
  ChatClearUserMessages(ChatClearUserMessages),
  ChatMessage(ChatMessage),
  ChatMessageDelete(ChatMessageDelete),
  ChatNotification(ChatNotification),
  ChatSettingsUpdate(ChatSettingsUpdate),
  ChatUserMessageHold(ChatUserMessageHold),
  ChatUserMessageUpdate(ChatUserMessageUpdate),
  SharedChatSessionBegin(SharedChatSessionBegin),
  SharedChatSessionUpdate(SharedChatSessionUpdate),
  SharedChatSessionEnd(SharedChatSessionEnd),
  Subscribe(Subscribe),
  SubscriptionEnd(SubscriptionEnd),
  SubscriptionGift(SubscriptionGift),
  SubscriptionMessage(SubscriptionMessage),
  Cheer(Cheer),
  Raid(Raid),
  Ban(Ban),
  Unban(Unban),
  UnbanRequestCreate(UnbanRequestCreate),
  UnbanRequestResolve(UnbanRequestResolve),
  Moderate(Moderate),
  ModeratorAdd(ModeratorAdd),
  ModeratorRemove(ModeratorRemove),
  PointsAutomaticRewardRedemption(PointsAutomaticRewardRedemption),
  PointsCustomRewardAdd(PointsCustomRewardAdd),
  PointsCustomRewardUpdate(PointsCustomRewardUpdate),
  PointsCustomRewardRemove(PointsCustomRewardRemove),
  PointsCustomRewardRedemptionAdd(PointsCustomRewardRedemptionAdd),
  PointsCustomRewardRedemptionUpdate(PointsCustomRewardRedemptionUpdate),
  PollBegin(PollBegin),
  PollProgress(PollProgress),
  PollEnd(PollEnd),
  PredictionBegin(PredictionBegin),
  PredictionProgress(PredictionProgress),
  PredictionLock(PredictionLock),
  PredictionEnd(PredictionEnd),
  SuspiciousUserMessage(SuspiciousUserMessage),
  SuspiciousUserUpdate(SuspiciousUserUpdate),
  VIPAdd(VIPAdd),
  VIPRemove(VIPRemove),
  WarningAcknowledge(WarningAcknowledge),
  WarningSend(WarningSend),
  CharityDonation(CharityDonation),
  CharityCampaignStart(CharityCampaignStart),
  CharityCampaignProgress(CharityCampaignProgress),
  CharityCampaignStop(CharityCampaignStop),
  GoalBegin(GoalBegin),
  GoalProgress(GoalProgress),
  GoalEnd(GoalEnd),
  HypeTrainBegin(HypeTrainBegin),
  HypeTrainProgress(HypeTrainProgress),
  HypeTrainEnd(HypeTrainEnd),
  ShieldModeBegin(ShieldModeBegin),
  ShieldModeEnd(ShieldModeEnd),
  ShoutoutCreate(ShoutoutCreate),
  ShoutoutReceived(ShoutoutReceived),
  StreamOnline(StreamOnline),
  StreamOffline(StreamOffline),
  UserUpdate(UserUpdate),
  WhisperReceived(WhisperReceived),
}

// Common event sub-components
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
  user_id:    String,
  user_login: String,
  user_name:  String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageSimple {
  pub text:   String,
  pub emotes: Vec<EmoteSimple>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmoteSimple {
  #[serde(deserialize_with = "maybe_string")]
  pub begin: u32,
  #[serde(deserialize_with = "maybe_string")]
  pub end:   u32,
  pub id:    String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
  pub text:      String,
  pub fragments: Vec<Fragment>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Fragment {
  Cheermote {
    text:      String,
    cheermote: Cheermote,
  },
  Emote {
    text:  String,
    emote: Emote,
  },
  Mention {
    text: String,
    #[serde(flatten)]
    user: User,
  },
  Text {
    text: String,
  },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cheermote {
  pub prefix: String,
  #[serde(deserialize_with = "maybe_string")]
  pub bits:   u32,
  #[serde(deserialize_with = "maybe_string")]
  pub tier:   u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Emote {
  pub id:           String,
  pub emote_set_id: String,
  pub owner_id:     String,
  pub format:       Vec<EmoteFormat>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum EmoteFormat {
  Static,
  Animated,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DonationAmount {
  #[serde(deserialize_with = "maybe_string")]
  pub value:          u32,
  #[serde(deserialize_with = "maybe_string")]
  pub decimal_places: u32,
  pub currency:       String,
}

// Deserialization helpers
////////////////////////////////////////////////////////////////////////////////////////////////////

with_prefix!(broadcaster "broadcaster_");
with_prefix!(source_broadcaster "source_broadcaster_");
with_prefix!(owner_broadcaster "owner_broadcaster_");
with_prefix!(host_broadcaster "host_broadcaster_");
with_prefix!(from_broadcaster "from_broadcaster_");
with_prefix!(to_broadcaster "to_broadcaster_");
with_prefix!(moderator "moderator_");
with_prefix!(requester "requester_");
with_prefix!(target "target_");
with_prefix!(chatter "chatter_");
with_prefix!(parent "parent_");
with_prefix!(thread "thread_");
with_prefix!(to "to_");
with_prefix!(from "from_");

fn maybe_string<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
  T: Deserialize<'de> + FromStr,
  D: Deserializer<'de>,
  <T as FromStr>::Err: Display,
{
  #[derive(Deserialize)]
  #[serde(untagged)]
  enum StringOrInt<T> {
    String(String),
    Value(T),
  }

  match StringOrInt::<T>::deserialize(deserializer)? {
    StringOrInt::String(s) => s.parse::<T>().map_err(serde::de::Error::custom),
    StringOrInt::Value(v) => Ok(v),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn event_notification() {
    let event = r##"
    {
      "metadata": {
        "message_id": "befa7b53-d79d-478f-86b9-120f112b044e",
        "message_type": "notification",
        "message_timestamp": "2022-11-16T10:11:12.464757833Z",
        "subscription_type": "channel.follow",
        "subscription_version": "1"
      },
      "payload": {
        "subscription": {
          "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
          "status": "enabled",
          "type": "channel.follow",
          "version": "1",
          "cost": 1,
          "condition": {
            "broadcaster_user_id": "12826"
          },
          "transport": {
            "method": "websocket",
            "session_id": "AQoQexAWVYKSTIu4ec_2VAxyuhAB"
          },
          "created_at": "2022-11-16T10:11:12.464757833Z"
        },
        "event": {
          "user_id": "1337",
          "user_login": "awesome_user",
          "user_name": "Awesome_User",
          "broadcaster_user_id": "12826",
          "broadcaster_user_login": "twitch",
          "broadcaster_user_name": "Twitch",
          "followed_at": "2023-07-15T18:16:11.17106713Z"
        }
      }
    }"##;
    serde_json::from_str::<EventsubMessage>(event).unwrap();
  }
}
