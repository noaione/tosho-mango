use serde::{Deserialize, Deserializer, Serialize};

use super::{IntBool, SubscriptionType};

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

/// An account subscription info.
///
/// This is a minimal representation of the subscription info.
/// Some field are discarded for simplicity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSubscription {
    #[serde(
        rename = "is_auto_renew",
        deserialize_with = "auto_renew_deser",
        serialize_with = "auto_renew_ser"
    )]
    pub sj_auto_renew: bool,
    #[serde(
        rename = "vm_is_auto_renew",
        deserialize_with = "auto_renew_deser",
        serialize_with = "auto_renew_ser"
    )]
    pub vm_auto_renew: bool,
    #[serde(rename = "valid_from")]
    pub sj_valid_from: Option<i64>,
    #[serde(rename = "valid_to")]
    pub sj_valid_to: Option<i64>,
    pub vm_valid_from: Option<i64>,
    pub vm_valid_to: Option<i64>,
}

impl AccountSubscription {
    /// Check if SJ subscription is active.
    pub fn is_sj_active(&self) -> bool {
        if let (Some(from), Some(to)) = (self.sj_valid_from, self.sj_valid_to) {
            let now = chrono::Utc::now().timestamp();
            now >= from && now <= to
        } else {
            false
        }
    }

    /// Check if VM subscription is active.
    pub fn is_vm_active(&self) -> bool {
        if let (Some(from), Some(to)) = (self.vm_valid_from, self.vm_valid_to) {
            let now = chrono::Utc::now().timestamp();
            now >= from && now <= to
        } else {
            false
        }
    }
}

fn auto_renew_deser<'de, D>(d: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(d)?;
    match s.as_str() {
        "yes" => Ok(true),
        "no" => Ok(false),
        _ => Err(serde::de::Error::custom(format!(
            "Invalid auto renew value: {}",
            s
        ))),
    }
}

fn auto_renew_ser<S>(v: &bool, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    if *v {
        s.serialize_str("yes")
    } else {
        s.serialize_str("no")
    }
}

/// An account subscription info.
///
/// This is a minimal representation of the subscription info.
/// Some field are discarded for simplicity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountArchive {
    pub ok: IntBool,
    pub subscription_type: SubscriptionType,
    #[serde(rename = "archive_limit")]
    pub read_limit: u32,
    #[serde(rename = "archive_reset_seconds")]
    pub read_reset: u32,
    pub download_limit: u32,
    #[serde(rename = "download_expire_seconds")]
    pub download_expire: u32,
    #[serde(rename = "next_reset_epoch")]
    pub next_reset: i64,
    #[serde(rename = "num_remaining")]
    pub remaining: u32,
}

/// A response for account entitlements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountEntitlementsResponse {
    #[serde(rename = "subscription_info")]
    pub subscriptions: AccountSubscription,
    #[serde(rename = "archive_info")]
    pub archive: AccountArchive,
}
