use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

use bitmask_enum::bitmask;
use twitch_eventsub_structs::NewAccessTokenResponse;

use crate::SubscriptionType;

////////////////////////////////////////////////////////////////////////////////////////////////////

const RESPONSE_OK: &str = "HTTP/1.1 200 OK\r\n\r\n";
const RESPONSE_SCRIPT: &str = "\
  HTTP/1.1 200 OK\r\n\
  Content-Length: 247\r\n\r\n\
  <html><head></head><body><script>
		  var url_parts = String(window.location).split(\"#\");
		  if(url_parts.length > 1) {
			  var redirect_url = url_parts[0] + \"?\" + url_parts[1];
			  window.location = redirect_url;
		  }
	</script></body></html>\
";

pub fn get_access_token(client_id: String, scopes: Scope) -> String {
  let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to start HTTP server.");

  let request = format!(
    "https://id.twitch.tv/oauth2/authorize?response_type=token&client_id={}&redirect_uri=http://localhost:8080&scope={}",
    client_id,
    scopes.get_scopes()
  );
  open::that(request).expect("Failed to open authorization page.");

  let (connection, _) = listener.accept().expect("Failed to establish HTTP connection.");
  let _ = handle_connection(connection, RESPONSE_SCRIPT);

  let (connection, _) = listener.accept().expect("Failed to establish HTTP connection.");
  let request = handle_connection(connection, RESPONSE_OK);

  let (token, _) = request
    .first()
    .unwrap()
    .strip_prefix("GET /?access_token=")
    .and_then(|s| s.split_once("&"))
    .unwrap();

  token.to_string()
}

pub fn get_refresh_token(
  client_id: String,
  client_secret: String,
  scopes: Scope,
) -> (String, String) {
  let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to start HTTP server.");

  let request = format!(
    "https://id.twitch.tv/oauth2/authorize?response_type=code&client_id={}&redirect_uri=http://localhost:8080&scope={}",
    client_id,
    scopes.get_scopes()
  );
  open::that(request).expect("Failed to open authorization page.");

  let (connection, _) = listener.accept().expect("Failed to establish HTTP connection.");
  let request = handle_connection(connection, RESPONSE_OK);

  let (code, _) =
    request.first().unwrap().strip_prefix("GET /?code=").and_then(|s| s.split_once("&")).unwrap();
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

fn handle_connection(mut stream: TcpStream, response: &str) -> Vec<String> {
  let request = BufReader::new(&stream)
    .lines()
    .map(|result| result.unwrap())
    .take_while(|line| !line.is_empty())
    .collect();
  stream.write_all(response.as_bytes()).expect("Failed to send response.");

  request
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[bitmask(u128)]
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
  fn get_scopes(&self) -> String {
    Self::scopes()
      .filter_map(|&(value, scope)| self.contains(value).then(|| scope))
      .collect::<Vec<&'static str>>()
      .join("+")
  }

  fn scopes() -> impl Iterator<Item = &'static (Self, &'static str)> {
    [
      (Self::AnalyticsReadExtensions, "analytics%3Aread%3Aextensions"),
      (Self::AnalyticsReadGames, "analytics%3Aread%3Agames"),
      (Self::BitsRead, "bits%3Aread"),
      (Self::ChannelBot, "channel%3Abot"),
      (Self::ChannelManageAds, "channel%3Amanage%3Aads"),
      (Self::ChannelReadAds, "channel%3Aread%3Aads"),
      (Self::ChannelManageBroadcast, "channel%3Amanage%3Abroadcast"),
      (Self::ChannelReadCharity, "channel%3Aread%3Acharity"),
      (Self::ChannelEditCommercial, "channel%3Aedit%3Acommercial"),
      (Self::ChannelReadEditors, "channel%3Aread%3Aeditors"),
      (Self::ChannelManageExtensions, "channel%3Amanage%3Aextensions"),
      (Self::ChannelReadGoals, "channel%3Aread%3Agoals"),
      (Self::ChannelReadGuestStar, "channel%3Aread%3Aguest_star"),
      (Self::ChannelManageGuestStar, "channel%3Amanage%3Aguest_star"),
      (Self::ChannelReadHypeTrain, "channel%3Aread%3Ahype_train"),
      (Self::ChannelManageModerators, "channel%3Amanage%3Amoderators"),
      (Self::ChannelReadPolls, "channel%3Aread%3Apolls"),
      (Self::ChannelManagePolls, "channel%3Amanage%3Apolls"),
      (Self::ChannelReadPredictions, "channel%3Aread%3Apredictions"),
      (Self::ChannelManagePredictions, "channel%3Amanage%3Apredictions"),
      (Self::ChannelManageRaids, "channel%3Amanage%3Araids"),
      (Self::ChannelReadRedemptions, "channel%3Aread%3Aredemptions"),
      (Self::ChannelManageRedemptions, "channel%3Amanage%3Aredemptions"),
      (Self::ChannelManageSchedule, "channel%3Amanage%3Aschedule"),
      (Self::ChannelReadStreamKey, "channel%3Aread%3Astream_key"),
      (Self::ChannelReadSubscriptions, "channel%3Aread%3Asubscriptions"),
      (Self::ChannelManageVideos, "channel%3Amanage%3Avideos"),
      (Self::ChannelReadVips, "channel%3Aread%3Avips"),
      (Self::ChannelManageVips, "channel%3Amanage%3Avips"),
      (Self::ClipsEdit, "clips%3Aedit"),
      (Self::ModerationRead, "moderation%3Aread"),
      (Self::ModeratorManageAnnouncements, "moderator%3Amanage%3Aannouncements"),
      (Self::ModeratorManageAutomod, "moderator%3Amanage%3Aautomod"),
      (Self::ModeratorReadAutomodSettings, "moderator%3Aread%3Aautomod_settings"),
      (Self::ModeratorManageAutomodSettings, "moderator%3Amanage%3Aautomod_settings"),
      (Self::ModeratorReadBannedUsers, "moderator%3Aread%3Abanned_users"),
      (Self::ModeratorManageBannedUsers, "moderator%3Amanage%3Abanned_users"),
      (Self::ModeratorReadBlockedTerms, "moderator%3Aread%3Ablocked_terms"),
      (Self::ModeratorReadChatMessages, "moderator%3Aread%3Achat_messages"),
      (Self::ModeratorManageBlockedTerms, "moderator%3Amanage%3Ablocked_terms"),
      (Self::ModeratorManageChatMessages, "moderator%3Amanage%3Achat_messages"),
      (Self::ModeratorReadChatSettings, "moderator%3Aread%3Achat_settings"),
      (Self::ModeratorManageChatSettings, "moderator%3Amanage%3Achat_settings"),
      (Self::ModeratorReadChatters, "moderator%3Aread%3Achatters"),
      (Self::ModeratorReadFollowers, "moderator%3Aread%3Afollowers"),
      (Self::ModeratorReadGuestStar, "moderator%3Aread%3Aguest_star"),
      (Self::ModeratorManageGuestStar, "moderator%3Amanage%3Aguest_star"),
      (Self::ModeratorReadModerators, "moderator%3Aread%3Amoderators"),
      (Self::ModeratorReadShieldMode, "moderator%3Aread%3Ashield_mode"),
      (Self::ModeratorManageShieldMode, "moderator%3Amanage%3Ashield_mode"),
      (Self::ModeratorReadShoutouts, "moderator%3Aread%3Ashoutouts"),
      (Self::ModeratorManageShoutouts, "moderator%3Amanage%3Ashoutouts"),
      (Self::ModeratorReadSuspiciousUsers, "moderator%3Aread%3Asuspicious_users"),
      (Self::ModeratorReadUnbanRequests, "moderator%3Aread%3Aunban_requests"),
      (Self::ModeratorManageUnbanRequests, "moderator%3Amanage%3Aunban_requests"),
      (Self::ModeratorReadVips, "moderator%3Aread%3Avips"),
      (Self::ModeratorReadWarnings, "moderator%3Aread%3Awarnings"),
      (Self::ModeratorManageWarnings, "moderator%3Amanage%3Awarnings"),
      (Self::UserBot, "user%3Abot"),
      (Self::UserEdit, "user%3Aedit"),
      (Self::UserEditBroadcast, "user%3Aedit%3Abroadcast"),
      (Self::UserReadBlockedUsers, "user%3Aread%3Ablocked_users"),
      (Self::UserManageBlockedUsers, "user%3Amanage%3Ablocked_users"),
      (Self::UserReadBroadcast, "user%3Aread%3Abroadcast"),
      (Self::UserReadChat, "user%3Aread%3Achat"),
      (Self::UserManageChatColor, "user%3Amanage%3Achat_color"),
      (Self::UserReadEmail, "user%3Aread%3Aemail"),
      (Self::UserReadEmotes, "user%3Aread%3Aemotes"),
      (Self::UserReadFollows, "user%3Aread%3Afollows"),
      (Self::UserReadModeratedChannels, "user%3Aread%3Amoderated_channels"),
      (Self::UserReadSubscriptions, "user%3Aread%3Asubscriptions"),
      (Self::UserReadWhispers, "user%3Aread%3Awhispers"),
      (Self::UserManageWhispers, "user%3Amanage%3Awhispers"),
      (Self::UserWriteChat, "user%3Awrite%3Achat"),
    ]
    .iter()
  }
}

impl From<SubscriptionType> for Scope {
  fn from(value: SubscriptionType) -> Self {
    match value {
      SubscriptionType::Follow => Scope::ModeratorReadFollowers,
      SubscriptionType::AdBreakBegin => Scope::ChannelReadAds,
      SubscriptionType::ChatClear => Scope::UserReadChat,
      SubscriptionType::ChatClearUserMessages => Scope::UserReadChat,
      SubscriptionType::ChatMessage => Scope::UserReadChat,
      SubscriptionType::ChatMessageDelete => Scope::UserReadChat,
      SubscriptionType::Subscribe => Scope::ChannelReadSubscriptions,
      SubscriptionType::SubscriptionGift => Scope::ChannelReadSubscriptions,
      SubscriptionType::SubscriptionMessage => Scope::ChannelReadSubscriptions,
      SubscriptionType::Cheer => Scope::BitsRead,
      SubscriptionType::Raid => Scope::none(),
      SubscriptionType::PointsCustomRewardRedemptionAdd => Scope::ChannelReadRedemptions,
      SubscriptionType::PollBegin => Scope::ChannelReadPolls,
      SubscriptionType::PollProgress => Scope::ChannelReadPolls,
      SubscriptionType::PollEnd => Scope::ChannelReadPolls,
      SubscriptionType::PredictionBegin => Scope::ChannelReadPredictions,
      SubscriptionType::PredictionProgress => Scope::ChannelReadPredictions,
      SubscriptionType::PredictionLock => Scope::ChannelReadPredictions,
      SubscriptionType::PredictionEnd => Scope::ChannelReadPredictions,
      SubscriptionType::CharityDonation => Scope::ChannelReadCharity,
      SubscriptionType::HypeTrainBegin => Scope::ChannelReadHypeTrain,
      SubscriptionType::HypeTrainProgress => Scope::ChannelReadHypeTrain,
      SubscriptionType::HypeTrainEnd => Scope::ChannelReadHypeTrain,
      SubscriptionType::ShoutoutCreate => Scope::ModeratorReadShoutouts,
    }
  }
}
