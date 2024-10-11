//! A module containing information related to user account.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

use serde::{Deserialize, Serialize};
use tosho_macros::AutoGetter;

use super::{Image, Label, MangaNode};

/// User account information
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct UserAccount {
    /// The UUID of the user.
    uuid: String,
    /// The username or handle of the user.
    #[serde(rename = "handle")]
    username: Option<String>,
    /// The email address of the user.
    #[serde(rename = "email_address")]
    email: String,
    /// The image or avatar of the user.
    image: Option<Image>,
    /// Does the account has premium?
    is_premium: bool,
    /// The date when the premium expires.
    ///
    /// If [`None`] then the account does not have premium.
    premium_expiration_date: Option<String>,
}

/// User reading list history
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ReadingListItem {
    /// The manga being read.
    manga: MangaNode,
    /// The specific chapter being read.
    chapter: Option<Label>,
}

pub mod google {
    use serde::{Deserialize, Serialize};
    use tosho_macros::AutoGetter;

    /// Object representing the response of the verification of user entered password.
    #[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
    pub struct IdentityToolkitVerifyPasswordResponse {
        kind: String,
        #[serde(rename = "localId")]
        local_id: String,
        email: String,
        #[serde(rename = "displayName")]
        display_name: String,
        #[serde(rename = "idToken")]
        id_token: String,
        #[serde(rename = "registered")]
        registered: bool,
        #[serde(rename = "refreshToken")]
        refresh_token: String,
        #[serde(rename = "expiresIn")]
        expires_in: String,
    }

    /// Object of each provider's information.
    #[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
    pub struct IdentityToolkitAccountProviderInfo {
        #[serde(rename = "providerId")]
        provider_id: String,
        #[serde(rename = "federatedId")]
        federated_id: String,
        email: String,
    }

    /// Object of each user's information from single token.
    #[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
    pub struct IdentityToolkitAccountInfo {
        #[serde(rename = "localId")]
        local_id: String,
        email: String,
        #[serde(rename = "passwordHash")]
        password_hash: String,
        #[serde(rename = "emailVerified")]
        email_verified: bool,
        #[serde(rename = "validSince")]
        valid_since: String,
        #[serde(rename = "lastLoginAt")]
        last_login_at: String,
        #[serde(rename = "createdAt")]
        created_at: String,
        #[serde(rename = "providerUserInfo")]
        provider_user_info: Vec<IdentityToolkitAccountProviderInfo>,
    }

    /// Object representing the response of the registered user's information.
    #[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
    pub struct IdentityToolkitAccountInfoResponse {
        kind: String,
        users: Vec<IdentityToolkitAccountInfo>,
    }

    /// Object representing the response of the token exchange.
    #[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
    pub struct SecureTokenResponse {
        access_token: String,
        expires_in: String,
        token_type: String,
        refresh_token: String,
        id_token: String,
        user_id: String,
        project_id: String,
    }
}
