#![warn(missing_docs, clippy::empty_docs, rustdoc::broken_intra_doc_links)]
#![doc = include_str!("../README.md")]

use crate::constants::SECURE_IMAGE_HOST;
use std::collections::HashMap;

use futures_util::TryStreamExt;
use tokio::io::{self, AsyncWriteExt};
use tosho_common::{
    ToshoAuthError, ToshoClientError, ToshoResult, bail_on_error, parse_json_response_failable,
};

use crate::{constants::BASE_API, models::ErrorResponse};

pub mod constants;
pub mod filters;
pub mod models;

pub use filters::*;

/// Main client for interacting with the NI API.
///
/// # Examples
/// ```rust,no_run
/// use tosho_nids::{Filter, NIClient};
///
/// #[tokio::main]
/// async fn main() {
///     let constants = tosho_nids::constants::get_constants(1); // Web
///     let client = NIClient::new(None, constants).unwrap();
///
///     let filter = Filter::default()
///        .add_filter(tosho_nids::FilterType::Title, "Attack on Titan")
///        .with_per_page(18);
///     let issues = client.get_issues(&filter).await.unwrap();
///     println!("Issues: {:?}", issues);
/// }
/// ```
#[derive(Clone)]
pub struct NIClient {
    inner: reqwest::Client,
    constants: &'static crate::constants::Constants,
    token: Option<String>,
}

impl std::fmt::Debug for NIClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NIClient")
            .field("inner", &"reqwest::Client")
            .field("constants", &self.constants)
            .field("token", &self.token.as_deref().map(|_| "****"))
            .finish()
    }
}

