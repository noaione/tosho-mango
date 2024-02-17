//! A module containing information related to user account.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

use serde::{Deserialize, Serialize};

use super::{IntBool, SubscriptionType};

/// A login result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountLoginResponse {
    /// The user ID.
    #[serde(rename = "user_id")]
    pub id: u32,
    /// Username used for login.
    #[serde(rename = "login")]
    pub username: String,
    /// The session ID.
    pub session_id: String,
    /// The token used for requests.
    #[serde(rename = "trust_user_jwt")]
    pub token: String,
    /// ID token, not used for now.
    #[serde(rename = "trust_user_id_token")]
    pub id_token: String,
    /// Firebase token, used for communicating with Firebase.
    ///
    /// Not used for now.
    #[serde(rename = "firebase_auth_jwt")]
    pub firebase_token: String,
}

/// An account subscription info.
///
/// This is a minimal representation of the subscription info.
/// Some field are discarded for simplicity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSubscription {
    /// The renewal status or type for SJ subscription.
    ///
    /// If `no` then it's not auto-renew, anything else is auto-renew.
    #[serde(rename = "is_auto_renew")]
    pub sj_auto_renew: String,
    /// The renewal status or type for VM subscription.
    ///
    /// If `no` then it's not auto-renew, anything else is auto-renew.
    #[serde(rename = "vm_is_auto_renew")]
    pub vm_auto_renew: String,
    /// The valid from date for SJ subscription.
    ///
    /// [`None`] if not subscribed.
    #[serde(rename = "valid_from")]
    pub sj_valid_from: Option<i64>,
    /// The valid to date for SJ subscription.
    ///
    /// [`None`] if not subscribed.
    #[serde(rename = "valid_to")]
    pub sj_valid_to: Option<i64>,
    /// The valid from date for VM subscription.
    ///
    /// [`None`] if not subscribed.
    pub vm_valid_from: Option<i64>,
    /// The valid to date for VM subscription.
    ///
    /// [`None`] if not subscribed.
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
