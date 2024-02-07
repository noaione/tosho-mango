use std::{collections::HashMap, sync::MutexGuard};

use constants::{
    get_constants, API_HOST, APP_NAME, BASE_API, HEADER_NAMES, IMAGE_HOST, MASKED_LOGIN,
};
use futures_util::StreamExt;
use helper::{generate_random_token, ComicPurchase};
use models::{
    APIResult, AccountUserResponse, ComicDiscovery, ComicDiscoveryPaginatedResponse,
    ComicSearchResponse, ComicStatus, StatusResult,
};
use reqwest_cookie_store::CookieStoreMutex;
use sha2::{Digest, Sha256};
use tokio::io::AsyncWriteExt;

pub use config::*;
pub mod config;
pub mod constants;
pub mod helper;
pub mod models;

const SCREEN_INCH: f64 = 61.1918658356194;

/// Main client for interacting with the AP AM
///
/// # Example
/// ```no_run,ignore
/// use tosho_amap::{AMClient, AMConfig};
///
/// #[tokio::main]
/// async fn main() {
///     let config = AMConfig {
///         token: "123",
///         identifier: "abcxyz",
///         session_v2: "xyz987abc",
///     };
///
///     let client = AMClient::new(config);
///     let manga = client.get_comic(48000051).await.unwrap();
///     println!("{:?}", manga);
/// }
/// ```
#[derive(Clone)]
pub struct AMClient {
    inner: reqwest::Client,
    config: AMConfig,
    constants: &'static constants::Constants,
    cookie_store: std::sync::Arc<CookieStoreMutex>,
}

