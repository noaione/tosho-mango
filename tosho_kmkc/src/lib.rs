use std::collections::HashMap;

pub use config::*;
pub mod config;
pub mod constants;
pub mod imaging;
pub mod models;
use constants::{ANDROID_CONSTANTS, BASE_API, WEB_CONSTANTS};
use md5::Md5;
use models::{
    AccountResponse, BulkEpisodePurchaseResponse, EpisodeNode, EpisodePurchaseResponse,
    EpisodeViewerResponse, EpisodesListResponse, GenreSearchResponse, KMAPINotEnoughPointsError,
    MagazineCategoryResponse, MobileEpisodeViewerResponse, RankingListResponse, SearchResponse,
    StatusResponse, TicketInfoType, TitleListResponse, TitleNode, TitlePurchaseNode,
    TitlePurchaseResponse, TitleTicketListNode, TitleTicketListResponse, UserPoint,
    UserPointResponse, WebEpisodeViewerResponse, WeeklyListResponse,
};
use reqwest_cookie_store::CookieStoreMutex;
use sha2::{Digest, Sha256, Sha512};

/// Main client for interacting with the SQ MU!
///
/// # Example
/// ```no_run,ignore
/// use tosho_kmkc::KMClient;
///
/// #[tokio::main]
/// async fn main() {
///     let config = KMConfig::Web(..);
///     let client = KMClient::new(config);
///     let manga = client.get_titles(vec![10007]).await.unwrap();
///     println!("{:?}", manga);
/// }
/// ```
#[derive(Debug)]
pub struct KMClient {
    inner: reqwest::Client,
    config: KMConfig,
}

impl KMClient {
    /// Create a new [`KMClient`] with the given config.
    ///
    /// # Arguments
    /// * `config` - The config to use for the client
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

                let hash_key = &mobile.hash_key;

                let mut query_params = query_params.clone();
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

    /// Make an authenticated request to the API.
    ///
    /// This request will automatically add all the required headers/cookies/auth method
    /// to the request.
    ///
    /// # Arguments
    /// * `method` - The HTTP method to use
    /// * `endpoint` - The endpoint to request (e.g. `/episode/list`)
    /// * `data` - The data to send in the request body (as form data)
    /// * `params` - The query params to send in the request
    /// * `headers` - The headers to send in the request
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
        let endpoint = format!("{}{}", BASE_API.as_str(), endpoint);
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

