#![warn(missing_docs, clippy::empty_docs, rustdoc::broken_intra_doc_links)]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod constants;
pub mod helper;
pub mod proto;

use crate::constants::{Constants, API_HOST, BASE_API, IMAGE_HOST};
use crate::proto::*;
use futures_util::TryStreamExt;
pub use helper::ConsumeCoin;
pub use helper::ImageQuality;
pub use helper::WeeklyCode;
use std::collections::HashMap;
use tokio::io::{self, AsyncWriteExt};
use tosho_common::{
    bail_on_error, make_error, parse_protobuf_response, ToshoClientError, ToshoError,
    ToshoParseError, ToshoResult,
};

/// Main client for interacting with the SQ MU!
///
/// # Example
/// ```rust,no_run
/// use tosho_musq::MUClient;
/// use tosho_musq::constants::get_constants;
///
/// #[tokio::main]
/// async fn main() {
///     let client = MUClient::new("1234", get_constants(1)).unwrap();
///     let manga = client.get_manga(240).await.unwrap();
///     println!("{:?}", manga);
/// }
/// ```
#[derive(Debug)]
pub struct MUClient {
    /// The inner client
    inner: reqwest::Client,
    /// Current secret used
    secret: String,
    /// The constants used
    constants: &'static Constants,
}

impl MUClient {
    /// Create a new client instance.
    ///
    /// # Parameters
    /// * `secret` - The secret key to use for the client.
    /// * `constants` - The constants to use for the client.
    pub fn new(secret: impl Into<String>, constants: &'static Constants) -> ToshoResult<Self> {
        Self::make_client(secret, constants, None)
    }

    /// Attach a proxy to the client.
    ///
    /// This will clone the client and return a new client with the proxy attached.
    ///
    /// # Arguments
    /// * `proxy` - The proxy to attach to the client
    pub fn with_proxy(&self, proxy: reqwest::Proxy) -> ToshoResult<Self> {
        Self::make_client(&self.secret, self.constants, Some(proxy))
    }

