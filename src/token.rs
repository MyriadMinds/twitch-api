use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

use twitch_eventsub_structs::NewAccessTokenResponse;

use crate::SubscriptionType;

pub fn get_refresh_token(
  client_id: String,
  client_secret: String,
  scopes: &[Scope],
) -> (String, String) {
  let scopes = "user%3Aread%3Achat";
  let request = format!(
    "https://id.twitch.tv/oauth2/authorize?response_type=code&client_id={}&redirect_uri=http://localhost:8080&scope={}",
    client_id, scopes
  );

  let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to start HTTP server.");

  open::that(request).expect("Failed to open authorization page.");
  let (mut connection, _) = listener.accept().expect("Failed to establish HTTP connection.");
  let reader = BufReader::new(&connection);

  let request = reader.lines().next().unwrap().unwrap();
  let (code, _) = request.strip_prefix("GET /?code=").and_then(|s| s.split_once("&")).unwrap();

  let response = "HTTP/1.1 200 OK\r\n\r\n";
  connection.write_all(response.as_bytes()).expect("Failed to send response.");

  let request = format!(
    "client_id={}&client_secret={}&code={}&grant_type=authorization_code&redirect_uri=http://localhost:8080",
    client_id, client_secret, code,
  );

  let response = ureq::post("https://id.twitch.tv/oauth2/token")
    .send_form(&[
      ("client_id", &client_id),
      ("client_secret", &client_secret),
      ("code", code),
      ("grant_type", "authorization_code"),
      ("redirect_uri", "http://localhost:8080"),
    ])
    .expect("Failed to request refresh token")
    .into_json::<NewAccessTokenResponse>()
    .expect("Failed to parse token request response");

  (
    response.access_token,
    response.refresh_token.expect("Token request response didn't contain refresh token"),
  )
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum Scope {
  AnalyticsReadExtensions,
  AnalyticsReadGames,
  BitsRead,
  ChannelBot,
  ChannelManageAds,
  ChannelReadAds,
  ChannelManageBroadcast,
  ChannelReadCharity,
  ChannelEditCommercial,
  ChannelReadEditors,
  ChannelManageExtensions,
  ChannelReadGoals,
  ChannelReadGuestStar,
  ChannelManageGuestStar,
  ChannelReadHypeTrain,
  ChannelManageModerators,
  ChannelReadPolls,
  ChannelManagePolls,
  ChannelReadPredictions,
  ChannelManagePredictions,
  ChannelManageRaids,
  ChannelReadRedemptions,
  ChannelManageRedemptions,
  ChannelManageSchedule,
  ChannelReadStreamKey,
  ChannelReadSubscriptions,
  ChannelManageVideos,
  ChannelReadVips,
  ChannelManageVips,
  ClipsEdit,
  ModerationRead,
  ModeratorManageAnnouncements,
  ModeratorManageAutomod,
  ModeratorReadAutomodSettings,
  ModeratorManageAutomodSettings,
  ModeratorReadBannedUsers,
  ModeratorManageBannedUsers,
  ModeratorReadBlockedTerms,
  ModeratorReadChatMessages,
  ModeratorManageBlockedTerms,
  ModeratorManageChatMessages,
  ModeratorReadChatSettings,
  ModeratorManageChatSettings,
  ModeratorReadChatters,
  ModeratorReadFollowers,
  ModeratorReadGuestStar,
  ModeratorManageGuestStar,
  ModeratorReadModerators,
  ModeratorReadShieldMode,
  ModeratorManageShieldMode,
  ModeratorReadShoutouts,
  ModeratorManageShoutouts,
  ModeratorReadSuspiciousUsers,
  ModeratorReadUnbanRequests,
  ModeratorManageUnbanRequests,
  ModeratorReadVips,
  ModeratorReadWarnings,
  ModeratorManageWarnings,
  UserBot,
  UserEdit,
  UserEditBroadcast,
  UserReadBlockedUsers,
  UserManageBlockedUsers,
  UserReadBroadcast,
  UserReadChat,
  UserManageChatColor,
  UserReadEmail,
  UserReadEmotes,
  UserReadFollows,
  UserReadModeratedChannels,
  UserReadSubscriptions,
  UserReadWhispers,
  UserManageWhispers,
  UserWriteChat,
}

impl Scope {
  fn scope(&self) -> &'static str {
    match self {
      Scope::AnalyticsReadExtensions => "analytics:read:extensions",
      Scope::AnalyticsReadGames => "analytics:read:games",
      Scope::BitsRead => "bits:read",
      Scope::ChannelBot => "channel:bot",
      Scope::ChannelManageAds => "channel:manage:ads",
      Scope::ChannelReadAds => "channel:read:ads",
      Scope::ChannelManageBroadcast => "channel:manage:broadcast",
      Scope::ChannelReadCharity => "channel:read:charity",
      Scope::ChannelEditCommercial => "channel:edit:commercial",
      Scope::ChannelReadEditors => "channel:read:editors",
      Scope::ChannelManageExtensions => "channel:manage:extensions",
      Scope::ChannelReadGoals => "channel:read:goals",
      Scope::ChannelReadGuestStar => "channel:read:guest_star",
      Scope::ChannelManageGuestStar => "channel:manage:guest_star",
      Scope::ChannelReadHypeTrain => "channel:read:hype_train",
      Scope::ChannelManageModerators => "channel:manage:moderators",
      Scope::ChannelReadPolls => "channel:read:polls",
      Scope::ChannelManagePolls => "channel:manage:polls",
      Scope::ChannelReadPredictions => "channel:read:predictions",
      Scope::ChannelManagePredictions => "channel:manage:predictions",
      Scope::ChannelManageRaids => "channel:manage:raids",
      Scope::ChannelReadRedemptions => "channel:read:redemptions",
      Scope::ChannelManageRedemptions => "channel:manage:redemptions",
      Scope::ChannelManageSchedule => "channel:manage:schedule",
      Scope::ChannelReadStreamKey => "channel:read:stream_key",
      Scope::ChannelReadSubscriptions => "channel:read:subscriptions",
      Scope::ChannelManageVideos => "channel:manage:videos",
      Scope::ChannelReadVips => "channel:read:vips	",
      Scope::ChannelManageVips => "channel:manage:vips",
      Scope::ClipsEdit => "clips:edit",
      Scope::ModerationRead => "moderation:read",
      Scope::ModeratorManageAnnouncements => "moderator:manage:announcements",
      Scope::ModeratorManageAutomod => "moderator:manage:automod",
      Scope::ModeratorReadAutomodSettings => "moderator:read:automod_settings	",
      Scope::ModeratorManageAutomodSettings => "moderator:manage:automod_settings",
      Scope::ModeratorReadBannedUsers => "moderator:read:banned_users	",
      Scope::ModeratorManageBannedUsers => "moderator:manage:banned_users",
      Scope::ModeratorReadBlockedTerms => "moderator:read:blocked_terms",
      Scope::ModeratorReadChatMessages => "moderator:read:chat_messages",
      Scope::ModeratorManageBlockedTerms => "moderator:manage:blocked_terms",
      Scope::ModeratorManageChatMessages => "moderator:manage:chat_messages",
      Scope::ModeratorReadChatSettings => "moderator:read:chat_settings",
      Scope::ModeratorManageChatSettings => "moderator:manage:chat_settings",
      Scope::ModeratorReadChatters => "moderator:read:chatters	",
      Scope::ModeratorReadFollowers => "moderator:read:followers",
      Scope::ModeratorReadGuestStar => "moderator:read:guest_star	",
      Scope::ModeratorManageGuestStar => "moderator:manage:guest_star",
      Scope::ModeratorReadModerators => "moderator:read:moderators	",
      Scope::ModeratorReadShieldMode => "moderator:read:shield_mode",
      Scope::ModeratorManageShieldMode => "moderator:manage:shield_mode",
      Scope::ModeratorReadShoutouts => "moderator:read:shoutouts",
      Scope::ModeratorManageShoutouts => "moderator:manage:shoutouts",
      Scope::ModeratorReadSuspiciousUsers => "moderator:read:suspicious_users",
      Scope::ModeratorReadUnbanRequests => "moderator:read:unban_requests	",
      Scope::ModeratorManageUnbanRequests => "moderator:manage:unban_requests",
      Scope::ModeratorReadVips => "moderator:read:vips",
      Scope::ModeratorReadWarnings => "moderator:read:warnings	",
      Scope::ModeratorManageWarnings => "moderator:manage:warnings",
      Scope::UserBot => "user:bot",
      Scope::UserEdit => "user:edit",
      Scope::UserEditBroadcast => "user:edit:broadcast",
      Scope::UserReadBlockedUsers => "user:read:blocked_users	",
      Scope::UserManageBlockedUsers => "user:manage:blocked_users",
      Scope::UserReadBroadcast => "user:read:broadcast",
      Scope::UserReadChat => "user:read:chat",
      Scope::UserManageChatColor => "user:manage:chat_color",
      Scope::UserReadEmail => "user:read:email	",
      Scope::UserReadEmotes => "user:read:emotes	",
      Scope::UserReadFollows => "user:read:follows",
      Scope::UserReadModeratedChannels => "user:read:moderated_channels",
      Scope::UserReadSubscriptions => "user:read:subscriptions",
      Scope::UserReadWhispers => "user:read:whispers",
      Scope::UserManageWhispers => "user:manage:whispers",
      Scope::UserWriteChat => "user:write:chat",
    }
  }
}

