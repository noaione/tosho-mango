//! A module containing information related to user account.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

use serde::{Deserialize, Serialize};
use tosho_macros::AutoGetter;

use super::{IntBool, SubscriptionType};

/// A login result.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct AccountLoginResponse {
    /// The user ID.
    #[serde(rename = "user_id")]
    id: u32,
    /// Username used for login.
    #[serde(rename = "login")]
    username: String,
    /// The session ID.
    session_id: String,
    /// The token used for requests.
    #[serde(rename = "trust_user_jwt")]
    token: String,
    /// ID token, not used for now.
    #[serde(rename = "trust_user_id_token")]
    id_token: String,
    /// Firebase token, used for communicating with Firebase.
    ///
    /// Not used for now.
    #[serde(rename = "firebase_auth_jwt")]
    firebase_token: String,
}

/// An account subscription info.
///
/// This is a minimal representation of the subscription info.
/// Some field are discarded for simplicity.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct AccountSubscription {
    /// The renewal status or type for SJ subscription.
    ///
    /// If `no` then it's not auto-renew, anything else is auto-renew.
    #[serde(rename = "is_auto_renew")]
    sj_auto_renew: String,
    /// The renewal status or type for VM subscription.
    ///
    /// If `no` then it's not auto-renew, anything else is auto-renew.
    #[serde(rename = "vm_is_auto_renew")]
    vm_auto_renew: String,
    /// The valid from date for SJ subscription.
    ///
    /// [`None`] if not subscribed.
    #[serde(rename = "valid_from")]
    sj_valid_from: Option<i64>,
    /// The valid to date for SJ subscription.
    ///
    /// [`None`] if not subscribed.
    #[serde(rename = "valid_to")]
    sj_valid_to: Option<i64>,
    /// The valid from date for VM subscription.
    ///
    /// [`None`] if not subscribed.
    vm_valid_from: Option<i64>,
    /// The valid to date for VM subscription.
    ///
    /// [`None`] if not subscribed.
    vm_valid_to: Option<i64>,
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
#[derive(Debug, Clone, Copy, AutoGetter, Serialize, Deserialize)]
#[auto_getters(unref = true)]
pub struct AccountArchive {
    /// Is the request successful
    ok: IntBool,
    /// The current subscription of the user
    subscription_type: SubscriptionType,
    /// Read limit
    #[serde(rename = "archive_limit")]
    read_limit: i32,
    /// Next read limit reset in seconds
    #[serde(rename = "archive_reset_seconds")]
    read_reset: i32,
    /// Download limit
    download_limit: i32,
    /// Download expiry in seconds
    #[serde(rename = "download_expire_seconds")]
    download_expire: i32,
    /// The next reset for free reading
    #[serde(rename = "next_reset_epoch")]
    next_reset: i64,
    /// The remaining number for free reading
    #[serde(rename = "num_remaining")]
    remaining: i32,
}

/// A response for account entitlements.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct AccountEntitlementsResponse {
    /// The user subscription information
    #[serde(rename = "subscription_info")]
    subscriptions: AccountSubscription,
    /// The user archive information
    #[serde(rename = "archive_info")]
    #[copyable]
    archive: AccountArchive,
}