impl NIClient {
    /// Create a new client instance.
    ///
    /// # Parameters
    /// * `token` - JWT token for download requests, if `None` you will only be able to make non-authenticated requests.
    /// * `constants` - Constants to use for the client, see [`crate::constants::get_constants`].
    pub fn new(
        token: Option<&str>,
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
        token: Option<impl Into<String>>,
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
            token: token.map(Into::into),
        })
    }

    /// Create an authenticated headers map.
    ///
    /// Has `prefix_bearer` to prefix the token with `Bearer ` since most endpoints does not require it.
    fn auth_headers(&self, prefix_bearer: bool) -> ToshoResult<reqwest::header::HeaderMap> {
        let token = self
            .token
            .as_deref()
            .ok_or(ToshoAuthError::UnknownSession)?;

        let header_token = if prefix_bearer {
            format!("Bearer {}", token)
        } else {
            token.to_string()
        };
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&header_token).map_err(|_| {
                ToshoClientError::HeaderParseError(
                    "Invalid bearer token provided to client".to_string(),
                )
            })?,
        );

        Ok(headers)
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
    /// * `authenticated` - Whether to make an authenticated request or not
    async fn request<T>(
        &self,
        method: reqwest::Method,
        endpoint: &str,
        data: Option<serde_json::Value>,
        params: Option<HashMap<String, String>>,
        headers: Option<reqwest::header::HeaderMap>,
    ) -> ToshoResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let endpoint = format!("{}/api/v1{}", BASE_API, endpoint);
        let mut extend_headers = reqwest::header::HeaderMap::new();
        // Check ir provided a custom headers
        if let Some(hdrs) = headers {
            for (key, value) in hdrs.iter() {
                extend_headers.insert(key, value.clone());
            }
        }

        let request = match (data.clone(), params.clone()) {
            (None, None) => self.inner.request(method, endpoint).headers(extend_headers),
            (Some(data), None) => {
                extend_headers.insert(
                    reqwest::header::CONTENT_TYPE,
                    reqwest::header::HeaderValue::from_static("application/json"),
                );
                self.inner
                    .request(method, endpoint)
                    .json(&data)
                    .headers(extend_headers)
            }
            (None, Some(params)) => self
                .inner
                .request(method, endpoint)
                .headers(extend_headers)
                .query(&params),
            (Some(_), Some(_)) => {
                bail_on_error!("Cannot have both data and params")
            }
        };

        parse_json_response_failable::<T, ErrorResponse>(request.send().await?).await
    }

    /// Get the list of issues
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the request
    pub async fn get_issues(
        &self,
        filters: &filters::Filter,
    ) -> ToshoResult<models::IssueListResponse> {
        let params = filters.to_params();
        self.request(reqwest::Method::GET, "/issues", None, Some(params), None)
            .await
    }

    /// Get single issue detail
    ///
    /// # Arguments
    /// * `issue_id` - The issue UUID to get the detail for
    pub async fn get_issue(&self, issue_id: u32) -> ToshoResult<models::IssueDetail> {
        let resp = self
            .request::<models::IssueDetailResponse>(
                reqwest::Method::GET,
                &format!("/issues/{}", issue_id),
                None,
                None,
                None,
            )
            .await?;

        Ok(resp.data())
    }

    /// Get the list of series runs
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the request
    pub async fn get_series_runs(
        &self,
        filters: &filters::Filter,
    ) -> ToshoResult<models::series::SeriesRunList> {
        let params = filters.to_params();
        self.request(
            reqwest::Method::GET,
            "/series_run",
            None,
            Some(params),
            None,
        )
        .await
    }

    /// Get single series run detail via ID
    ///
    /// # Arguments
    /// * `series_run_id` - The series run ID to get the detail for
    pub async fn get_series_run(
        &self,
        series_run_id: u32,
    ) -> ToshoResult<models::series::SeriesRunWithEditions> {
        let resp = self
            .request::<models::series::SeriesRunWithEditionsResponse>(
                reqwest::Method::GET,
                &format!("/series_run/{}", series_run_id),
                None,
                None,
                None,
            )
            .await?;

        Ok(resp.data())
    }

    /// Get the list of publishers
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the request
    pub async fn get_publishers(
        &self,
        filters: Option<&filters::Filter>,
    ) -> ToshoResult<models::others::PublishersList> {
        let params = match filters {
            Some(f) => f.to_params(),
            None => filters::Filter::default()
                .with_order(filters::SortBy::Name, filters::SortOrder::ASC)
                .with_per_page(25)
                .to_params(),
        };

        self.request(
            reqwest::Method::GET,
            "/publishers",
            None,
            Some(params),
            None,
        )
        .await
    }

    /// Get single publisher detail via slug
    ///
    /// # Arguments
    /// * `publisher_slug` - The publisher slug to get the detail for
    pub async fn get_publisher(
        &self,
        publisher_slug: impl Into<String>,
    ) -> ToshoResult<models::common::Publisher> {
        let resp = self
            .request::<models::others::PublisherDetailResponse>(
                reqwest::Method::GET,
                &format!("/publishers/{}", publisher_slug.into()),
                None,
                None,
                None,
            )
            .await?;

        Ok(resp.data())
    }

    /// Get a list of publisher imprints for a publisher
    ///
    /// # Arguments
    /// * `publisher_slug` - The publisher slug to get the imprints for
    pub async fn get_publisher_imprints(
        &self,
        publisher_slug: impl Into<String>,
    ) -> ToshoResult<models::others::ImprintsList> {
        let params = HashMap::from([("slug".to_string(), publisher_slug.into())]);
        self.request(
            reqwest::Method::GET,
            "/publisher_imprints",
            None,
            Some(params),
            None,
        )
        .await
    }

    /// Get the list of genres
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the request
    pub async fn get_genres(
        &self,
        filters: Option<&filters::Filter>,
    ) -> ToshoResult<models::others::GenresList> {
        let params = match filters {
            Some(f) => f.to_params(),
            None => filters::Filter::default()
                .with_order(filters::SortBy::Name, filters::SortOrder::ASC)
                .with_per_page(100)
                .to_params(),
        };

        self.request(reqwest::Method::GET, "/genres", None, Some(params), None)
            .await
    }

    /// Get the list of creators
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the request
    pub async fn get_creators(
        &self,
        filters: Option<&filters::Filter>,
    ) -> ToshoResult<models::others::CreatorsList> {
        let params = match filters {
            Some(f) => f.to_params(),
            None => filters::Filter::default()
                .with_order(filters::SortBy::DisplayName, filters::SortOrder::ASC)
                .with_per_page(25)
                .to_params(),
        };

        self.request(reqwest::Method::GET, "/creators", None, Some(params), None)
            .await
    }

    /// Get the list of books/issues sold in the marketplace
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the request
    pub async fn get_marketplace_books(
        &self,
        filters: Option<&filters::Filter>,
    ) -> ToshoResult<models::others::MarketplaceBooksList> {
        let params = match filters {
            Some(f) => f.to_params(),
            None => filters::Filter::default()
                .with_order(filters::SortBy::EditionPriceMin, filters::SortOrder::ASC)
                .with_per_page(25)
                .to_params(),
        };

        self.request(
            reqwest::Method::GET,
            "/marketplace/books",
            None,
            Some(params),
            None,
        )
        .await
    }

    /// Get the list of editions in the marketplaces
    ///
    /// # Arguments
    /// * `filters` - The filter to apply to the request
    pub async fn get_marketplace_editions(
        &self,
        filters: Option<&filters::Filter>,
    ) -> ToshoResult<models::others::MarketplaceDetailedEditionsList> {
        let params = match filters {
            Some(f) => f.to_params(),
            None => filters::Filter::default()
                .with_order(filters::SortBy::MarketplacePrice, filters::SortOrder::ASC)
                .with_per_page(25)
                .to_params(),
        };

        self.request(
            reqwest::Method::GET,
            "/marketplace/editions",
            None,
            Some(params),
            None,
        )
        .await
    }

    /// Get the list of editions sold for an issue in the marketplace
    ///
    /// # Arguments
    /// * `issue_id` - The issue UUID to get the editions for
    /// * `filter` - The filter to apply to the request
    pub async fn get_marketplace_book_editions(
        &self,
        issue_id: impl Into<String>,
        filters: Option<&filters::Filter>,
    ) -> ToshoResult<models::others::MarketplaceEditionsList> {
        let mut params = match filters {
            Some(f) => f.to_params(),
            None => filters::Filter::default()
                .clear_filters()
                .with_order(filters::SortBy::BookIndex, filters::SortOrder::ASC)
                .to_params(),
        };
        params.insert("book_id".to_string(), issue_id.into());

        self.request(
            reqwest::Method::GET,
            "/marketplace/editions",
            None,
            Some(params),
            None,
        )
        .await
    }

    /// Get the list of series run in your collections
    ///
    /// This needs authentication.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the request
    pub async fn get_series_run_collections(
        &self,
        filters: Option<&filters::Filter>,
    ) -> ToshoResult<models::series::SeriesRunList> {
        let params = match filters {
            Some(f) => f.to_params(),
            None => filters::Filter::default()
                .with_order(filters::SortBy::Title, filters::SortOrder::ASC)
                .with_per_page(18)
                .to_params(),
        };

        let headers = self.auth_headers(false)?;
        self.request(
            reqwest::Method::GET,
            "/collection/series_run",
            None,
            Some(params),
            Some(headers),
        )
        .await
    }

    /// Get the list of issues in your collection
    ///
    /// This needs authentication.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the request
    pub async fn get_issue_collections(
        &self,
        filters: &filters::Filter,
    ) -> ToshoResult<models::PurchasedIssuesResponse> {
        let params = filters.to_params();

        let headers = self.auth_headers(false)?;
        self.request(
            reqwest::Method::GET,
            "/collection/books",
            None,
            Some(params),
            Some(headers),
        )
        .await
    }

    /// Get the list of editions for an issue in your collection
    ///
    /// This needs authentication.
    ///
    /// # Arguments
    /// * `issue_id` - The issue UUID to get the editions for
    pub async fn get_issue_editions_collections(
        &self,
        issue_id: impl Into<String>,
    ) -> ToshoResult<models::others::CollectedEditionList> {
        let headers = self.auth_headers(false)?;
        self.request(
            reqwest::Method::GET,
            &format!("/collection/books/{}/editions", issue_id.into()),
            None,
            None,
            Some(headers),
        )
        .await
    }

    /// Get your reading history list
    ///
    /// This needs authentication.
    pub async fn get_reading_history(&self) -> ToshoResult<models::others::ReadingHistoryList> {
        let headers = self.auth_headers(false)?;
        self.request(
            reqwest::Method::GET,
            "/collection/books/bookmarked",
            None,
            None,
            Some(headers),
        )
        .await
    }

    /// Get issue reader information
    ///
    /// This needs authentication.
    ///
    /// # Arguments
    /// * `issue_id` - The issue ID to get the reader info for
    pub async fn get_issue_reader(
        &self,
        issue_id: u32,
    ) -> ToshoResult<models::reader::ReaderPagesWithMeta> {
        let headers = self.auth_headers(false)?;

        let response = self
            .request::<models::reader::ReaderPagesResponse>(
                reqwest::Method::GET,
                &format!("/frameflow/{}", issue_id),
                None,
                None,
                Some(headers),
            )
            .await?;

        // Instant deref clone
        Ok(response.data())
    }

    /// Report a page as being viewed/read
    ///
    /// This needs authentication.
    ///
    /// # Arguments
    /// * `issue_uuid` - The issue UUID to report the page for
    /// * `page_number` - The page number to report as being viewed/read, this is 1-based from the pages list
    pub async fn report_page_view(
        &self,
        issue_uuid: impl Into<String>,
        page_number: u32,
    ) -> ToshoResult<models::AckResponse> {
        let data = serde_json::json!({
            "book": {
                "page": page_number,
            }
        });

        let headers = self.auth_headers(true)?;

        self.request(
            reqwest::Method::PATCH,
            &format!("/collection/books/{}/bookmark", issue_uuid.into()),
            Some(data),
            None,
            Some(headers),
        )
        .await
    }

    /// Stream download the image from the given URL.
    ///
    /// The URL can be obtained from [`get_issue_reader`].
    ///
    /// # Parameters
    /// * `url` - The URL to download the image from.
    /// * `writer` - The writer to write the image to.
    pub async fn stream_download(
        &self,
        url: impl AsRef<str>,
        mut writer: impl io::AsyncWrite + Unpin,
    ) -> ToshoResult<()> {
        let res = self
            .inner
            .get(url.as_ref())
            .headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    "Host",
                    reqwest::header::HeaderValue::from_static(SECURE_IMAGE_HOST),
                );
                headers.insert(
                    "User-Agent",
                    reqwest::header::HeaderValue::from_static(self.constants.image_ua),
                );
                headers
            })
            .send()
            .await?;

        // bail if not success
        if !res.status().is_success() {
            Err(tosho_common::ToshoError::from(res.status()))
        } else {
            let mut stream = res.bytes_stream();
            while let Some(item) = stream.try_next().await? {
                writer.write_all(&item).await?;
                writer.flush().await?;
            }

            Ok(())
        }
    }

    /// Get the customer profile
    ///
    /// This needs authentication.
    pub async fn get_profile(&self) -> ToshoResult<models::others::CustomerDetail> {
        let headers = self.auth_headers(true)?;

        let resp = self
            .request::<models::others::CustomerDetailResponse>(
                reqwest::Method::GET,
                "/profile",
                None,
                None,
                Some(headers),
            )
            .await?;

        Ok(resp.data())
    }

    /// Refresh the JWT token
    ///
    /// This needs authentication and needs refresh token
    ///
    /// # Arguments
    /// * `refresh_token` - The refresh token to use for refreshing the JWT token
    pub async fn refresh_token(
        &self,
        refresh_token: impl Into<String>,
    ) -> ToshoResult<models::common::RefreshedTokenResponse> {
        let refresh_tok: String = refresh_token.into();
        let data = serde_json::json!({
            "refresh_token": refresh_tok
        });
        let headers = self.auth_headers(true)?;

        self.request(
            reqwest::Method::POST,
            "/auth/refresh_token",
            Some(data),
            None,
            Some(headers),
        )
        .await
    }

    /// Login to NI and get the auth tokens
    ///
    /// # Arguments
    /// * `email` - The email to use for login
    /// * `password` - The password to use for login
    pub async fn login(
        email: impl Into<String>,
        password: impl Into<String>,
        proxy: Option<reqwest::Proxy>,
    ) -> ToshoResult<models::others::LoginResponse> {
        let data = serde_json::json!({
            "customer": {
                "email": email.into(),
                "password": password.into(),
            }
        });

        let client = reqwest::Client::builder()
            .http2_adaptive_window(true)
            .use_rustls_tls()
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    reqwest::header::USER_AGENT,
                    reqwest::header::HeaderValue::from_static(constants::get_constants(1).ua),
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
                headers
            });

        let client = match proxy {
            Some(proxy) => client
                .proxy(proxy)
                .build()
                .map_err(ToshoClientError::BuildError)?,
            None => client.build().map_err(ToshoClientError::BuildError)?,
        };

        let request = client
            .post(format!("{}/api/v1/auth/login", BASE_API))
            .json(&data);

        parse_json_response_failable::<models::others::LoginResponse, ErrorResponse>(
            request.send().await?,
        )
        .await
    }
}

/// Format a price in USD from the API format to a human-readable format.
///
/// This follows the Stripe currency convention (i.e. 199 = $1.99).
pub fn format_price(price: u64) -> f64 {
    (price as f64) / 100.0
}
