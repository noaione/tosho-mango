use std::collections::HashMap;

pub use config::*;
pub mod config;
pub mod constants;
pub mod imaging;
pub mod models;
use constants::{ANDROID_CONSTANTS, BASE_API, WEB_CONSTANTS};
use md5::Md5;
use models::{EpisodeNode, EpisodesListResponse, StatusResponse};
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

    fn format_request(&self, query_params: &mut HashMap<String, String>) -> String {
        let platform = match &self.config {
            KMConfig::Web(_) => WEB_CONSTANTS.platform,
            KMConfig::Mobile(_) => ANDROID_CONSTANTS.platform,
        };
        let version = match &self.config {
            KMConfig::Web(_) => WEB_CONSTANTS.version,
            KMConfig::Mobile(_) => ANDROID_CONSTANTS.version,
        };
        query_params.insert("platform".to_string(), platform.to_string());
        query_params.insert("version".to_string(), version.to_string());

        let hash = self.create_request_hash(query_params.clone());
        hash
    }

    async fn request<T>(
        &self,
        method: reqwest::Method,
        endpoint: &str,
        data: Option<HashMap<String, String>>,
        params: Option<HashMap<String, String>>,
        headers: Option<reqwest::header::HeaderMap>,
    ) -> anyhow::Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut extend_headers = match headers {
            Some(headers) => headers,
            None => reqwest::header::HeaderMap::new(),
        };
        let hash_header = match &self.config {
            KMConfig::Web(_) => WEB_CONSTANTS.hash.as_str(),
            KMConfig::Mobile(_) => ANDROID_CONSTANTS.hash.as_str(),
        };

        let hash_value = match data.clone() {
            Some(mut data) => self.format_request(&mut data),
            None => match params.clone() {
                Some(mut params) => self.format_request(&mut params),
                None => "".to_string(),
            },
        };

        let mut empty_params: HashMap<String, String> = HashMap::new();
        let mut empty_headers = reqwest::header::HeaderMap::new();
        empty_headers
            .insert(hash_header, self.format_request(&mut empty_params).parse()?)
            .unwrap();

        extend_headers
            .insert(hash_header, hash_value.parse()?)
            .unwrap();

        let request = match (data.clone(), params.clone()) {
            (None, None) => self
                .inner
                .request(method, endpoint)
                .query(&empty_params)
                .headers(empty_headers),
            (Some(data), None) => {
                extend_headers.insert(
                    reqwest::header::CONTENT_TYPE,
                    "application/x-www-form-urlencoded".parse()?,
                );
                self.inner
                    .request(method, endpoint)
                    .form(&data)
                    .headers(extend_headers)
            }
            (None, Some(params)) => self
                .inner
                .request(method, endpoint)
                .query(&params)
                .headers(extend_headers),
            (Some(_), Some(_)) => {
                anyhow::bail!("Cannot have both data and params")
            }
        };

        Ok(parse_response(request.send().await?).await?)
    }

    pub async fn get_episodes(&self, episodes: Vec<i32>) -> anyhow::Result<Vec<EpisodeNode>> {
        let mut data = HashMap::new();
        let episode_str = episodes
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        data.insert("episode_id_list".to_string(), episode_str.join(","));

        let responses = self
            .request::<EpisodesListResponse>(
                reqwest::Method::POST,
                &format!("{}/episode/list", BASE_API.as_str()),
                Some(data),
                None,
                None,
            )
            .await?;

        Ok(responses.episodes)
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

async fn parse_response<T>(response: reqwest::Response) -> anyhow::Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let raw_text = response.text().await.unwrap();
    let status_resp = serde_json::from_str::<StatusResponse>(&raw_text.clone()).unwrap();

    match status_resp.raise_for_status() {
        Ok(_) => {
            let parsed = serde_json::from_str(&raw_text).unwrap();
            Ok(parsed)
        }
        Err(e) => Err(anyhow::Error::new(e)),
    }
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
