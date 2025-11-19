#![warn(missing_docs, clippy::empty_docs, rustdoc::broken_intra_doc_links)]
#![doc = include_str!("../README.md")]

use std::{collections::HashMap, sync::MutexGuard};

pub use config::*;
pub mod config;
pub mod constants;
pub mod imaging;
pub mod models;
use constants::{
    API_HOST, BASE_API, HEADER_CRAWLER, HEADER_PLATFORM, IMAGE_HOST, WEB_CONSTANTS, get_constants,
};
use futures_util::TryStreamExt;
use md5::Md5;
use models::{
    AccountResponse, BulkEpisodePurchaseResponse, EpisodeNode, EpisodePurchaseResponse,
    EpisodeViewerFinishResponse, EpisodeViewerResponse, EpisodesListResponse, GenreSearchResponse,
    KMAPINotEnoughPointsError, MagazineCategoryResponse, MobileEpisodeViewerResponse,
    RankingListResponse, SearchResponse, StatusResponse, TicketInfoType, TitleFavoriteResponse,
    TitleListResponse, TitleNode, TitlePurchaseNode, TitlePurchaseResponse, TitleTicketListNode,
    TitleTicketListResponse, UserAccount, UserInfoResponse, UserPoint, UserPointResponse,
    WebEpisodeViewerResponse, WeeklyListResponse,
};
use reqwest_cookie_store::CookieStoreMutex;
use sha2::{Digest, Sha256, Sha512};
use tokio::io::AsyncWriteExt;
use tosho_common::{
    ToshoAuthError, ToshoClientError, ToshoResult, bail_on_error, make_error, parse_json_response,
    parse_json_response_failable,
};
use tosho_macros::AutoGetter;

/// Login result for the API.
///
/// This will return either a [`KMConfig::Web`] or [`KMConfig::Mobile`] depending on the login type.
///
/// And will also include the current account info.
#[derive(Clone, AutoGetter)]
pub struct KMLoginResult {
    /// Config data
    config: KMConfig,
    /// The user account itself
    account: UserAccount,
}

/// Main client for interacting with the SQ MU!
///
/// # Example
/// ```rust,no_run
/// use tosho_kmkc::{KMClient, KMConfig, KMConfigMobile, KMConfigMobilePlatform};
///
/// #[tokio::main]
/// async fn main() {
///     let config = KMConfigMobile::new("123", "abcxyz", KMConfigMobilePlatform::Android);
///     let client = KMClient::new(KMConfig::Mobile(config)).unwrap();
///
///     let manga = client.get_titles(vec![10007]).await.unwrap();
///     println!("{:?}", manga[0]);
/// }
/// ```
#[derive(Clone)]
pub struct KMClient {
    inner: reqwest::Client,
    config: KMConfig,
    constants: &'static constants::Constants,
    cookie_store: std::sync::Arc<CookieStoreMutex>,
}

impl KMClient {
    /// Create a new [`KMClient`] with the given config.
    ///
    /// # Arguments
    /// * `config` - The config to use for the client
    pub fn new(config: KMConfig) -> ToshoResult<Self> {
        Self::make_client(config, None)
    }

    /// Attach a proxy to the client.
    ///
    /// This will clone the client and return a new client with the proxy attached.
    ///
    /// # Arguments
    /// * `proxy` - The proxy to attach to the client
    pub fn with_proxy(&self, proxy: reqwest::Proxy) -> ToshoResult<Self> {
        Self::make_client(self.config.clone(), Some(proxy))
    }

