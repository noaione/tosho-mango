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
#[derive(Debug, Clone)]
pub struct SJConfig {
    pub user_id: u32,
    pub token: String,
    pub instance: String,
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
