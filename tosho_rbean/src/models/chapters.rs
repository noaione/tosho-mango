//! A module containing information related to chapters.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{MangaNode, Volume};

/// A minimal model for chapter information.
///
/// Commonly used in [`crate::models::Carousel`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterListNode {
    /// The UUID of the chapter.
    pub uuid: String,
    /// The chapter number/label.
    #[serde(rename = "label")]
    pub chapter: String,
    /// Is this a new chapter?
    #[serde(rename = "is_new")]
    pub new: bool,
    /// Is this an upcoming chapter?
    #[serde(rename = "is_upcoming")]
    pub upcoming: bool,
    /// Is this a premium chapter?
    #[serde(rename = "is_premium")]
    pub premium: bool,
}

/// A struct containing information about a chapter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    /// The UUID of the chapter.
    pub uuid: String,
    /// The chapter number/label.
    #[serde(rename = "label")]
    pub chapter: String,
    /// The title of the chapter.
    pub title: String,
    /// The release date of the chapter.
    #[serde(rename = "release_date", with = "super::datetime")]
    pub published: chrono::DateTime<chrono::FixedOffset>,
    /// The free release date of the chapter.
    #[serde(
        rename = "free_release_date",
        serialize_with = "super::datetime::serialize_opt",
        deserialize_with = "super::datetime::deserialize_opt"
    )]
    pub free_published: Option<chrono::DateTime<chrono::FixedOffset>>,
    /// The original published date of the chapter.
    #[serde(
        rename = "original_published_date",
        serialize_with = "super::datetime::serialize_opt",
        deserialize_with = "super::datetime::deserialize_opt"
    )]
    pub original_published: Option<chrono::DateTime<chrono::FixedOffset>>,
    /// Is this a new chapter?
    #[serde(rename = "is_new")]
    pub new: bool,
    /// Is this an upcoming chapter?
    #[serde(rename = "is_upcoming")]
    pub upcoming: bool,
    /// Is this a premium chapter?
    #[serde(rename = "is_premium")]
    pub premium: bool,
    /// Last updated date of the chapter.
    #[serde(
        rename = "last_updated_at",
        serialize_with = "super::datetime::serialize_opt",
        deserialize_with = "super::datetime::deserialize_opt"
    )]
    pub last_updated: Option<chrono::DateTime<chrono::FixedOffset>>,
    /// Volume UUID of the chapter.
    pub volume_uuid: Option<String>,
}

/// A chapter detail response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterDetailsResponse {
    /// The chapter information.
    pub chapter: Chapter,
    /// The manga information.
    pub manga: MangaNode,
}

/// A chapter list response for a manga.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterListResponse {
    /// The chapters of the manga.
    pub chapters: Vec<Chapter>,
    /// The volume mapping of the chapters.
    ///
    /// This map the `volume_uuid` to the [`Volume`] information.
    #[serde(rename = "volume_uuid_to_volume")]
    pub volumes: HashMap<String, Volume>,
    /// The separators of the chapters.
    pub separators: Vec<super::common::Separator>,
    /// The volume UUID sort order
    #[serde(rename = "volume_uuid_order")]
    pub volume_order: Vec<String>,
}
