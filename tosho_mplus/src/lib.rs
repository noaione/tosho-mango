#![allow(dead_code)]

pub mod constants;
pub mod helper;
pub mod proto;

use std::io::Cursor;

use constants::{Constants, API_HOST};
use helper::RankingType;
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
///     let home_view = client.get_home_page().await.unwrap();
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
            .http1_only()
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
    fn build_params(&self, params: &mut Vec<(String, String)>, with_lang: bool) {
        if with_lang {
            params.push((
                "lang".to_string(),
                self.language.as_language_code().to_owned(),
            ));
            params.push((
                "clang".to_string(),
                self.language.as_language_code().to_owned(),
            ));
        }
        params.push(("os".to_string(), self.constants.os_name.to_string()));
        params.push(("os_ver".to_string(), self.constants.os_ver.to_string()));
        params.push(("app_ver".to_string(), self.constants.app_ver.to_string()));
        params.push(("secret".to_string(), self.secret.clone()));
    }

    fn build_url(&self, path: &str) -> String {
        if path.starts_with('/') {
            return format!("{}{}", *BASE_API, path);
        }

        format!("{}/{}", *BASE_API, path)
    }

    fn empty_params(&self, with_lang: bool) -> Vec<(String, String)> {
        let mut params: Vec<(String, String)> = vec![];

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
        query_params.insert(0, ("viewer_mode".to_string(), "horizontal".to_string()));

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

    /// Get the user profile
    pub async fn get_user_profile(
        &self,
    ) -> anyhow::Result<APIResponse<proto::UserProfileSettings>> {
        let query = self.empty_params(false);
        let request = self
            .inner
            .get(&self.build_url("profile"))
            .query(&query)
            .send()
            .await?;

        let response = parse_response(request).await?;

        match response {
            SuccessOrError::Success(data) => match data.user_profile_settings {
                Some(inner_data) => Ok(APIResponse::Success(Box::new(inner_data))),
                None => anyhow::bail!("No user profile found"),
            },
            SuccessOrError::Error(error) => Ok(APIResponse::Error(error)),
        }
    }

    /// Get the user settings.
    pub async fn get_user_settings(&self) -> anyhow::Result<APIResponse<proto::UserSettingsV2>> {
        let mut query_params = self.empty_params(true);
        query_params.insert(0, ("viewer_mode".to_string(), "horizontal".to_string()));

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

    /// Get the subscriptions list and details.
    pub async fn get_subscriptions(
        &self,
    ) -> anyhow::Result<APIResponse<proto::SubscriptionResponse>> {
        let request = self
            .inner
            .get(&self.build_url("subscription"))
            .query(&self.empty_params(false))
            .send()
            .await?;

        let response = parse_response(request).await?;

        match response {
            SuccessOrError::Success(data) => match data.subscriptions {
                Some(inner_data) => Ok(APIResponse::Success(Box::new(inner_data))),
                None => anyhow::bail!("No subscription response found"),
            },
            SuccessOrError::Error(error) => Ok(APIResponse::Error(error)),
        }
    }

    /// Get all the available titles.
    pub async fn get_all_titles(&self) -> anyhow::Result<APIResponse<proto::TitleListOnlyV2>> {
        let request = self
            .inner
            .get(&self.build_url("title_list/all_v2"))
            .query(&self.empty_params(false))
            .send()
            .await?;

        let response = parse_response(request).await?;

        match response {
            SuccessOrError::Success(data) => match data.all_titles_v2 {
                Some(inner_data) => Ok(APIResponse::Success(Box::new(inner_data))),
                None => anyhow::bail!("No title list found"),
            },
            SuccessOrError::Error(error) => Ok(APIResponse::Error(error)),
        }
    }

    /// Get title ranking list.
    ///
    /// # Arguments
    /// * `kind` - The type of ranking to get.
    pub async fn get_title_ranking(
        &self,
        kind: Option<RankingType>,
    ) -> anyhow::Result<APIResponse<proto::TitleRankingList>> {
        let kind = kind.unwrap_or(RankingType::Hottest);
        let mut query_params = self.empty_params(true);
        query_params.insert(0, ("type".to_string(), kind.to_string()));

        let request = self
            .inner
            .get(&self.build_url("title_list/rankingV2"))
            .query(&query_params)
            .send()
            .await?;

        let response = parse_response(request).await?;

        match response {
            SuccessOrError::Success(data) => match data.title_ranking_v2 {
                Some(inner_data) => Ok(APIResponse::Success(Box::new(inner_data))),
                None => anyhow::bail!("No title ranking found"),
            },
            SuccessOrError::Error(error) => Ok(APIResponse::Error(error)),
        }
    }

    /// Get all free titles
    pub async fn get_free_titles(&self) -> anyhow::Result<APIResponse<proto::FreeTitles>> {
        let request = self
            .inner
            .get(&self.build_url("title_list/free_titles"))
            .query(&self.empty_params(false))
            .send()
            .await?;

        let response = parse_response(request).await?;

        match response {
            SuccessOrError::Success(data) => match data.free_titles {
                Some(inner_data) => Ok(APIResponse::Success(Box::new(inner_data))),
                None => anyhow::bail!("No free titles found"),
            },
            SuccessOrError::Error(error) => Ok(APIResponse::Error(error)),
        }
    }

    /// Get the bookmarked titles
    pub async fn get_bookmarked_titles(&self) -> anyhow::Result<APIResponse<proto::TitleListOnly>> {
        let request = self
            .inner
            .get(&self.build_url("title_list/bookmark"))
            .query(&self.empty_params(false))
            .send()
            .await?;

        let response = parse_response(request).await?;

        match response {
            SuccessOrError::Success(data) => match data.subscribed_titles {
                Some(inner_data) => Ok(APIResponse::Success(Box::new(inner_data))),
                None => anyhow::bail!("No bookmarked titles found"),
            },
            SuccessOrError::Error(error) => Ok(APIResponse::Error(error)),
        }
    }

    /// Get list of titles for specific language
    ///
    /// Internally, this use the "search" API which does not take any
    /// query information for some unknown reason.
    pub async fn get_titles_for_language(
        &self,
        language: Language,
    ) -> anyhow::Result<APIResponse<proto::SearchResults>> {
        let mut query_params = vec![];
        query_params.push(("lang".to_string(), language.as_language_code().to_owned()));
        query_params.push(("clang".to_string(), language.as_language_code().to_owned()));
        self.build_params(&mut query_params, false);

        let request = self
            .inner
            .get(&self.build_url("title_list/search"))
            .query(&query_params)
            .send()
            .await?;

        let response = parse_response(request).await?;

        match response {
            SuccessOrError::Success(data) => match data.search_results {
                Some(inner_data) => Ok(APIResponse::Success(Box::new(inner_data))),
                None => anyhow::bail!("No titles found"),
            },
            SuccessOrError::Error(error) => Ok(APIResponse::Error(error)),
        }
    }

    /// Get detailed information about a title.
    ///
    /// # Arguments
    /// * `title_id` - The ID of the title to get information about.
    pub async fn get_title_details(
        &self,
        title_id: u64,
    ) -> anyhow::Result<APIResponse<proto::TitleDetail>> {
        let mut query_params = self.empty_params(true);
        query_params.insert(0, ("title_id".to_string(), title_id.to_string()));

        let request = self
            .inner
            .get(&self.build_url("title_detailV3"))
            .query(&query_params)
            .send()
            .await?;

        let response = parse_response(request).await?;

        match response {
            SuccessOrError::Success(data) => match data.title_detail {
                Some(inner_data) => Ok(APIResponse::Success(Box::new(inner_data))),
                None => anyhow::bail!("No title details found"),
            },
            SuccessOrError::Error(error) => Ok(APIResponse::Error(error)),
        }
    }

    /// Get chapter viewer information.
    ///
    /// # Arguments
    /// * `chapter` - The chapter to get information about.
    /// * `title` - The title of the chapter.
    /// * `quality` - The quality of the image to get.
    /// * `split` - Whether to split the image spread or not.
    pub async fn get_chapter_viewer(
        &self,
        chapter: &proto::Chapter,
        title: &proto::TitleDetail,
        quality: ImageQuality,
        split: bool,
    ) -> anyhow::Result<APIResponse<proto::ChapterViewer>> {
        let mut query_params = vec![];
        query_params.push(("chapter_id".to_string(), chapter.chapter_id.to_string()));
        query_params.push((
            "split".to_string(),
            if split { "yes" } else { "no" }.to_string(),
        ));
        query_params.push(("img_quality".to_string(), quality.to_string()));
        query_params.push(("viewer_mode".to_string(), chapter.default_view_mode()));
        // Determine the way to read the chapter
        if chapter.is_free() {
            query_params.push(("free_reading".to_string(), "yes".to_string()));
            query_params.push(("subscription_reading".to_string(), "no".to_string()));
            query_params.push(("ticket_reading".to_string(), "no".to_string()));
        } else if chapter.is_ticketed() {
            query_params.push(("ticket_reading".to_string(), "yes".to_string()));
            query_params.push(("free_reading".to_string(), "no".to_string()));
            query_params.push(("subscription_reading".to_string(), "no".to_string()));
        } else {
            let user_sub = title.user_subscription.clone().unwrap_or_default();
            let title_labels = title.title_labels.clone().unwrap_or_default();
            if user_sub.plan() >= title_labels.plan_type() {
                query_params.push(("subscription_reading".to_string(), "yes".to_string()));
                query_params.push(("ticket_reading".to_string(), "no".to_string()));
                query_params.push(("free_reading".to_string(), "no".to_string()));
            } else {
                anyhow::bail!(
                    "Chapter is not free and user does not have minimum subscription: {:?} < {:?}",
                    user_sub.plan(),
                    title_labels.plan_type()
                );
            }
        }
        self.build_params(&mut query_params, false);

        let request = self
            .inner
            .get(&self.build_url("manga_viewer"))
            .query(&query_params)
            .send()
            .await?;

        let response = parse_response(request).await?;

        match response {
            SuccessOrError::Success(data) => match data.chapter_viewer {
                Some(inner_data) => Ok(APIResponse::Success(Box::new(inner_data))),
                None => anyhow::bail!("No chapter viewer found"),
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
    if !res.status().is_success() {
        anyhow::bail!("Error response: {:?}", res.status());
    }

    let bytes_data = res.bytes().await?;
    let cursor = bytes_data.as_ref();

    let decoded_response = crate::proto::Response::decode(&mut Cursor::new(cursor));

    if let Err(e) = decoded_response {
        anyhow::bail!("Error decoding response: {:?}", e);
    }

    let decoded_response = decoded_response.unwrap();

    // oneof response on .response
    match decoded_response.response {
        Some(response) => Ok(response),
        None => anyhow::bail!("No response found"),
    }
}
