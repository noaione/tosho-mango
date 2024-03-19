//! # tosho-musq
//!
//! ![crates.io version](https://img.shields.io/crates/v/tosho-musq)
//!
//! An asynchronous client for the MU! API by SQ.
//!
//! The following crate is used by the [`tosho`] app.
//!
//! ## Usage
//!
//! Download the [`tosho`] app, or you can utilize this crate like any other Rust crate:
//!
//! ```rust,no_run
//! use tosho_musq::MUClient;
//! use tosho_musq::constants::get_constants;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = MUClient::new("1234", get_constants(1));
//!     let manga = client.get_manga(240).await.unwrap();
//!     println!("{:?}", manga);
//! }
//! ```
//!
//! ## Authentication
//!
//! The following sources do not have any easy authentication method.
//!
//! The command to authenticate is `tosho mu auth`.
//!
//! It's recommended that you set up network intercepting first; please read [INTERCEPTING](https://github.com/noaione/tosho-mango/blob/master/INTERCEPTING.md).
//!
//! Using the CLI, you can do this:
//!
//! ```bash
//! $ tosho mu auth secret -t android
//! ```
//!
//! Or, with Apple constants:
//!
//! ```bash
//! $ tosho mu auth secret -t ios
//! ```
//!
//! With crates, you can follow the above usages.
//!
//! ### Android
//!
//! 1. Open the source app.
//! 2. Click on the home page or my page.
//! 3. Observe the requests on HTTP Toolkit and find the request to the API that has `secret` as the query parameters.
//! 4. Save that secret elsewhere and authenticate with `tosho`.
//!
//! ### Apple
//!
//! 1. Open the Stream app and click `Sniff Now`.
//! 2. Go to the source app and open the `Home` or `My Page`.
//! 3. Return to the Stream app and click `Sniff History`, then select the most recent item.
//! 4. Find the request that goes to the API of the source app and locate the request that has `secret=xxxxx` in them.
//! 5. Copy the link and save the secret value somewhere so you can authenticate with `tosho`.
//!
//! ## Disclaimer
//!
//! This project is designed as an experiment and to create a local copy for personal use.
//! These tools will not circumvent any paywall, and you will need to purchase and own each chapter
//! with your own account to be able to make your own local copy.
//!
//! We're not responsible if your account got deactivated.
//!
//! ## License
//!
//! This project is licensed with MIT License ([LICENSE](https://github.com/noaione/tosho-mango/blob/master/LICENSE) or <http://opensource.org/licenses/MIT>)
//!
//! [`tosho`]: https://crates.io/crates/tosho

pub mod constants;
pub mod helper;
pub mod proto;

use crate::constants::Constants;
use crate::constants::API_HOST;
use crate::constants::BASE_API;
use crate::constants::IMAGE_HOST;
use crate::proto::*;
use futures_util::StreamExt;
pub use helper::ConsumeCoin;
pub use helper::ImageQuality;
pub use helper::WeeklyCode;
use std::collections::HashMap;
use std::io::Cursor;
use tokio::io::{self, AsyncWriteExt};

/// Main client for interacting with the SQ MU!
///
/// # Example
/// ```no_run
/// use tosho_musq::MUClient;
/// use tosho_musq::constants::get_constants;
///
/// #[tokio::main]
/// async fn main() {
///     let client = MUClient::new("1234", get_constants(1));
///     let manga = client.get_manga(240).await.unwrap();
///     println!("{:?}", manga);
/// }
/// ```
#[derive(Debug)]
pub struct MUClient {
    inner: reqwest::Client,
    secret: String,
    constants: &'static Constants,
}

impl MUClient {
    /// Create a new client instance.
    ///
    /// # Parameters
    /// * `secret` - The secret key to use for the client.
    /// * `constants` - The constants to use for the client.
    pub fn new(secret: &str, constants: &'static Constants) -> Self {
        Self::make_client(secret, constants, None)
    }

    /// Attach a proxy to the client.
    ///
    /// This will clone the client and return a new client with the proxy attached.
    ///
    /// # Arguments
    /// * `proxy` - The proxy to attach to the client
    pub fn with_proxy(&self, proxy: reqwest::Proxy) -> Self {
        Self::make_client(&self.secret, self.constants, Some(proxy))
    }

    fn make_client(
        secret: &str,
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

        let client = reqwest::Client::builder().default_headers(headers);

        let client = match proxy {
            Some(proxy) => client.proxy(proxy).build().unwrap(),
            None => client.build().unwrap(),
        };

        Self {
            inner: client,
            secret: secret.to_string(),
            constants,
        }
    }

