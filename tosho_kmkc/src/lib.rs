use std::collections::HashMap;

pub use config::*;
pub mod config;
pub mod constants;
pub mod imaging;
pub mod models;
use constants::{ANDROID_CONSTANTS, BASE_API, WEB_CONSTANTS};
use md5::Md5;
use reqwest_cookie_store::CookieStoreMutex;
use sha2::{Digest, Sha256, Sha512};

#[derive(Debug)]
pub struct KMClient {
    inner: reqwest::Client,
    config: KMConfig,
}

impl KMClient {
    pub fn new(config: KMConfig) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            reqwest::header::HOST,
            reqwest::header::HeaderValue::from_static(&BASE_API),
        );
        match config {
            KMConfig::Web(web) => {
                headers.insert(
                    reqwest::header::USER_AGENT,
                    reqwest::header::HeaderValue::from_str(&WEB_CONSTANTS.ua).unwrap(),
                );

                let cookie_store = CookieStoreMutex::from(web.clone());
                let cookie_store = std::sync::Arc::new(cookie_store);

                // make cookie store
                let client = reqwest::Client::builder()
                    .default_headers(headers)
                    .cookie_provider(cookie_store)
                    .build()
                    .unwrap();

                Self {
                    inner: client,
                    config: KMConfig::Web(web),
                }
            }
            KMConfig::Mobile(mobile) => {
                headers.insert(
                    reqwest::header::USER_AGENT,
                    reqwest::header::HeaderValue::from_static(&ANDROID_CONSTANTS.ua),
                );

                let client = reqwest::Client::builder()
                    .default_headers(headers)
                    .build()
                    .unwrap();

                Self {
                    inner: client,
                    config: KMConfig::Mobile(mobile),
                }
            }
        }
    }

    /// Create the request hash for any given query params
    ///
    /// # Arguments
    /// * `query_params` - The query params to hash
    fn create_request_hash(&self, query_params: HashMap<String, String>) -> String {
        match &self.config {
            KMConfig::Web(web) => {
                let birthday = &web.birthday.value;
                let expires = web.birthday.expires.to_string();

                let mut keys = query_params.keys().collect::<Vec<&String>>();
                keys.sort();

                let mut qi_s: Vec<String> = vec![];
                for key in keys {
                    let value = query_params.get(key).unwrap();
                    let hashed = hash_kv(key, value);
                    qi_s.push(hashed);
                }

                let qi_s_hashed = Sha256::digest(qi_s.join(",").as_bytes());
                let birth_expire_hash = hash_kv(&birthday, &expires);

                let merged_hash =
                    Sha512::digest(format!("{:x}{}", qi_s_hashed, birth_expire_hash).as_bytes());

                format!("{:x}", merged_hash)
            }
            KMConfig::Mobile(mobile) => {
                let mut hasher = Sha256::new();

                let hash_key = &mobile.user_token;

                let mut query_params = query_params;
                query_params.insert("hash_key".to_string(), hash_key.to_string());

                // iterate sorted keys
                let mut keys = query_params.keys().collect::<Vec<&String>>();
                keys.sort();

                for key in keys {
                    let value = query_params.get(key).unwrap();
                    let hashed_value = Md5::digest(value.as_bytes());

                    hasher.update(hashed_value);
                }

                let hashed = hasher.finalize();
                format!("{:x}", hashed)
            }
        }
    }
}

fn hash_kv(key: &str, value: &str) -> String {
    // convert to bytes (utf-8)
    let key = key.as_bytes();
    let value = value.as_bytes();

    // create hasher
    let hasher256 = Sha256::digest(key);
    let hasher512 = Sha512::digest(value);

    format!("{:x}_{:x}", hasher256, hasher512)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_kv() {
        let key = "key";
        let value = "value";

        let hashed = hash_kv(key, value);
        assert_eq!(hashed, "2c70e12b7a0646f92279f427c7b38e7334d8e5389cff167a1dc30e73f826b683_ec2c83edecb60304d154ebdb85bdfaf61a92bd142e71c4f7b25a15b9cb5f3c0ae301cfb3569cf240e4470031385348bc296d8d99d09e06b26f09591a97527296".to_string())
    }
}
