use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Deserializer, Serialize};

use super::{MangaImprint, MangaRating, SimpleResponse, SubscriptionType};

/// A node of a single chapter information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaChapterDetail {
    pub id: u32,
    pub chapter: Option<String>,
    pub volume: Option<u32>,
    pub title: Option<String>,
    #[serde(
        rename = "publication_date",
        deserialize_with = "super::datetime::deserialize_opt",
        serialize_with = "super::datetime::serialize_opt"
    )]
    pub published_at: Option<DateTime<FixedOffset>>,
    pub author: String,
    #[serde(rename = "thumburl")]
    pub thumbnail: Option<String>,
    pub description: String,
    #[serde(rename = "manga_series_common_id")]
    pub series_id: u32,
    pub series_title: String,
    #[serde(rename = "series_vanityurl")]
    pub series_slug: String,
    pub series_title_sort: String,
    pub subscription_type: Option<SubscriptionType>,
    pub rating: MangaRating,
    #[serde(rename = "numpages")]
    pub pages: u32,
    #[serde(with = "super::datetime")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(
        deserialize_with = "super::datetime::deserialize_opt",
        serialize_with = "super::datetime::serialize_opt"
    )]
    pub updated_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "epoch_exp_date")]
    pub expiry_at: Option<i64>,
    pub new: bool,
    pub free: bool,
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
    pub id: u32,
    pub title: String,
    pub tagline: Option<String>,
    pub synopsis: String,
    #[serde(rename = "vanityurl")]
    pub slug: String,
    pub copyright: String,
    pub rating: MangaRating,
    #[serde(rename = "link_img_url")]
    pub thumbnail: String,
    #[serde(rename = "keyart_url")]
    pub keyart: Option<String>,
    #[serde(rename = "latest_author")]
    pub author: Option<String>,
    pub title_sort: String,
    #[serde(with = "super::datetime")]
    pub updated_at: DateTime<FixedOffset>,
    pub subscription_type: Option<SubscriptionType>,
    #[serde(
        rename = "imprint_id",
        deserialize_with = "parse_imprint_extra",
        default
    )]
    pub imprint: MangaImprint,
    #[serde(rename = "num_chapters")]
    pub total_chapters: u64,
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
    #[serde(rename = "msg")]
    pub message: String,
    pub offset: f64,
    #[serde(with = "super::datetime")]
    pub show_from: DateTime<FixedOffset>,
}

/// A wrapper for [`MangaChapterDetail`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaChapterNode {
    #[serde(rename = "manga")]
    pub chapter: MangaChapterDetail,
}

/// A response for requesting manga detail or chapter list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaSeriesResponse {
    #[serde(rename = "chpt_msgs")]
    pub notices: Vec<ChapterMessage>,
    #[serde(rename = "data")]
    pub chapters: Vec<MangaChapterNode>,
}

/// A wrapper for both MangaNode and MangaChapterNode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MangaStoreInfo {
    #[serde(rename = "manga_series")]
    Manga(MangaDetail),
    #[serde(rename = "manga")]
    Chapter(MangaChapterDetail),
    #[serde(rename = "featured_section_series_id")]
    FeaturedSeriesId(Option<serde_json::Value>),
    #[serde(rename = "featured_section_series")]
    FeaturedSeries(Option<String>),
    #[serde(rename = "featured_section_title")]
    FeaturedTitle(Option<String>),
    #[serde(rename = "featured_chapter_offset_start")]
    FeaturedChapterStart(f64),
    #[serde(rename = "featured_chapter_offset_end")]
    FeaturedChapterEnd(f64),
}

/// A response for requesting cached manga list and featured data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaStoreResponse {
    #[serde(rename = "data")]
    pub contents: Vec<MangaStoreInfo>,
}

/// A response for verifying manga chapter ownership
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaAuthResponse {
    #[serde(rename = "archive_info")]
    pub info: SimpleResponse,
}

/// A response for getting URL of a manga
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaUrlResponse {
    #[serde(rename = "data", default)]
    pub url: Option<String>,
    #[serde(default)]
    pub metadata: Option<String>,
}

/// A response containing metadata of a chapter for reading
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaReadMetadataResponse {
    pub title: String,
    pub height: u32,
    pub width: u32,
    #[serde(default, rename = "hdwidth")]
    pub hd_width: Option<u32>,
    #[serde(default, rename = "hdheight")]
    pub hd_height: Option<u32>,
    // pages: Vec<_>,
    // spreads: Vec<_>,
}
