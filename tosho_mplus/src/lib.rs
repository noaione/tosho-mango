#![warn(missing_docs, clippy::empty_docs, rustdoc::broken_intra_doc_links)]
#![doc = include_str!("../README.md")]

pub mod constants;
pub mod helper;
pub mod proto;

use futures_util::TryStreamExt;
use tokio::io::{self, AsyncWriteExt};
use tosho_common::{
    bail_on_error, parse_protobuf_response, ToshoClientError, ToshoError, ToshoParseError,
    ToshoResult,
};

use constants::{Constants, API_HOST, IMAGE_HOST};
use helper::RankingType;
use proto::{CommentList, ErrorResponse, Language, SuccessOrError};

use crate::constants::BASE_API;
pub use crate::helper::ImageQuality;

/// Main client for interacting with the M+ API.
///
/// # Example
/// ```rust,no_run
/// use tosho_mplus::MPClient;
/// use tosho_mplus::proto::Language;
/// use tosho_mplus::constants::get_constants;
///
/// #[tokio::main]
/// async fn main() {
///     let client = MPClient::new("1234", Language::English, get_constants(1)).unwrap();
///     let home_view = client.get_home_page().await.unwrap();
/// }
/// ```
///
/// # Boxed
///
/// All responses are [`Box`]-ed since it has widely varying sizes.
#[derive(Clone, Debug)]
pub struct MPClient {
    inner: reqwest::Client,
    secret: String,
    language: Language,
    constants: &'static Constants,
    app_ver: Option<u32>,
}

impl MPClient {
    /// Create a new client instance.
    ///
    /// # Parameters
    /// * `secret` - The secret key to use for the client.
    /// * `language` - The language to use for the client.
    /// * `constants` - The constants to use for the client.
    pub fn new(
        secret: impl Into<String>,
        language: Language,
        constants: &'static Constants,
    ) -> ToshoResult<Self> {
        Self::make_client(secret, language, constants, None)
    }

    /// Attach a proxy to the client.
    ///
    /// This will clone the client and return a new client with the proxy attached.
    ///
    /// # Arguments
    /// * `proxy` - The proxy to attach to the client
    pub fn with_proxy(&self, proxy: reqwest::Proxy) -> ToshoResult<Self> {
        Self::make_client(&self.secret, self.language, self.constants, Some(proxy))
    }

    /// Override the app version for the client.
    ///
    /// This will clone the client and return a new client with the app version overridden.
    ///
    /// # Arguments
    /// * `app_ver` - The app version to use for the client.
    pub fn with_app_version(&self, app_ver: Option<u32>) -> Self {
        let mut new_client = self.clone();
        new_client.app_ver = app_ver;
        new_client
    }

    fn make_client(
        secret: impl Into<String>,
        language: Language,
        constants: &'static Constants,
        proxy: Option<reqwest::Proxy>,
    ) -> ToshoResult<Self> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Host", reqwest::header::HeaderValue::from_static(&API_HOST));
        headers.insert(
            "User-Agent",
            reqwest::header::HeaderValue::from_static(&constants.api_ua),
        );

