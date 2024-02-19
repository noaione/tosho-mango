pub mod config;
pub mod constants;
pub mod models;

pub use config::*;
use constants::API_HOST;
use serde_json::json;

/// Main client for interacting with the 小豆 (Red Bean) API
///
/// # Examples
/// ```no_run
/// use tosho_rbean::{RBClient, RBConfig, RBPlatform};
///
/// #[tokio::main]
/// async fn main() {
///     let config = RBConfig {
///         token: "123".to_string(),
///         refresh_token: "abcxyz".to_string(),
///         platform: RBPlatform::Android,
///     };
///
///     let mut client = RBClient::new(config);
///     // Refresh token
///     client.refresh_token().await.unwrap();
/// }
/// ```
#[derive(Debug)]
pub struct RBClient {
    #[allow(dead_code)]
    inner: reqwest::Client,
    config: RBConfig,
    constants: &'static crate::constants::Constants,
    token: String,
    expiry_at: Option<i64>,
}

impl RBClient {
    /// Create a new client instance.
    ///
    /// # Arguments
    /// * `config` - The configuration to use for the client.
    pub fn new(config: RBConfig) -> Self {
        Self::make_client(config, None)
    }

    /// Attach a proxy to the client.
    ///
    /// This will clone the client and return a new client with the proxy attached.
    ///
    /// # Arguments
    /// * `proxy` - The proxy to attach to the client
    pub fn with_proxy(&self, proxy: reqwest::Proxy) -> Self {
        Self::make_client(self.config.clone(), Some(proxy))
    }

    fn make_client(config: RBConfig, proxy: Option<reqwest::Proxy>) -> Self {
        let constants = crate::constants::get_constants(config.platform as u8);
        let mut headers = reqwest::header::HeaderMap::new();

        headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static(constants.ua),
        );
        headers.insert(
            reqwest::header::HOST,
            reqwest::header::HeaderValue::from_static(&API_HOST),
        );
        headers.insert(
            "public",
            reqwest::header::HeaderValue::from_static(&constants.public),
        );
        headers.insert("x-user-token", config.token.parse().unwrap());

        let client = reqwest::Client::builder().default_headers(headers);

        let client = match proxy {
            Some(proxy) => client.proxy(proxy).build().unwrap(),
            None => client.build().unwrap(),
        };

        Self {
            inner: client,
            config: config.clone(),
            constants,
            token: config.token.clone(),
            expiry_at: None,
        }
    }

    /// Refresh the token of the client.
    ///
    /// The following function will be called on each request to ensure the token is always valid.
    ///
    /// The first request will always be a token refresh, and subsequent requests will only refresh
    /// if the token is expired.
    pub async fn refresh_token(&mut self) -> anyhow::Result<()> {
        // If the expiry time is set and it's not expired, return early
        if let Some(expiry_at) = self.expiry_at {
            if expiry_at > chrono::Utc::now().timestamp() {
                return Ok(());
            }
        }

        let json_data = json!({
            "grantType": "refresh_token",
            "refreshToken": self.config.refresh_token,
        });

        let client = reqwest::Client::new();
        let request = client
            .post("https://securetoken.googleapis.com/v1/token")
            .header("User-Agent", self.constants.image_ua)
            .json(&json_data)
            .send()
            .await?;

        let response = request
            .json::<crate::models::accounts::google::SecureTokenResponse>()
            .await?;

        self.token = response.access_token.clone();
        self.config.token = response.access_token;
        let expiry_in = response.expires_in.parse::<i64>().unwrap();
        // Set the expiry time to 3 seconds before the actual expiry time
        self.expiry_at = Some(chrono::Utc::now().timestamp() + expiry_in - 3);

        Ok(())
    }
}
