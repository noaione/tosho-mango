#![doc = include_str!("../README.md")]

use constants::{API_HOST, USER_AGENT};
use tosho_common::{ToshoClientError, ToshoResult};

pub mod constants;

/// Main client for interacting with D.io API
///
/// # Example
/// ```rust,no_run
/// // TODO: Add example
/// ```
#[derive(Debug)]
#[allow(dead_code)]
pub struct DioClient {
    inner: reqwest::Client,
    secret: String,
}

impl DioClient {
    /// Create a new [`DioClient`] with the given secret.
    ///
    /// # Arguments
    /// * `secret` - The secret to use for the client
    pub fn new(secret: impl Into<String>) -> ToshoResult<Self> {
        Self::make_client(secret, None)
    }

    /// Attach a proxy to the client.
    ///
    /// This will clone the client and return a new client with the proxy attached.
    ///
    /// # Arguments
    /// * `proxy` - The proxy to attach to the client
    pub fn with_proxy(&self, proxy: reqwest::Proxy) -> ToshoResult<Self> {
        Self::make_client(self.secret.clone(), Some(proxy))
    }

    fn make_client(secret: impl Into<String>, proxy: Option<reqwest::Proxy>) -> ToshoResult<Self> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static(&USER_AGENT),
        );
        headers.insert(
            reqwest::header::HOST,
            reqwest::header::HeaderValue::from_static(&API_HOST),
        );

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
            secret: secret.into(),
        })
    }
}
