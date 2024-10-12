//! A module containing information related to comic/manga.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

use serde::{Deserialize, Serialize};
use tosho_macros::AutoGetter;

use super::{ComicStatus, IAPInfo};

/// A simple comic information node used in search and discovery.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ComicSimpleInfoNode {
    /// The comic ID
    #[serde(rename = "manga_sele_id")]
    id: u64,
    /// The comic title
    title: String,
    /// The comic last update in UNIX timestamp
    update_date: Option<u64>,
    /// The comic cover URL
    cover_url: String,
    /// Is there any new update?
    #[serde(rename = "new_flg")]
    new_update: bool,
}

/// Wrapper for [`ComicSimpleInfoNode`]
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ComicSimpleInfo {
    /// The comic information
    #[serde(rename = "comic_info")]
    info: ComicSimpleInfoNode,
}

/// The current banner information for the comic discovery.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ComicBannerInfoNode {
    /// The comic ID
    #[serde(rename = "manga_sele_id")]
    id: u64,
    /// The comic banner URL
    #[serde(rename = "url")]
    cover_url: String,
}

/// Wrapper for [`ComicBannerInfoNode`]
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ComicBannerInfo {
    /// The comic banner information
    #[serde(rename = "banner")]
    info: ComicBannerInfoNode,
}

/// The comic discovery header information.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ComicDiscoveryHeader {
    /// The comic discovery header ID
    title: String,
    /// The comic discovery tag ID if it's a tag
    tag_id: Option<u64>,
    /// Is the comic discovery is a completed comic?
    complete: Option<u64>, // what?
}

/// The comic discovery node.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ComicDiscoveryNode {
    /// The discovery node header
    header: ComicDiscoveryHeader,
    /// The discovery node comics
    #[serde(rename = "comic_info_list")]
    comics: Vec<ComicSimpleInfo>,
}

/// The paginated response for comic discovery.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ComicDiscoveryPaginatedResponse {
    /// The discovery node comics
    #[serde(rename = "comic_info_list")]
    comics: Vec<ComicSimpleInfo>,
    /// Is there any next page?
    next_page: bool,
}

/// The search response for comic discovery.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ComicSearchResponse {
    /// The discovery node comics
    #[serde(rename = "comic_info_list")]
    comics: Vec<ComicSimpleInfo>,
    /// The total count of the search results
    total_count: String,
    /// Is there any next page?
    next_page: bool,
}

/// The comic discovery response.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ComicDiscovery {
    /// The comic discovery banners
    #[serde(rename = "manga_banner_list")]
    banners: Vec<ComicBannerInfo>,
    /// The comic discovery updated comics
    updated: Vec<ComicDiscoveryNode>,
    /// The comic discovery free campaigns
    free_campaigns: Vec<ComicDiscoveryNode>,
    /// The comic discovery (first tags, random tags)
    #[serde(rename = "tag_contents1")]
    tags1: Vec<ComicDiscoveryNode>,
    /// The comic discovery (second tags, random tags)
    #[serde(rename = "tag_contents2")]
    tags2: Vec<ComicDiscoveryNode>,
    /// The comic discovery completed comics
    completed: Vec<ComicDiscoveryNode>,
}

/// The comic free daily ticket usage information.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ComicInfoFreeDaily {
    /// The next free daily time in UNIX timestamp
    #[serde(rename = "next_free_daily_time")]
    next: u64,
    /// The free daily term, used when requesting for the comic viewer
    #[serde(rename = "free_daily_term")]
    term: String,
}

/// A single comic episode information node.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ComicEpisodeInfoNode {
    /// The episode ID
    #[serde(rename = "story_no")]
    id: u64,
    /// The episode title
    title: String,
    /// The episode price
    #[serde(rename = "i_price")]
    price: u64,
    /// The episode update date in UNIX timestamp
    #[serde(rename = "update_timestamp")]
    update_date: u64,
    /// The episode thumbnail URL
    thumbnail: String,
    /// The episode likes
    likes: String,
    /// The episode comments
    comments: String,
    /// The episode total page count
    #[serde(rename = "total_page_count")]
    page_count: u64,
    /// The episode start status
    #[serde(rename = "page_start_status")]
    start_status: i32,
    /// Does the episode can be purchased with the free daily ticket?
    #[serde(rename = "is_free_daily_episode")]
    is_free_daily: bool,
    /// The episode free campaign end date in UNIX timestamp
    campaign_end_at: Option<u64>,
    /// The episode expiry time in UNIX timestamp
    #[serde(rename = "i_expire_time")]
    expiry_time: Option<u64>,
    close_time: Option<u64>, // ???
    /// The volume that includes the episode
    #[serde(rename = "included_volume")]
    included_in: Option<String>,
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

/// Wrapper for [`ComicEpisodeInfoNode`]
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ComicEpisodeInfo {
    /// The episode information
    #[serde(rename = "comic_body_info")]
    info: ComicEpisodeInfoNode,
}