    /// Get the list of episodes from the given list of episode IDs
    ///
    /// # Arguments
    /// * `episodes` - The list of episode IDs to get
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
                "/episode/list",
                Some(data),
                None,
                None,
            )
            .await?;

        Ok(responses.episodes)
    }

    /// Get the list of titles from the given list of title IDs
    ///
    /// # Arguments
    /// * `titles` - The list of title IDs to get
    pub async fn get_titles(&self, titles: Vec<i32>) -> anyhow::Result<Vec<TitleNode>> {
        let mut data = HashMap::new();
        let title_str = titles
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        data.insert("title_id_list".to_string(), title_str.join(","));

        let responses = self
            .request::<TitleListResponse>(
                reqwest::Method::GET,
                "/title/list",
                None,
                Some(data),
                None,
            )
            .await?;

        Ok(responses.titles)
    }

    /// Get the episode viewer for the given episode ID.
    ///
    /// The following will return an enum depending on the config used.
    ///
    /// If you're using web config, please remember to descramble the images
    /// with the [`imaging::descramble_image`] function.
    ///
    /// # Arguments
    /// * `episode_id` - The episode ID to get the viewer for
    pub async fn get_episode_viewer(
        &self,
        episode: EpisodeNode,
    ) -> anyhow::Result<EpisodeViewerResponse> {
        match &self.config {
            KMConfig::Web(_) => {
                let mut params = HashMap::new();
                params.insert("episode_id".to_string(), episode.id.to_string());

                let response = self
                    .request::<WebEpisodeViewerResponse>(
                        reqwest::Method::GET,
                        "/web/episode/viewer",
                        None,
                        Some(params),
                        None,
                    )
                    .await?;

                Ok(EpisodeViewerResponse::Web(response))
            }
            KMConfig::Mobile(_) => {
                let mut params = HashMap::new();
                params.insert("episode_id".to_string(), episode.id.to_string());
                params.insert("force_master".to_string(), "1".to_string());
                params.insert("is_download".to_string(), "1".to_string());
                if let Some(magazine_id) = episode.magazine_id {
                    params.insert("magazine_id".to_string(), magazine_id.to_string());
                }

                let response = self
                    .request::<MobileEpisodeViewerResponse>(
                        reqwest::Method::GET,
                        "/episode/viewer",
                        None,
                        Some(params),
                        None,
                    )
                    .await?;

                Ok(EpisodeViewerResponse::Mobile(response))
            }
        }
    }

    /// Get the title ticket for the given title ID.
    ///
    /// # Arguments
    /// * `title_id` - The title ID to get the ticket for
    pub async fn get_title_ticket(&self, title_id: i32) -> anyhow::Result<TitleTicketListNode> {
        let mut params = HashMap::new();
        params.insert("title_id_list".to_string(), title_id.to_string());

        let response = self
            .request::<TitleTicketListResponse>(
                reqwest::Method::GET,
                "/title/ticket/list",
                None,
                Some(params),
                None,
            )
            .await?;

        Ok(response.tickets[0].clone())
    }

    /// Claim or purchase an episode with a user's point.
    ///
    /// # Arguments
    /// * `episode` - The episode to claim
    /// * `wallet` - The user's point wallet (mutable).
    pub async fn claim_episode(
        &self,
        episode: &EpisodeNode,
        wallet: &mut UserPoint,
    ) -> anyhow::Result<EpisodePurchaseResponse> {
        if !wallet.can_purchase(episode.point.try_into().unwrap_or(0)) {
            // bail with custom error
            return Err(anyhow::Error::new(KMAPINotEnoughPointsError {
                message: "Not enough points to purchase episode".to_string(),
                points_needed: episode.point.try_into().unwrap_or(0),
                points_have: wallet.total_point(),
            }));
        }

        let mut data = HashMap::new();
        data.insert("episode_id".to_owned(), episode.id.to_string());
        data.insert("check_point".to_owned(), episode.point.to_string());

        let response = self
            .request::<EpisodePurchaseResponse>(
                reqwest::Method::POST,
                "/episode/paid",
                Some(data),
                None,
                None,
            )
            .await?;

        wallet.subtract(response.paid.try_into().unwrap_or(0));
        wallet.add(episode.bonus_point.try_into().unwrap_or(0));

        Ok(response)
    }

    /// Bulk claim or purchase episodes with a user's point.
    ///
    /// # Arguments
    /// * `episodes` - The episodes to claim
    /// * `wallet` - The user's point wallet (mutable).
    pub async fn claim_episodes(
        &self,
        episodes: Vec<&EpisodeNode>,
        wallet: &mut UserPoint,
    ) -> anyhow::Result<BulkEpisodePurchaseResponse> {
        let mut data = HashMap::new();
        let mut episode_ids = vec![];

        let mut paid_point = 0_u64;
        let mut bonus_point = 0_u64;

        for episode in episodes {
            episode_ids.push(episode.id.to_string());

            paid_point += episode.point.try_into().unwrap_or(0);
            bonus_point += episode.bonus_point.try_into().unwrap_or(0);
        }

        let mut cloned_wallet = wallet.clone();
        cloned_wallet.add(bonus_point);
        if !cloned_wallet.can_purchase(paid_point) {
            // bail with custom error
            return Err(anyhow::Error::new(KMAPINotEnoughPointsError {
                message: "Not enough points to purchase episode".to_string(),
                points_needed: paid_point,
                points_have: cloned_wallet.total_point(),
            }));
        }

        data.insert("episode_id_list".to_owned(), episode_ids.join(","));
        data.insert("paid_point".to_owned(), paid_point.to_string());
        data.insert("point_back".to_owned(), bonus_point.to_string());

        let response = self
            .request::<BulkEpisodePurchaseResponse>(
                reqwest::Method::POST,
                "/episode/paid/bulk",
                Some(data),
                None,
                None,
            )
            .await?;

        wallet.subtract(response.paid.try_into().unwrap_or(0));
        wallet.add(response.point_back.try_into().unwrap_or(0));

        Ok(response)
    }

    /// Claim or purchase an episode with a ticket.
    ///
    /// This will return the status of the claim, and whether or not the ticket is a title ticket.
    ///
    /// # Arguments
    /// * `episode_id` - The episode ID to claim
    /// * `ticket` - The ticket to use to claim the episode
    pub async fn claim_episode_with_ticket(
        &self,
        episode_id: i32,
        ticket: &TicketInfoType,
    ) -> anyhow::Result<(StatusResponse, bool)> {
        let mut data = HashMap::new();
        data.insert("episode_id".to_owned(), episode_id.to_string());

        let mut is_title = false;
        match ticket {
            TicketInfoType::Premium(_) => {
                data.insert("ticket_version".to_owned(), "1".to_owned());
                data.insert("ticket_type".to_owned(), "99".to_owned());
            }
            TicketInfoType::Title(title) => {
                data.insert("ticket_version".to_owned(), title.version.to_string());
                data.insert("ticket_type".to_owned(), title.r#type.to_string());
                is_title = true;
            }
        }

        let response = self
            .request::<StatusResponse>(
                reqwest::Method::POST,
                "/episode/rental/ticket",
                Some(data),
                None,
                None,
            )
            .await?;

        Ok((response, is_title))
    }

    /// Get the user's point.
    pub async fn get_user_point(&self) -> anyhow::Result<UserPointResponse> {
        let response = self
            .request::<UserPointResponse>(reqwest::Method::GET, "/account/point", None, None, None)
            .await?;

        Ok(response)
    }

    /// Search for a title by name.
    ///
    /// # Arguments
    /// * `query` - The query to search for
    /// * `limit` - The limit of results to return
    pub async fn search(&self, query: &str, limit: Option<u32>) -> anyhow::Result<Vec<TitleNode>> {
        let mut params = HashMap::new();
        params.insert("keyword".to_owned(), query.to_owned());
        let limit = limit.unwrap_or(99_999);
        params.insert("limit".to_owned(), limit.to_string());

        let response = self
            .request::<SearchResponse>(
                reqwest::Method::GET,
                "/search/title",
                None,
                Some(params),
                None,
            )
            .await?;

        Ok(response.titles)
    }

    /// Get the weekly ranking/list.
    pub async fn get_weekly(&self) -> anyhow::Result<WeeklyListResponse> {
        let response = self
            .request::<WeeklyListResponse>(reqwest::Method::GET, "/title/weekly", None, None, None)
            .await?;

        Ok(response)
    }

    /// Get the current user's account information.
    pub async fn get_account(&self) -> anyhow::Result<AccountResponse> {
        let response = self
            .request::<AccountResponse>(reqwest::Method::GET, "/account", None, None, None)
            .await?;

        Ok(response)
    }

    /// Get the user's purchased titles.
    pub async fn get_purchased(&self) -> anyhow::Result<Vec<TitlePurchaseNode>> {
        let response = self
            .request::<TitlePurchaseResponse>(
                reqwest::Method::GET,
                "/web/title/purchased",
                None,
                None,
                None,
            )
            .await?;

        Ok(response.titles)
    }

    /// Get the magazine list.
    pub async fn get_magazines(&self) -> anyhow::Result<MagazineCategoryResponse> {
        let mut params = HashMap::new();
        params.insert("limit".to_owned(), "99999".to_owned());
        params.insert("offset".to_owned(), "0".to_owned());
        let response = self
            .request::<MagazineCategoryResponse>(
                reqwest::Method::GET,
                "/magazine/category/list",
                None,
                Some(params),
                None,
            )
            .await?;

        Ok(response)
    }

    /// Get the genre list.
    pub async fn get_genres(&self) -> anyhow::Result<GenreSearchResponse> {
        let response = self
            .request::<GenreSearchResponse>(
                reqwest::Method::GET,
                "/genre/search/list",
                None,
                None,
                None,
            )
            .await?;

        Ok(response)
    }

    /// Get title rankings for a specific ranking ID.
    ///
    /// See [`constants::RANKING_TABS`] for the list of available ranking IDs.
    ///
    /// # Arguments
    /// * `ranking_id` - The ranking ID to get
    /// * `limit` - The limit of results to return
    /// * `offset` - The offset of results to return
    pub async fn get_all_rankings(
        &self,
        ranking_id: u32,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> anyhow::Result<RankingListResponse> {
        let mut params = HashMap::new();
        params.insert("ranking_id".to_owned(), ranking_id.to_string());
        params.insert("limit".to_owned(), limit.unwrap_or(101).to_string());
        params.insert("offset".to_owned(), offset.unwrap_or(0).to_string());

        let response = self
            .request::<RankingListResponse>(
                reqwest::Method::GET,
                "/ranking/all",
                None,
                Some(params),
                None,
            )
            .await?;

        Ok(response)
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