    /// Internal function to create new client.
    fn make_client(config: KMConfig, proxy: Option<reqwest::Proxy>) -> ToshoResult<Self> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            reqwest::header::HOST,
            reqwest::header::HeaderValue::from_static(API_HOST),
        );
        match config {
            KMConfig::Web(web) => {
                headers.insert(
                    reqwest::header::USER_AGENT,
                    reqwest::header::HeaderValue::from_static(WEB_CONSTANTS.ua),
                );

                let cookie_store = CookieStoreMutex::try_from(web.clone())?;
                let cookie_store = std::sync::Arc::new(cookie_store);

                // make cookie store
                let client = reqwest::Client::builder()
                    .http2_adaptive_window(true)
                    .use_rustls_tls()
                    .default_headers(headers)
                    .cookie_provider(std::sync::Arc::clone(&cookie_store));

                let client = match proxy {
                    Some(proxy) => client
                        .proxy(proxy)
                        .build()
                        .map_err(ToshoClientError::BuildError),
                    None => client.build().map_err(ToshoClientError::BuildError),
                }?;

                Ok(Self {
                    inner: client,
                    config: KMConfig::Web(web),
                    constants: get_constants(3),
                    cookie_store,
                })
            }
            KMConfig::Mobile(mobile) => {
                let consts = get_constants(mobile.platform() as u8);
                headers.insert(
                    reqwest::header::USER_AGENT,
                    reqwest::header::HeaderValue::from_static(consts.ua),
                );

                let cookie_store = CookieStoreMutex::default();
                let cookie_store = std::sync::Arc::new(cookie_store);

                let client = reqwest::Client::builder()
                    .http2_adaptive_window(true)
                    .use_rustls_tls()
                    .default_headers(headers)
                    .cookie_provider(std::sync::Arc::clone(&cookie_store));

                let client = match proxy {
                    Some(proxy) => client
                        .proxy(proxy)
                        .build()
                        .map_err(ToshoClientError::BuildError),
                    None => client.build().map_err(ToshoClientError::BuildError),
                }?;

                Ok(Self {
                    inner: client,
                    config: KMConfig::Mobile(mobile),
                    constants: consts,
                    cookie_store,
                })
            }
        }
    }

    fn apply_query_params(&self, query_params: &mut HashMap<String, String>) {
        let platform = self.constants.platform;
        let version = self.constants.version;
        query_params.insert("platform".to_string(), platform.to_string());
        query_params.insert("version".to_string(), version.to_string());
        if let KMConfig::Mobile(mobile) = &self.config {
            query_params.insert("user_id".to_string(), mobile.user_id().to_string());
        }
        if let Some(disp_ver) = self.constants.display_version {
            query_params.insert("disp_version".to_string(), disp_ver.to_string());
        }
    }

    fn format_request(&self, query_params: &mut HashMap<String, String>) -> ToshoResult<String> {
        self.apply_query_params(query_params);

        create_request_hash(&self.config, query_params.clone())
    }

    /// Get the underlying cookie store.
    pub fn get_cookie_store(&self) -> MutexGuard<'_, reqwest_cookie_store::CookieStore> {
        self.cookie_store.lock().unwrap()
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
    ) -> ToshoResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let endpoint = format!("{}{}", BASE_API, endpoint);
        let mut extend_headers = match headers {
            Some(headers) => headers,
            None => reqwest::header::HeaderMap::new(),
        };

        let hash_value = match data.clone() {
            Some(mut data) => self.format_request(&mut data)?,
            None => match params.clone() {
                Some(mut params) => self.format_request(&mut params)?,
                None => "".to_string(),
            },
        };

        let mut empty_params: HashMap<String, String> = HashMap::new();
        let mut empty_headers = reqwest::header::HeaderMap::new();
        let empty_hash = self.format_request(&mut empty_params)?;

        empty_headers.insert(
            self.constants.hash,
            empty_hash
                .parse()
                .map_err(|e| make_error!("Failed to parse empty hash header: {}", e))?,
        );
        extend_headers.insert(
            self.constants.hash,
            hash_value
                .parse()
                .map_err(|e| make_error!("Failed to parse value hash header: {}", e))?,
        );
        empty_headers.insert(
            HEADER_PLATFORM,
            self.constants
                .platform_number
                .to_string()
                .parse()
                .map_err(|e| make_error!("Failed to parse empty platform header: {}", e))?,
        );
        extend_headers.insert(
            HEADER_PLATFORM,
            self.constants
                .platform_number
                .to_string()
                .parse()
                .map_err(|e| make_error!("Failed to parse extended platform header: {}", e))?,
        );
        empty_headers.insert(
            HEADER_CRAWLER,
            "false"
                .parse()
                .map_err(|e| make_error!("Failed to parse empty crawler header: {}", e))?,
        );
        extend_headers.insert(
            HEADER_CRAWLER,
            "false"
                .parse()
                .map_err(|e| make_error!("Failed to parse extended crawler header: {}", e))?,
        );

        let request = match (data.clone(), params.clone()) {
            (None, None) => self
                .inner
                .request(method, endpoint)
                .query(&empty_params)
                .headers(empty_headers),
            (Some(mut data), None) => {
                extend_headers.insert(
                    reqwest::header::CONTENT_TYPE,
                    reqwest::header::HeaderValue::from_static("application/x-www-form-urlencoded"),
                );
                self.apply_query_params(&mut data);
                self.inner
                    .request(method, endpoint)
                    .form(&data)
                    .headers(extend_headers)
            }
            (None, Some(mut params)) => {
                self.apply_query_params(&mut params);
                self.inner
                    .request(method, endpoint)
                    .query(&params)
                    .headers(extend_headers)
            }
            (Some(_), Some(_)) => {
                bail_on_error!("Cannot have both data and params")
            }
        };

        parse_json_response_failable::<T, StatusResponse>(request.send().await?).await
    }

    /// Get the list of episodes from the given list of episode IDs
    ///
    /// # Arguments
    /// * `episodes` - The list of episode IDs to get
    pub async fn get_episodes(&self, episodes: Vec<i32>) -> ToshoResult<Vec<EpisodeNode>> {
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

        Ok(responses.episodes().to_vec())
    }

    /// Get the list of titles from the given list of title IDs
    ///
    /// # Arguments
    /// * `titles` - The list of title IDs to get
    pub async fn get_titles(&self, titles: Vec<i32>) -> ToshoResult<Vec<TitleNode>> {
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

        Ok(responses.titles().to_vec())
    }

    /// Get the episode viewer for the given episode ID.
    ///
    /// The following will return an enum depending on the config used.
    ///
    /// If you're using web config, please remember to descramble the images
    /// with the [`imaging::descramble_image`] function.
    ///
    /// # Arguments
    /// * `episode` - The episode to get the viewer for
    pub async fn get_episode_viewer(
        &self,
        episode: &EpisodeNode,
    ) -> ToshoResult<EpisodeViewerResponse> {
        match &self.config {
            KMConfig::Web(_) => {
                let mut params = HashMap::new();
                params.insert("episode_id".to_string(), episode.id().to_string());

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
                params.insert("episode_id".to_string(), episode.id().to_string());
                params.insert("force_master".to_string(), "1".to_string());
                params.insert("is_download".to_string(), "1".to_string());
                if let Some(magazine_id) = episode.magazine_id() {
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

    /// Finish the episode viewer for the given episode ID.
    ///
    /// You should be using this after you fetch the episode viewer.
    /// The following would claim the bonus point available.
    ///
    /// # Arguments
    /// * `episode` - The episode to get the viewer for
    pub async fn finish_episode_viewer(
        &self,
        episode: &EpisodeNode,
    ) -> ToshoResult<EpisodeViewerFinishResponse> {
        let mut params = HashMap::new();
        params.insert("episode_id".to_string(), episode.id().to_string());

        let response = self
            .request::<EpisodeViewerFinishResponse>(
                reqwest::Method::GET,
                "/episode/viewer/finish",
                None,
                Some(params),
                None,
            )
            .await?;

        Ok(response)
    }

    /// Get the title ticket for the given title ID.
    ///
    /// # Arguments
    /// * `title_id` - The title ID to get the ticket for
    pub async fn get_title_ticket(&self, title_id: i32) -> ToshoResult<TitleTicketListNode> {
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

        let first_ticket = response.tickets().first().ok_or_else(|| {
            make_error!("Failed to get first ticket information, first index is empty!")
        })?;

        Ok(first_ticket.clone())
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
    ) -> ToshoResult<EpisodePurchaseResponse> {
        if !wallet.can_purchase(episode.point().try_into().unwrap_or(0)) {
            let km_error = KMAPINotEnoughPointsError {
                message: "Not enough points to purchase episode".to_string(),
                points_needed: episode.point().try_into().unwrap_or(0),
                points_have: wallet.total_point(),
            };
            // bail with custom error
            return Err(km_error.into());
        }

        let mut data = HashMap::new();
        data.insert("episode_id".to_owned(), episode.id().to_string());
        data.insert("check_point".to_owned(), episode.point().to_string());

        let response = self
            .request::<EpisodePurchaseResponse>(
                reqwest::Method::POST,
                "/episode/paid",
                Some(data),
                None,
                None,
            )
            .await?;

        wallet.subtract(response.paid().try_into().unwrap_or(0));

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
    ) -> ToshoResult<BulkEpisodePurchaseResponse> {
        let mut data = HashMap::new();
        let mut episode_ids = vec![];

        let mut paid_point = 0_u64;
        let mut bonus_point = 0_u64;

        for episode in episodes {
            episode_ids.push(episode.id().to_string());

            paid_point += episode.point().try_into().unwrap_or(0);
            bonus_point += episode.bonus_point().try_into().unwrap_or(0);
        }

        let mut cloned_wallet = wallet.clone();
        cloned_wallet.add(bonus_point);
        if !cloned_wallet.can_purchase(paid_point) {
            // bail with custom error
            let km_error = KMAPINotEnoughPointsError {
                message: "Not enough points to purchase episode".to_string(),
                points_needed: paid_point,
                points_have: cloned_wallet.total_point(),
            };
            return Err(km_error.into());
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

        wallet.subtract(response.paid().try_into().unwrap_or(0));
        wallet.add(response.point_back().try_into().unwrap_or(0));

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
    ) -> ToshoResult<(StatusResponse, bool)> {
        let mut data = HashMap::new();
        data.insert("episode_id".to_owned(), episode_id.to_string());

        let mut is_title = false;
        match ticket {
            TicketInfoType::Premium(_) => {
                data.insert("ticket_version".to_owned(), "1".to_owned());
                data.insert("ticket_type".to_owned(), "99".to_owned());
            }
            TicketInfoType::Title(title) => {
                data.insert("ticket_version".to_owned(), title.version().to_string());
                data.insert("ticket_type".to_owned(), title.r#type().to_string());
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
    pub async fn get_user_point(&self) -> ToshoResult<UserPointResponse> {
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
    pub async fn search(
        &self,
        query: impl Into<String>,
        limit: Option<u32>,
    ) -> ToshoResult<Vec<TitleNode>> {
        let mut params = HashMap::new();
        params.insert("keyword".to_string(), query.into());
        let limit = limit.unwrap_or(99_999);
        params.insert("limit".to_string(), limit.to_string());

        let response = self
            .request::<SearchResponse>(
                reqwest::Method::GET,
                "/search/title",
                None,
                Some(params),
                None,
            )
            .await?;

        Ok(response.titles().to_vec())
    }

    /// Get the weekly ranking/list.
    pub async fn get_weekly(&self) -> ToshoResult<WeeklyListResponse> {
        let response = self
            .request::<WeeklyListResponse>(reqwest::Method::GET, "/title/weekly", None, None, None)
            .await?;

        Ok(response)
    }

    /// Get the current user's account information.
    pub async fn get_account(&self) -> ToshoResult<UserAccount> {
        let response = self
            .request::<AccountResponse>(reqwest::Method::GET, "/account", None, None, None)
            .await?;

        Ok(response.account().clone())
    }

    /// Get a user information
    ///
    /// This is different to [`Self::get_account`] as it needs
    /// the user ID to get the user information.
    pub async fn get_user(&self, user_id: u32) -> ToshoResult<UserInfoResponse> {
        let mut params = HashMap::new();
        params.insert("user_id".to_owned(), user_id.to_string());

        let response = self
            .request::<UserInfoResponse>(reqwest::Method::GET, "/user", None, Some(params), None)
            .await?;

        Ok(response)
    }

    /// Get the user's purchased titles.
    pub async fn get_purchased(&self) -> ToshoResult<Vec<TitlePurchaseNode>> {
        let response = self
            .request::<TitlePurchaseResponse>(
                reqwest::Method::GET,
                "/web/title/purchased",
                None,
                None,
                None,
            )
            .await?;

        Ok(response.titles().to_vec())
    }

    /// Get the user's favorites.
    pub async fn get_favorites(&self) -> ToshoResult<TitleFavoriteResponse> {
        let mut params = HashMap::new();
        params.insert("limit".to_owned(), "0".to_owned());
        params.insert("offset".to_owned(), "0".to_owned());
        // give back title list too
        params.insert("needs_title_list".to_owned(), "1".to_owned());

        let response = self
            .request::<TitleFavoriteResponse>(
                reqwest::Method::GET,
                "/favorite/list",
                None,
                Some(params),
                None,
            )
            .await?;

        Ok(response)
    }

    /// Get the magazine list.
    pub async fn get_magazines(&self) -> ToshoResult<MagazineCategoryResponse> {
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
    pub async fn get_genres(&self) -> ToshoResult<GenreSearchResponse> {
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
    /// See [``static@constants::RANKING_TABS``] for the list of available ranking IDs.
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
    ) -> ToshoResult<RankingListResponse> {
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

    /// Stream download the image from the given URL.
    ///
    /// The URL can be obtained from [`Self::get_episode_viewer`]
    ///
    /// The Web version will be automatically descrambled, so it will not be a "stream" download.
    ///
    /// # Arguments
    /// * `url` - The URL to download the image from
    /// * `scramble_seed` - The scramble seed to use to descramble the image (only for Web, please provide it!)
    /// * `writer` - The writer to write the image to
    pub async fn stream_download(
        &self,
        url: impl AsRef<str>,
        scramble_seed: Option<u32>,
        mut writer: impl tokio::io::AsyncWrite + std::marker::Unpin,
    ) -> ToshoResult<()> {
        let res = self
            .inner
            .get(url.as_ref())
            .headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    reqwest::header::USER_AGENT,
                    reqwest::header::HeaderValue::from_static(self.constants.image_ua),
                );
                headers.insert(
                    reqwest::header::HOST,
                    reqwest::header::HeaderValue::from_static(IMAGE_HOST),
                );
                headers
            })
            .send()
            .await?;

        match scramble_seed {
            Some(scramble_seed) => {
                let image_bytes = res.bytes().await?;
                let descrambled = tokio::task::spawn_blocking(move || {
                    imaging::descramble_image(&image_bytes, 4, scramble_seed)
                })
                .await
                .map_err(|e| make_error!("Failed to execute blocking descrambling task: {}", e))?;

                match descrambled {
                    Ok(descram_bytes) => {
                        writer.write_all(&descram_bytes).await?;
                        writer.flush().await?;
                        Ok(())
                    }
                    Err(e) => Err(e),
                }
            }
            None => {
                let mut stream = res.bytes_stream();
                while let Some(bytes) = stream.try_next().await? {
                    writer.write_all(&bytes).await?;
                    writer.flush().await?;
                }
                Ok(())
            }
        }
    }

    /// Login to the API with the given username and password.
    ///
    /// You can use this to get either Web version of the token or
    /// the Mobile version of the token.
    ///
    /// # Arguments
    /// * `email` - The email to login with
    /// * `password` - The password to login with
    /// * `mobile` - Whether to login as mobile or not
    pub async fn login(
        email: impl Into<String>,
        password: impl Into<String>,
        mobile_platform: Option<KMConfigMobilePlatform>,
    ) -> ToshoResult<KMLoginResult> {
        // Create a new client
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            reqwest::header::HOST,
            reqwest::header::HeaderValue::from_static(API_HOST),
        );
        headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static(WEB_CONSTANTS.ua),
        );

        let default_web = KMConfigWeb::default();
        let cookie_store =
            reqwest_cookie_store::CookieStoreMutex::new(default_web.clone().try_into()?);
        let cookie_store = std::sync::Arc::new(cookie_store);

        let client = reqwest::Client::builder()
            .http2_adaptive_window(true)
            .use_rustls_tls()
            .default_headers(headers)
            .cookie_provider(std::sync::Arc::clone(&cookie_store))
            .build()
            .map_err(ToshoClientError::BuildError)?;

        // Perform web login
        let mut req_data = HashMap::new();
        req_data.insert("email".to_string(), email.into());
        req_data.insert("password".to_string(), password.into());
        req_data.insert("platform".to_string(), WEB_CONSTANTS.platform.to_string());
        req_data.insert("version".to_string(), WEB_CONSTANTS.version.to_string());

        // hash
        let req_hash = create_request_hash(&KMConfig::Web(default_web.clone()), req_data.clone())?;

        let mut extend_headers = reqwest::header::HeaderMap::new();
        extend_headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/x-www-form-urlencoded"),
        );
        extend_headers.insert(
            WEB_CONSTANTS.hash,
            req_hash
                .parse()
                .map_err(|e| make_error!("Failed to parse hash header: {}", e))?,
        );
        let response = client
            .post(format!("{}/web/user/login", BASE_API))
            .form(&req_data)
            .headers(extend_headers)
            .send()
            .await?;

        let login_status = parse_json_response::<StatusResponse>(response).await?;

        if login_status.response_code() != 0 {
            return Err(ToshoAuthError::InvalidCredentials(
                login_status.error_message().to_string(),
            )
            .into());
        }

        let unparse_web = KMConfigWeb::try_from(cookie_store.lock().unwrap().clone())?;
        // Get account info
        let km_client = KMClient::new(KMConfig::Web(unparse_web.clone()))?;
        let account = km_client.get_account().await?;

        match mobile_platform {
            Some(platform) => {
                // Authenticate as mobile
                let user_info = km_client.get_user(account.user_id()).await?;

                Ok(KMLoginResult {
                    config: KMConfig::Mobile(KMConfigMobile::new(
                        user_info.id().to_string(),
                        user_info.hash_key(),
                        platform,
                    )),
                    account,
                })
            }
            None => Ok(KMLoginResult {
                config: KMConfig::Web(unparse_web),
                account,
            }),
        }
    }
}

/// Create the request hash for any given query params
///
/// # Arguments
/// * `query_params` - The query params to hash
fn create_request_hash(
    config: &KMConfig,
    query_params: HashMap<String, String>,
) -> ToshoResult<String> {
    match config {
        KMConfig::Web(web) => {
            let birthday = web.birthday().value();
            let expires = web.birthday().expires().to_string();

            let mut keys = query_params.keys().collect::<Vec<&String>>();
            keys.sort();

            let mut qi_s: Vec<String> = vec![];
            for key in keys {
                let value = query_params.get(key).ok_or_else(|| {
                    make_error!("Key {} not found when hashing query params", key)
                })?;
                let hashed = hash_kv(key, value);
                qi_s.push(hashed);
            }

            let qi_s_hashed = <Sha256 as Digest>::digest(qi_s.join(",").as_bytes());
            let birth_expire_hash = hash_kv(birthday, &expires);

            let merged_hash = <Sha512 as Digest>::digest(
                format!("{qi_s_hashed:x}{birth_expire_hash}").as_bytes(),
            );

            Ok(format!("{merged_hash:x}"))
        }
        KMConfig::Mobile(mobile) => {
            let mut hasher = <Sha256 as Digest>::new();

            let mut query_params = query_params.clone();
            query_params.insert("hash_key".to_string(), mobile.hash_key().to_string());

            // iterate sorted keys
            let mut keys = query_params.keys().collect::<Vec<&String>>();
            keys.sort();

            for key in keys {
                let value = query_params.get(key).ok_or_else(|| {
                    make_error!("Key {} not found when hashing query params", key)
                })?;
                let hashed_value = <Md5 as Digest>::digest(value.as_bytes());
                let hash_digest = format!("{hashed_value:x}");

                hasher.update(hash_digest);
            }

            let hashed = hasher.finalize();
            Ok(format!("{hashed:x}"))
        }
    }
}

fn hash_kv(key: &str, value: &str) -> String {
    // convert to bytes (utf-8)
    let key = key.as_bytes();
    let value = value.as_bytes();

    // create hasher
    let hasher256 = <Sha256 as Digest>::digest(key);
    let hasher512 = <Sha512 as Digest>::digest(value);

    format!("{hasher256:x}_{hasher512:x}")
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
