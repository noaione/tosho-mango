//! Provides the configuration Struct for the client.
//!
//! ```rust
//! use tosho_rbean::{RBConfig, RBPlatform};
//!
//! let config = RBConfig {
//!     token: "123".to_string(),
//!     refresh_token: "abcxyz".to_string(),
//!     platform: RBPlatform::Android,
//! };
//! ```

use crate::models::accounts::google::{IdentityToolkitVerifyPasswordResponse, SecureTokenResponse};

/// Represents the platform for the client.
///
/// ```
/// use tosho_rbean::RBPlatform;
///
/// let platform = RBPlatform::Android;
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum RBPlatform {
    /// Android platform.
    Android = 1,
    /// Apple/iOS platform.
    Apple = 2,
    /// Web platform.
    Web = 3,
}

/// Represents the configuration for the client.
#[derive(Debug, Clone)]
pub struct RBConfig {
    /// The token of the account
    pub token: String,
    /// The refresh token of the account
    pub refresh_token: String,
    /// The platform of the account
    pub platform: RBPlatform,
}

impl RBConfig {
    /// Convert [`SecureTokenResponse`] to [`RBConfig`].
    pub fn from_secure_token(value: &SecureTokenResponse, platform: RBPlatform) -> Self {
        RBConfig {
            token: value.access_token.clone(),
            refresh_token: value.refresh_token.clone(),
            platform,
        }
    }

    /// Convert [`IdentityToolkitVerifyPasswordResponse`] to [`RBConfig`].
    pub fn from_verify_password(
        value: &IdentityToolkitVerifyPasswordResponse,
        platform: RBPlatform,
    ) -> Self {
        RBConfig {
            token: value.id_token.clone(),
            refresh_token: value.refresh_token.clone(),
            platform,
        }
    }
}
