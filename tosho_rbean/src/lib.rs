pub mod config;
pub mod constants;
pub mod models;

use std::collections::HashMap;

use crate::models::UserAccount;
pub use config::*;
use constants::{API_HOST, BASE_API, TOKEN_AUTH};
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
///     let user = client.get_user().await.unwrap();
///     println!("{:?}", user);
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
            .query(&[("key", TOKEN_AUTH.to_string())])
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

    /// Get the current token of the client.
    pub fn token(&self) -> &str {
        &self.token
    }

    /// Get the expiry time of the token.
    pub fn expiry_at(&self) -> Option<i64> {
        self.expiry_at
    }

    // <-- Common Helper

    async fn request<T>(
        &mut self,
        method: reqwest::Method,
        url: &str,
        json_body: Option<HashMap<String, String>>,
    ) -> anyhow::Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        self.refresh_token().await?;

        let endpoint = format!("{}{}", *BASE_API, url);

        let request = match json_body {
            Some(json_body) => self.inner.request(method, endpoint).json(&json_body),
            None => self.inner.request(method, endpoint),
        };

        let response = request.send().await?;

        if response.status().is_success() {
            let response = response.json::<T>().await?;

            Ok(response)
        } else {
            anyhow::bail!("Request failed with status: {}", response.status())
        }
    }

    // --> Common Helper

    // <-- UserApiInterface.kt

    /// Get the current user account information.
    pub async fn get_user(&mut self) -> anyhow::Result<UserAccount> {
        self.request(reqwest::Method::GET, "/user/v0", None).await
    }

    // --> UserApiInterface.kt

    pub async fn login(
        email: &str,
        password: &str,
        platform: RBPlatform,
    ) -> anyhow::Result<RBLoginResponse> {
        let constants = crate::constants::get_constants(platform as u8);

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static(constants.image_ua),
        );

        let client_type = match platform {
            RBPlatform::Android => Some("CLIENT_TYPE_ANDROID"),
            RBPlatform::Apple => Some("CLIENT_TYPE_IOS"),
            _ => None,
        };

        let mut json_data = json!({
            "email": email,
            "password": password,
            "returnSecureToken": true,
        });
        if let Some(client_type) = client_type {
            json_data["clientType"] = client_type.into();
        }

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        let key_param = &[("key", TOKEN_AUTH.to_string())];

        // Step 1: Verify password
        let request = client
            .post("https://www.googleapis.com/identitytoolkit/v3/relyingparty/verifyPassword")
            .query(key_param)
            .json(&json_data)
            .send()
            .await?;

        let verify_resp = request
            .json::<crate::models::accounts::google::IdentityToolkitVerifyPasswordResponse>()
            .await?;

        // Step 2: Get account info
        let json_data = json!({
            "idToken": verify_resp.id_token,
        });

        let request = client
            .post("https://www.googleapis.com/identitytoolkit/v3/relyingparty/getAccountInfo")
            .query(key_param)
            .json(&json_data)
            .send()
            .await?;

        let acc_info_resp = request
            .json::<crate::models::accounts::google::IdentityToolkitAccountInfoResponse>()
            .await?;

        // Step 2.5: Find user
        let goog_user = acc_info_resp
            .users
            .iter()
            .find(|user| user.local_id == verify_resp.local_id);

        if goog_user.is_none() {
            anyhow::bail!(
                "Google user information not found for {}",
                verify_resp.local_id
            );
        }

        let goog_user = goog_user.unwrap().clone();

        // Step 3: Refresh token
        let json_data = json!({
            "grantType": "refresh_token",
            "refreshToken": verify_resp.refresh_token,
        });

        let request = client
            .post("https://securetoken.googleapis.com/v1/token")
            .query(key_param)
            .json(&json_data)
            .send()
            .await?;

        let secure_token_resp = request
            .json::<crate::models::accounts::google::SecureTokenResponse>()
            .await?;

        // Step 4: Auth with 小豆
        let request = client
            .post(&format!("{}/user/v0", *BASE_API))
            .headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    reqwest::header::USER_AGENT,
                    reqwest::header::HeaderValue::from_static(constants.ua),
                );
                headers.insert(
                    "public",
                    reqwest::header::HeaderValue::from_static(&constants.public),
                );
                headers.insert(
                    "x-user-token",
                    reqwest::header::HeaderValue::from_str(&secure_token_resp.access_token)
                        .unwrap(),
                );
                headers
            })
            .send()
            .await?;

        let user_resp = request.json::<UserAccount>().await?;

        Ok(RBLoginResponse {
            token: secure_token_resp.access_token,
            refresh_token: secure_token_resp.refresh_token,
            platform,
            user: user_resp,
            google_account: goog_user,
        })
    }
}

/// Represents the login response from the 小豆 (Red Bean) API
///
/// The following struct is returned when you use [`RBClient::login`] method.
///
/// This struct wraps some other struct that can be useful for config building yourself.
#[derive(Debug, Clone)]
pub struct RBLoginResponse {
    /// The token of the account
    pub token: String,
    /// The refresh token of the account
    pub refresh_token: String,
    /// The platform of the account
    pub platform: RBPlatform,
    /// Detailed account information
    pub user: UserAccount,
    /// Detailed google account information
    pub google_account: crate::models::accounts::google::IdentityToolkitAccountInfo,
}
