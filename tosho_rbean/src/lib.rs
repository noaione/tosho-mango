//! # tosho-rbean
//!
//! A minimal asynchronous client for 小豆 (Red Bean) API.
//!
//! The following crate is used by the [`tosho`] app.
//!
//! ## Usage
//!
//! Download the [`tosho`] app, or you can utilize this crate like any other Rust crate:
//!
//! ```rust,no_run
//! use tosho_rbean::{RBClient, RBConfig, RBPlatform};
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = RBConfig {
//!         token: "123".to_string(),
//!         refresh_token: "abcxyz".to_string(),
//!         platform: RBPlatform::Android,
//!     };
//!     let mut client = RBClient::new(config);
//!     // Refresh token
//!     client.refresh_token().await.unwrap();
//!     let user = client.get_user().await.unwrap();
//!     println!("{:?}", user);
//! }
//! ```
//!
//! ## Authentication
//!
//! The following sources only have one method of authentication, and that method uses your email and password.
//!
//! ```bash
//! $ tosho rb auth email password --help
//! ```
//!
//! Or, if you use the crates:
//!
//! ```rust,no_run
//! use tosho_rbean::{RBClient, RBPlatform};
//!
//! #[tokio::main]
//! async fn main() {
//!     let login_results = RBClient::login("email@test.com", "mypassword", RBPlatform::Android).await.unwrap();
//!     println!("{:?}", login_results);
//! }
//! ```
//!
//! ## Disclaimer
//!
//! This project is designed as an experiment and to create a local copy for personal use.
//! These tools will not circumvent any paywall, and you will need to purchase and own each chapter
//! with your own account to be able to make your own local copy.
//!
//! We're not responsible if your account got deactivated.
//!
//! ## License
//!
//! This project is licensed with MIT License ([LICENSE](https://github.com/noaione/tosho-mango/blob/master/LICENSE) or <http://opensource.org/licenses/MIT>)
//!
//! [`tosho`]: https://crates.io/crates/tosho

use std::collections::HashMap;
use tokio::io::{self, AsyncWriteExt};

use crate::models::UserAccount;
pub use config::*;
use constants::{API_HOST, BASE_API, IMAGE_HOST, TOKEN_AUTH};
use models::{
    ChapterDetailsResponse, ChapterListResponse, ChapterPageDetailsResponse, HomeResponse, Manga,
    MangaListResponse, Publisher, ReadingListItem, SortOption,
};
use serde_json::json;

pub mod config;
pub mod constants;
pub mod models;

const PATTERN: [u8; 1] = [174];

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
#[derive(Clone, Debug)]
pub struct RBClient {
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

    pub fn set_expiry_at(&mut self, expiry_at: Option<i64>) {
        self.expiry_at = expiry_at;
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
            .header(reqwest::header::USER_AGENT, self.constants.image_ua)
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
    pub fn get_token(&self) -> &str {
        &self.token
    }

    /// Get the expiry time of the token.
    pub fn get_expiry_at(&self) -> Option<i64> {
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
            let response = response.text().await?;

            let json_de = serde_json::from_str::<T>(&response);

            match json_de {
                Ok(json_de) => Ok(json_de),
                Err(error) => {
                    let row_line = error.line() - 1;
                    let split_lines = &response.split('\n').collect::<Vec<&str>>();
                    let position = error.column();
                    let start_index = position.saturating_sub(25); // Start 25 characters before the error position
                    let end_index = position.saturating_add(25); // End 25 characters after the error position
                    let excerpt = &split_lines[row_line][start_index..end_index];

                    anyhow::bail!(
                        "Error parsing JSON at line {}, column {}: {}\nExcerpt: '{}'",
                        error.line(),
                        error.column(),
                        error,
                        excerpt
                    )
                }
            }
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

    /// Get the current user reading list.
    pub async fn get_reading_list(&mut self) -> anyhow::Result<Vec<ReadingListItem>> {
        self.request(reqwest::Method::GET, "/user/reading_list/v0", None)
            .await
    }

    // --> UserApiInterface.kt

    // <-- MangaApiInterface.kt

    /// Get the manga information for a specific manga.
    ///
    /// # Arguments
    /// * `uuid` - The UUID of the manga.
    pub async fn get_manga(&mut self, uuid: &str) -> anyhow::Result<Manga> {
        self.request(reqwest::Method::GET, &format!("/manga/{}/v0", uuid), None)
            .await
    }

    /// Get the manga filters for searching manga.
    pub async fn get_manga_filters(&mut self) -> anyhow::Result<Manga> {
        self.request(reqwest::Method::GET, "/manga/filters/v0", None)
            .await
    }

    /// Get chapter list for a specific manga.
    ///
    /// # Arguments
    /// * `uuid` - The UUID of the manga.
    pub async fn get_chapter_list(&mut self, uuid: &str) -> anyhow::Result<ChapterListResponse> {
        self.request(
            reqwest::Method::GET,
            &format!("/mangas/{}/chapters/v4?order=asc&count=9999&offset=0", uuid),
            None,
        )
        .await
    }

    /// Get the chapter details for a specific chapter.
    ///
    /// # Arguments
    /// * `uuid` - The UUID of the chapter.
    pub async fn get_chapter(&mut self, uuid: &str) -> anyhow::Result<ChapterDetailsResponse> {
        self.request(
            reqwest::Method::GET,
            &format!("/chapters/{}/v2", uuid),
            None,
        )
        .await
    }

    /// Get the chapter viewer for a specific chapter.
    ///
    /// # Arguments
    /// * `uuid` - The UUID of the chapter.
    pub async fn get_chapter_viewer(
        &mut self,
        uuid: &str,
    ) -> anyhow::Result<ChapterPageDetailsResponse> {
        self.request(
            reqwest::Method::GET,
            &format!("/chapters/{}/pages/v1", uuid),
            None,
        )
        .await
    }

    /// Do a search for a manga.
    ///
    /// # Arguments
    /// * `query` - The query to search for.
    /// * `offset` - The offset of the search result, default to `0`
    /// * `count` - The count of the search result, default to `999`
    /// * `sort` - The sort option of the search result, default to [`SortOption::Alphabetical`]
    pub async fn search(
        &mut self,
        query: &str,
        offset: Option<u32>,
        count: Option<u32>,
        sort: Option<SortOption>,
    ) -> anyhow::Result<MangaListResponse> {
        let offset = offset.unwrap_or(0);
        let count = count.unwrap_or(999);
        let sort = sort.unwrap_or(SortOption::Alphabetical);

        let query_param = format!(
            "sort={}&offset={}&count={}&tags=&search_string={}&publisher_slug=",
            sort.to_string(),
            offset,
            count,
            query
        );

        self.request(
            reqwest::Method::GET,
            &format!("/mangas/v1?{}", query_param),
            None,
        )
        .await
    }

    /// Get the home page information.
    pub async fn get_home_page(&mut self) -> anyhow::Result<HomeResponse> {
        self.request(reqwest::Method::GET, "/home/v0", None).await
    }

    /// Get specific publisher information by their slug.
    ///
    /// # Arguments
    /// * `slug` - The slug of the publisher.
    pub async fn get_publisher(&mut self, slug: &str) -> anyhow::Result<Publisher> {
        self.request(
            reqwest::Method::GET,
            &format!("/publisher/slug/{}/v0", slug),
            None,
        )
        .await
    }

    // --> Image

    /// Stream download the image from the given URL.
    ///
    /// The URL can be obtained from [`RBClient::get_chapter_viewer`].
    ///
    /// # Parameters
    /// * `url` - The URL to download the image from.
    /// * `writer` - The writer to write the image to.
    pub async fn stream_download(
        &self,
        url: &str,
        mut writer: impl io::AsyncWrite + Unpin,
    ) -> anyhow::Result<()> {
        let res = self
            .inner
            .get(url)
            .query(&[("drm", "1")])
            .headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    reqwest::header::USER_AGENT,
                    reqwest::header::HeaderValue::from_static(self.constants.image_ua),
                );
                headers.insert(
                    reqwest::header::HOST,
                    reqwest::header::HeaderValue::from_str(IMAGE_HOST.as_str()).unwrap(),
                );
                headers
            })
            .send()
            .await?;

        if !res.status().is_success() {
            anyhow::bail!("Failed to download image: {}", res.status())
        }

        let image_bytes = res.bytes().await?;
        let image_dec = decrypt_image(&image_bytes);
        drop(image_bytes);

        writer.write_all(&image_dec).await?;

        drop(image_dec);

        Ok(())
    }

    // <-- Image

    // --> MangaApiInterface.kt

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

        let expires_in = secure_token_resp.expires_in.parse::<i64>().unwrap();
        let expiry_at = chrono::Utc::now().timestamp() + expires_in - 3;

        // Step 4: Auth with 小豆
        let request = client
            .get(&format!("{}/user/v0", *BASE_API))
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
            expiry: expiry_at,
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
    /// Expiry time of the token
    pub expiry: i64,
}

/// A simple image decryptor for the 小豆 (Red Bean) API
///
/// # Arguments
/// * `data` - The image data to decrypt
pub fn decrypt_image(data: &[u8]) -> Vec<u8> {
    let image_data = data.to_vec();
    let length = image_data.len();
    let mut decrypted: Vec<u8> = vec![0; length];
    for i in 0..length {
        decrypted[i] = PATTERN[0] ^ image_data[i];
    }
    decrypted
}
