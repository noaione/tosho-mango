#![allow(dead_code)]

pub mod constants;
pub mod helper;
pub mod proto;

use std::{collections::HashMap, io::Cursor};

use constants::{Constants, API_HOST};
use prost::Message;
use proto::{ErrorResponse, Language, SuccessOrError};

use crate::constants::BASE_API;
pub use crate::helper::ImageQuality;

/// Main client for interacting with the M+ API.
///
/// # Example
/// ```no_run
/// use tosho_mplus::MPClient;
/// use tosho_mplus::proto::Language;
/// use tosho_mplus::constants::get_constants;
///
/// #[tokio::main]
/// async fn main() {
///     let client = MPClient::new("1234", Language::English, get_constants(1));
/// }
/// ```
///
/// # Boxed
///
/// All responses are [`Box`]-ed since it has widely varying sizes.
/// ```
#[derive(Debug)]
pub struct MPClient {
    inner: reqwest::Client,
    secret: String,
    language: Language,
    constants: &'static Constants,
}

impl MPClient {
    /// Create a new client instance.
    ///
    /// # Parameters
    /// * `secret` - The secret key to use for the client.
    /// * `language` - The language to use for the client.
    /// * `constants` - The constants to use for the client.
    pub fn new(secret: &str, language: Language, constants: &'static Constants) -> Self {
        Self::make_client(secret, language, constants, None)
    }

    /// Attach a proxy to the client.
    ///
    /// This will clone the client and return a new client with the proxy attached.
    ///
    /// # Arguments
    /// * `proxy` - The proxy to attach to the client
    pub fn with_proxy(&self, proxy: reqwest::Proxy) -> Self {
        Self::make_client(&self.secret, self.language, self.constants, Some(proxy))
    }

    fn make_client(
        secret: &str,
        language: Language,
        constants: &'static Constants,
        proxy: Option<reqwest::Proxy>,
    ) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Host",
            reqwest::header::HeaderValue::from_str(&API_HOST).unwrap(),
        );
        headers.insert(
            "User-Agent",
            reqwest::header::HeaderValue::from_str(&constants.api_ua).unwrap(),
        );

        let client = reqwest::Client::builder()
            .http2_adaptive_window(true)
            .use_rustls_tls()
            .default_headers(headers);

        let client = match proxy {
            Some(proxy) => client.proxy(proxy).build().unwrap(),
            None => client.build().unwrap(),
        };

        Self {
            inner: client,
            secret: secret.to_string(),
            language,
            constants,
        }
    }

    /// Modify the HashMap to add the required parameters.
    fn build_params(&self, params: &mut HashMap<String, String>, with_lang: bool) {
        params.insert("os".to_string(), self.constants.os_name.to_string());
        params.insert("os_ver".to_string(), self.constants.os_ver.to_string());
        params.insert("app_ver".to_string(), self.constants.app_ver.to_string());
        params.insert("secret".to_string(), self.secret.clone());
        if with_lang {
            params.insert(
                "clang".to_string(),
                self.language.as_language_code().to_owned(),
            );
            params.insert(
                "lang".to_string(),
                self.language.as_language_code().to_owned(),
            );
        }
    }

    fn build_url(&self, path: &str) -> String {
        if path.starts_with('/') {
            return format!("{}{}", *BASE_API, path);
        }

        format!("{}/{}", *BASE_API, path)
    }

    fn empty_params(&self, with_lang: bool) -> HashMap<String, String> {
        let mut params: HashMap<String, String> = HashMap::new();

        self.build_params(&mut params, with_lang);

        params
    }

    /// Get the initial view of the app.
    pub async fn get_initial(&self) -> anyhow::Result<APIResponse<proto::InitialViewV2>> {
        let request = self
            .inner
            .get(&self.build_url("init_v2"))
            .query(&self.empty_params(false))
            .send()
            .await?;

        let response = parse_response(request).await?;

        match response {
            SuccessOrError::Success(data) => match data.initial_view_v2 {
                Some(inner_data) => Ok(APIResponse::Success(Box::new(inner_data))),
                None => anyhow::bail!("No initial view found"),
            },
            SuccessOrError::Error(error) => Ok(APIResponse::Error(error)),
        }
    }

    /// Get the main home view of the app.
    pub async fn get_home_page(&self) -> anyhow::Result<APIResponse<proto::HomeViewV3>> {
        let mut query_params = self.empty_params(true);
        query_params.insert("viewer_mode".to_string(), "horizontal".to_string());

        let request = self
            .inner
            .get(&self.build_url("home_v4"))
            .query(&query_params)
            .send()
            .await?;

        let response = parse_response(request).await?;

        match response {
            SuccessOrError::Success(data) => match data.home_view_v3 {
                Some(inner_data) => Ok(APIResponse::Success(Box::new(inner_data))),
                None => anyhow::bail!("No home view found"),
            },
            SuccessOrError::Error(error) => Ok(APIResponse::Error(error)),
        }
    }

    /// Get the user settings.
    pub async fn get_user_settings(&self) -> anyhow::Result<APIResponse<proto::UserSettingsV2>> {
        let mut query_params = self.empty_params(true);
        query_params.insert("viewer_mode".to_string(), "horizontal".to_string());

        let request = self
            .inner
            .get(&self.build_url("settings_v2"))
            .query(&query_params)
            .send()
            .await?;

        let response = parse_response(request).await?;

        match response {
            SuccessOrError::Success(data) => match data.user_settings_v2 {
                Some(inner_data) => Ok(APIResponse::Success(Box::new(inner_data))),
                None => anyhow::bail!("No user settings found"),
            },
            SuccessOrError::Error(error) => Ok(APIResponse::Error(error)),
        }
    }
}

/// A common return type for all API calls.
///
/// It either returns the specifid success response or an error.
pub enum APIResponse<T: ::prost::Message + Clone> {
    Error(Box<ErrorResponse>),
    Success(Box<T>),
}

async fn parse_response(res: reqwest::Response) -> anyhow::Result<SuccessOrError> {
    let bytes_data = res.bytes().await?;
    let cursor = bytes_data.as_ref();

    let decoded_response = crate::proto::Response::decode(&mut Cursor::new(cursor))?;
    // oneof response on .response
    match decoded_response.response {
        Some(response) => Ok(response),
        None => anyhow::bail!("No response found"),
    }
}
