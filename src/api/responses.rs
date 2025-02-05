use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewAccessTokenResponse {
  pub access_token:  String,
  pub refresh_token: Option<String>,
}
