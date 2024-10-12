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

use tosho_macros::AutoGetter;

use crate::models::accounts::google::{IdentityToolkitVerifyPasswordResponse, SecureTokenResponse};

/// Represents the platform for the client.
///
/// ```rust
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
#[derive(Debug, Clone, AutoGetter)]
pub struct RBConfig {
    /// The token of the account
    token: String,
    /// The refresh token of the account
    refresh_token: String,
    /// The platform of the account
    #[copyable]
    platform: RBPlatform,
}

impl RBConfig {
    /// Create a new custom instance for [`RBConfig`]
    pub fn new(
        token: impl Into<String>,
        refresh_token: impl Into<String>,
        platform: RBPlatform,
    ) -> Self {
        Self {
            token: token.into(),
            refresh_token: refresh_token.into(),
            platform,
        }
    }

    /// Set a new token
    pub fn set_token(&mut self, token: impl Into<String>) {
        self.token = token.into();
    }

    /// Set a new refresh token
    pub fn set_refresh_token(&mut self, refresh_token: impl Into<String>) {
        self.refresh_token = refresh_token.into();
    }

    /// Convert [`SecureTokenResponse`] to [`RBConfig`].
    pub fn from_secure_token(value: &SecureTokenResponse, platform: RBPlatform) -> Self {
        RBConfig {
            token: value.access_token().to_string(),
            refresh_token: value.refresh_token().to_string(),
            platform,
        }
    }

    /// Convert [`IdentityToolkitVerifyPasswordResponse`] to [`RBConfig`].
    pub fn from_verify_password(
        value: &IdentityToolkitVerifyPasswordResponse,
        platform: RBPlatform,
    ) -> Self {
        RBConfig {
            token: value.id_token().to_string(),
            refresh_token: value.refresh_token().to_string(),
            platform,
        }
    }
}
