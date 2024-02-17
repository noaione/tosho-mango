//! Provides the configuration Struct for the client.
//!
//! ```rust
//! use tosho_sjv::{SJConfig, SJPlatform};
//!
//! let config = SJConfig {
//!     user_id: 123,
//!     token: "xyz987abc".to_string(),
//!     instance: "abcxyz".to_string(),
//!     platform: SJPlatform::Android,
//! };
//! ```

use crate::models::AccountLoginResponse;

/// The client mode to use.
///
/// Since the original has two separate application.
///
/// ```
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
/// ```
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
/// ```
/// use tosho_sjv::{SJConfig, SJPlatform};
///
/// let config = SJConfig {
///     user_id: 123,
///     token: "xyz987abc".to_string(),
///     instance: "abcxyz".to_string(),
///     platform: SJPlatform::Android,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct SJConfig {
    /// User ID.
    pub user_id: u32,
    /// Token or also known as trust_user_jwt
    pub token: String,
    /// Instance ID or device token
    pub instance: String,
    /// Platform to use.
    pub platform: SJPlatform,
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
    pub fn new(user_id: u32, token: String, instance: String, platform: SJPlatform) -> Self {
        Self {
            user_id,
            token,
            instance,
            platform,
        }
    }

    /// Create a new configuration from a login response.
    ///
    /// # Arguments
    /// * `response` - The login response.
    /// * `instance` - The instance ID.
    ///
    /// ```no_run
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
            user_id: response.id,
            token: response.token.clone(),
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