    /// Modify the HashMap to add the required parameters.
    fn build_params(&self, params: &mut HashMap<String, String>) {
        params.insert("secret".to_string(), self.secret.clone());
        params.insert("app_ver".to_string(), self.constants.app_ver.clone());
        params.insert("os_ver".to_string(), self.constants.os_ver.to_string());
        params.insert("lang".to_string(), "en".to_string());
    }

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
    /// ```no_run
    /// use tosho_musq::MUClient;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = MUClient::new("1234", tosho_musq::constants::get_constants(1));
    ///    
    ///     let user_point = client.get_user_point().await.unwrap();
    ///     let manga = client.get_manga(240).await.unwrap();
    ///     let first_ch = &manga.chapters[0];
    ///    
    ///     let coins = client.calculate_coin(&user_point, first_ch);
    ///     assert!(coins.is_possible());
    /// }
    /// ```
    pub fn calculate_coin(&self, user_point: &UserPoint, chapter: &ChapterV2) -> ConsumeCoin {
        if chapter.is_free() {
            return self.build_coin(0, 0, None, None);
        }

        match chapter.consumption() {
            ConsumptionType::Any => {
                // Prioritization: Free > Event > Paid
                let free = user_point.free;
                let event = user_point.event;
                let paid = user_point.paid;

                let need = ((chapter.price - free) as i64).max(0);
                if need <= 0 {
                    return self.build_coin(chapter.price, chapter.price, Some(0), Some(0));
                }

                let need = (need - event as i64).max(0);
                if need <= 0 {
                    let event_diff = chapter.price.saturating_sub(free);

                    return self.build_coin(chapter.price, free, Some(event_diff), Some(0));
                }

                let need = (need - paid as i64).max(0);
                let mut paid_diff = chapter
                    .price
                    .saturating_sub(free)
                    .saturating_sub(event)
                    .max(0);
                if need > 0 {
                    paid_diff = paid;
                }

                self.build_coin(chapter.price, free, Some(event), Some(paid_diff))
            }
            ConsumptionType::EventOrPaid => {
                // Prioritization: Event > Paid
                let event = user_point.event;
                let paid = user_point.paid;

                let need = ((chapter.price - event) as i64).max(0);
                if need <= 0 {
                    return self.build_coin(chapter.price, chapter.price, Some(0), Some(0));
                }

                let need = (need - paid as i64).max(0);
                let mut paid_diff = chapter.price.saturating_sub(event).max(0);
                if need > 0 {
                    paid_diff = paid;
                }

                self.build_coin(chapter.price, event, Some(paid_diff), Some(0))
            }
            ConsumptionType::Paid => {
                let paid_left: i64 = user_point.paid as i64 - chapter.price as i64;

                if paid_left < 0 {
                    return self.build_coin(chapter.price, 0, Some(0), Some(0));
                }

                self.build_coin(chapter.price, 0, Some(0), Some(chapter.price))
            }
            _ => {
                panic!("Unknown consumption type: {:?}", chapter.consumption());
            }
        }
    }

    fn build_url(&self, path: &str) -> String {
        if path.starts_with('/') {
            return format!("{}{}", *BASE_API, path);
        }

        format!("{}/{}", *BASE_API, path)
    }

    fn empty_params(&self) -> HashMap<String, String> {
        let mut params: HashMap<String, String> = HashMap::new();

        self.build_params(&mut params);

        params
    }

    // <-- Helper methods

    // --> PointEndpoints.kt

    /// Get your current user point.
    pub async fn get_user_point(&self) -> anyhow::Result<UserPoint> {
        let res = self
            .inner
            .get(self.build_url("/point/shop"))
            .query(&self.empty_params())
            .send()
            .await?;

        parse_response::<PointShopView>(res)
            .await
            .map(|x| x.user_point.unwrap())
    }

    /// Get your point acquisition history.
    pub async fn get_point_history(&self) -> anyhow::Result<PointHistoryView> {
        let res = self
            .inner
            .get(self.build_url("/point/history"))
            .query(&self.empty_params())
            .send()
            .await?;

        parse_response(res).await
    }

    // <-- PointEndpoints.kt

    // --> MangaEndpoints.kt

    /// Get manga detail information.
    ///
    /// # Parameters
    /// * `manga_id` - The manga ID.
    pub async fn get_manga(&self, manga_id: u64) -> anyhow::Result<MangaDetailV2> {
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

        let manga = parse_response::<MangaDetailV2>(res).await?;

        if manga.status() != Status::Success {
            anyhow::bail!("Failed to get manga detail: {:?}", manga.status())
        }

        Ok(manga)
    }

    /// Get weekly manga updates.
    ///
    /// # Parameters
    /// * `weekday` - The day of the week to get the updates from.
    pub async fn get_weekly_titles(&self, weekday: WeeklyCode) -> anyhow::Result<MangaResults> {
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert("code".to_string(), weekday.to_string());

        self.build_params(&mut params);

        let res = self
            .inner
            .get(self.build_url("/manga/weekly"))
            .query(&params)
            .send()
            .await?;

        parse_response(res).await
    }

