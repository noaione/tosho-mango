//! A module containing information related to user account.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

use serde::{Deserialize, Serialize};

use super::{Image, Label, MangaNode};

/// User account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAccount {
    /// The UUID of the user.
    pub uuid: String,
    /// The username or handle of the user.
    #[serde(rename = "handle")]
    pub username: Option<String>,
    /// The email address of the user.
    #[serde(rename = "email_address")]
    pub email: String,
    /// The image or avatar of the user.
    pub image: Option<Image>,
    /// Does the account has premium?
    pub is_premium: bool,
    /// The date when the premium expires.
    ///
    /// If [`None`] then the account does not have premium.
    pub premium_expiration_date: Option<String>,
}

/// User reading list history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadingListItem {
    /// The manga being read.
    pub manga: MangaNode,
    /// The specific chapter being read.
    pub chapter: Option<Label>,
}

pub mod google {
    use serde::{Deserialize, Serialize};

    /// Object representing the response of the verification of user entered password.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct IdentityToolkitVerifyPasswordResponse {
        pub kind: String,
        #[serde(rename = "localId")]
        pub local_id: String,
        pub email: String,
        #[serde(rename = "displayName")]
        pub display_name: String,
        #[serde(rename = "idToken")]
        pub id_token: String,
        #[serde(rename = "registered")]
        pub registered: bool,
        #[serde(rename = "refreshToken")]
        pub refresh_token: String,
        #[serde(rename = "expiresIn")]
        pub expires_in: String,
    }

    /// Object of each provider's information.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct IdentityToolkitAccountProviderInfo {
        #[serde(rename = "providerId")]
        pub provider_id: String,
        #[serde(rename = "federatedId")]
        pub federated_id: String,
        pub email: String,
    }

    /// Object of each user's information from single token.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct IdentityToolkitAccountInfo {
        #[serde(rename = "localId")]
        pub local_id: String,
        pub email: String,
        #[serde(rename = "passwordHash")]
        pub password_hash: String,
        #[serde(rename = "emailVerified")]
        pub email_verified: bool,
        #[serde(rename = "validSince")]
        pub valid_since: String,
        #[serde(rename = "lastLoginAt")]
        pub last_login_at: String,
        #[serde(rename = "createdAt")]
        pub created_at: String,
        #[serde(rename = "providerUserInfo")]
        pub provider_user_info: Vec<IdentityToolkitAccountProviderInfo>,
    }

    /// Object representing the response of the registered user's information.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct IdentityToolkitAccountInfoResponse {
        pub kind: String,
        pub users: Vec<IdentityToolkitAccountInfo>,
    }

    /// Object representing the response of the token exchange.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SecureTokenResponse {
        pub access_token: String,
        pub expires_in: String,
        pub token_type: String,
        pub refresh_token: String,
        pub id_token: String,
        pub user_id: String,
        pub project_id: String,
    }
}
