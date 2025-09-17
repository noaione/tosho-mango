#![warn(missing_docs, clippy::empty_docs, rustdoc::broken_intra_doc_links)]
#![doc = include_str!("../README.md")]

use std::collections::HashMap;

use tosho_common::{ToshoClientError, ToshoResult, bail_on_error, parse_json_response_failable};

use crate::{constants::BASE_API, models::ErrorResponse};

pub mod constants;
pub mod models;

/// Main client for interacting with the NI API.
///
/// # Examples
/// ```rust,no_run
/// use tosho_nids::NidsClient;
///
/// #[tokio::main]
/// async fn main() {
///     let constants = tosho_nids::constants::get_constants(1); // Web
///     let client = NidsClient::new(None, constants);
/// }
/// ```
#[derive(Clone)]
pub struct NidsClient {
    inner: reqwest::Client,
    constants: &'static crate::constants::Constants,
    token: Option<String>,
}

impl std::fmt::Debug for NidsClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NidsClient")
            .field("inner", &"reqwest::Client")
            .field("constants", &self.constants)
            .field("token", &self.token.as_deref().map(|_| "****"))
            .finish()
    }
}

impl NidsClient {
    /// Create a new client instance.
    ///
    /// # Parameters
    /// * `token` - JWT token for download requests, if `None` you will only be able to make non-authenticated requests.
    /// * `constants` - Constants to use for the client, see [`crate::constants::get_constants`].
    pub fn new(
        token: Option<impl AsRef<str>>,
        constants: &'static crate::constants::Constants,
    ) -> ToshoResult<Self> {
        Self::make_client(token, constants, None)
    }

    /// Attach a proxy to the client.
    ///
    /// This will clone the client and return a new client with the proxy attached.
    ///
    /// # Arguments
    /// * `proxy` - The proxy to attach to the client
    pub fn with_proxy(&self, proxy: reqwest::Proxy) -> ToshoResult<Self> {
        Self::make_client(self.token.as_deref(), self.constants, Some(proxy))
    }

    fn make_client(
        token: Option<impl AsRef<str>>,
        constants: &'static crate::constants::Constants,
        proxy: Option<reqwest::Proxy>,
    ) -> ToshoResult<Self> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static(constants.ua),
        );
        headers.insert(
            reqwest::header::ORIGIN,
            reqwest::header::HeaderValue::from_static(crate::constants::BASE_WEB),
        );
        headers.insert(
            reqwest::header::REFERER,
            reqwest::header::HeaderValue::from_static(crate::constants::BASE_WEB),
        );
        headers.insert(
            reqwest::header::HOST,
            reqwest::header::HeaderValue::from_static(crate::constants::API_HOST),
        );
        let token_data: Option<&str> = token.as_ref().map(|t| t.as_ref());
        if let Some(t) = token_data {
            headers.insert(
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(t).map_err(|_| {
                    ToshoClientError::HeaderParseError("Failed to parse token value".into())
                })?,
            );
        }

        let client = reqwest::Client::builder()
            .http2_adaptive_window(true)
            .use_rustls_tls()
            .default_headers(headers);

        let client = match proxy {
            Some(proxy) => client
                .proxy(proxy)
                .build()
                .map_err(ToshoClientError::BuildError),
            None => client.build().map_err(ToshoClientError::BuildError),
        }?;

        Ok(Self {
            inner: client,
            constants,
            token: token_data.map(|s| s.to_string()),
        })
    }

    /// Make an authenticated request to the API.
    ///
    /// This request will automatically add all the required headers/cookies/auth method
    /// to the request.
    ///
    /// # Arguments
    /// * `method` - The HTTP method to use
    /// * `endpoint` - The endpoint to request (e.g. `/list`) - without the `/api/v1` prefix
    /// * `data` - The data to send in the request body (as form data)
    /// * `params` - The query params to send in the request
    #[expect(dead_code)]
    async fn request<T>(
        &self,
        method: reqwest::Method,
        endpoint: &str,
        data: Option<HashMap<String, String>>,
        params: Option<HashMap<String, String>>,
    ) -> ToshoResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let endpoint = format!("{}/api/v1{}", BASE_API, endpoint);

        let request = match (data.clone(), params.clone()) {
            (None, None) => self.inner.request(method, endpoint),
            (Some(data), None) => {
                let mut extend_headers = reqwest::header::HeaderMap::new();
                extend_headers.insert(
                    reqwest::header::CONTENT_TYPE,
                    reqwest::header::HeaderValue::from_static("application/json"),
                );
                self.inner
                    .request(method, endpoint)
                    .json(&data)
                    .headers(extend_headers)
            }
            (None, Some(params)) => self.inner.request(method, endpoint).query(&params),
            (Some(_), Some(_)) => {
                bail_on_error!("Cannot have both data and params")
            }
        };

        parse_json_response_failable::<T, ErrorResponse>(request.send().await?).await
    }
}
