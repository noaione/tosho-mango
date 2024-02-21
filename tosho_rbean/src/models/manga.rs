//! A module containing information related to manga.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

use serde::{Deserialize, Serialize};

use super::{Chapter, ChapterListNode, Creator, HomeGenre, Image, Publisher, Tag};

/// Reading modes available for a manga.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReadingModes {
    /// Is the manga available in single page mode?
    #[serde(rename = "single")]
    pub single_page: bool,
    /// Is the manga available in double page mode?
    #[serde(rename = "spread")]
    pub double_page: bool,
    /// Is the manga available in vertical mode?
    pub vertical: bool,
}

/// Alternative titles of a manga.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternativeTitles {
    /// The alternative title of the manga.
    #[serde(rename = "name")]
    pub title: String,
    /// The language of the alternative title.
    pub locale: String,
    /// The order of the alternative title.
    #[serde(rename = "order_number")]
    pub order: i32,
    /// Is the alternative title will be promoted/shown in the UI?
    #[serde(rename = "is_promoted")]
    pub promoted: bool,
}

/// A struct containing information about a manga.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manga {
    /// The UUID of the manga.
    pub uuid: String,
    /// The title of the manga.
    #[serde(rename = "name")]
    pub title: String,
    /// The URL slug of the manga.
    pub slug: String,
    /// The description of the manga.
    #[serde(rename = "short_description")]
    pub description: String,
    /// The cover image of the manga.
    #[serde(rename = "image")]
    pub cover: Image,
    /// The tags of the manga.
    pub tags: Vec<String>,
    /// Publisher of the manga.
    pub publisher: Publisher,
    /// Creator/authors of the manga.
    pub creators: Vec<Creator>,
    /// Translation credits or publication credits
    #[serde(default)]
    pub credits: Option<String>,
    /// Release schedule of the manga.
    #[serde(default)]
    pub release_schedule: Option<String>,
    /// Genres of the manga.
    pub genres: Vec<Tag>,
    /// Reading modes available for the manga.
    #[serde(default)]
    pub reading_modes: ReadingModes,
    /// Alternative titles of the manga.
    #[serde(rename = "alt_titles", default)]
    pub alternative_titles: Vec<AlternativeTitles>,
    /// Total available chapters
    #[serde(rename = "total_available_chapters")]
    pub chapters: u32,
    /// Total available chapters that can be purchased (ala carte)
    #[serde(rename = "alc_available_chapters")]
    pub purchaseables: u32,
    /// Total premium available chapters
    #[serde(rename = "premium_available_chapters")]
    pub premium_chapters: u32,
    /// Total free available chapters
    #[serde(rename = "free_available_chapters")]
    pub free_chapters: u32,
    /// Is pass eligible?
    #[serde(rename = "is_pass_eligible")]
    pub pass_eligible: bool,
    /// Total available passes
    #[serde(rename = "total_passes")]
    pub passes: Option<i32>,
    /// Pass recharge in hours
    #[serde(rename = "pass_recharge_hours")]
    pub pass_recharge: Option<i32>,
    /// How long can the chapters be read with the pass
    #[serde(rename = "pass_unlock_hours")]
    pub pass_unlock: Option<i32>,
    /// Is the manga read from left to right?
    pub is_ltr: bool,
    /// Last updated date of the manga.
    #[serde(rename = "last_updated_at", with = "super::datetime")]
    pub last_updated: chrono::DateTime<chrono::FixedOffset>,
    /// Amazon affiliate link
    #[serde(default)]
    pub amazon_affiliate: Option<String>,
}