/// Author information node on a series/comic.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ComicAuthorInfoNode {
    /// The author ID
    #[serde(rename = "a_id")]
    id: u64,
    /// The author name
    #[serde(rename = "a_name")]
    name: String,
    /// The author kind (e.g. "Author", "Illustrator")
    #[serde(rename = "disp_a_kind")]
    kind: String,
    /// The author description
    #[serde(rename = "a_comment")]
    description: Option<String>,
}

/// Wrapper for [`ComicAuthorInfoNode`]
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ComicAuthorInfo {
    /// The author information
    #[serde(rename = "author_info")]
    info: ComicAuthorInfoNode,
}

/// Tag information node on a series/comic.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ComicTagInfoNode {
    /// The tag ID
    #[serde(rename = "tag_id")]
    id: u64,
    /// The tag name
    #[serde(rename = "tag_name")]
    name: String,
}

/// Wrapper for [`ComicTagInfoNode`]
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ComicTagInfo {
    /// The tag information
    #[serde(rename = "tag_info")]
    info: ComicTagInfoNode,
}

/// A complete comic information.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ComicInfo {
    /// The comic title
    title: String,
    /// The comic description
    #[serde(rename = "shoukai")]
    description: String,
    /// The comic last update in UNIX timestamp, if any
    update_date: Option<u64>,
    /// The comic next update in UNIX timestamp
    next_update_date: Option<u64>,
    /// The comic cover URL
    cover_url: String,
    /// The comic thumbnail URL
    thumbnail_url: String,
    /// The comic web outgoing URL
    #[serde(rename = "cont_url")]
    web_url: Option<String>,
    /// The comic episodes list
    #[serde(rename = "comic_body_info_list")]
    episodes: Vec<ComicEpisodeInfo>,
    /// The comic next update text
    next_update_text: Option<String>,
    /// Is the comic a favorite?
    favorite: bool,
    /// The comic rental term (used when requesting for the comic viewer)
    rental_term: Option<String>,
    /// The comic authors list
    #[serde(rename = "author_info_list")]
    authors: Vec<ComicAuthorInfo>,
    /// The comic tags list
    #[serde(rename = "tag_info_list")]
    tags: Vec<ComicTagInfo>,
    /// The comic likes
    likes: String,
    /// The comic comments
    comments: String,
    /// The comic status
    #[serde(rename = "complete")]
    status: ComicStatus,
    /// The comic production participants (usually the Translator)
    #[serde(rename = "production_participants")]
    productions: String,
    /// Is the comic has episode that can use the free daily ticket?
    #[serde(rename = "is_free_daily")]
    has_free_daily: bool,
    /// The comic free daily ticket information, if any.
    free_daily: Option<ComicInfoFreeDaily>,
}

/// A single volume information node.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ComicVolumeBookInfoNode {
    /// The volume title
    title: String,
    /// The volume cover URL
    cover_url: String,
    /// The volume detail URL, usually outgoing link to Amazon (shortened with x.gd)
    detail_url: String,
}

/// Wrapper for [`ComicVolumeBookInfoNode`]
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ComicVolumeBookInfo {
    /// The volume information
    #[serde(rename = "book_info")]
    info: ComicVolumeBookInfoNode,
}

/// The comic information response.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ComicInfoResponse {
    /// The comic information
    #[serde(rename = "comic_info")]
    info: ComicInfo,
    /// The comic volumes list
    #[serde(rename = "book_info_list")]
    volumes: Vec<ComicVolumeBookInfo>,
    /// The in-app purchase information
    #[serde(rename = "iap_info")]
    account: IAPInfo,
}

/// The comic read page information node.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ComicReadPageNode {
    /// The page URL
    url: String,
}

/// Wrapper for [`ComicReadPageNode`]
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ComicReadPage {
    /// The page information
    #[serde(rename = "iap_url_info")]
    info: ComicReadPageNode,
}

/// The comic read information.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ComicReadInfo {
    /// The episode ID
    #[serde(rename = "story_no")]
    id: u64,
    /// The episode expiry time in UNIX timestamp
    #[serde(rename = "i_expire_time")]
    expiry_time: Option<u64>,
    /// The episode likes
    likes: String,
    /// My likes on the episode
    my_likes: u64,
    post_remain: u64, // ???
    /// The episode pages list
    #[serde(rename = "iap_url_list")]
    pages: Vec<ComicReadPage>,
    /// The episode last page URL
    #[serde(rename = "last_page_announce_url")]
    last_page: Option<String>,
}

/// The comic read response.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ComicReadResponse {
    /// The episode information
    #[serde(rename = "iap_story_info")]
    info: ComicReadInfo,
    /// The free daily ticket information, if any.
    free_daily: Option<ComicInfoFreeDaily>,
    /// The in-app purchase information
    #[serde(rename = "iap_info")]
    account: IAPInfo,
}