impl AMClient {
    /// Create a new client instance.
    ///
    /// # Parameters
    /// * `config` - The configuration to use for the client.
    pub fn new(config: AMConfig) -> Self {
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

    fn make_client(config: AMConfig, proxy: Option<reqwest::Proxy>) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            reqwest::header::HOST,
            reqwest::header::HeaderValue::from_static(&API_HOST),
        );
        let constants = get_constants(1);
        headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static(&constants.ua),
        );

        let cookie_store = CookieStoreMutex::from(config.clone());
        let cookie_store = std::sync::Arc::new(cookie_store);

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .cookie_provider(std::sync::Arc::clone(&cookie_store));

        let client = match proxy {
            Some(proxy) => client.proxy(proxy).build().unwrap(),
            None => client.build().unwrap(),
        };

        Self {
            inner: client,
            config,
            constants,
            cookie_store,
        }
    }

    /// Apply the JSON object with the default values.
    fn apply_json_object(&self, json_obj: &mut HashMap<String, serde_json::Value>) {
        json_with_common(json_obj, self.constants)
    }

    /// Get the underlying cookie store.
    pub fn get_cookie_store(&self) -> MutexGuard<'_, reqwest_cookie_store::CookieStore> {
        self.cookie_store.lock().unwrap()
    }

    async fn request<T>(
        &self,
        method: reqwest::Method,
        endpoint: &str,
        json: Option<HashMap<String, serde_json::Value>>,
    ) -> anyhow::Result<APIResult<T>>
    where
        T: serde::de::DeserializeOwned + std::clone::Clone,
    {
        let endpoint = format!("{}{}", *BASE_API, endpoint);

        let mut cloned_json = json.clone().unwrap_or_default();
        self.apply_json_object(&mut cloned_json);

        let headers = make_header(&self.config, self.constants)?;

        let req = self
            .inner
            .request(method, &endpoint)
            .headers(headers)
            .json(&cloned_json)
            .send()
            .await?;

        parse_response(req).await
    }

    /// Get the account information or remainder.
    ///
    /// This request has data related to user point and more.
    pub async fn get_remainder(&self) -> anyhow::Result<models::IAPRemainder> {
        let mut json_body = HashMap::new();
        json_body.insert(
            "i_token".to_string(),
            serde_json::Value::String(self.config.token.clone()),
        );
        json_body.insert(
            "iap_product_version".to_string(),
            serde_json::Value::Number(serde_json::Number::from(0_u32)),
        );
        json_body.insert("app_login".to_string(), serde_json::Value::Bool(true));

        let result = self
            .request::<models::IAPRemainder>(
                reqwest::Method::POST,
                "/iap/remainder.json",
                Some(json_body),
            )
            .await?;

        result
            .result
            .body
            .ok_or_else(|| anyhow::anyhow!("No content in response"))
    }

    /// Get a single comic information by ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the comic.
    pub async fn get_comic(&self, id: u64) -> anyhow::Result<models::ComicInfoResponse> {
        let mut json_body = HashMap::new();
        json_body.insert(
            "manga_sele_id".to_string(),
            serde_json::Value::Number(serde_json::Number::from(id)),
        );
        json_body.insert(
            "i_token".to_string(),
            serde_json::Value::String(self.config.token.clone()),
        );
        json_body.insert("app_login".to_string(), serde_json::Value::Bool(true));

        let result = self
            .request::<models::ComicInfoResponse>(
                reqwest::Method::POST,
                "/iap/comicCover.json",
                Some(json_body),
            )
            .await?;

        result
            .result
            .body
            .ok_or_else(|| anyhow::anyhow!("No content in response"))
    }

    /// Get reader/viewer for an episode.
    ///
    /// # Arguments
    /// * `comic_id` - The ID of the comic.
    /// * `episode` - The episode being read.
    pub async fn get_comic_viewer(
        &self,
        id: u64,
        episode: &ComicPurchase,
    ) -> anyhow::Result<models::ComicReadResponse> {
        let mut json_body = HashMap::new();
        json_body.insert(
            "manga_sele_id".to_string(),
            serde_json::Value::Number(serde_json::Number::from(id)),
        );
        json_body.insert(
            "story_no".to_string(),
            serde_json::Value::Number(serde_json::Number::from(episode.id)),
        );
        if let Some(rental_term) = episode.rental_term.clone() {
            json_body.insert(
                "rental_term".to_string(),
                serde_json::Value::String(rental_term),
            );
        }
        json_body.insert(
            "bonus".to_string(),
            serde_json::Value::Number(serde_json::Number::from(episode.bonus)),
        );
        json_body.insert(
            "product".to_string(),
            serde_json::Value::Number(serde_json::Number::from(episode.purchased)),
        );
        json_body.insert(
            "premium".to_string(),
            serde_json::Value::Number(serde_json::Number::from(episode.premium)),
        );
        if let Some(point) = episode.point {
            json_body.insert(
                "point".to_string(),
                serde_json::Value::Number(serde_json::Number::from(point)),
            );
        }
        json_body.insert(
            "is_free_daily".to_string(),
            serde_json::Value::Bool(episode.is_free_daily),
        );
        json_body.insert(
            "i_token".to_string(),
            serde_json::Value::String(self.config.token.clone()),
        );
        json_body.insert("app_login".to_string(), serde_json::Value::Bool(true));

        let result = self
            .request::<models::ComicReadResponse>(
                reqwest::Method::POST,
                "/iap/mangaDownload.json",
                Some(json_body),
            )
            .await?;

        result
            .result
            .body
            .ok_or_else(|| anyhow::anyhow!("No content in response"))
    }

    /// Get the account for the current session.
    pub async fn get_account(&self) -> anyhow::Result<AccountUserResponse> {
        let mut json_body = HashMap::new();
        json_body.insert("mine".to_string(), serde_json::Value::Bool(true));

        let result = self
            .request::<AccountUserResponse>(
                reqwest::Method::POST,
                "/author/profile.json",
                Some(json_body),
            )
            .await?;

        result
            .result
            .body
            .ok_or_else(|| anyhow::anyhow!("No content in response"))
    }

    /// Get account favorites.
    pub async fn get_favorites(&self) -> anyhow::Result<ComicDiscoveryPaginatedResponse> {
        let result = self
            .request::<ComicDiscoveryPaginatedResponse>(
                reqwest::Method::POST,
                "/mypage/favOfficialComicList.json",
                None,
            )
            .await?;

        result
            .result
            .body
            .ok_or_else(|| anyhow::anyhow!("No content in response"))
    }

    /// Search for comics.
    ///
    /// # Arguments
    /// * `query` - The query to search for.
    /// * `page` - The page to search for. (default to 1)
    /// * `limit` - The limit of results per page. (default to 30)
    pub async fn search(
        &self,
        query: &str,
        status: Option<ComicStatus>,
        tag_id: Option<u64>,
        page: Option<u64>,
        limit: Option<u64>,
    ) -> anyhow::Result<ComicSearchResponse> {
        let mut json_body = HashMap::new();

        let mut conditions = serde_json::Map::new();
        conditions.insert(
            "free_word".to_string(),
            serde_json::Value::String(query.to_string()),
        );
        conditions.insert(
            "tag_id".to_string(),
            serde_json::Value::Number(serde_json::Number::from(tag_id.unwrap_or(0))),
        );
        if let Some(status) = status {
            conditions.insert(
                "complete".to_string(),
                serde_json::Value::Number(serde_json::Number::from(status as i32)),
            );
        }
        json_body.insert(
            "conditions".to_string(),
            serde_json::Value::Object(conditions),
        );
        json_body.insert(
            "page".to_string(),
            serde_json::Value::Number(serde_json::Number::from(page.unwrap_or(1))),
        );
        json_body.insert(
            "limit".to_string(),
            serde_json::Value::Number(serde_json::Number::from(limit.unwrap_or(30))),
        );

        let result = self
            .request::<ComicSearchResponse>(
                reqwest::Method::POST,
                "/manga/official.json",
                Some(json_body),
            )
            .await?;

        result
            .result
            .body
            .ok_or_else(|| anyhow::anyhow!("No content in response"))
    }

    /// Get home discovery.
    pub async fn get_discovery(&self) -> anyhow::Result<ComicDiscovery> {
        let results = self
            .request::<ComicDiscovery>(reqwest::Method::POST, "/manga/discover.json", None)
            .await?;

        results
            .result
            .body
            .ok_or_else(|| anyhow::anyhow!("No content in response"))
    }

    /// Stream download the image from the given URL.
    ///
    /// # Arguments
    /// * `url` - The URL of the image.
    /// * `writer` - The writer to write the image to.
    pub async fn stream_download(
        &self,
        url: &str,
        mut writer: impl tokio::io::AsyncWrite + Unpin,
    ) -> anyhow::Result<()> {
        let mut headers = make_header(&self.config, self.constants)?;
        headers.insert(
            "Host",
            reqwest::header::HeaderValue::from_str(&IMAGE_HOST).unwrap(),
        );
        headers.insert(
            "User-Agent",
            reqwest::header::HeaderValue::from_str(&self.constants.image_ua).unwrap(),
        );

        let res = self.inner.get(url).headers(headers).send().await.unwrap();

        // bail if not success
        if !res.status().is_success() {
            anyhow::bail!("Failed to download image: {}", res.status())
        }

        let mut stream = res.bytes_stream();
        while let Some(item) = stream.next().await {
            let item = item.unwrap();
            writer.write_all(&item).await?;
        }

        Ok(())
    }

    /// Perform a login request.
    ///
    /// # Arguments
    /// * `email` - The email of the user.
    /// * `password` - The password of the user.
    pub async fn login(email: &str, password: &str) -> anyhow::Result<AMConfig> {
        let cookie_store = CookieStoreMutex::default();
        let cookie_store = std::sync::Arc::new(cookie_store);

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            reqwest::header::HOST,
            reqwest::header::HeaderValue::from_static(&API_HOST),
        );
        let constants = get_constants(1);
        headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static(&constants.ua),
        );

        let session = reqwest::Client::builder()
            .cookie_provider(std::sync::Arc::clone(&cookie_store))
            .default_headers(headers)
            .build()
            .unwrap();

        let secret_token = generate_random_token();
        let temp_config = AMConfig {
            token: secret_token.clone(),
            identifier: "".to_string(),
            session_v2: "".to_string(),
        };
        let android_c = get_constants(1);

        let mut json_body = HashMap::new();
        json_body.insert(
            "i_token".to_string(),
            serde_json::Value::String(secret_token.clone()),
        );
        json_body.insert(
            "iap_product_version".to_string(),
            serde_json::Value::Number(serde_json::Number::from(0_u32)),
        );
        json_body.insert("app_login".to_string(), serde_json::Value::Bool(false));
        json_with_common(&mut json_body, android_c);

        let req = session
            .request(
                reqwest::Method::POST,
                format!("{}/iap/remainder.json", *BASE_API),
            )
            .headers(make_header(&temp_config, android_c)?)
            .json(&json_body)
            .send()
            .await?;

        let result = parse_response::<models::IAPRemainder>(req).await?;
        let result = result.result.body.unwrap();

        // Step 2: Perform login
        let mut json_body_login = HashMap::new();
        json_body_login.insert(
            "email".to_string(),
            serde_json::Value::String(email.to_string()),
        );
        json_body_login.insert(
            "citi_pass".to_string(),
            serde_json::Value::String(password.to_string()),
        );
        json_body_login.insert(
            "iap_token".to_string(),
            serde_json::Value::String(secret_token.clone()),
        );
        json_with_common(&mut json_body_login, android_c);

        let temp_config = AMConfig {
            token: secret_token.clone(),
            identifier: result.info.guest_id,
            session_v2: "".to_string(),
        };

        let req = session
            .request(
                reqwest::Method::POST,
                format!("{}/{}", *BASE_API, *MASKED_LOGIN),
            )
            .headers(make_header(&temp_config, android_c)?)
            .json(&json_body_login)
            .send()
            .await
            .unwrap();

        let result = parse_response::<models::LoginResult>(req).await.unwrap();
        let result = result.result.body.unwrap();

        // final step: get session_v2
        let mut json_body_session = HashMap::new();
        json_body_session.insert(
            "i_token".to_string(),
            serde_json::Value::String(secret_token.clone()),
        );
        json_body_session.insert(
            "iap_product_version".to_string(),
            serde_json::Value::Number(serde_json::Number::from(0_u32)),
        );
        json_body_session.insert("app_login".to_string(), serde_json::Value::Bool(true));
        json_with_common(&mut json_body_session, android_c);

        let temp_config = AMConfig {
            token: secret_token.clone(),
            identifier: result.info.guest_id.clone(),
            session_v2: "".to_string(),
        };

        let req = session
            .request(
                reqwest::Method::POST,
                format!("{}/iap/remainder.json", *BASE_API),
            )
            .headers(make_header(&temp_config, android_c)?)
            .json(&json_body_session)
            .send()
            .await?;

        if req.status() != reqwest::StatusCode::OK {
            anyhow::bail!("Failed to get session_v2");
        }

        // session_v2 is cookies
        let mut session_v2 = String::new();
        let cookie_name = SESSION_COOKIE_NAME.to_string();
        for cookie in cookie_store.lock().unwrap().iter_any() {
            if cookie.name() == cookie_name {
                session_v2 = cookie.value().to_string();
                break;
            }
        }

        if session_v2.is_empty() {
            anyhow::bail!("Failed to get session_v2");
        }

        Ok(AMConfig {
            token: secret_token,
            identifier: result.info.guest_id,
            session_v2,
        })
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
struct BasicWrapStatus {
    result: StatusResult,
}

