use serde::{Deserialize, Serialize};

use super::{Message, User, broadcaster, maybe_string, moderator, source_broadcaster};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ban {
  #[serde(flatten)]
  pub user:         User,
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster:  User,
  #[serde(flatten, with = "moderator")]
  pub moderator:    User,
  pub reason:       String,
  pub banned_at:    String,
  pub ends_at:      String,
  #[serde(deserialize_with = "maybe_string")]
  pub is_permanent: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Unban {
  #[serde(flatten)]
  pub user:        User,
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster: User,
  #[serde(flatten, with = "moderator")]
  pub moderator:   User,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnbanRequestCreate {
  pub id:          String,
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster: User,
  #[serde(flatten)]
  pub user:        User,
  pub text:        String,
  pub created_at:  String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnbanRequestResolve {
  pub id:              String,
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster:     User,
  #[serde(flatten, with = "moderator")]
  pub moderator:       Option<User>,
  #[serde(flatten)]
  pub user:            User,
  pub resolution_text: Option<String>,
  pub status:          UnbanStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Moderate {
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster:        User,
  #[serde(flatten, with = "source_broadcaster")]
  pub source_broadcaster: User,
  #[serde(flatten, with = "moderator")]
  pub moderator:          User,
  #[serde(flatten)]
  pub action:             Action,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SuspiciousUserMessage {
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster:            User,
  #[serde(flatten)]
  pub user:                   User,
  pub low_trust_status:       TrustStatus,
  pub shared_ban_channel_ids: Vec<String>,
  pub types:                  Vec<SuspicionTypes>,
  pub ban_evasion_evaluation: EvasionEvaluation,
  pub message:                Message,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SuspiciousUserUpdate {
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster:      User,
  #[serde(flatten, with = "moderator")]
  pub moderator:        User,
  #[serde(flatten)]
  pub user:             User,
  pub low_trust_status: TrustStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WarningAcknowledge {
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster: User,
  #[serde(flatten)]
  pub user:        User,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WarningSend {
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster:  User,
  #[serde(flatten, with = "moderator")]
  pub moderator:    User,
  #[serde(flatten)]
  pub user:         User,
  reason:           Option<String>,
  chat_rules_cited: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShieldModeBegin {
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster: User,
  #[serde(flatten, with = "moderator")]
  pub moderator:   User,
  pub started_at:  String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShieldModeEnd {
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster: User,
  #[serde(flatten, with = "moderator")]
  pub moderator:   User,
  pub ended_at:    String,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum UnbanStatus {
  Approved,
  Canceled,
  Denied,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum Action {
  Ban {
    ban: BanAction,
  },
  Unban {
    unban: BanAction,
  },
  #[serde(rename = "timeout")]
  Timeout {
    timeout: TimeoutAction,
  },
  Untimeout {
    untimeout: UntimeoutAction,
  },
  Clear,
  Emoteonly,
  EmoteonlyOff,
  Followers {
    followers: FollowersAction,
  },
  FollowersOff,
  Uniquechat,
  UniquechatOff,
  Slow {
    slow: SlowAction,
  },
  SlowOff,
  Subscribers,
  SubscribersOff,
  Vip {
    vip: VipAction,
  },
  Unvip {
    unvip: VipAction,
  },
  Raid {
    raid: RaidAction,
  },
  Unraid {
    unraid: UnraidAction,
  },
  Mod {
    #[serde(rename = "mod")]
    new_mod: ModAction,
  },
  Unmod {
    unmod: ModAction,
  },
  Delete {
    delete: DeleteAction,
  },
  Warn {
    warn: WarnAction,
  },
  AddBlockedTerm {
    automod_terms: TermAction,
  },
  AddPermittedTerm {
    automod_terms: TermAction,
  },
  RemoveBlockedTerm {
    automod_terms: TermAction,
  },
  RemovePermittedTerm {
    automod_terms: TermAction,
  },
  ApproveUnbanRequest {
    unban_request: UnbanRequestAction,
  },
  DenyUnbanRequest {
    unban_request: UnbanRequestAction,
  },
  SharedChatBan {
    shared_chat_ban: BanAction,
  },
  SharedChatUnban {
    shared_chat_unban: BanAction,
  },
  SharedChatTimeout {
    shared_chat_timeout: TimeoutAction,
  },
  SharedChatUntimeout {
    shared_chat_untimeout: UntimeoutAction,
  },
  SharedChatDelete {
    shared_chat_delete: DeleteAction,
  },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BanAction {
  #[serde(flatten)]
  pub user:   User,
  pub reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeoutAction {
  #[serde(flatten)]
  pub user:       User,
  pub reason:     Option<String>,
  pub expires_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UntimeoutAction {
  #[serde(flatten)]
  pub user: User,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FollowersAction {
  #[serde(deserialize_with = "maybe_string")]
  pub follow_duration_minutes: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SlowAction {
  #[serde(deserialize_with = "maybe_string")]
  pub wait_time_seconds: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VipAction {
  #[serde(flatten)]
  pub user: User,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RaidAction {
  #[serde(flatten)]
  pub user:         User,
  #[serde(deserialize_with = "maybe_string")]
  pub viewer_count: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnraidAction {
  #[serde(flatten)]
  pub user: User,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModAction {
  #[serde(flatten)]
  pub user: User,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteAction {
  #[serde(flatten)]
  pub user:         User,
  pub message_id:   String,
  pub message_body: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WarnAction {
  #[serde(flatten)]
  pub user:             User,
  pub reason:           Option<String>,
  pub chat_rules_cited: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TermAction {
  pub terms:        Vec<String>,
  #[serde(deserialize_with = "maybe_string")]
  pub from_automod: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnbanRequestAction {
  #[serde(flatten)]
  pub user:              User,
  pub moderator_message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TrustStatus {
  None,
  ActiveMonitoring,
  Restricted,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SuspicionTypes {
  Manual,
  BanEvader,
  SharedChannelBan,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum EvasionEvaluation {
  Unknown,
  Possible,
  Likely,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn channel_ban() {
    let event = r#"
    {
      "user_id": "1234",
      "user_login": "cool_user",
      "user_name": "Cool_User",
      "broadcaster_user_id": "1337",
      "broadcaster_user_login": "cooler_user",
      "broadcaster_user_name": "Cooler_User",
      "moderator_user_id": "1339",
      "moderator_user_login": "mod_user",
      "moderator_user_name": "Mod_User",
      "reason": "Offensive language",
      "banned_at": "2020-07-15T18:15:11.17106713Z",
      "ends_at": "2020-07-15T18:16:11.17106713Z",
      "is_permanent": false
    }"#;
    serde_json::from_str::<Ban>(event).unwrap();
  }

  #[test]
  fn channel_unban() {
    let event = r#"
    {
      "user_id": "1234",
      "user_login": "cool_user",
      "user_name": "Cool_User",
      "broadcaster_user_id": "1337",
      "broadcaster_user_login": "cooler_user",
      "broadcaster_user_name": "Cooler_User",
      "moderator_user_id": "1339",
      "moderator_user_login": "mod_user",
      "moderator_user_name": "Mod_User"
    }"#;
    serde_json::from_str::<Unban>(event).unwrap();
  }

  #[test]
  fn channel_unban_request_create() {
    let event = r#"
    {
      "id": "60",
      "broadcaster_user_id": "1337",
      "broadcaster_user_login": "cool_user",
      "broadcaster_user_name": "Cool_User",
      "user_id": "1339",
      "user_login": "not_cool_user",
      "user_name": "Not_Cool_User",
      "text": "unban me",
      "created_at": "2023-11-16T10:11:12.634234626Z"
    }"#;
    serde_json::from_str::<UnbanRequestCreate>(event).unwrap();
  }

  #[test]
  fn channel_unban_request_resolve() {
    let event = r#"
    {
      "id": "60",
      "broadcaster_user_id": "1337",
      "broadcaster_user_login": "cool_user",
      "broadcaster_user_name": "Cool_User",
      "moderator_user_id": "1337",
      "moderator_user_login": "cool_user",
      "moderator_user_name": "Cool_User",
      "user_id": "1339",
      "user_login": "not_cool_user",
      "user_name": "Not_Cool_User",
      "resolution_text": "no",
      "status": "denied"
    }"#;
    serde_json::from_str::<UnbanRequestResolve>(event).unwrap();
  }

  #[test]
  fn channel_moderate_events() {
    let event = r#"
    {
      "broadcaster_user_id": "423374343",
      "broadcaster_user_login": "glowillig",
      "broadcaster_user_name": "glowillig",
      "source_broadcaster_user_id": "41292030",
      "source_broadcaster_user_login": "adflynn404",
      "source_broadcaster_user_name": "adflynn404",
      "moderator_user_id": "424596340",
      "moderator_user_login": "quotrok",
      "moderator_user_name": "quotrok",
      "action": "warn",
      "followers": null,
      "slow": null,
      "vip": null,
      "unvip": null,
      "warn": {
        "user_id": "141981764",
        "user_login": "twitchdev",
        "user_name": "TwitchDev",
        "reason": "cut it out",
        "chat_rules_cited": null
      },
      "unmod": null,
      "ban": null,
      "unban": null,
      "timeout": null,
      "untimeout": null,
      "raid": null,
      "unraid": null,
      "delete": null,
      "automod_terms": null,
      "unban_request": null,
      "shared_chat_ban": null,
      "shared_chat_unban": null,
      "shared_chat_timeout": null,
      "shared_chat_untimeout": null,
      "shared_chat_delete": null
    }"#;
    serde_json::from_str::<Moderate>(event).unwrap();

    let event = r#"
    {
      "broadcaster_user_id": "423374343",
      "broadcaster_user_login": "glowillig",
      "broadcaster_user_name": "glowillig",
      "source_broadcaster_user_id": "41292030",
      "source_broadcaster_user_login": "adflynn404",
      "source_broadcaster_user_name": "adflynn404",
      "moderator_user_id": "424596340",
      "moderator_user_login": "quotrok",
      "moderator_user_name": "quotrok",
      "action": "shared_chat_timeout",
      "followers": null,
      "slow": null,
      "vip": null,
      "unvip": null,
      "warn": null,
      "unmod": null,
      "ban": null,
      "unban": null,
      "timeout": null,
      "untimeout": null,
      "raid": null,
      "unraid": null,
      "delete": null,
      "automod_terms": null,
      "unban_request": null,
      "shared_chat_ban": null,
      "shared_chat_unban": null,
      "shared_chat_timeout": {
          "user_id": "141981764",
          "user_login": "twitchdev",
          "user_name": "TwitchDev",
          "reason": "Has never seen the Harry Potter films.",
          "expires_at": "2022-03-15T02:00:28Z"
      },
      "shared_chat_untimeout": null,
      "shared_chat_delete": null
    }"#;
    serde_json::from_str::<Moderate>(event).unwrap();

    let event = r#"
    {
      "broadcaster_user_id": "423374343",
      "broadcaster_user_login": "glowillig",
      "broadcaster_user_name": "glowillig",
      "source_broadcaster_user_id": "41292030",
      "source_broadcaster_user_login": "adflynn404",
      "source_broadcaster_user_name": "adflynn404",
      "moderator_user_id": "424596340",
      "moderator_user_login": "quotrok",
      "moderator_user_name": "quotrok",
      "action": "mod",
      "followers": null,
      "slow": null,
      "vip": null,
      "unvip": null,
      "warn": null,
      "mod": {
        "user_id": "141981764",
        "user_login": "twitchdev",
        "user_name": "TwitchDev"
      },
      "unmod": null,
      "ban": null,
      "unban": null,
      "timeout": null,
      "untimeout": null,
      "raid": null,
      "unraid": null,
      "delete": null,
      "automod_terms": null,
      "unban_request": null,
      "shared_chat_ban": null,
      "shared_chat_unban": null,
      "shared_chat_timeout": null,
      "shared_chat_untimeout": null,
      "shared_chat_delete": null
    }"#;
    serde_json::from_str::<Moderate>(event).unwrap();

    let event = r#"
    {
      "broadcaster_user_id": "423374343",
      "broadcaster_user_login": "glowillig",
      "broadcaster_user_name": "glowillig",
      "source_broadcaster_user_id": "41292030",
      "source_broadcaster_user_login": "adflynn404",
      "source_broadcaster_user_name": "adflynn404",
      "moderator_user_id": "424596340",
      "moderator_user_login": "quotrok",
      "moderator_user_name": "quotrok",
      "action": "timeout",
      "followers": null,
      "slow": null,
      "vip": null,
      "unvip": null,
      "mod": null,
      "unmod": null,
      "ban": null,
      "unban": null,
      "warn": null,
      "timeout": {
        "user_id": "141981764",
        "user_login": "twitchdev",
        "user_name": "TwitchDev",
        "reason": "Does not like pineapple on pizza.",
        "expires_at": "2022-03-15T02:00:28Z"
      },
      "untimeout": null,
      "raid": null,
      "unraid": null,
      "delete": null,
      "automod_terms": null,
      "unban_request": null,
      "shared_chat_ban": null,
      "shared_chat_unban": null,
      "shared_chat_timeout": null,
      "shared_chat_untimeout": null,
      "shared_chat_delete": null
    }"#;
    serde_json::from_str::<Moderate>(event).unwrap();

    let event = r#"
    {
      "broadcaster_user_id": "423374343",
      "broadcaster_user_login": "glowillig",
      "broadcaster_user_name": "glowillig",
      "source_broadcaster_user_id": "41292030",
      "source_broadcaster_user_login": "adflynn404",
      "source_broadcaster_user_name": "adflynn404",
      "moderator_user_id": "424596340",
      "moderator_user_login": "quotrok",
      "moderator_user_name": "quotrok",
      "action": "emoteonly",
      "followers": null,
      "slow": null,
      "vip": null,
      "unvip": null,
      "mod": null,
      "unmod": null,
      "ban": null,
      "unban": null,
      "warn": null,
      "timeout": null,
      "untimeout": null,
      "raid": null,
      "unraid": null,
      "delete": null,
      "automod_terms": null,
      "unban_request": null,
      "shared_chat_ban": null,
      "shared_chat_unban": null,
      "shared_chat_timeout": null,
      "shared_chat_untimeout": null,
      "shared_chat_delete": null
    }"#;
    serde_json::from_str::<Moderate>(event).unwrap();
  }

  #[test]
  fn channel_suspicious_user_message() {
    let event = r#"
    {
      "broadcaster_user_id": "1050263432",
      "broadcaster_user_name": "dcf9dd9336034d23b65",
      "broadcaster_user_login": "dcf9dd9336034d23b65",
      "user_id": "1050263434",
      "user_name": "4a46e2cf2e2f4d6a9e6",
      "user_login": "4a46e2cf2e2f4d6a9e6",
      "low_trust_status": "active_monitoring",
      "shared_ban_channel_ids": [
        "100",
        "200"
      ],
      "types": [
        "ban_evader"
      ],
      "ban_evasion_evaluation": "likely",
      "message": {
        "message_id": "101010",
        "text": "bad stuff pogchamp",
        "fragments": [
          {
            "type": "emote",
            "text": "bad stuff",
            "cheermote": null,
            "emote": {
              "id": "899",
              "emote_set_id": "1"
            }
          },
          {
            "type": "cheermote",
            "text": "pogchamp",
            "cheermote": {
              "prefix": "pogchamp",
              "bits": 100,
              "tier": 1
            },
            "emote": null
          }
        ]
      }
    }"#;
    serde_json::from_str::<SuspiciousUserMessage>(event).unwrap();
  }

  #[test]
  fn channel_suspicious_user_update() {
    let event = r#"
    {
      "broadcaster_user_id": "1050263435",
      "broadcaster_user_name": "77f111cbb75341449f5",
      "broadcaster_user_login": "77f111cbb75341449f5",
      "moderator_user_id": "1050263436",
      "moderator_user_name": "29087e59dfc441968f6",
      "moderator_user_login": "29087e59dfc441968f6",
      "user_id": "1050263437",
      "user_name": "06fbcc75952245c5a87",
      "user_login": "06fbcc75952245c5a87",
      "low_trust_status": "restricted"
    }"#;
    serde_json::from_str::<SuspiciousUserUpdate>(event).unwrap();
  }

  #[test]
  fn channel_warning_acknowledge() {
    let event = r#"
    {
      "broadcaster_user_id": "423374343",
      "broadcaster_user_login": "glowillig",
      "broadcaster_user_name": "glowillig",
      "user_id": "141981764",
      "user_login": "twitchdev",
      "user_name": "TwitchDev"
    }"#;
    serde_json::from_str::<WarningAcknowledge>(event).unwrap();
  }

  #[test]
  fn channel_warning_send() {
    let event = r#"
    {
      "broadcaster_user_id": "423374343",
      "broadcaster_user_login": "glowillig",
      "broadcaster_user_name": "glowillig",
      "moderator_user_id": "424596340",
      "moderator_user_login": "quotrok",
      "moderator_user_name": "quotrok",
      "user_id": "141981764",
      "user_login": "twitchdev",
      "user_name": "TwitchDev",
      "reason": "cut it out",
      "chat_rules_cited": null
    }"#;
    serde_json::from_str::<WarningSend>(event).unwrap();
  }

  #[test]
  fn channel_shieldmode_begin() {
    let event = r#"
    {
      "broadcaster_user_id": "12345",
      "broadcaster_user_name": "SimplySimple",
      "broadcaster_user_login": "simplysimple",
      "moderator_user_id": "98765",
      "moderator_user_name": "ParticularlyParticular123",
      "moderator_user_login": "particularlyparticular123",
      "started_at": "2022-07-26T17:00:03.17106713Z"
    }"#;
    serde_json::from_str::<ShieldModeBegin>(event).unwrap();
  }

  #[test]
  fn channel_shieldmode_end() {
    let event = r#"
    {
      "broadcaster_user_id": "12345",
      "broadcaster_user_name": "SimplySimple",
      "broadcaster_user_login": "simplysimple",
      "moderator_user_id": "98765",
      "moderator_user_name": "ParticularlyParticular123",
      "moderator_user_login": "particularlyparticular123",
      "ended_at": "2022-07-27T01:30:23.17106713Z"
    }"#;
    serde_json::from_str::<ShieldModeEnd>(event).unwrap();
  }
}
