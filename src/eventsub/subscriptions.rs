use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use ureq::json;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Subscription {
  id:                Option<String>,
  status:            Option<String>,
  #[serde(rename = "type")]
  subscription_type: String,
  version:           String,
  condition:         Map<String, Value>,
  transport:         Value,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum SubscriptionType {
  Follow,
  AdBreakBegin,
  ChatClear,
  ChatClearUserMessages,
  ChatMessage,
  ChatMessageDelete,
  Subscribe,
  SubscriptionGift,
  SubscriptionMessage,
  Cheer,
  Raid,
  PointsCustomRewardRedemptionAdd,
  PollBegin,
  PollProgress,
  PollEnd,
  PredictionBegin,
  PredictionProgress,
  PredictionLock,
  PredictionEnd,
  CharityDonation,
  HypeTrainBegin,
  HypeTrainProgress,
  HypeTrainEnd,
  ShoutoutCreate,
}

impl SubscriptionType {
  pub fn build_subscription(&self, session_id: &str, conditions: &Conditions) -> Subscription {
    let (subscription_type, version) = self.details();
    let condition = self.conditions(&conditions);
    let transport = json!({
      "method": "websocket",
      "session_id": session_id
    });

    Subscription { subscription_type, version, condition, transport, ..Default::default() }
  }

  fn details(&self) -> (String, String) {
    let (kind, version) = match self {
      SubscriptionType::Follow => ("channel.ChannelFollow", "2"),
      SubscriptionType::AdBreakBegin => ("channel.ad_break.begin", "1"),
      SubscriptionType::ChatClear => ("channel.chat.clear", "1"),
      SubscriptionType::ChatClearUserMessages => ("channel.chat.clear_user_messages", "1"),
      SubscriptionType::ChatMessage => ("channel.chat.message", "1"),
      SubscriptionType::ChatMessageDelete => ("channel.chat.message_delete", "1"),
      SubscriptionType::Subscribe => ("channel.ChannelSubscribe", "1"),
      SubscriptionType::SubscriptionGift => ("channel.subscription.gift", "1"),
      SubscriptionType::SubscriptionMessage => ("channel.subscription.message", "1"),
      SubscriptionType::Cheer => ("channel.ChannelCheer", "1"),
      SubscriptionType::Raid => ("channel.ChannelRaid", "1"),
      SubscriptionType::PointsCustomRewardRedemptionAdd =>
        ("channel.channel_points_custom_reward_redemption.add", "1"),
      SubscriptionType::PollBegin => ("channel.poll.begin", "1"),
      SubscriptionType::PollProgress => ("channel.poll.progress", "1"),
      SubscriptionType::PollEnd => ("channel.poll.end", "1"),
      SubscriptionType::PredictionBegin => ("channel.prediction.begin", "1"),
      SubscriptionType::PredictionProgress => ("channel.prediction.progress", "1"),
      SubscriptionType::PredictionLock => ("channel.prediction.lock", "1"),
      SubscriptionType::PredictionEnd => ("channel.prediction.end", "1"),
      SubscriptionType::CharityDonation => ("channel.charity_campaign.donate", "1"),
      SubscriptionType::HypeTrainBegin => ("channel.hype_train.begin", "1"),
      SubscriptionType::HypeTrainProgress => ("channel.hype_train.progress", "1"),
      SubscriptionType::HypeTrainEnd => ("channel.hype_train.end", "1"),
      SubscriptionType::ShoutoutCreate => ("channel.shoutout.create", "1"),
    };

    (kind.to_owned(), version.to_owned())
  }

  fn conditions(&self, cond: &Conditions) -> Map<String, Value> {
    let conditions: Vec<(String, Value)> = match self {
      SubscriptionType::Follow => vec![cond.broadcaster_user_id(), cond.moderator_user_id()],
      SubscriptionType::AdBreakBegin => vec![cond.broadcaster_user_id()],
      SubscriptionType::ChatClear => vec![cond.broadcaster_user_id(), cond.user_id()],
      SubscriptionType::ChatClearUserMessages => vec![cond.broadcaster_user_id(), cond.user_id()],
      SubscriptionType::ChatMessage => vec![cond.broadcaster_user_id(), cond.user_id()],
      SubscriptionType::ChatMessageDelete => vec![cond.broadcaster_user_id(), cond.user_id()],
      SubscriptionType::Subscribe => vec![cond.broadcaster_user_id()],
      SubscriptionType::SubscriptionGift => vec![cond.broadcaster_user_id()],
      SubscriptionType::SubscriptionMessage => vec![cond.broadcaster_user_id()],
      SubscriptionType::Cheer => vec![cond.broadcaster_user_id()],
      SubscriptionType::Raid => vec![cond.to_broadcaster_user_id()],
      SubscriptionType::PointsCustomRewardRedemptionAdd => vec![cond.broadcaster_user_id()],
      SubscriptionType::PollBegin => vec![cond.broadcaster_user_id()],
      SubscriptionType::PollProgress => vec![cond.broadcaster_user_id()],
      SubscriptionType::PollEnd => vec![cond.broadcaster_user_id()],
      SubscriptionType::PredictionBegin => vec![cond.broadcaster_user_id()],
      SubscriptionType::PredictionProgress => vec![cond.broadcaster_user_id()],
      SubscriptionType::PredictionLock => vec![cond.broadcaster_user_id()],
      SubscriptionType::PredictionEnd => vec![cond.broadcaster_user_id()],
      SubscriptionType::CharityDonation => vec![cond.broadcaster_user_id()],
      SubscriptionType::HypeTrainBegin => vec![cond.broadcaster_user_id()],
      SubscriptionType::HypeTrainProgress => vec![cond.broadcaster_user_id()],
      SubscriptionType::HypeTrainEnd => vec![cond.broadcaster_user_id()],
      SubscriptionType::ShoutoutCreate =>
        vec![cond.broadcaster_user_id(), cond.moderator_user_id()],
    };

    conditions.into_iter().collect::<Map<String, Value>>()
  }

  pub fn name(&self) -> &'static str {
    match self {
      SubscriptionType::Follow => "Follow",
      SubscriptionType::AdBreakBegin => "Ad Break Begin",
      SubscriptionType::ChatClear => "Chat Clear",
      SubscriptionType::ChatClearUserMessages => "Chat Clear User",
      SubscriptionType::ChatMessage => "Chat Message",
      SubscriptionType::ChatMessageDelete => "Chat Message Delete",
      SubscriptionType::Subscribe => "Subscribe",
      SubscriptionType::SubscriptionGift => "Subscription Gift",
      SubscriptionType::SubscriptionMessage => "Subscription Message",
      SubscriptionType::Cheer => "Cheer",
      SubscriptionType::Raid => "Raid",
      SubscriptionType::PointsCustomRewardRedemptionAdd => "Points Custom Reward Redemption Add",
      SubscriptionType::PollBegin => "Poll Begin",
      SubscriptionType::PollProgress => "Poll Progress",
      SubscriptionType::PollEnd => "Poll End",
      SubscriptionType::PredictionBegin => "Prediction Begin",
      SubscriptionType::PredictionProgress => "Prediction Progress",
      SubscriptionType::PredictionLock => "Prediction Lock",
      SubscriptionType::PredictionEnd => "Prediction End",
      SubscriptionType::CharityDonation => "Charity Donation",
      SubscriptionType::HypeTrainBegin => "Hype Train Begin",
      SubscriptionType::HypeTrainProgress => "Hype Train Progress",
      SubscriptionType::HypeTrainEnd => "Hype Train End",
      SubscriptionType::ShoutoutCreate => "Shoutout Received",
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Conditions {
  broadcaster_id: String,
  token_user_id:  String,
}

impl Conditions {
  pub fn new(broadcaster_id: String, token_user_id: String) -> Self {
    Self { broadcaster_id, token_user_id }
  }

  fn from_broadcaster_user_id(&self) -> (String, Value) {
    ("from_broadcaster_user_id".to_string(), json!(self.broadcaster_id))
  }

  fn to_broadcaster_user_id(&self) -> (String, Value) {
    ("to_broadcaster_user_id".to_string(), json!(self.broadcaster_id))
  }

  fn broadcaster_user_id(&self) -> (String, Value) {
    ("broadcaster_user_id".to_string(), json!(self.broadcaster_id))
  }

  fn moderator_user_id(&self) -> (String, Value) {
    ("moderator_user_id".to_string(), json!(self.token_user_id))
  }

  fn user_id(&self) -> (String, Value) {
    ("user_id".to_string(), json!(self.token_user_id))
  }
}
