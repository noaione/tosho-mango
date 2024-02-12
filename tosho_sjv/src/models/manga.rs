use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use super::{MangaRating, SubscriptionType};

/// A node of a single chapter information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaChapterDetail {
    pub id: u32,
    pub chapter: String,
    pub volume: Option<String>,
    pub title: Option<String>,
    #[serde(rename = "publication_date", with = "super::datetime")]
    pub published_at: DateTime<FixedOffset>,
    pub author: String,
    pub url: String,
    #[serde(rename = "thumburl")]
    pub thumbnail: String,
    pub description: String,
    #[serde(rename = "manga_series_common_id")]
    pub series_id: u32,
    pub series_title: String,
    #[serde(rename = "series_vanityurl")]
    pub series_vanity: String,
    pub series_title_sort: String,
    pub subscription_type: SubscriptionType,
    pub rating: MangaRating,
    #[serde(rename = "numpages")]
    pub pages: u32,
    #[serde(with = "super::datetime")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(with = "super::datetime")]
    pub updated_at: DateTime<FixedOffset>,
    #[serde(rename = "epoch_exp_date")]
    pub expiry_at: Option<i64>,
    pub new: bool,
}

impl MangaChapterDetail {
    /// Check if chapter can be read or downloaded
    pub fn is_available(&self) -> bool {
        self.expiry_at.is_none()
    }
}