    /// Search manga by query.
    ///
    /// # Parameters
    /// * `query` - The query to search for.
    pub async fn search(&self, query: &str) -> anyhow::Result<MangaResults> {
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert("word".to_string(), query.to_string());

        self.build_params(&mut params);

        let res = self
            .inner
            .get(self.build_url("/manga/search"))
            .query(&params)
            .send()
            .await?;

        parse_response(res).await
    }

    /// Search manga by tag.
    ///
    /// # Parameters
    /// * `tag_id` - The tag ID to search for.
    pub async fn search_by_tag(&self, tag_id: u64) -> anyhow::Result<MangaResults> {
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert("tag_id".to_string(), tag_id.to_string());

        self.build_params(&mut params);

        let res = self
            .inner
            .get(self.build_url("/manga/tag"))
            .form(&params)
            .send()
            .await?;

        parse_response(res).await
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
    ) -> anyhow::Result<ChapterViewerV2> {
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
            .await
            .unwrap();

        let viewer: ChapterViewerV2 = parse_response(res).await?;
        if viewer.status() != Status::Success {
            anyhow::bail!("Failed to get chapter viewer: {:?}", viewer.status())
        }

        Ok(viewer)
    }

    // <-- ChapterEndpoints.kt

    // --> AccountEndpoints.kt

    /// Get your account information.
    pub async fn get_account(&self) -> anyhow::Result<AccountView> {
        let res = self
            .inner
            .get(self.build_url("/account/account"))
            .query(&self.empty_params())
            .send()
            .await?;

        parse_response(res).await
    }

    /// Get your account setting.
    pub async fn get_setting(&self) -> anyhow::Result<SettingView> {
        let res = self
            .inner
            .get(self.build_url("/setting/setting"))
            .query(&self.empty_params())
            .send()
            .await?;

        parse_response(res).await
    }

    // <-- AccountEndpoints.kt

    // --> Api.kt (Personalized)

    /// Get your manga list for your account.
    pub async fn get_my_manga(&self) -> anyhow::Result<MyPageView> {
        let res = self
            .inner
            .get(self.build_url("/my_page"))
            .query(&self.empty_params())
            .send()
            .await?;

        parse_response(res).await
    }

    /// Get your personalized home view.
    ///
    /// Same result when you click the ``Home`` button in the app.
    pub async fn get_my_home(&self) -> anyhow::Result<HomeViewV2> {
        let mut params = HashMap::new();
        params.insert("ui_lang".to_string(), "en".to_string());

        self.build_params(&mut params);

        let res = self
            .inner
            .get(self.build_url("/home_v2"))
            .query(&params)
            .send()
            .await?;

        parse_response(res).await
    }

    // <-- Api.kt (Personalized)

    // --> Downloader

    /// Replace the image host with the valid and correct host.
    ///
    /// Sometimes the API would return a URL with cloudfront host,
    /// which can't be accessed directly but need to use the "mirror" host
    /// provided by the client.
    fn replace_image_host(&self, url: &str) -> anyhow::Result<::reqwest::Url> {
        match ::reqwest::Url::parse(url) {
            Ok(mut parsed_url) => {
                let valid_host =
                    ::reqwest::Url::parse(format!("https://{}", *IMAGE_HOST).as_str())?;
                parsed_url.set_host(Some(valid_host.host_str().unwrap()))?;

                Ok(parsed_url)
            }
            Err(_) => {
                // parse url failed, assume it's a relative path
                let full_url = format!("https://{}{}", *IMAGE_HOST, url);
                let parse_url = ::reqwest::Url::parse(full_url.as_str())?;
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
        url: &str,
        mut writer: impl io::AsyncWrite + Unpin,
    ) -> anyhow::Result<()> {
        let actual_url = self.replace_image_host(url)?;

        let res = self
            .inner
            .get(actual_url)
            .headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    "Host",
                    reqwest::header::HeaderValue::from_str(&IMAGE_HOST).unwrap(),
                );
                headers.insert(
                    "User-Agent",
                    reqwest::header::HeaderValue::from_str(&self.constants.image_ua).unwrap(),
                );
                headers.insert(
                    "Cache-Control",
                    reqwest::header::HeaderValue::from_static("no-cache"),
                );

                headers
            })
            .send()
            .await
            .unwrap();

        // bail if not success
        if !res.status().is_success() {
            anyhow::bail!("Failed to download image: {}", res.status())
        }

        let mut stream = res.bytes_stream();
        while let Some(item) = stream.next().await {
            let item = item.unwrap();
            writer.write_all(&item).await?;
        }

        Ok(())
    }

    // <-- Downloader
}

async fn parse_response<T>(res: reqwest::Response) -> anyhow::Result<T>
where
    T: ::prost::Message + Default + Clone,
{
    if res.status().is_success() {
        let bytes_data = res.bytes().await?;
        let cursor = bytes_data.as_ref();

        Ok(T::decode(&mut Cursor::new(cursor))?)
    } else {
        anyhow::bail!("MU! request failed with status: {}", res.status())
    }
}
