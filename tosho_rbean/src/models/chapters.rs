//! A module containing information related to chapters.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tosho_macros::AutoGetter;

use super::{MangaNode, Volume};

/// A minimal model for chapter information.
///
/// Commonly used in [`crate::models::Carousel`]
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ChapterListNode {
    /// The UUID of the chapter.
    uuid: String,
    /// The chapter number/label.
    #[serde(rename = "label")]
    chapter: String,
    /// Is this a new chapter?
    #[serde(rename = "is_new")]
    new: bool,
    /// Is this an upcoming chapter?
    #[serde(rename = "is_upcoming")]
    upcoming: bool,
    /// Is this a premium chapter?
    #[serde(rename = "is_premium")]
    premium: bool,
}

/// A struct containing information about a chapter.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct Chapter {
    /// The UUID of the chapter.
    uuid: String,
    /// The chapter number/label.
    #[serde(rename = "label")]
    chapter: String,
    /// The title of the chapter.
    title: Option<String>,
    /// The release date of the chapter.
    #[serde(
        rename = "release_date",
        serialize_with = "super::datetime::serialize_opt",
        deserialize_with = "super::datetime::deserialize_opt"
    )]
    #[copyable]
    published: Option<chrono::DateTime<chrono::FixedOffset>>,
    /// The free release date of the chapter.
    #[serde(
        rename = "free_release_date",
        serialize_with = "super::datetime::serialize_opt",
        deserialize_with = "super::datetime::deserialize_opt"
    )]
    #[copyable]
    free_published: Option<chrono::DateTime<chrono::FixedOffset>>,
    /// The original published date of the chapter.
    #[serde(
        rename = "original_published_date",
        serialize_with = "super::datetime::serialize_opt",
        deserialize_with = "super::datetime::deserialize_opt"
    )]
    original_published: Option<chrono::DateTime<chrono::FixedOffset>>,
    /// Is this a new chapter?
    #[serde(rename = "is_new")]
    new: bool,
    /// Is this an upcoming chapter?
    #[serde(rename = "is_upcoming")]
    upcoming: bool,
    /// Is this a premium chapter?
    #[serde(rename = "is_premium")]
    premium: bool,
    /// Last updated date of the chapter.
    #[serde(
        rename = "last_updated_at",
        serialize_with = "super::datetime::serialize_opt",
        deserialize_with = "super::datetime::deserialize_opt"
    )]
    #[copyable]
    last_updated: Option<chrono::DateTime<chrono::FixedOffset>>,
    /// Volume UUID of the chapter.
    volume_uuid: Option<String>,
}

impl Chapter {
    /// Get a formatted chapter number with the title.
    pub fn formatted_title(&self) -> String {
        let title = self.title.as_deref().unwrap_or("");
        if title.is_empty() {
            format!("Chapter {}", self.chapter)
        } else {
            format!("Chapter {} - {}", self.chapter, title)
        }
    }
}

/// A chapter detail response.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ChapterDetailsResponse {
    /// The chapter information.
    chapter: Chapter,
    /// The manga information.
    manga: MangaNode,
}

/// A chapter list response for a manga.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ChapterListResponse {
    /// The chapters of the manga.
    chapters: Vec<Chapter>,
    /// The volume mapping of the chapters.
    ///
    /// This map the `volume_uuid` to the [`Volume`] information.
    #[serde(rename = "volume_uuid_to_volume")]
    volumes: HashMap<String, Volume>,
    /// The separators of the chapters.
    separators: Vec<super::common::Separator>,
    /// The volume UUID sort order
    #[serde(rename = "volume_uuid_order")]
    volume_order: Vec<String>,
}

/// A single spread of a chapter.
///
/// If one of them is [`None`], then it's a single page only (should not be a spread).
pub type Spread = (Option<i32>, Option<i32>);

/// A struct containing a single page information of a chapter.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ChapterPage {
    /// The UUID of the page.
    uuid: String,
    /// The image of the page.
    image: super::Image,
    /// The watermarked image of the page.
    #[serde(rename = "image_wm")]
    watermarked_image: super::Image,
    /// Is double page?
    #[serde(rename = "is_double_page")]
    double_page: bool,
    /// The index of the spread info.
    ///
    /// This is a tuple value of `(left, right)` from [`ChapterPageDetails`].
    #[serde(rename = "spread_index")]
    spread: i32,
    /// The side of the page in the spread.
    ///
    /// Either `left` or `right`, you should realize that you need to reverse the side order if
    /// using right-to-left reading mode.
    side: String,
}

/// A struct containing information about a chapter pages.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ChapterPageDetails {
    /// Spreads information mapping.
    spreads: Vec<Spread>,
    /// The pages of the chapter.
    pages: Vec<ChapterPage>,
}

/// A response for chapter pages.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ChapterPageDetailsResponse {
    /// The pages information.
    data: ChapterPageDetails,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_spreads_unpack() {
        let json_test = r#"{
            "spreads": [
                [null, 0],
                [1, 2],
                [3, 4],
                [5, null]
            ],
            "pages": []
        }"#;

        let spreads: super::ChapterPageDetails = serde_json::from_str(json_test).unwrap();
        assert_eq!(spreads.spreads.len(), 4);
        assert_eq!(spreads.spreads[0], (None, Some(0)));
        assert_eq!(spreads.spreads[1], (Some(1), Some(2)));
        assert_eq!(spreads.spreads[2], (Some(3), Some(4)));
        assert_eq!(spreads.spreads[3], (Some(5), None));
    }
}
