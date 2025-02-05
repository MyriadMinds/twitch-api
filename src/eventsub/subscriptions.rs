use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use ureq::json;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
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

#[derive(Debug)]
pub enum SubscriptionType {
  AutomodMessageHold,
  AutomodMessageUpdate,
  AutomodSettingsUpdate,
  AutomodTermsUpdate,
  ChannelUpdate,
  Follow,
  AdBreakBegin,
  ChatClear,
  ChatClearUserMessages,
  ChatMessage,
  ChatMessageDelete,
  ChatNotification,
  ChatSettingsUpdate,
  ChatUserMessageHold,
  ChatUserMessageUpdate,
  SharedChatSessionBegin,
  SharedChatSessionUpdate,
  SharedChatSessionEnd,
  Subscribe,
  SubscriptionEnd,
  SubscriptionGift,
  SubscriptionMessage,
  Cheer,
  Raid(Raid),
  Ban,
  Unban,
  UnbanRequestCreate,
  UnbanRequestResolve,
  Moderate,
  ModeratorAdd,
  ModeratorRemove,
  PointsAutomaticRewardRedemption,
  PointsCustomRewardAdd,
  PointsCustomRewardUpdate(Option<String>),
  PointsCustomRewardRemove(Option<String>),
  PointsCustomRewardRedemptionAdd(Option<String>),
  PointsCustomRewardRedemptionUpdate(Option<String>),
  PollBegin,
  PollProgress,
  PollEnd,
  PredictionBegin,
  PredictionProgress,
  PredictionLock,
  PredictionEnd,
  SuspiciousUserMessage,
  SuspiciousUserUpdate,
  VIPAdd,
  VIPRemove,
  WarningAcknowledgement,
  WarningSend,
  CharityDonation,
  CharityCampaignStart,
  CharityCampaignProgress,
  CharityCampaignStop,
  GoalBegin,
  GoalProgress,
  GoalEnd,
  HypeTrainBegin,
  HypeTrainProgress,
  HypeTrainEnd,
  ShieldModeBegin,
  ShieldModeEnd,
  ShoutoutCreate,
  ShoutoutReceived,
  StreamOnline,
  StreamOffline,
  UserUpdate,
  WhisperReceived,
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
      Self::AutomodMessageHold => ("automod.message.hold", "2"),
      Self::AutomodMessageUpdate => ("automod.message.update", "2"),
      Self::AutomodSettingsUpdate => ("automod.settings.update", "1"),
      Self::AutomodTermsUpdate => ("automod.terms.update", "1"),
      Self::ChannelUpdate => ("channel.update", "2"),
      Self::Follow => ("channel.follow", "2"),
      Self::AdBreakBegin => ("channel.ad_break.begin", "1"),
      Self::ChatClear => ("channel.chat.clear", "1"),
      Self::ChatClearUserMessages => ("channel.chat.clear_user_messages", "1"),
      Self::ChatMessage => ("channel.chat.message", "1"),
      Self::ChatMessageDelete => ("channel.chat.message_delete", "1"),
      Self::ChatNotification => ("channel.chat.notification", "1"),
      Self::ChatSettingsUpdate => ("channel.chat_settings.update", "1"),
      Self::ChatUserMessageHold => ("channel.chat.user_message_hold", "1"),
      Self::ChatUserMessageUpdate => ("channel.chat.user_message_update", "1"),
      Self::SharedChatSessionBegin => ("channel.shared_chat.begin", "1"),
      Self::SharedChatSessionUpdate => ("channel.shared_chat.update", "1"),
      Self::SharedChatSessionEnd => ("channel.shared_chat.end", "1"),
      Self::Subscribe => ("channel.subscribe", "1"),
      Self::SubscriptionEnd => ("channel.subscription.end", "1"),
      Self::SubscriptionGift => ("channel.subscription.gift", "1"),
      Self::SubscriptionMessage => ("channel.subscription.message", "1"),
      Self::Cheer => ("channel.ChannelCheer", "1"),
      Self::Raid(_) => ("channel.ChannelRaid", "1"),
      Self::Ban => ("channel.ban", "1"),
      Self::Unban => ("channel.unban", "1"),
      Self::UnbanRequestCreate => ("channel.unban_request.create", "1"),
      Self::UnbanRequestResolve => ("channel.unban_request.resolve", "1"),
      Self::Moderate => ("channel.moderate", "2"),
      Self::ModeratorAdd => ("channel.moderator.add", "1"),
      Self::ModeratorRemove => ("channel.moderator.remove", "1"),
      Self::PointsAutomaticRewardRedemption =>
        ("channel.channel_points_automatic_reward_redemption.add", "1"),
      Self::PointsCustomRewardAdd => ("channel.channel_points_custom_reward.add", "1"),
      Self::PointsCustomRewardUpdate(_) => ("channel.channel_points_custom_reward.update", "1"),
      Self::PointsCustomRewardRemove(_) => ("channel.channel_points_custom_reward.remove", "1"),
      Self::PointsCustomRewardRedemptionAdd(_) =>
        ("channel.channel_points_custom_reward_redemption.add", "1"),
      Self::PointsCustomRewardRedemptionUpdate(_) =>
        ("channel.channel_points_custom_reward_redemption.update", "1"),
      Self::PollBegin => ("channel.poll.begin", "1"),
      Self::PollProgress => ("channel.poll.progress", "1"),
      Self::PollEnd => ("channel.poll.end", "1"),
      Self::PredictionBegin => ("channel.prediction.begin", "1"),
      Self::PredictionProgress => ("channel.prediction.progress", "1"),
      Self::PredictionLock => ("channel.prediction.lock", "1"),
      Self::PredictionEnd => ("channel.prediction.end", "1"),
      Self::SuspiciousUserMessage => ("channel.suspicious_user.message", "1"),
      Self::SuspiciousUserUpdate => ("channel.suspicious_user.update", "1"),
      Self::VIPAdd => ("channel.vip.add", "1"),
      Self::VIPRemove => ("channel.vip.remove", "1"),
      Self::WarningAcknowledgement => ("channel.warning.acknowledge", "1"),
      Self::WarningSend => ("channel.warning.send", "1"),
      Self::CharityDonation => ("channel.charity_campaign.donate", "1"),
      Self::CharityCampaignStart => ("channel.charity_campaign.start", "1"),
      Self::CharityCampaignProgress => ("channel.charity_campaign.progress", "1"),
      Self::CharityCampaignStop => ("channel.charity_campaign.stop", "1"),
      Self::GoalBegin => ("channel.goal.begin", "1"),
      Self::GoalProgress => ("channel.goal.progress", "1"),
      Self::GoalEnd => ("channel.goal.end", "1"),
      Self::HypeTrainBegin => ("channel.hype_train.begin", "1"),
      Self::HypeTrainProgress => ("channel.hype_train.progress", "1"),
      Self::HypeTrainEnd => ("channel.hype_train.end", "1"),
      Self::ShieldModeBegin => ("channel.shield_mode.begin", "1"),
      Self::ShieldModeEnd => ("channel.shield_mode.end", "1"),
      Self::ShoutoutCreate => ("channel.shoutout.create", "1"),
      Self::ShoutoutReceived => ("channel.shoutout.receive", "1"),
      Self::StreamOnline => ("stream.online", "1"),
      Self::StreamOffline => ("stream.offline", "1"),
      Self::UserUpdate => ("user.update", "1"),
      Self::WhisperReceived => ("user.whisper.message", "1"),
    };

    (kind.to_owned(), version.to_owned())
  }

  fn conditions(&self, cond: &Conditions) -> Map<String, Value> {
    use Conditions as Cond;
    let conditions: Vec<(String, Value)> = match self {
      Self::AutomodMessageHold => vec![cond.broadcaster_user_id(), cond.moderator_user_id()],
      Self::AutomodMessageUpdate => vec![cond.broadcaster_user_id(), cond.moderator_user_id()],
      Self::AutomodSettingsUpdate => vec![cond.broadcaster_user_id(), cond.moderator_user_id()],
      Self::AutomodTermsUpdate => vec![cond.broadcaster_user_id(), cond.moderator_user_id()],
      Self::ChannelUpdate => vec![cond.broadcaster_user_id()],
      Self::Follow => vec![cond.broadcaster_user_id(), cond.moderator_user_id()],
      Self::AdBreakBegin => vec![cond.broadcaster_user_id()],
      Self::ChatClear => vec![cond.broadcaster_user_id(), cond.user_id()],
      Self::ChatClearUserMessages => vec![cond.broadcaster_user_id(), cond.user_id()],
      Self::ChatMessage => vec![cond.broadcaster_user_id(), cond.user_id()],
      Self::ChatMessageDelete => vec![cond.broadcaster_user_id(), cond.user_id()],
      Self::ChatNotification => vec![cond.broadcaster_user_id(), cond.user_id()],
      Self::ChatSettingsUpdate => vec![cond.broadcaster_user_id(), cond.user_id()],
      Self::ChatUserMessageHold => vec![cond.broadcaster_user_id(), cond.user_id()],
      Self::ChatUserMessageUpdate => vec![cond.broadcaster_user_id(), cond.user_id()],
      Self::SharedChatSessionBegin => vec![cond.broadcaster_user_id()],
      Self::SharedChatSessionUpdate => vec![cond.broadcaster_user_id()],
      Self::SharedChatSessionEnd => vec![cond.broadcaster_user_id()],
      Self::Subscribe => vec![cond.broadcaster_user_id()],
      Self::SubscriptionEnd => vec![cond.broadcaster_user_id()],
      Self::SubscriptionGift => vec![cond.broadcaster_user_id()],
      Self::SubscriptionMessage => vec![cond.broadcaster_user_id()],
      Self::Cheer => vec![cond.broadcaster_user_id()],
      Self::Raid(Raid::To) => vec![cond.to_broadcaster_user_id()],
      Self::Raid(Raid::From) => vec![cond.from_broadcaster_user_id()],
      Self::Ban => vec![cond.broadcaster_user_id()],
      Self::Unban => vec![cond.broadcaster_user_id()],
      Self::UnbanRequestCreate => vec![cond.broadcaster_user_id(), cond.moderator_user_id()],
      Self::UnbanRequestResolve => vec![cond.broadcaster_user_id(), cond.moderator_user_id()],
      Self::Moderate => vec![cond.broadcaster_user_id(), cond.moderator_user_id()],
      Self::ModeratorAdd => vec![cond.broadcaster_user_id()],
      Self::ModeratorRemove => vec![cond.broadcaster_user_id()],
      Self::PointsAutomaticRewardRedemption => vec![cond.broadcaster_user_id()],
      Self::PointsCustomRewardAdd => vec![cond.broadcaster_user_id()],
      Self::PointsCustomRewardUpdate(None) => vec![cond.broadcaster_user_id()],
      Self::PointsCustomRewardUpdate(Some(id)) =>
        vec![cond.broadcaster_user_id(), Cond::reward_id(id)],
      Self::PointsCustomRewardRemove(None) => vec![cond.broadcaster_user_id()],
      Self::PointsCustomRewardRemove(Some(id)) =>
        vec![cond.broadcaster_user_id(), Cond::reward_id(id)],
      Self::PointsCustomRewardRedemptionAdd(None) => vec![cond.broadcaster_user_id()],
      Self::PointsCustomRewardRedemptionAdd(Some(id)) =>
        vec![cond.broadcaster_user_id(), Cond::reward_id(id)],
      Self::PointsCustomRewardRedemptionUpdate(None) => vec![cond.broadcaster_user_id()],
      Self::PointsCustomRewardRedemptionUpdate(Some(id)) =>
        vec![cond.broadcaster_user_id(), Cond::reward_id(id)],
      Self::PollBegin => vec![cond.broadcaster_user_id()],
      Self::PollProgress => vec![cond.broadcaster_user_id()],
      Self::PollEnd => vec![cond.broadcaster_user_id()],
      Self::PredictionBegin => vec![cond.broadcaster_user_id()],
      Self::PredictionProgress => vec![cond.broadcaster_user_id()],
      Self::PredictionLock => vec![cond.broadcaster_user_id()],
      Self::PredictionEnd => vec![cond.broadcaster_user_id()],
      Self::SuspiciousUserMessage => vec![cond.broadcaster_user_id(), cond.moderator_user_id()],
      Self::SuspiciousUserUpdate => vec![cond.broadcaster_user_id(), cond.moderator_user_id()],
      Self::VIPAdd => vec![cond.broadcaster_user_id()],
      Self::VIPRemove => vec![cond.broadcaster_user_id()],
      Self::WarningAcknowledgement => vec![cond.broadcaster_user_id(), cond.moderator_user_id()],
      Self::WarningSend => vec![cond.broadcaster_user_id(), cond.moderator_user_id()],
      Self::CharityDonation => vec![cond.broadcaster_user_id()],
      Self::CharityCampaignStart => vec![cond.broadcaster_user_id()],
      Self::CharityCampaignProgress => vec![cond.broadcaster_user_id()],
      Self::CharityCampaignStop => vec![cond.broadcaster_user_id()],
      Self::GoalBegin => vec![cond.broadcaster_user_id()],
      Self::GoalProgress => vec![cond.broadcaster_user_id()],
      Self::GoalEnd => vec![cond.broadcaster_user_id()],
      Self::HypeTrainBegin => vec![cond.broadcaster_user_id()],
      Self::HypeTrainProgress => vec![cond.broadcaster_user_id()],
      Self::HypeTrainEnd => vec![cond.broadcaster_user_id()],
      Self::ShieldModeBegin => vec![cond.broadcaster_user_id(), cond.moderator_user_id()],
      Self::ShieldModeEnd => vec![cond.broadcaster_user_id(), cond.moderator_user_id()],
      Self::ShoutoutCreate => vec![cond.broadcaster_user_id(), cond.moderator_user_id()],
      Self::ShoutoutReceived => vec![cond.broadcaster_user_id(), cond.moderator_user_id()],
      Self::StreamOnline => vec![cond.broadcaster_user_id()],
      Self::StreamOffline => vec![cond.broadcaster_user_id()],
      Self::UserUpdate => vec![cond.broadcaster_user_id(), cond.user_id()],
      Self::WhisperReceived => vec![cond.user_id()],
    };

    conditions.into_iter().collect::<Map<String, Value>>()
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum Raid {
  To,
  From,
}

pub struct Conditions {
  broadcaster_id: String,
  token_user_id:  String,
}

impl Conditions {
  pub fn new(broadcaster_id: String, token_user_id: String) -> Self {
    Self { broadcaster_id, token_user_id }
  }

  fn reward_id(id: &str) -> (String, Value) {
    ("reward_id".to_string(), json!(id))
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
