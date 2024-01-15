use std::{collections::HashMap, sync::MutexGuard};

use constants::{get_constants, API_HOST, APP_NAME, BASE_API, HEADER_NAMES, IMAGE_HOST};
use futures_util::StreamExt;
use helper::ComicPurchase;
use models::{APIResult, StatusResult};
use reqwest_cookie_store::CookieStoreMutex;
use sha2::{Digest, Sha256};
use tokio::io::AsyncWriteExt;

pub use config::*;
pub mod config;
pub mod constants;
pub mod helper;
pub mod models;

const SCREEN_INCH: f64 = 61.1918658356194;

#[derive(Clone)]
pub struct AMClient {
    inner: reqwest::Client,
    config: AMConfig,
    constants: &'static constants::Constants,
    cookie_store: std::sync::Arc<CookieStoreMutex>,
}

impl AMClient {
    pub fn new(config: AMConfig) -> Self {
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
            .cookie_provider(std::sync::Arc::clone(&cookie_store))
            .build()
            .unwrap();

        Self {
            inner: client,
            config,
            constants,
            cookie_store,
        }
    }

    /// Apply the JSON object with the default values.
    fn apply_json_object(&self, json_obj: &mut HashMap<String, serde_json::Value>) {
        let platform = self.constants.platform.to_string();
        let version = self.constants.version.to_string();
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

    /// Create the request headers used for the API.
    fn make_header(&self) -> anyhow::Result<reqwest::header::HeaderMap> {
        let mut req_headers = reqwest::header::HeaderMap::new();

        let current_unix = chrono::Utc::now().timestamp();
        let av = format!("{}/{}", *APP_NAME, self.constants.version);
        let formulae = format!("{}{}{}", self.config.token, current_unix, av);

        let formulae_hashed = <Sha256 as Digest>::digest(formulae.as_bytes());
        let formulae_hashed = format!("{:x}", formulae_hashed);

        req_headers.insert(HEADER_NAMES.s.as_str(), formulae_hashed.parse()?);
        req_headers.insert(HEADER_NAMES.i.as_str(), self.config.identifier.parse()?);
        req_headers.insert(HEADER_NAMES.n.as_str(), current_unix.to_string().parse()?);
        req_headers.insert(HEADER_NAMES.t.as_str(), self.config.token.clone().parse()?);

        Ok(req_headers)
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

        let headers = self.make_header()?;

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
            .content
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
            .content
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
            serde_json::Value::Number(serde_json::Number::from(episode.product)),
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
            .content
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
        let mut headers = self.make_header()?;
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