    /// Internal function to make the new client
    fn make_client(
        secret: impl Into<String>,
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
            // Force use HTTP/1.1 since API has problem with HTTP/2
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
            constants,
        })
    }

    /// Modify the HashMap to add the required parameters.
    fn build_params(&self, params: &mut HashMap<String, String>) {
        params.insert("secret".to_string(), self.secret.clone());
        params.insert("app_ver".to_string(), self.constants.app_ver.clone());
        params.insert("os_ver".to_string(), self.constants.os_ver.to_string());
        params.insert("lang".to_string(), "en".to_string());
    }

    /// Create a custom cosume coin object.
    fn build_coin(
        &self,
        need_coin: u64,
        free_coin: u64,
        event_coin: Option<u64>,
        paid_coin: Option<u64>,
    ) -> ConsumeCoin {
        let event_coin = event_coin.unwrap_or(free_coin);
        let paid_coin = paid_coin.unwrap_or(free_coin);

        ConsumeCoin::new(free_coin, event_coin, paid_coin, need_coin)
    }

    // --> Helper methods

    /// Calculate how many coins you need to get this chapter.
    ///
    /// After using this, I recommend subtracting your current User Point value
    /// or getting it when you call any other endpoint to update your value.
    ///
    /// Call this before you call [`get_chapter_images`](#method.get_chapter_images).
    /// Then call the [`.is_possible`](struct.ConsumeCoin.html#method.is_possible) method to check if you have enough coins.
    ///
    /// # Parameters
    /// * `user_point` - Your current user point value, you can get it by calling [`get_user_point`](#method.get_user_point).
    /// * `chapter` - The chapter you want to check with.
    ///
    /// # Example
    /// ```rust,no_run
    /// use tosho_musq::MUClient;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = MUClient::new("1234", tosho_musq::constants::get_constants(1)).unwrap();
    ///
    ///     let user_point = client.get_user_point().await.unwrap();
    ///     let manga = client.get_manga(240).await.unwrap();
    ///     let first_ch = &manga.chapters()[0];
    ///
    ///     let coins = client.calculate_coin(&user_point, first_ch);
    ///     assert!(coins.is_possible());
    /// }
    /// ```
    pub fn calculate_coin(
        &self,
        user_point: &UserPoint,
        chapter: &ChapterV2,
    ) -> ToshoResult<ConsumeCoin> {
        if chapter.is_free() {
            return Ok(self.build_coin(0, 0, None, None));
        }

        match chapter.consumption() {
            ConsumptionType::Any => {
                // Prioritization: Free > Event > Paid
                let free = user_point.free();
                let event = user_point.event();
                let paid = user_point.paid();

                let need = ((chapter.price() - free) as i64).max(0);
                if need <= 0 {
                    return Ok(self.build_coin(chapter.price(), chapter.price(), Some(0), Some(0)));
                }

                let need = (need - event as i64).max(0);
                if need <= 0 {
                    let event_diff = chapter.price().saturating_sub(free);

                    return Ok(self.build_coin(chapter.price(), free, Some(event_diff), Some(0)));
                }

                let need = (need - paid as i64).max(0);
                let mut paid_diff = chapter.price().saturating_sub(free).saturating_sub(event);
                if need > 0 {
                    paid_diff = paid;
                }

                Ok(self.build_coin(chapter.price(), free, Some(event), Some(paid_diff)))
            }
            ConsumptionType::EventOrPaid => {
                // Prioritization: Event > Paid
                let event = user_point.event();
                let paid = user_point.paid();

                let need = ((chapter.price() - event) as i64).max(0);
                if need <= 0 {
                    return Ok(self.build_coin(chapter.price(), chapter.price(), Some(0), Some(0)));
                }

                let need = (need - paid as i64).max(0);
                let mut paid_diff = chapter.price().saturating_sub(event);
                if need > 0 {
                    paid_diff = paid;
                }

                Ok(self.build_coin(chapter.price(), event, Some(paid_diff), Some(0)))
            }
            ConsumptionType::Paid => {
                let paid_left: i64 = user_point.paid() as i64 - chapter.price() as i64;

                if paid_left < 0 {
                    return Ok(self.build_coin(chapter.price(), 0, Some(0), Some(0)));
                }

                Ok(self.build_coin(chapter.price(), 0, Some(0), Some(chapter.price())))
            }
            ConsumptionType::Free
            | ConsumptionType::Rental
            | ConsumptionType::Purchased
            | ConsumptionType::Subscription => Ok(self.build_coin(chapter.price(), 0, None, None)),
            ConsumptionType::Unrecognized => {
                Err(ToshoError::ParseError(ToshoParseError::ExpectedResponse(
                    "valid consumption type (got code -1 instead)".to_string(),
                )))
            }
        }
    }

    /// Build and merge URL into a full API url
    fn build_url(&self, path: &str) -> String {
        if path.starts_with('/') {
            return format!("{}{}", &*BASE_API, path);
        }

        format!("{}/{}", &*BASE_API, path)
    }

    /// Create an empty params
    fn empty_params(&self) -> HashMap<String, String> {
        let mut params: HashMap<String, String> = HashMap::new();

        self.build_params(&mut params);

        params
    }

    // <-- Helper methods

    // --> PointEndpoints.kt

    /// Get the point shop information.
    pub async fn get_point_shop(&self) -> ToshoResult<PointShopView> {
        let res = self
            .inner
            .get(self.build_url("/point/shop"))
            .query(&self.empty_params())
            .send()
            .await?;

        parse_protobuf_response(res).await
    }

    /// Get your current user point.
    pub async fn get_user_point(&self) -> ToshoResult<UserPoint> {
        // Guarantee that the user point is always available
        let point = self.get_point_shop().await?;
        match point.user_point() {
            Some(point) => Ok(point),
            None => Err(ToshoParseError::expect("user point")),
        }
    }

    /// Get your point acquisition history.
    pub async fn get_point_history(&self) -> ToshoResult<PointHistoryView> {
        let res = self
            .inner
            .get(self.build_url("/point/history"))
            .query(&self.empty_params())
            .send()
            .await?;

        parse_protobuf_response(res).await
    }

    // <-- PointEndpoints.kt

    // --> MangaEndpoints.kt

    /// Get manga detail information.
    ///
    /// # Parameters
    /// * `manga_id` - The manga ID.
    pub async fn get_manga(&self, manga_id: u64) -> ToshoResult<MangaDetailV2> {
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert("title_id".to_string(), manga_id.to_string());
        params.insert("ui_lang".to_string(), "en".to_string());
        params.insert("quality".to_string(), ImageQuality::High.to_string());

        self.build_params(&mut params);

        let res = self
            .inner
            .get(self.build_url("/manga/detail_v2"))
            .query(&params)
            .send()
            .await?;

        let manga = parse_protobuf_response::<MangaDetailV2>(res).await?;

        if manga.status() != Status::Success {
            bail_on_error!("Failed to get manga detail: {:?}", manga)
        } else {
            Ok(manga)
        }
    }

    /// Get weekly manga updates.
    ///
    /// # Parameters
    /// * `weekday` - The day of the week to get the updates from.
    pub async fn get_weekly_titles(&self, weekday: WeeklyCode) -> ToshoResult<MangaResults> {
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert("code".to_string(), weekday.to_string());

        self.build_params(&mut params);

        let res = self
            .inner
            .get(self.build_url("/manga/weekly"))
            .query(&params)
            .send()
            .await?;

        parse_protobuf_response(res).await
    }

    /// Search manga by query.
    ///
    /// # Parameters
    /// * `query` - The query to search for.
    pub async fn search(&self, query: impl Into<String>) -> ToshoResult<MangaResults> {
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert("word".to_string(), query.into());

        self.build_params(&mut params);

        let res = self
            .inner
            .get(self.build_url("/manga/search"))
            .query(&params)
            .send()
            .await?;

        parse_protobuf_response(res).await
    }

    /// Search manga by tag.
    ///
    /// # Parameters
    /// * `tag_id` - The tag ID to search for.
    pub async fn search_by_tag(&self, tag_id: u64) -> ToshoResult<MangaResults> {
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert("tag_id".to_string(), tag_id.to_string());

        self.build_params(&mut params);

        let res = self
            .inner
            .get(self.build_url("/manga/tag"))
            .form(&params)
            .send()
            .await?;

        parse_protobuf_response(res).await
    }

    // <-- MangaEndpoints.kt

    // --> ChapterEndpoints.kt

    /// Get chapter viewer that contains images.
    ///
    /// # Parameters
    /// * `chapter_id` - The chapter ID.
    /// * `quality` - The image quality to get.
    /// * `coins` - The coins to consume.
    pub async fn get_chapter_images(
        &self,
        chapter_id: u64,
        quality: ImageQuality,
        coins: Option<ConsumeCoin>,
    ) -> ToshoResult<ChapterViewerV2> {
        let coins = coins.unwrap_or_default();

        let mut params = HashMap::new();
        params.insert("chapter_id".to_string(), chapter_id.to_string());
        params.insert("quality".to_string(), quality.to_string());
        params.insert("free_point".to_string(), coins.get_free().to_string());
        params.insert("event_point".to_string(), coins.get_event().to_string());
        params.insert("paid_point".to_string(), coins.get_paid().to_string());

        self.build_params(&mut params);

        let res = self
            .inner
            .post(self.build_url("/manga/viewer_v2"))
            .form(&params)
            .send()
            .await?;

        let viewer: ChapterViewerV2 = parse_protobuf_response(res).await?;

        if viewer.status() != Status::Success {
            bail_on_error!("Failed to get chapter viewer: {:?}", viewer)
        } else {
            Ok(viewer)
        }
    }

    // <-- ChapterEndpoints.kt

    // --> AccountEndpoints.kt

    /// Get your account information.
    pub async fn get_account(&self) -> ToshoResult<AccountView> {
        let res = self
            .inner
            .get(self.build_url("/account/account"))
            .query(&self.empty_params())
            .send()
            .await?;

        parse_protobuf_response(res).await
    }

    /// Get your account setting.
    pub async fn get_setting(&self) -> ToshoResult<SettingView> {
        let res = self
            .inner
            .get(self.build_url("/setting/setting"))
            .query(&self.empty_params())
            .send()
            .await?;

        parse_protobuf_response(res).await
    }

    // <-- AccountEndpoints.kt

    // --> Api.kt (Personalized)

    /// Get your manga list for your account.
    pub async fn get_my_manga(&self) -> ToshoResult<MyPageView> {
        let res = self
            .inner
            .get(self.build_url("/my_page"))
            .query(&self.empty_params())
            .send()
            .await?;

        parse_protobuf_response(res).await
    }

    /// Get your personalized home view.
    ///
    /// Same result when you click the ``Home`` button in the app.
    pub async fn get_my_home(&self) -> ToshoResult<HomeViewV2> {
        let mut params = HashMap::new();
        params.insert("ui_lang".to_string(), "en".to_string());

        self.build_params(&mut params);

        let res = self
            .inner
            .get(self.build_url("/home_v2"))
            .query(&params)
            .send()
            .await?;

        parse_protobuf_response(res).await
    }

    // <-- Api.kt (Personalized)

    // --> Downloader

    /// Replace the image host with the valid and correct host.
    ///
    /// Sometimes the API would return a URL with cloudfront host,
    /// which can't be accessed directly but need to use the "mirror" host
    /// provided by the client.
    fn replace_image_host(&self, url: impl AsRef<str>) -> ToshoResult<::reqwest::Url> {
        let url = url.as_ref();
        match ::reqwest::Url::parse(url) {
            Ok(mut parsed_url) => {
                let valid_host = ::reqwest::Url::parse(
                    format!("https://{}", &*IMAGE_HOST).as_str(),
                )
                .map_err(|e| make_error!("Failed to parse image host: {}: {}", &*IMAGE_HOST, e))?;
                let host_name = valid_host
                    .host_str()
                    .ok_or_else(|| make_error!("Failed to get host from: {}", &valid_host))?;
                parsed_url.set_host(Some(host_name)).map_err(|e| {
                    make_error!(
                        "Failed to replace image host: {} with {}: {}",
                        url,
                        &*IMAGE_HOST,
                        e
                    )
                })?;

                Ok(parsed_url)
            }
            Err(_) => {
                // parse url failed, assume it's a relative path
                let full_url = format!("https://{}{}", &*IMAGE_HOST, url);
                let parse_url = ::reqwest::Url::parse(full_url.as_str()).map_err(|e| {
                    make_error!("Failed to parse image host: {}: {}", &*IMAGE_HOST, e)
                })?;
                Ok(parse_url)
            }
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
        url: impl AsRef<str>,
        mut writer: impl io::AsyncWrite + Unpin,
    ) -> ToshoResult<()> {
        let actual_url = self.replace_image_host(url)?;

        let res = self
            .inner
            .get(actual_url)
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

    // <-- Downloader
}

/// Decrypt the image if it's encrypted.
///
/// # Parameters
/// * `image` - The image to decrypt.
/// * `page` - The chapter page information which contains the key.
#[cfg(feature = "aes-dec")]
pub fn decrypt_image(image: &[u8], page: &proto::ChapterPage) -> ToshoResult<Vec<u8>> {
    use aes::cipher::block_padding::Pkcs7;
    use aes::cipher::{BlockDecryptMut, KeyIvInit};
    use aes::Aes256;

    let key_bytes = page.key_as_bytes()?;
    let iv_bytes = page.iv_as_bytes()?;

    match (key_bytes, iv_bytes) {
        (Some(key), Some(iv)) => {
            let decryptor = cbc::Decryptor::<Aes256>::new_from_slices(&key, &iv)
                .map_err(|e| make_error!("Failed to create decryptor: {}", e))?;

            let mut dec_image = image.to_vec();
            decryptor
                .decrypt_padded_mut::<Pkcs7>(&mut dec_image)
                .map_err(|e| make_error!("Failed to decrypt image: {}", e))?;

            Ok(dec_image)
        }
        _ => {
            // Just return the image
            Ok(image.to_vec())
        }
    }
}
