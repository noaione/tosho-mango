//! A module containing information related to manga or chapters.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Deserializer, Serialize};
use tosho_macros::AutoGetter;

use super::{MangaImprint, MangaRating, SimpleResponse, SubscriptionType};

/// A node of a single chapter information.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct MangaChapterDetail {
    /// Chapter ID
    id: u32,
    /// Chapter number, if [`None`] then it's not a chapter
    chapter: Option<String>,
    /// Volume number, if [`None`] then it's not a volume
    volume: Option<u32>,
    /// Chapter title
    title: Option<String>,
    /// Published date
    #[serde(
        rename = "publication_date",
        deserialize_with = "super::datetime::deserialize_opt",
        serialize_with = "super::datetime::serialize_opt"
    )]
    #[copyable]
    published_at: Option<DateTime<FixedOffset>>,
    /// Author of the chapter
    author: String,
    /// Thumbnail URL
    #[serde(rename = "thumburl")]
    thumbnail: Option<String>,
    /// Description of the chapter
    description: String,
    /// Associated series ID
    #[serde(rename = "manga_series_common_id")]
    series_id: u32,
    /// Associated series title
    series_title: String,
    /// Associated series URL slug
    #[serde(rename = "series_vanityurl")]
    series_slug: String,
    /// Associated series sort title
    series_title_sort: String,
    /// Subscription type of the series
    ///
    /// If [`None`], the only way to read/download is to purchase it
    #[copyable]
    subscription_type: Option<SubscriptionType>,
    /// Rating of the series
    #[copyable]
    rating: MangaRating,
    /// Total pages of the chapter
    #[serde(rename = "numpages")]
    pages: u32,
    /// Date of creation or added to the API
    #[serde(with = "super::datetime")]
    #[copyable]
    created_at: DateTime<FixedOffset>,
    /// Date of last update
    #[serde(
        deserialize_with = "super::datetime::deserialize_opt",
        serialize_with = "super::datetime::serialize_opt"
    )]
    #[copyable]
    updated_at: Option<DateTime<FixedOffset>>,
    /// Date of read or download expiry
    #[serde(rename = "epoch_exp_date")]
    expiry_at: Option<i64>,
    /// Is this a new chapter
    new: bool,
    /// Is this chapter free
    free: bool,
    /// Is this chapter featured
    featured: bool,
    /// Start page of the chapter
    #[serde(rename = "contents_start_page")]
    start_page: u32,
}

impl MangaChapterDetail {
    /// Check if chapter can be read or downloaded
    pub fn is_available(&self) -> bool {
        if let Some(expiry_at) = self.expiry_at {
            let now = chrono::Utc::now().timestamp();
            now < expiry_at
        } else {
            true
        }
    }

    /// Create pretty title for the chapter
    pub fn pretty_title(&self) -> String {
        let mut text_data = String::new();
        if let Some(ref volume) = self.volume {
            text_data.push_str(&format!("Vol. {volume:02} "));
        }
        if let Some(ref chapter) = self.chapter {
            text_data.push_str(&format!("Ch. {chapter}"));
        }
        if let Some(ref title) = self.title {
            let pretty_title = if text_data.is_empty() {
                title.clone()
            } else {
                format!(" - {title}")
            };
            text_data.push_str(&pretty_title);
        }

        if text_data.is_empty() {
            text_data = format!("ID: {}", self.id);
        }

        text_data.trim().to_string()
    }
}

/// A node of a single series information.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct MangaDetail {
    /// Series ID
    id: u32,
    /// Series title
    title: String,
    /// Series tagline
    tagline: Option<String>,
    /// Series synopsis
    synopsis: String,
    /// Series URL slug
    #[serde(rename = "vanityurl")]
    slug: String,
    /// Series copyright info
    copyright: Option<String>,
    /// Series rating
    #[copyable]
    rating: MangaRating,
    /// Series thumbnail URL
    #[serde(rename = "link_img_url")]
    thumbnail: String,
    /// Series banner URL
    #[serde(rename = "keyart_url")]
    keyart: Option<String>,
    /// Series author
    #[serde(rename = "latest_author")]
    author: Option<String>,
    /// Series title sort
    title_sort: String,
    /// Last updated date
    #[serde(with = "super::datetime")]
    #[copyable]
    updated_at: DateTime<FixedOffset>,
    /// Subscription type of the series
    ///
    /// If [`None`], the only way to read/download is to purchase it
    #[copyable]
    subscription_type: Option<SubscriptionType>,
    /// Imprint of the series
    #[serde(
        rename = "imprint_id",
        deserialize_with = "parse_imprint_extra",
        default
    )]
    #[copyable]
    imprint: MangaImprint,
    /// Total chapters of the series
    #[serde(rename = "num_chapters")]
    total_chapters: u64,
    /// Total volumes of the series
    #[serde(rename = "num_gns")]
    total_volumes: u64,
}

