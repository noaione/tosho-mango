use serde::{Deserialize, Serialize};

use super::{ComicStatus, IAPInfo};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComicSimpleInfoNode {
    #[serde(rename = "manga_sele_id")]
    pub id: u64,
    pub title: String,
    pub update_date: Option<u64>,
    pub cover_url: String,
    #[serde(rename = "new_flg")]
    pub new_update: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComicSimpleInfo {
    #[serde(rename = "comic_info")]
    pub info: ComicSimpleInfoNode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComicBannerInfoNode {
    #[serde(rename = "manga_sele_id")]
    pub id: u64,
    #[serde(rename = "url")]
    pub cover_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComicBannerInfo {
    #[serde(rename = "banner")]
    pub info: ComicBannerInfoNode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComicDiscoveryHeader {
    pub title: String,
    pub tag_id: Option<u64>,
    pub complete: Option<u64>, // what?
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComicDiscoveryNode {
    pub header: ComicDiscoveryHeader,
    #[serde(rename = "comic_info_list")]
    pub comics: Vec<ComicSimpleInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComicDiscoveryPaginatedResponse {
    #[serde(rename = "comic_info_list")]
    pub comics: Vec<ComicSimpleInfo>,
    pub next_page: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComicSearchResponse {
    #[serde(rename = "comic_info_list")]
    pub comics: Vec<ComicSimpleInfo>,
    pub total_count: String,
    pub next_page: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComicDiscovery {
    #[serde(rename = "manga_banner_list")]
    pub banners: Vec<ComicBannerInfo>,
    pub updated: Vec<ComicDiscoveryNode>,
    pub free_campaigns: Vec<ComicDiscoveryNode>,
    #[serde(rename = "tag_contents1")]
    pub tags1: Vec<ComicDiscoveryNode>,
    #[serde(rename = "tag_contents2")]
    pub tags2: Vec<ComicDiscoveryNode>,
    pub completed: Vec<ComicDiscoveryNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComicInfoFreeDaily {
    #[serde(rename = "next_free_daily_time")]
    pub next: u64,
    #[serde(rename = "free_daily_term")]
    pub term: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComicEpisodeInfoNode {
    #[serde(rename = "story_no")]
    pub id: u64,
    pub title: String,
    #[serde(rename = "i_price")]
    pub price: u64,
    #[serde(rename = "update_timestamp")]
    pub update_date: u64,
    pub thumbnail: String,
    pub likes: String,
    pub comments: String,
    #[serde(rename = "total_page_count")]
    pub page_count: u64,
    #[serde(rename = "page_start_status")]
    pub start_status: i32,
    #[serde(rename = "is_free_daily_episode")]
    pub is_free_daily: bool,
    pub campaign_end_at: Option<u64>,
    #[serde(rename = "i_expire_time")]
    pub expiry_time: Option<u64>,
    pub close_time: Option<u64>, // ???
    #[serde(rename = "included_volume")]
    pub included_in: Option<String>,
}

impl ComicEpisodeInfoNode {
    /// Check if the episode is free
    pub fn is_free(&self) -> bool {
        self.price == 0
    }

    /// Check if the episode is available to read
    pub fn is_available(&self) -> bool {
        let current_unix = chrono::Utc::now().timestamp();
        self.is_free() || (self.expiry_time.unwrap_or(0) as i64) > current_unix
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComicEpisodeInfo {
    #[serde(rename = "comic_body_info")]
    pub info: ComicEpisodeInfoNode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComicAuthorInfoNode {
    #[serde(rename = "a_id")]
    pub id: u64,
    #[serde(rename = "a_name")]
    pub name: String,
    #[serde(rename = "disp_a_kind")]
    pub kind: String,
    #[serde(rename = "a_comment")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComicAuthorInfo {
    #[serde(rename = "author_info")]
    pub info: ComicAuthorInfoNode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComicTagInfoNode {
    #[serde(rename = "tag_id")]
    pub id: u64,
    #[serde(rename = "tag_name")]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComicTagInfo {
    #[serde(rename = "tag_info")]
    pub info: ComicTagInfoNode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComicInfo {
    pub title: String,
    #[serde(rename = "shoukai")]
    pub description: String,
    pub update_date: Option<u64>,
    pub next_update_date: Option<u64>,
    pub cover_url: String,
    pub thumbnail_url: String,
    #[serde(rename = "cont_url")]
    pub web_url: Option<String>,
    #[serde(rename = "comic_body_info_list")]
    pub episodes: Vec<ComicEpisodeInfo>,
    pub next_update_text: Option<String>,
    pub favorite: bool,
    pub rental_term: Option<String>,
    #[serde(rename = "author_info_list")]
    pub authors: Vec<ComicAuthorInfo>,
    #[serde(rename = "tag_info_list")]
    pub tags: Vec<ComicTagInfo>,
    pub likes: String,
    pub comments: String,
    #[serde(rename = "complete")]
    pub status: ComicStatus,
    #[serde(rename = "production_participants")]
    pub productions: String,
    #[serde(rename = "is_free_daily")]
    pub has_free_daily: bool,
    pub free_daily: Option<ComicInfoFreeDaily>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComicVolumeBookInfoNode {
    pub title: String,
    pub cover_url: String,
    pub detail_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComicVolumeBookInfo {
    #[serde(rename = "book_info")]
    pub info: ComicVolumeBookInfoNode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComicInfoResponse {
    #[serde(rename = "comic_info")]
    pub info: ComicInfo,
    #[serde(rename = "book_info_list")]
    pub volumes: Vec<ComicVolumeBookInfo>,
    #[serde(rename = "iap_info")]
    pub account: IAPInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComicReadPageNode {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComicReadPage {
    #[serde(rename = "iap_url_info")]
    pub info: ComicReadPageNode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComicReadInfo {
    #[serde(rename = "story_no")]
    pub id: u64,
    #[serde(rename = "i_expire_time")]
    pub expiry_time: Option<u64>,
    pub likes: String,
    pub my_likes: u64,
    pub post_remain: u64,
    #[serde(rename = "iap_url_list")]
    pub pages: Vec<ComicReadPage>,
    #[serde(rename = "last_page_announce_url")]
    pub last_page: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComicReadResponse {
    #[serde(rename = "iap_story_info")]
    pub info: ComicReadInfo,
    pub free_daily: Option<ComicInfoFreeDaily>,
    #[serde(rename = "iap_info")]
    pub account: IAPInfo,
}
