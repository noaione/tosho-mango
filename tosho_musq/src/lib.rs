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
use prost::Message;
use std::collections::HashMap;
use std::io::Cursor;
use tokio::io::{self, AsyncWriteExt};

/// Main client for interacting with the SQ MU!
///
/// # Example
/// ```no_run,ignore
/// use tosho_musq::MUClient;
/// use tosho_musq::constants::ANDROID_CONSTANTS;
///
/// #[tokio::main]
/// async fn main() {
///     let client = MUClient::new("1234", ANDROID_CONSTANTS);
///     let manga = client.get_manga(240).await.unwrap();
///     println!("{:?}", manga);
/// }
/// ```
#[derive(Debug)]
pub struct MUClient {
    inner: reqwest::Client,
    secret: String,
    constants: Constants,
}

impl MUClient {
    /// Create a new client instance.
    ///
    /// # Parameters
    /// * `secret` - The secret key to use for the client.
    /// * `constants` - The constants to use for the client.
    pub fn new(secret: &str, constants: Constants) -> Self {
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
            .default_headers(headers)
            .build()
            .unwrap();

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
    /// ```no_run,ignore
    /// use tosho_musq::MUClient;
    ///
    /// let client = MUClient::new("1234", tosho_musq::constants::ANDROID_CONSTANTS);
    ///
    /// let user_point = client.get_user_point();
    /// let chapter = client.get_chapter(12345);
    ///
    /// let coins = client.calculate_coin(&user_point, &chapter);
    /// assert_eq!(coins.is_possible(), true);
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

        if res.status().is_success() {
            let bytes_data = res.bytes().await?;
            let cursor = bytes_data.as_ref();

            Ok(PointShopView::decode(&mut Cursor::new(cursor))
                .unwrap()
                .user_point
                .unwrap())
        } else {
            anyhow::bail!("Failed to get user point: {}", res.status())
        }
    }

    /// Get your point acquisition history.
    pub async fn get_point_history(&self) -> anyhow::Result<PointHistoryView> {
        let res = self
            .inner
            .get(self.build_url("/point/history"))
            .query(&self.empty_params())
            .send()
            .await?;

        if res.status().is_success() {
            let bytes_data = res.bytes().await?;
            let cursor = bytes_data.as_ref();

            Ok(PointHistoryView::decode(&mut Cursor::new(cursor)).unwrap())
        } else {
            anyhow::bail!("Failed to get point history: {}", res.status())
        }
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

        if res.status().is_success() {
            let bytes_data = res.bytes().await?;
            let cursor = bytes_data.as_ref();

            let manga = MangaDetailV2::decode(&mut Cursor::new(cursor)).unwrap();
            if manga.status() != Status::Success {
                anyhow::bail!("Failed to get manga detail: {:?}", manga.status())
            }

            Ok(manga)
        } else {
            anyhow::bail!("Failed to get manga detail: {}", res.status())
        }
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

        if res.status().is_success() {
            let bytes_data = res.bytes().await?;
            let cursor = bytes_data.as_ref();

            Ok(MangaResults::decode(&mut Cursor::new(cursor)).unwrap())
        } else {
            anyhow::bail!("Failed to get weekly titles: {}", res.status())
        }
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

        if res.status().is_success() {
            let bytes_data = res.bytes().await?;
            let cursor = bytes_data.as_ref();

            Ok(MangaResults::decode(&mut Cursor::new(cursor)).unwrap())
        } else {
            anyhow::bail!("Failed to search manga: {}", res.status())
        }
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

        if res.status().is_success() {
            let bytes_data = res.bytes().await?;
            let cursor = bytes_data.as_ref();

            Ok(MangaResults::decode(&mut Cursor::new(cursor)).unwrap())
        } else {
            anyhow::bail!("Failed to search manga by tag: {}", res.status())
        }
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

        if res.status().is_success() {
            let bytes_data = res.bytes().await.unwrap();
            let cursor = bytes_data.as_ref();

            let viewer = ChapterViewerV2::decode(&mut Cursor::new(cursor)).unwrap();
            if viewer.status() != Status::Success {
                anyhow::bail!("Failed to get chapter viewer: {:?}", viewer.status())
            }

            Ok(viewer)
        } else {
            anyhow::bail!("Failed to get chapter viewer: {}", res.status())
        }
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

        println!("{:?}", res.url());

        if res.status().is_success() {
            let bytes_data = res.bytes().await?;
            let cursor = bytes_data.as_ref();

            Ok(AccountView::decode(&mut Cursor::new(cursor)).unwrap())
        } else {
            anyhow::bail!("Failed to get account information: {}", res.status())
        }
    }

    /// Get your account setting.
    pub async fn get_setting(&self) -> anyhow::Result<SettingView> {
        let res = self
            .inner
            .get(self.build_url("/setting/setting"))
            .query(&self.empty_params())
            .send()
            .await?;

        if res.status().is_success() {
            let bytes_data = res.bytes().await?;
            let cursor = bytes_data.as_ref();

            Ok(SettingView::decode(&mut Cursor::new(cursor)).unwrap())
        } else {
            anyhow::bail!("Failed to get account setting: {}", res.status())
        }
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

        if res.status().is_success() {
            let bytes_data = res.bytes().await?;
            let cursor = bytes_data.as_ref();

            Ok(MyPageView::decode(&mut Cursor::new(cursor)).unwrap())
        } else {
            anyhow::bail!("Failed to get my manga: {}", res.status())
        }
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

        if res.status().is_success() {
            let bytes_data = res.bytes().await?;
            let cursor = bytes_data.as_ref();

            Ok(HomeViewV2::decode(&mut Cursor::new(cursor)).unwrap())
        } else {
            anyhow::bail!("Failed to get my home: {}", res.status())
        }
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
