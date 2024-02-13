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
#[derive(Debug, Clone, Copy, Default)]
pub enum SJMode {
    /// VM (Manga) mode.
    VM,
    /// SJ (Jump) mode.
    #[default]
    SJ,
}

/// The configuration for the client.
#[derive(Debug, Clone)]
pub struct SJConfig {
    pub user_id: u32,
    pub token: String,
    pub instance: String,
}

impl SJConfig {
    /// Create a new configuration.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID.
    /// * `token` - The token.
    /// * `instance` - The instance.
    pub fn new(user_id: u32, token: String, instance: String) -> Self {
        Self {
            user_id,
            token,
            instance,
        }
    }

    /// Create a new configuration from a login response.
    ///
    /// # Arguments
    /// * `response` - The login response.
    /// * `instance` - The instance ID.
    pub fn from_login_response(response: &AccountLoginResponse, instance: String) -> Self {
        Self {
            user_id: response.id,
            token: response.token.clone(),
            instance,
        }
    }
}