impl From<SubscriptionType> for Scope {
  fn from(value: SubscriptionType) -> Self {
    match value {
      SubscriptionType::Follow => Scope::ModeratorReadFollowers,
      SubscriptionType::AdBreakBegin => Scope::ChannelReadAds,
      SubscriptionType::ChatClear => todo!(),
      SubscriptionType::ChatClearUserMessages => todo!(),
      SubscriptionType::ChatMessage => todo!(),
      SubscriptionType::ChatMessageDelete => todo!(),
      SubscriptionType::Subscribe => todo!(),
      SubscriptionType::SubscriptionGift => todo!(),
      SubscriptionType::SubscriptionMessage => todo!(),
      SubscriptionType::Cheer => todo!(),
      SubscriptionType::Raid => todo!(),
      SubscriptionType::PointsCustomRewardRedemptionAdd => todo!(),
      SubscriptionType::PollBegin => todo!(),
      SubscriptionType::PollProgress => todo!(),
      SubscriptionType::PollEnd => todo!(),
      SubscriptionType::PredictionBegin => todo!(),
      SubscriptionType::PredictionProgress => todo!(),
      SubscriptionType::PredictionLock => todo!(),
      SubscriptionType::PredictionEnd => todo!(),
      SubscriptionType::CharityDonation => todo!(),
      SubscriptionType::HypeTrainBegin => todo!(),
      SubscriptionType::HypeTrainProgress => todo!(),
      SubscriptionType::HypeTrainEnd => todo!(),
      SubscriptionType::ShoutoutReceived => todo!(),
    }
  }
}
