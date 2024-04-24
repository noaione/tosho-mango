#![allow(dead_code)]

pub mod constants;
pub mod helper;
pub mod proto;

use std::{collections::HashMap, io::Cursor};

use constants::{Constants, API_HOST};
use prost::Message;
use proto::SuccessOrError;

use crate::constants::BASE_API;
pub use crate::helper::ImageQuality;

/// Main client for interacting with the M+ API.
///
/// # Example
/// ```no_run
/// use tosho_mplus::MPClient;
/// use tosho_mplus::constants::get_constants;
///
/// #[tokio::main]
/// async fn main() {
///     let client = MPClient::new("1234", get_constants(1));
/// }
/// ```
#[derive(Debug)]
pub struct MPClient {
    inner: reqwest::Client,
    secret: String,
    constants: &'static Constants,
}

impl MPClient {
    /// Create a new client instance.
    ///
    /// # Parameters
    /// * `secret` - The secret key to use for the client.
    /// * `constants` - The constants to use for the client.
    pub fn new(secret: &str, constants: &'static Constants) -> Self {
        Self::make_client(secret, constants, None)
    }

    fn make_client(
        secret: &str,
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
            constants,
        }
    }

    /// Modify the HashMap to add the required parameters.
    fn build_params(&self, params: &mut HashMap<String, String>) {
        params.insert("os".to_string(), self.constants.os_name.to_string());
        params.insert("os_ver".to_string(), self.constants.os_ver.to_string());
        params.insert("app_ver".to_string(), self.constants.app_ver.to_string());
        params.insert("secret".to_string(), self.secret.clone());
    }

    fn build_url(&self, path: &str) -> String {
        if path.starts_with('/') {
            return format!("{}{}", *BASE_API, path);
        }

        format!("{}/{}", *BASE_API, path)
    }

    fn empty_params(&self) -> HashMap<String, String> {
        let mut params: HashMap<String, String> = HashMap::new();

        self.build_params(&mut params);

        params
    }
}

async fn parse_response(
    res: reqwest::Response,
) -> anyhow::Result<Box<crate::proto::SuccessResponse>> {
    let bytes_data = res.bytes().await?;
    let cursor = bytes_data.as_ref();

    let decoded_response = crate::proto::Response::decode(&mut Cursor::new(cursor))?;
    // oneof response on .response
    match decoded_response.response {
        Some(SuccessOrError::Error(e)) => anyhow::bail!("Error response: {:?}", e),
        Some(SuccessOrError::Success(s)) => Ok(s),
        None => anyhow::bail!("No response found"),
    }
}