fn parse_imprint_extra<'de, D>(d: D) -> Result<MangaImprint, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or(MangaImprint::Undefined))
}

/// A node of a chapter notice information.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ChapterMessage {
    /// The message/notification
    #[serde(rename = "msg")]
    message: String,
    /// Starting chapter offset
    offset: f64,
    /// When the message will be shown
    #[serde(
        default,
        deserialize_with = "super::datetime::deserialize_opt",
        serialize_with = "super::datetime::serialize_opt"
    )]
    #[copyable]
    show_from: Option<DateTime<FixedOffset>>,
    /// When the message will stop being shown
    #[serde(
        default,
        deserialize_with = "super::datetime::deserialize_opt",
        serialize_with = "super::datetime::serialize_opt"
    )]
    #[copyable]
    show_to: Option<DateTime<FixedOffset>>,
}

impl ChapterMessage {
    /// Check if the message is currently active
    pub fn is_active(&self) -> bool {
        let now = chrono::Utc::now();

        if let Some(show_from) = self.show_from {
            if now < show_from {
                return false;
            }
        }

        if let Some(show_to) = self.show_to {
            if now > show_to {
                return false;
            }
        }

        true
    }
}

/// A wrapper for [`MangaChapterDetail`]
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct MangaChapterNode {
    /// The chapter information
    #[serde(rename = "manga")]
    chapter: MangaChapterDetail,
}

/// A response for requesting manga detail or chapter list
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct MangaSeriesResponse {
    /// The notices for the chapter
    #[serde(rename = "chpt_msgs")]
    notices: Vec<ChapterMessage>,
    /// The data of the response
    #[serde(rename = "data")]
    chapters: Vec<MangaChapterNode>,
}

/// A wrapper for both MangaNode and MangaChapterNode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MangaStoreInfo {
    /// A manga series information
    #[serde(rename = "manga_series")]
    Manga(MangaDetail),
    /// A manga chapter information
    #[serde(rename = "manga")]
    Chapter(MangaChapterDetail),
    /// The current featured series ID,
    /// This is either a string value `_not_defined_` or a number value which is series ID
    #[serde(rename = "featured_section_series_id")]
    FeaturedSeriesId(Option<serde_json::Value>),
    /// The current featured series
    ///
    /// Can be `_not_defined_` which means no featured title
    #[serde(rename = "featured_section_series")]
    FeaturedSeries(Option<String>),
    /// The current featured section title
    ///
    /// Can be `_not_defined_` which means no featured title
    #[serde(rename = "featured_section_title")]
    FeaturedTitle(Option<String>),
    /// The featured chapter start offset
    ///
    /// If < 0, then it's not defined
    #[serde(rename = "featured_chapter_offset_start")]
    FeaturedChapterStart(f64),
    /// The featured chapter end offset
    ///
    /// If < 0, then it's not defined
    #[serde(rename = "featured_chapter_offset_end")]
    FeaturedChapterEnd(f64),
}

/// A response for requesting cached manga list and featured data
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct MangaStoreResponse {
    /// The data of the response
    #[serde(rename = "data")]
    contents: Vec<MangaStoreInfo>,
}

/// A response for verifying manga chapter ownership
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct MangaAuthResponse {
    /// The data of the response
    #[serde(rename = "archive_info")]
    info: SimpleResponse,
}

/// A response for getting URL of a manga
///
/// `url` will be [`None`] if you request for metadata and vice-versa
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct MangaUrlResponse {
    /// The URL of the requested page
    #[serde(rename = "data", default)]
    url: Option<String>,
    /// The URL of the requested metadata
    #[serde(default)]
    metadata: Option<String>,
}

/// A response containing metadata of a chapter for reading
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct MangaReadMetadataResponse {
    /// The chapter title
    title: String,
    /// The chapter image height
    height: u32,
    /// The chapter image width
    width: u32,
    /// The chapter image height for HD quality
    #[serde(default, rename = "hdwidth")]
    hd_width: Option<u32>,
    /// The chapter image width for HD quality
    #[serde(default, rename = "hdheight")]
    hd_height: Option<u32>,
    // pages: Vec<_>,
    // spreads: Vec<_>,
}
