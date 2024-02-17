//! A module containing information related to manga or chapters.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Deserializer, Serialize};

use super::{MangaImprint, MangaRating, SimpleResponse, SubscriptionType};

/// A node of a single chapter information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaChapterDetail {
    /// Chapter ID
    pub id: u32,
    /// Chapter number, if [`None`] then it's not a chapter
    pub chapter: Option<String>,
    /// Volume number, if [`None`] then it's not a volume
    pub volume: Option<u32>,
    /// Chapter title
    pub title: Option<String>,
    /// Published date
    #[serde(
        rename = "publication_date",
        deserialize_with = "super::datetime::deserialize_opt",
        serialize_with = "super::datetime::serialize_opt"
    )]
    pub published_at: Option<DateTime<FixedOffset>>,
    /// Author of the chapter
    pub author: String,
    /// Thumbnail URL
    #[serde(rename = "thumburl")]
    pub thumbnail: Option<String>,
    /// Description of the chapter
    pub description: String,
    /// Associated series ID
    #[serde(rename = "manga_series_common_id")]
    pub series_id: u32,
    /// Associated series title
    pub series_title: String,
    /// Associated series URL slug
    #[serde(rename = "series_vanityurl")]
    pub series_slug: String,
    /// Associated series sort title
    pub series_title_sort: String,
    /// Subscription type of the series
    ///
    /// If [`None`], the only way to read/download is to purchase it
    pub subscription_type: Option<SubscriptionType>,
    /// Rating of the series
    pub rating: MangaRating,
    /// Total pages of the chapter
    #[serde(rename = "numpages")]
    pub pages: u32,
    /// Date of creation or added to the API
    #[serde(with = "super::datetime")]
    pub created_at: DateTime<FixedOffset>,
    /// Date of last update
    #[serde(
        deserialize_with = "super::datetime::deserialize_opt",
        serialize_with = "super::datetime::serialize_opt"
    )]
    pub updated_at: Option<DateTime<FixedOffset>>,
    /// Date of read or download expiry
    #[serde(rename = "epoch_exp_date")]
    pub expiry_at: Option<i64>,
    /// Is this a new chapter
    pub new: bool,
    /// Is this chapter free
    pub free: bool,
    /// Is this chapter featured
    pub featured: bool,
}

impl MangaChapterDetail {
    /// Check if chapter can be read or downloaded
    pub fn is_available(&self) -> bool {
        self.expiry_at.is_none()
    }

    /// Create pretty title for the chapter
    pub fn pretty_title(&self) -> String {
        let mut text_data = String::new();
        if let Some(ref volume) = self.volume {
            text_data.push_str(&format!("Vol. {:02} ", volume));
        }
        if let Some(ref chapter) = self.chapter {
            text_data.push_str(&format!("Ch. {}", chapter));
        }
        if let Some(ref title) = self.title {
            let pretty_title = if text_data.is_empty() {
                title.clone()
            } else {
                format!(" - {}", title)
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaDetail {
    /// Series ID
    pub id: u32,
    /// Series title
    pub title: String,
    /// Series tagline
    pub tagline: Option<String>,
    /// Series synopsis
    pub synopsis: String,
    /// Series URL slug
    #[serde(rename = "vanityurl")]
    pub slug: String,
    /// Series copyright info
    pub copyright: String,
    /// Series rating
    pub rating: MangaRating,
    /// Series thumbnail URL
    #[serde(rename = "link_img_url")]
    pub thumbnail: String,
    /// Series banner URL
    #[serde(rename = "keyart_url")]
    pub keyart: Option<String>,
    /// Series author
    #[serde(rename = "latest_author")]
    pub author: Option<String>,
    /// Series title sort
    pub title_sort: String,
    /// Last updated date
    #[serde(with = "super::datetime")]
    pub updated_at: DateTime<FixedOffset>,
    /// Subscription type of the series
    ///
    /// If [`None`], the only way to read/download is to purchase it
    pub subscription_type: Option<SubscriptionType>,
    /// Imprint of the series
    #[serde(
        rename = "imprint_id",
        deserialize_with = "parse_imprint_extra",
        default
    )]
    pub imprint: MangaImprint,
    /// Total chapters of the series
    #[serde(rename = "num_chapters")]
    pub total_chapters: u64,
    /// Total volumes of the series
    #[serde(rename = "num_gns")]
    pub total_volumes: u64,
}

fn parse_imprint_extra<'de, D>(d: D) -> Result<MangaImprint, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or(MangaImprint::Undefined))
}

/// A node of a chapter notice information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterMessage {
    /// The message/notification
    #[serde(rename = "msg")]
    pub message: String,
    /// Starting chapter offset
    pub offset: f64,
    /// When the message will be shown
    #[serde(with = "super::datetime")]
    pub show_from: DateTime<FixedOffset>,
}

/// A wrapper for [`MangaChapterDetail`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaChapterNode {
    /// The chapter information
    #[serde(rename = "manga")]
    pub chapter: MangaChapterDetail,
}

/// A response for requesting manga detail or chapter list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaSeriesResponse {
    /// The notices for the chapter
    #[serde(rename = "chpt_msgs")]
    pub notices: Vec<ChapterMessage>,
    /// The data of the response
    #[serde(rename = "data")]
    pub chapters: Vec<MangaChapterNode>,
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaStoreResponse {
    /// The data of the response
    #[serde(rename = "data")]
    pub contents: Vec<MangaStoreInfo>,
}

/// A response for verifying manga chapter ownership
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaAuthResponse {
    /// The data of the response
    #[serde(rename = "archive_info")]
    pub info: SimpleResponse,
}

/// A response for getting URL of a manga
///
/// `url` will be [`None`] if you request for metadata and vice-versa
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaUrlResponse {
    /// The URL of the requested page
    #[serde(rename = "data", default)]
    pub url: Option<String>,
    /// The URL of the requested metadata
    #[serde(default)]
    pub metadata: Option<String>,
}

/// A response containing metadata of a chapter for reading
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaReadMetadataResponse {
    /// The chapter title
    pub title: String,
    /// The chapter image height
    pub height: u32,
    /// The chapter image width
    pub width: u32,
    /// The chapter image height for HD quality
    #[serde(default, rename = "hdwidth")]
    pub hd_width: Option<u32>,
    /// The chapter image width for HD quality
    #[serde(default, rename = "hdheight")]
    pub hd_height: Option<u32>,
    // pages: Vec<_>,
    // spreads: Vec<_>,
}
