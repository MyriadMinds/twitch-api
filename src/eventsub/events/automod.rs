use serde::{Deserialize, Serialize};

use super::{Message, User, broadcaster, maybe_string, moderator, owner_broadcaster};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutomodMessageHold {
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster: User,
  #[serde(flatten)]
  pub user:        User,
  pub message_id:  String,
  pub message:     Message,
  #[serde(flatten)]
  pub hold_reason: HoldReason,
  pub held_at:     String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutomodMessageUpdate {
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster: User,
  #[serde(flatten)]
  pub user:        User,
  #[serde(flatten, with = "moderator")]
  pub moderator:   User,
  pub message_id:  String,
  pub message:     Message,
  #[serde(flatten)]
  pub hold_reason: HoldReason,
  pub status:      UpdateStatus,
  pub held_at:     String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutomodSettingsUpdate {
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster:                User,
  #[serde(flatten, with = "moderator")]
  pub moderator:                  User,
  #[serde(deserialize_with = "maybe_string")]
  pub bullying:                   u32,
  pub overall_level:              Option<u32>,
  #[serde(deserialize_with = "maybe_string")]
  pub disability:                 u32,
  #[serde(deserialize_with = "maybe_string")]
  pub race_ethnicity_or_religion: u32,
  #[serde(deserialize_with = "maybe_string")]
  pub misogyny:                   u32,
  #[serde(deserialize_with = "maybe_string")]
  pub sexuality_sex_or_gender:    u32,
  #[serde(deserialize_with = "maybe_string")]
  pub aggression:                 u32,
  #[serde(deserialize_with = "maybe_string")]
  pub sex_based_terms:            u32,
  #[serde(deserialize_with = "maybe_string")]
  pub swearing:                   u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutomodTermsUpdate {
  #[serde(flatten, with = "broadcaster")]
  pub broadcaster:  User,
  #[serde(flatten, with = "moderator")]
  pub moderator:    User,
  pub action:       TermsAction,
  #[serde(deserialize_with = "maybe_string")]
  pub from_automod: bool,
  pub terms:        Vec<String>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "reason", rename_all = "snake_case")]
pub enum HoldReason {
  Automod { automod: ReasonAutomod },
  BlockedTerm { blocked_term: ReasonBlockedTerm },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReasonAutomod {
  pub category:   String,
  #[serde(deserialize_with = "maybe_string")]
  pub level:      u32,
  pub boundaries: Vec<Boundary>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReasonBlockedTerm {
  pub terms_found: Vec<Term>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Term {
  pub term_id:           String,
  pub boundary:          Boundary,
  #[serde(flatten, with = "owner_broadcaster")]
  pub owner_broadcaster: User,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Boundary {
  #[serde(deserialize_with = "maybe_string")]
  pub start_pos: u32,
  #[serde(deserialize_with = "maybe_string")]
  pub end_pos:   u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum UpdateStatus {
  Approved,
  Denied,
  Expired,
  Invalid,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TermsAction {
  AddPermitted,
  RemovePermitted,
  AddBlocked,
  RemoveBlocked,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn automod_message_hold() {
    let event = r#"
    {
      "broadcaster_user_id": "1337",
      "broadcaster_user_login": "blah",
      "broadcaster_user_name": "blahblah",
      "user_id": "4242",
      "user_login": "baduser",
      "user_name": "badbaduser",
      "message_id": "bad-message-id",
      "message": {
        "text": "This is a bad message… pogchamp",
        "fragments": [
          {
            "type": "text",
            "text": "This is a bad message… ",
            "cheermote": null,
            "emote": null
          },
          {
            "type": "cheermote",
            "text": "pogchamp",
            "cheermote": {
              "prefix": "pogchamp",
              "bits": 1000,
              "tier": 1
            },
            "emote": null
          },
          {
            "type": "emote",
            "text": "pogchamp",
            "cheermote": null,
            "emote": {
              "id": "1",
              "emote_set_id": "1",
              "owner_id": "1",
              "format": [
                "static",
                "animated"
              ]
            }
          }
        ]
      },
      "reason": "automod",
      "automod": {
        "category": "aggressive",
        "level": 1,
        "boundaries": [
          {
            "start_pos": 0,
            "end_pos": 10
          },
          {
            "start_pos": 20,
            "end_pos": 30
          }
        ]
      },
      "blocked_term": null,
      "held_at": "2022-12-02T15:00:00.00Z"
    }"#;
    serde_json::from_str::<AutomodMessageHold>(event).unwrap();
  }

  #[test]
  fn automod_message_update() {
    let event = r#"
    {
      "broadcaster_user_id": "1337",
      "broadcaster_user_login": "blah",
      "broadcaster_user_name": "blahblah",
      "moderator_user_id": "9001",
      "moderator_user_login": "the_mod",
      "moderator_user_name": "The_Mod",
      "user_id": "4242",
      "user_login": "baduser",
      "user_name": "badbaduser",
      "message_id": "bad-message-id",
      "message": {
        "text": "This is a bad message… pogchamp",
        "fragments": [
          {
            "type": "text",
            "text": "This is a bad message… ",
            "cheermote": null,
            "emote": null
          },
          {
            "type": "cheermote",
            "text": "pogchamp",
            "cheermote": {
              "prefix": "pogchamp",
              "bits": 1000,
              "tier": 1
            },
            "emote": null
          }
        ]
      },
      "reason": "blocked_term",
      "automod": null,
      "blocked_term": {
        "terms_found": [
          {
            "term_id": "123",
            "owner_broadcaster_user_id": "1337",
            "owner_broadcaster_user_login": "blah",
            "owner_broadcaster_user_name": "blahblah",
            "boundary": {
              "start_pos": 0,
              "end_pos": 30
            }
          }
        ]
      },
      "status": "approved",
      "held_at": "2022-12-02T15:00:00.00Z"
    }"#;
    serde_json::from_str::<AutomodMessageUpdate>(event).unwrap();
  }

  #[test]
  fn automod_settings_update() {
    let event = r#"
    {
      "broadcaster_user_id": "1337",
      "broadcaster_user_name": "CoolUser",
      "broadcaster_user_login": "cooluser",
      "moderator_user_id": "9001",
      "moderator_user_name": "CoolMod",
      "moderator_user_login": "coolmod",
      "overall_level": null,
      "disability": 3,
      "aggression": 3,
      "sexuality_sex_or_gender": 3,
      "misogyny": 3,
      "bullying": 3,
      "swearing": 0,
      "race_ethnicity_or_religion": 3,
      "sex_based_terms":30
    }"#;
    serde_json::from_str::<AutomodSettingsUpdate>(event).unwrap();
  }

  #[test]
  fn automod_terms_update() {
    let event = r#"
    {
      "broadcaster_user_id": "1337",
      "broadcaster_user_name": "blah",
      "broadcaster_user_login": "blahblah",
      "moderator_user_id": "9001",
      "moderator_user_login": "the_mod",
      "moderator_user_name": "The_Mod",
      "action": "add_blocked",
      "from_automod": true,
      "terms": ["automodterm1", "automodterm2", "automodterm3"]
    }"#;
    serde_json::from_str::<AutomodTermsUpdate>(event).unwrap();
  }
}
