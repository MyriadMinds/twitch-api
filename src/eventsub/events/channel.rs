use serde::de::Error;
use serde::{Deserialize, Serialize};

use super::{
  User, broadcaster, from_broadcaster, maybe_string, moderator, requester, to_broadcaster,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelUpdate {
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster: User,
  pub title: String,
  pub language: String,
  pub category_id: String,
  pub category_name: String,
  pub content_classification_labels: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Follow {
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster: User,
  #[serde(flatten)]
  pub user:        User,
  pub followed_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdBreakBegin {
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster:      User,
  #[serde(flatten, with = "requester")]
  pub requester:        User,
  #[serde(deserialize_with = "maybe_string")]
  pub duration_seconds: u64,
  pub started_at:       String,
  #[serde(deserialize_with = "maybe_string")]
  pub is_automatic:     bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cheer {
  #[serde(flatten)]
  pub user:        Cheerer,
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster: User,
  pub message:     String,
  #[serde(deserialize_with = "maybe_string")]
  pub bits:        u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Raid {
  #[serde(flatten, with = "from_broadcaster")]
  pub from_broadcaster: User,
  #[serde(flatten, with = "to_broadcaster")]
  pub to_broadcaster:   User,
  #[serde(deserialize_with = "maybe_string")]
  pub viewers:          u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModeratorAdd {
  #[serde(flatten, with = "broadcaster")]
  pub from_broadcaster: User,
  #[serde(flatten)]
  pub user:             User,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModeratorRemove {
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster: User,
  #[serde(flatten)]
  pub user:        User,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VIPAdd {
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster: User,
  #[serde(flatten)]
  pub user:        User,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VIPRemove {
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster: User,
  #[serde(flatten)]
  pub user:        User,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShoutoutCreate {
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster:             User,
  #[serde(flatten, with = "to_broadcaster")]
  pub to_broadcaster:          User,
  #[serde(flatten, with = "moderator")]
  pub moderator:               User,
  #[serde(deserialize_with = "maybe_string")]
  pub viewer_count:            u64,
  pub started_at:              String,
  pub cooldown_ends_at:        String,
  pub target_cooldown_ends_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShoutoutReceived {
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster:      User,
  #[serde(flatten, with = "from_broadcaster")]
  pub from_broadcaster: User,
  #[serde(deserialize_with = "maybe_string")]
  pub viewer_count:     u64,
  pub started_at:       String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StreamOnline {
  pub id:          String,
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster: User,
  #[serde(rename = "type")]
  pub stream_type: StreamType,
  pub started_at:  String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StreamOffline {
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster: User,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum StreamType {
  Live,
  Playlist,
  WatchPart,
  Premiere,
  Rerun,
}

#[derive(Debug, Clone)]
pub enum Cheerer {
  Cheerer { user: User },
  Anonymous,
}

impl Serialize for Cheerer {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    #[derive(Default, Serialize)]
    struct InnerCheerer<'a> {
      user_id:      Option<&'a str>,
      user_name:    Option<&'a str>,
      user_login:   Option<&'a str>,
      is_anonymous: bool,
    }

    match self {
      Self::Cheerer { user } => InnerCheerer {
        user_id:      Some(&user.user_id),
        user_name:    Some(&user.user_name),
        user_login:   Some(&user.user_login),
        is_anonymous: false,
      },
      Self::Anonymous => InnerCheerer { is_anonymous: true, ..Default::default() },
    }
    .serialize(serializer)
  }
}

impl<'de> Deserialize<'de> for Cheerer {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    #[derive(Deserialize)]
    struct InnerCheerer {
      #[serde(rename = "gifter_user_id")]
      user_id:      Option<String>,
      #[serde(rename = "gifter_user_name")]
      user_name:    Option<String>,
      #[serde(rename = "gifter_user_login")]
      user_login:   Option<String>,
      #[serde(rename = "gifter_is_anonymous")]
      is_anonymous: bool,
    }
    let cheerer = InnerCheerer::deserialize(deserializer)?;

    let error = |field: &'static str| move || D::Error::missing_field(field);
    if cheerer.is_anonymous {
      return Ok(Self::Anonymous);
    }

    Ok(Self::Cheerer {
      user: User {
        user_id:    cheerer.user_id.ok_or_else(error("gifter_user_id"))?,
        user_name:  cheerer.user_name.ok_or_else(error("gifter_user_name"))?,
        user_login: cheerer.user_login.ok_or_else(error("gifter_user_login"))?,
      },
    })
  }
}

//Tests
////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn channel_update() {
    let event = r#"
    {
      "broadcaster_user_id": "1337",
      "broadcaster_user_login": "cool_user",
      "broadcaster_user_name": "Cool_User",
      "title": "Best Stream Ever",
      "language": "en",
      "category_id": "12453",
      "category_name": "Grand Theft Auto",
      "content_classification_labels": [ "MatureGame" ]
    }"#;
    serde_json::from_str::<ChannelUpdate>(event).unwrap();
  }

  #[test]
  fn channel_follow() {
    let event = r#"
    {
      "user_id": "1234",
      "user_login": "cool_user",
      "user_name": "Cool_User",
      "broadcaster_user_id": "1337",
      "broadcaster_user_login": "cooler_user",
      "broadcaster_user_name": "Cooler_User",
      "followed_at": "2020-07-15T18:16:11.17106713Z"
    }"#;
    serde_json::from_str::<Follow>(event).unwrap();
  }

  #[test]
  fn channel_ad_break_begin() {
    let event = r#"
    {
      "duration_seconds": "60",
      "started_at": "2019-11-16T10:11:12.634234626Z",
      "is_automatic": "false",
      "broadcaster_user_id": "1337",
      "broadcaster_user_login": "cool_user",
      "broadcaster_user_name": "Cool_User",
      "requester_user_id": "1337",
      "requester_user_login": "cool_user",
      "requester_user_name": "Cool_User"
    }"#;
    serde_json::from_str::<AdBreakBegin>(event).unwrap();
  }

  #[test]
  fn channel_moderator_add() {
    let event = r#"
    {
      "user_id": "1234",
      "user_login": "mod_user",
      "user_name": "Mod_User",
      "broadcaster_user_id": "1337",
      "broadcaster_user_login": "cooler_user",
      "broadcaster_user_name": "Cooler_User"
    }"#;
    serde_json::from_str::<ModeratorAdd>(event).unwrap();
  }

  #[test]
  fn channel_moderator_remove() {
    let event = r#"
    {
      "user_id": "1234",
      "user_login": "not_mod_user",
      "user_name": "Not_Mod_User",
      "broadcaster_user_id": "1337",
      "broadcaster_user_login": "cooler_user",
      "broadcaster_user_name": "Cooler_User"
    }"#;
    serde_json::from_str::<ModeratorRemove>(event).unwrap();
  }

  #[test]
  fn channel_vip_add() {
    let event = r#"
    {
      "user_id": "1234",
      "user_login": "mod_user",
      "user_name": "Mod_User",
      "broadcaster_user_id": "1337",
      "broadcaster_user_login": "cooler_user",
      "broadcaster_user_name": "Cooler_User"
    }"#;
    serde_json::from_str::<VIPAdd>(event).unwrap();
  }

  #[test]
  fn channel_vip_remove() {
    let event = r#"
    {
      "user_id": "1234",
      "user_login": "mod_user",
      "user_name": "Mod_User",
      "broadcaster_user_id": "1337",
      "broadcaster_user_login": "cooler_user",
      "broadcaster_user_name": "Cooler_User"
    }"#;
    serde_json::from_str::<VIPRemove>(event).unwrap();
  }

  #[test]
  fn channel_shoutout_create() {
    let event = r#"
    {
      "broadcaster_user_id": "12345",
      "broadcaster_user_name": "SimplySimple",
      "broadcaster_user_login": "simplysimple",
      "moderator_user_id": "98765",
      "moderator_user_name": "ParticularlyParticular123",
      "moderator_user_login": "particularlyparticular123",
      "to_broadcaster_user_id": "626262",
      "to_broadcaster_user_name": "SandySanderman",
      "to_broadcaster_user_login": "sandysanderman",
      "started_at": "2022-07-26T17:00:03.17106713Z",
      "viewer_count": 860,
      "cooldown_ends_at": "2022-07-26T17:02:03.17106713Z",
      "target_cooldown_ends_at":"2022-07-26T18:00:03.17106713Z"
    }"#;
    serde_json::from_str::<ShoutoutCreate>(event).unwrap();
  }

  #[test]
  fn channel_shoutout_received() {
    let event = r#"
    {
      "broadcaster_user_id": "626262",
      "broadcaster_user_name": "SandySanderman",
      "broadcaster_user_login": "sandysanderman",
      "from_broadcaster_user_id": "12345",
      "from_broadcaster_user_name": "SimplySimple",
      "from_broadcaster_user_login": "simplysimple",
      "viewer_count": 860,
      "started_at": "2022-07-26T17:00:03.17106713Z"
    }"#;
    serde_json::from_str::<ShoutoutReceived>(event).unwrap();
  }

  #[test]
  fn channel_stream_online() {
    let event = r#"
    {
      "id": "9001",
      "broadcaster_user_id": "1337",
      "broadcaster_user_login": "cool_user",
      "broadcaster_user_name": "Cool_User",
      "type": "live",
      "started_at": "2020-10-11T10:11:12.123Z"
    }"#;
    serde_json::from_str::<StreamOnline>(event).unwrap();
  }

  #[test]
  fn channel_stream_offline() {
    let event = r#"
    {
      "broadcaster_user_id": "1337",
      "broadcaster_user_login": "cool_user",
      "broadcaster_user_name": "Cool_User"
    }"#;
    serde_json::from_str::<StreamOffline>(event).unwrap();
  }
}