async fn parse_response<T>(response: reqwest::Response) -> anyhow::Result<models::APIResult<T>>
where
    T: serde::de::DeserializeOwned + std::clone::Clone,
{
    let stat_code = response.status();
    let headers = response.headers().clone();
    let url = response.url().clone();
    let raw_text = response.text().await.unwrap();
    let status_resp = serde_json::from_str::<BasicWrapStatus>(&raw_text.clone()).unwrap_or_else(|_| panic!(
        "Failed to parse response.\nURL: {}\nStatus code: {}\nHeaders: {:?}\nContents: {}\nBacktrace",
        url, stat_code, headers, raw_text
    ));

    match status_resp.result.raise_for_status() {
        Ok(_) => {
            let parsed = serde_json::from_str(&raw_text).unwrap_or_else(|err| {
                panic!(
                    "Failed when deserializing response, error: {}\nURL: {}\nContents: {}",
                    err, url, raw_text
                )
            });
            Ok(parsed)
        }
        Err(e) => Err(anyhow::Error::new(e)),
    }
}

/// Create the request headers used for the API.
fn make_header(
    config: &AMConfig,
    constants: &constants::Constants,
) -> anyhow::Result<reqwest::header::HeaderMap> {
    let mut req_headers = reqwest::header::HeaderMap::new();

    let current_unix = chrono::Utc::now().timestamp();
    let av = format!("{}/{}", *APP_NAME, constants.version);
    let formulae = format!("{}{}{}", config.token, current_unix, av);

    let formulae_hashed = <Sha256 as Digest>::digest(formulae.as_bytes());
    let formulae_hashed = format!("{:x}", formulae_hashed);

    req_headers.insert(HEADER_NAMES.s.as_str(), formulae_hashed.parse()?);
    if !config.identifier.is_empty() {
        req_headers.insert(HEADER_NAMES.i.as_str(), config.identifier.parse()?);
    }
    req_headers.insert(HEADER_NAMES.n.as_str(), current_unix.to_string().parse()?);
    req_headers.insert(HEADER_NAMES.t.as_str(), config.token.clone().parse()?);

    Ok(req_headers)
}

fn json_with_common(
    json_obj: &mut HashMap<String, serde_json::Value>,
    constants: &constants::Constants,
) {
    let platform = constants.platform.to_string();
    let version = constants.version.to_string();
    let app_name = APP_NAME.to_string();

    json_obj.insert("app_name".to_string(), serde_json::Value::String(app_name));
    json_obj.insert("platform".to_string(), serde_json::Value::String(platform));
    json_obj.insert("version".to_string(), serde_json::Value::String(version));

    let mut screen = serde_json::Map::new();
    screen.insert(
        "inch".to_string(),
        serde_json::Value::Number(serde_json::Number::from_f64(SCREEN_INCH).unwrap()),
    );
    json_obj.insert("screen".to_string(), serde_json::Value::Object(screen));
}
