//! Provides the configuration Struct for the client.
//!
//! ```rust
//! use tosho_sjv::{SJConfig, SJPlatform};
//!
//! let config = SJConfig::new(123, "xyz987abc", "abcxyz", SJPlatform::Android);
//! ```

use tosho_macros::AutoGetter;

use crate::models::AccountLoginResponse;

/// The client mode to use.
///
/// Since the original has two separate application.
///
/// ```rust
/// use tosho_sjv::SJMode;
///
/// let mode = SJMode::SJ;
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SJMode {
    /// VM (Manga) mode.
    VM,
    /// SJ (Jump) mode.
    #[default]
    SJ,
}

/// The platform to use.
///
/// ```rust
/// use tosho_sjv::SJPlatform;
///
/// let platform = SJPlatform::Android;
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum SJPlatform {
    /// Android platform.
    Android = 1,
    /// Apple/iOS platform.
    Apple = 2,
    /// Web platform.
    Web = 3,
}

/// The configuration for the client.
///
/// ```rust
/// use tosho_sjv::{SJConfig, SJPlatform};
///
/// let config = SJConfig::new(123, "xyz987abc", "abcxyz", SJPlatform::Android);
/// ```
#[derive(Debug, Clone, AutoGetter)]
pub struct SJConfig {
    /// User ID.
    user_id: u32,
    /// Token or also known as trust_user_jwt
    token: String,
    /// Instance ID or device token
    instance: String,
    /// Platform to use.
    #[copyable]
    platform: SJPlatform,
}

impl SJConfig {
    /// Create a new configuration.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID.
    /// * `token` - The token.
    /// * `instance` - The instance.
    /// * `platform` - The platform.
    pub fn new(
        user_id: u32,
        token: impl Into<String>,
        instance: impl Into<String>,
        platform: SJPlatform,
    ) -> Self {
        Self {
            user_id,
            token: token.into(),
            instance: instance.into(),
            platform,
        }
    }

    /// Create a new configuration from a login response.
    ///
    /// # Arguments
    /// * `response` - The login response.
    /// * `instance` - The instance ID.
    ///
    /// ```rust,no_run
    /// use tosho_sjv::{SJClient, SJConfig, SJMode, SJPlatform};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let (account, instance_id) = SJClient::login(
    ///         "test@mail.com",
    ///         "mypassword",
    ///         SJMode::SJ,
    ///         SJPlatform::Android
    ///     ).await.unwrap();
    ///
    ///     let config = SJConfig::from_login_response(&account, instance_id, SJPlatform::Android);
    /// }
    /// ```
    pub fn from_login_response(
        response: &AccountLoginResponse,
        instance: String,
        platform: SJPlatform,
    ) -> Self {
        Self {
            user_id: response.id(),
            token: response.token().to_string(),
            instance,
            platform,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sj_mode() {
        use super::SJMode;

        let mode = SJMode::SJ;
        assert_eq!(mode, SJMode::SJ);
    }

    #[test]
    fn test_sj_platform() {
        use super::SJPlatform;

        let platform = SJPlatform::Android;
        assert_eq!(platform, SJPlatform::Android);
        assert_eq!(platform as u8, 1);
        let web_plat = SJPlatform::Web;
        assert_eq!(web_plat as u8, 3);
    }
}
