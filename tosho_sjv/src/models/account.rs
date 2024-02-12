use serde::{Deserialize, Serialize};

/// A login result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountLoginResponse {
    #[serde(rename = "user_id")]
    pub id: u32,
    #[serde(rename = "login")]
    pub username: String,
    pub session_id: String,
    #[serde(rename = "trust_user_jwt")]
    pub token: String,
    #[serde(rename = "trust_user_id_token")]
    pub id_token: String,
    #[serde(rename = "firebase_auth_jwt")]
    pub firebase_token: String,
}