/// A minimal version of [`Manga`] struct.
///
/// Commonly used in search result and reading list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaNode {
    /// The UUID of the manga.
    pub uuid: String,
    /// The title of the manga.
    #[serde(rename = "name")]
    pub title: String,
    /// The URL slug of the manga.
    pub slug: String,
    /// The description of the manga.
    #[serde(rename = "short_description")]
    pub description: String,
    /// The cover image of the manga.
    #[serde(rename = "image")]
    pub cover: Image,
    /// The tags of the manga.
    pub tags: Vec<String>,
    /// Publisher UUID of the manga.
    pub publisher_uuid: String,
    /// Total available chapters
    #[serde(rename = "total_available_chapters")]
    pub chapters: u32,
    /// Total available chapters that can be purchased (ala carte)
    #[serde(rename = "alc_available_chapters")]
    pub purchaseables: u32,
    /// Total premium available chapters
    #[serde(rename = "premium_available_chapters")]
    pub premium_chapters: u32,
    /// Total free available chapters
    #[serde(rename = "free_available_chapters")]
    pub free_chapters: u32,
    /// Is pass eligible?
    #[serde(rename = "is_pass_eligible")]
    pub pass_eligible: bool,
    /// Total available passes
    #[serde(rename = "total_passes")]
    pub passes: Option<i32>,
    /// Pass recharge in hours
    #[serde(rename = "pass_recharge_hours")]
    pub pass_recharge: Option<i32>,
    /// How long can the chapters be read with the pass
    #[serde(rename = "pass_unlock_hours")]
    pub pass_unlock: Option<i32>,
    /// Is the manga read from left to right?
    pub is_ltr: bool,
    /// Last updated date of the manga.
    #[serde(rename = "last_updated_at", with = "super::datetime")]
    pub last_updated: chrono::DateTime<chrono::FixedOffset>,
    /// Amazon affiliate link
    #[serde(default)]
    pub amazon_affiliate: Option<String>,
    /// Latest available chapters
    #[serde(default)]
    pub latest_chapters: Option<Vec<ChapterListNode>>,
}

/// A struct containing search results of a manga.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaListResponse {
    /// The total count of the search result.
    #[serde(rename = "total_count")]
    pub total: String,
    /// The search results of the manga.
    #[serde(rename = "mangas")]
    pub results: Vec<MangaNode>,
}

/// A struct containing information about hero banner
///
/// Used in [`HomeResponse`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeroBanner {
    /// The banner title
    pub title: String,
    /// The banner subtitle
    pub subtitle: String,
    /// The banner alt text
    pub alt_text: String,
    /// The banner/cover image
    pub image: String,
    /// The background image of the manga cover
    pub background: Image,
    /// The link to the manga
    pub link: Option<String>,
    /// The manga info, if [`None`] there is no featured manga
    pub manga: Option<MangaNode>,
}

/// A struct containing information about the featured manga/chapter
///
/// Used in [`HomeResponse`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeaturedManga {
    /// The feature label/title
    #[serde(rename = "label")]
    pub title: String,
    /// The description of the featured manga
    pub description: String,
    /// The manga info
    pub manga: MangaNode,
}

/// A node for the carousel continue reading items
///
/// Used in [`HomeResponse`] at [`CarouselContinueReading`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarouselContinueReadingNode {
    /// The manga info
    pub manga: MangaNode,
    /// The chapter info
    pub chapter: Chapter,
}

/// A struct containing information about continue reading carousel
///
/// Used in [`HomeResponse`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarouselContinueReading {
    /// The carousel title
    #[serde(rename = "label")]
    pub title: String,
    /// The carousel items
    #[serde(rename = "mangas")]
    pub items: Vec<CarouselContinueReadingNode>,
}

/// A struct containing information about common carousels
///
/// Used in [`HomeResponse`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarouselCommon {
    /// The carousel title
    #[serde(rename = "label")]
    pub title: String,
    /// The carousel items
    #[serde(rename = "mangas")]
    pub items: Vec<MangaNode>,
}

/// A struct containing information about carousels in the home page
///
/// Used in [`HomeResponse`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Carousel {
    /// Continue reading carousel
    #[serde(rename = "series_list_continue")]
    ContinueReading(CarouselContinueReading),
    /// Carousel with a list of manga that will always have [`MangaNode::latest_chapters`] filled
    #[serde(rename = "series_list_chapters")]
    MangaWithChapters(CarouselCommon),
    /// Carousel with a list of manga
    #[serde(rename = "manga_list_standard")]
    MangaList(CarouselCommon),
}

/// A struct containing information about the home page response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomeResponse {
    /// The hero banner
    pub hero: HeroBanner,
    /// The featured manga
    pub featured: Vec<FeaturedManga>,
    /// The carousel items
    pub carousels: Vec<Carousel>,
    /// The genres available
    pub genres: Vec<HomeGenre>,
}
