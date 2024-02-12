pub mod config;
pub mod constants;
pub mod helper;
pub mod imaging;
pub mod models;
pub use config::*;
use constants::{API_HOST, HEADER_PIECE, VALUE_PIECE};

/// Main client for interacting with the VM API.
///
/// # Examples
/// ```no_run,ignore
/// use tosho_sjv::{SJClient, SJConfig, SJMode};
/// use tosho_sjv::constants::get_constants;
///
/// #[tokio::main]
/// async fn main() {
///     let config = SJConfig {
///         user_id: 123,
///         token: "xyz987abc",
///         instance: "abcxyz",
///     };
///     let constants = get_constants(1);
///
///     let client = SJClient::new(config, constants, SJMode::VM);
///     let manga = client.get_manga(777).await.unwrap();
///     println!("{:?}", manga);
/// }
/// ```
#[derive(Debug)]
pub struct SJClient {
    #[allow(dead_code)]
    inner: reqwest::Client,
    config: SJConfig,
    constants: &'static crate::constants::Constants,
    mode: SJMode,
}

impl SJClient {
    /// Create a new client instance.
    ///
    /// # Parameters
    /// * `config` - The configuration to use for the client.
    /// * `constants` - The constants to use for the client.
    /// * `mode` - The mode to use for the client.
    pub fn new(
        config: SJConfig,
        constants: &'static crate::constants::Constants,
        mode: SJMode,
    ) -> Self {
        Self::make_client(config, constants, mode, None)
    }

    /// Attach a proxy to the client.
    ///
    /// This will clone the client and return a new client with the proxy attached.
    ///
    /// # Arguments
    /// * `proxy` - The proxy to attach to the client
    pub fn with_proxy(&self, proxy: reqwest::Proxy) -> Self {
        Self::make_client(self.config.clone(), self.constants, self.mode, Some(proxy))
    }

    fn make_client(
        config: SJConfig,
        constants: &'static crate::constants::Constants,
        mode: SJMode,
        proxy: Option<reqwest::Proxy>,
    ) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static(constants.ua),
        );
        headers.insert(
            reqwest::header::HOST,
            reqwest::header::HeaderValue::from_static(&API_HOST),
        );
        let referer = match mode {
            SJMode::VM => &constants.vm_name,
            SJMode::SJ => &constants.sj_name,
        };
        headers.insert(
            reqwest::header::REFERER,
            reqwest::header::HeaderValue::from_str(referer).unwrap(),
        );

        let x_header = format!("{} {}", constants.app_ver, *VALUE_PIECE);
        headers.insert(
            reqwest::header::HeaderName::from_static(&HEADER_PIECE),
            reqwest::header::HeaderValue::from_str(&x_header).unwrap(),
        );

        let client = reqwest::Client::builder().default_headers(headers);

        let client = match proxy {
            Some(proxy) => client.proxy(proxy).build().unwrap(),
            None => client.build().unwrap(),
        };

        Self {
            inner: client,
            config,
            constants,
            mode,
        }
    }
}
