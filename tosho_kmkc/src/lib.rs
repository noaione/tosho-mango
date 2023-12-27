pub use config::*;
pub mod config;
pub mod constants;
pub mod imaging;
pub mod models;

#[derive(Debug)]
pub struct KMClient {
    inner: reqwest::Client,
    config: KMConfig,
}

impl KMClient {
    pub fn new(config: KMConfig) -> Self {
        Self {
            inner: reqwest::Client::new(),
            config,
        }
    }
}