        let client = reqwest::Client::builder()
            .http2_adaptive_window(true)
            .http1_only()
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
            language,
            constants,
            app_ver: None,
        })
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
        params.push((
            "app_ver".to_string(),
            if let Some(app_ver) = self.app_ver {
                app_ver.to_string()
            } else {
                self.constants.app_ver.to_string()
            },
        ));
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
    pub async fn get_initial(&self) -> ToshoResult<APIResponse<proto::InitialViewV2>> {
        let request = self
            .inner
            .get(self.build_url("init_v2"))
            .query(&self.empty_params(false))
            .send()
            .await?;

        let response = parse_response(request).await?;

        match response {
            SuccessOrError::Success(data) => match data.initial_view_v2 {
                Some(inner_data) => Ok(APIResponse::Success(Box::new(inner_data.clone()))),
                None => Err(ToshoParseError::expect("initial view")),
            },
            SuccessOrError::Error(error) => Ok(APIResponse::Error(error)),
        }
    }

    /// Get the main home view of the app.
    pub async fn get_home_page(&self) -> ToshoResult<APIResponse<proto::HomeViewV3>> {
        let mut query_params = self.empty_params(true);
        query_params.insert(0, ("viewer_mode".to_string(), "horizontal".to_string()));

        let request = self
            .inner
            .get(self.build_url("home_v4"))
            .query(&query_params)
            .send()
            .await?;

        let response = parse_response(request).await?;

        match response {
            SuccessOrError::Success(data) => match data.home_view_v3 {
                Some(inner_data) => Ok(APIResponse::Success(Box::new(inner_data))),
                None => Err(ToshoParseError::expect("home view v3")),
            },
            SuccessOrError::Error(error) => Ok(APIResponse::Error(error)),
        }
    }

    /// Get the user profile
    pub async fn get_user_profile(&self) -> ToshoResult<APIResponse<proto::UserProfileSettings>> {
        let query = self.empty_params(false);
        let request = self
            .inner
            .get(self.build_url("profile"))
            .query(&query)
            .send()
            .await?;

        let response = parse_response(request).await?;

        match response {
            SuccessOrError::Success(data) => match data.user_profile_settings {
                Some(inner_data) => Ok(APIResponse::Success(Box::new(inner_data))),
                None => Err(ToshoParseError::expect("user profile settings")),
            },
            SuccessOrError::Error(error) => Ok(APIResponse::Error(error)),
        }
    }

    /// Get the user settings.
    pub async fn get_user_settings(&self) -> ToshoResult<APIResponse<proto::UserSettingsV2>> {
        let mut query_params = self.empty_params(true);
        query_params.insert(0, ("viewer_mode".to_string(), "horizontal".to_string()));

        let request = self
            .inner
            .get(self.build_url("settings_v2"))
            .query(&query_params)
            .send()
            .await?;

        let response = parse_response(request).await?;

        match response {
            SuccessOrError::Success(data) => match data.user_settings_v2 {
                Some(inner_data) => Ok(APIResponse::Success(Box::new(inner_data))),
                None => Err(ToshoParseError::expect("user settings v2")),
            },
            SuccessOrError::Error(error) => Ok(APIResponse::Error(error)),
        }
    }

    /// Get the subscriptions list and details.
    pub async fn get_subscriptions(&self) -> ToshoResult<APIResponse<proto::SubscriptionResponse>> {
        let request = self
            .inner
            .get(self.build_url("subscription"))
            .query(&self.empty_params(false))
            .send()
            .await?;

        let response = parse_response(request).await?;

        match response {
            SuccessOrError::Success(data) => match data.subscriptions {
                Some(inner_data) => Ok(APIResponse::Success(Box::new(inner_data))),
                None => Err(ToshoParseError::expect("subscriptions")),
            },
            SuccessOrError::Error(error) => Ok(APIResponse::Error(error)),
        }
    }

    /// Get all the available titles.
    pub async fn get_all_titles(&self) -> ToshoResult<APIResponse<proto::TitleListOnlyV2>> {
        let request = self
            .inner
            .get(self.build_url("title_list/all_v2"))
            .query(&self.empty_params(false))
            .send()
            .await?;

        let response = parse_response(request).await?;

        match response {
            SuccessOrError::Success(data) => match data.all_titles_v2 {
                Some(inner_data) => Ok(APIResponse::Success(Box::new(inner_data))),
                None => Err(ToshoParseError::expect("all titles v2")),
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
    ) -> ToshoResult<APIResponse<proto::TitleRankingList>> {
        let kind = kind.unwrap_or(RankingType::Hottest);
        let mut query_params = self.empty_params(true);
        query_params.insert(0, ("type".to_string(), kind.to_string()));

        let request = self
            .inner
            .get(self.build_url("title_list/rankingV2"))
            .query(&query_params)
            .send()
            .await?;

        let response = parse_response(request).await?;

        match response {
            SuccessOrError::Success(data) => match data.title_ranking_v2 {
                Some(inner_data) => Ok(APIResponse::Success(Box::new(inner_data))),
                None => Err(ToshoParseError::expect("title ranking v2")),
            },
            SuccessOrError::Error(error) => Ok(APIResponse::Error(error)),
        }
    }

    /// Get all free titles
    pub async fn get_free_titles(&self) -> ToshoResult<APIResponse<proto::FreeTitles>> {
        let request = self
            .inner
            .get(self.build_url("title_list/free_titles"))
            .query(&self.empty_params(false))
            .send()
            .await?;

        let response = parse_response(request).await?;

        match response {
            SuccessOrError::Success(data) => match data.free_titles {
                Some(inner_data) => Ok(APIResponse::Success(Box::new(inner_data))),
                None => Err(ToshoParseError::expect("free titles")),
            },
            SuccessOrError::Error(error) => Ok(APIResponse::Error(error)),
        }
    }

    /// Get the bookmarked titles
    pub async fn get_bookmarked_titles(&self) -> ToshoResult<APIResponse<proto::TitleListOnly>> {
        let request = self
            .inner
            .get(self.build_url("title_list/bookmark"))
            .query(&self.empty_params(false))
            .send()
            .await?;

        let response = parse_response(request).await?;

        match response {
            SuccessOrError::Success(data) => match data.subscribed_titles {
                Some(inner_data) => Ok(APIResponse::Success(Box::new(inner_data))),
                None => Err(ToshoParseError::expect("subscribed/bookmarked titles")),
            },
            SuccessOrError::Error(error) => Ok(APIResponse::Error(error)),
        }
    }

    /// Get list of titles for specific language
    ///
    /// Internally, this use the "search" API which does not take any
    /// query information for some unknown reason.
    pub async fn get_search(&self) -> ToshoResult<APIResponse<proto::SearchResults>> {
        let request = self
            .inner
            .get(self.build_url("title_list/search"))
            .query(&self.empty_params(true))
            .send()
            .await?;

        let response = parse_response(request).await?;

        match response {
            SuccessOrError::Success(data) => match data.search_results {
                Some(inner_data) => Ok(APIResponse::Success(Box::new(inner_data))),
                None => Err(ToshoParseError::expect("search results")),
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
    ) -> ToshoResult<APIResponse<proto::TitleDetail>> {
        let mut query_params = self.empty_params(true);
        query_params.insert(0, ("title_id".to_string(), title_id.to_string()));

        let request = self
            .inner
            .get(self.build_url("title_detailV3"))
            .query(&query_params)
            .send()
            .await?;

        let response = parse_response(request).await?;

        match response {
            SuccessOrError::Success(data) => match data.title_detail {
                Some(inner_data) => {
                    let mut cloned_data = inner_data.clone();
                    cloned_data.chapter_groups.iter_mut().for_each(|group| {
                        group
                            .first_chapters
                            .iter_mut()
                            .for_each(|ch| ch.set_position(proto::ChapterPosition::First));

                        group
                            .last_chapters
                            .iter_mut()
                            .for_each(|ch| ch.set_position(proto::ChapterPosition::Last));

                        group
                            .mid_chapters
                            .iter_mut()
                            .for_each(|ch| ch.set_position(proto::ChapterPosition::Middle));
                    });

                    Ok(APIResponse::Success(Box::new(cloned_data)))
                }
                None => Err(ToshoParseError::expect("title_detail")),
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
    ) -> ToshoResult<APIResponse<proto::ChapterViewer>> {
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
                bail_on_error!(
                    "Chapter is not free and user does not have minimum subscription: {:?} < {:?}",
                    user_sub.plan(),
                    title_labels.plan_type()
                );
            }
        }
        self.build_params(&mut query_params, false);

        let request = self
            .inner
            .get(self.build_url("manga_viewer"))
            .query(&query_params)
            .send()
            .await?;

        let response = parse_response(request).await?;

        match response {
            SuccessOrError::Success(data) => match data.chapter_viewer {
                Some(inner_data) => Ok(APIResponse::Success(Box::new(inner_data))),
                None => Err(ToshoParseError::expect("chapter viewer")),
            },
            SuccessOrError::Error(error) => Ok(APIResponse::Error(error)),
        }
    }

    /// Get comments for a chapter
    ///
    /// # Parameters
    /// * `id` - The ID of the chapter to get comments for.
    pub async fn get_comments(&self, id: u64) -> ToshoResult<APIResponse<CommentList>> {
        let mut query_params = self.empty_params(false);
        query_params.insert(0, ("chapter_id".to_string(), id.to_string()));

        let request = self
            .inner
            .get(self.build_url("comments"))
            .query(&query_params)
            .send()
            .await?;

        let response = parse_response(request).await?;

        match response {
            SuccessOrError::Success(data) => match data.comment_list {
                Some(inner_data) => Ok(APIResponse::Success(Box::new(inner_data))),
                None => Err(ToshoParseError::expect("comment list")),
            },
            SuccessOrError::Error(error) => Ok(APIResponse::Error(error)),
        }
    }

    /// Stream download the image from the given URL.
    ///
    /// The URL can be obtained from [`get_chapter_images`](#method.get_chapter_images).
    ///
    /// # Parameters
    /// * `url` - The URL to download the image from.
    /// * `writer` - The writer to write the image to.
    pub async fn stream_download(
        &self,
        url: impl Into<String>,
        mut writer: impl io::AsyncWrite + Unpin,
    ) -> ToshoResult<()> {
        let url: String = url.into();
        let res = self
            .inner
            .get(url)
            .headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    "Host",
                    reqwest::header::HeaderValue::from_static(&IMAGE_HOST),
                );
                headers.insert(
                    "User-Agent",
                    reqwest::header::HeaderValue::from_static(&self.constants.image_ua),
                );
                headers.insert(
                    "Cache-Control",
                    reqwest::header::HeaderValue::from_static("no-cache"),
                );
                headers
            })
            .send()
            .await?;

        // bail if not success
        if !res.status().is_success() {
            Err(ToshoError::from(res.status()))
        } else {
            let mut stream = res.bytes_stream();
            while let Some(item) = stream.try_next().await? {
                writer.write_all(&item).await?;
                writer.flush().await?;
            }

            Ok(())
        }
    }
}

/// A common return type for all API calls.
///
/// It either returns the specified success response or an error.
pub enum APIResponse<T: ::prost::Message + Clone> {
    /// A [`Box`]-ed [`ErrorResponse`]
    Error(Box<ErrorResponse>),
    /// Successfull response, also [`Box`]-ed and depends on the API call
    Success(Box<T>),
}

// impl unwrap for APIResponse
impl<T: ::prost::Message + Clone> APIResponse<T> {
    /// Unwrap the response.
    ///
    /// # Panics
    /// Panics if the response is an error.
    pub fn unwrap(self) -> T {
        match self {
            APIResponse::Success(data) => *data,
            APIResponse::Error(error) => panic!("Error response: {:?}", *error),
        }
    }
}

/// A quick wrapper for [`parse_protobuf_response`]
async fn parse_response(res: reqwest::Response) -> ToshoResult<SuccessOrError> {
    let decoded_response = parse_protobuf_response::<crate::proto::Response>(res).await?;

    // oneof response on .response
    match decoded_response.response {
        Some(response) => Ok(response),
        None => Err(tosho_common::ToshoParseError::empty()),
    }
}
