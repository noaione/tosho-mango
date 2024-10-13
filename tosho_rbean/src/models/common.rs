//! A module containing some common models.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

use serde::{Deserialize, Serialize};
use tosho_macros::AutoGetter;

use super::Image;

/// Creator or author of a manga.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct Creator {
    /// The name of the creator.
    name: String,
    /// The UUID of the creator.
    uuid: String,
}

/// Publisher of a manga.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct Publisher {
    /// The name of the publisher.
    name: String,
    /// The UUID of the publisher.
    uuid: String,
    /// The URL slug of the publisher.
    slug: String,
}

/// A label of a manga with UUID.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct Label {
    /// The name of the label.
    #[serde(rename = "label")]
    name: String,
    /// The UUID of the label.
    uuid: String,
}

/// A sort options filters for searching.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct SortOptions {
    /// The sort type.
    r#type: String,
    /// The sort name.
    name: String,
}

/// Tags available for searching.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct Tag {
    /// The name of the tag.
    name: String,
    /// The slug of the tag.
    slug: String,
}

/// Genres available from [`crate::models::HomeResponse`].
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct HomeGenre {
    /// The name of the genre.
    name: String,
    /// The tag of the genre.
    tag: String,
}

/// A collection of manga filters.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct MangaFilters {
    /// The sort options.
    sort_options: Vec<SortOptions>,
    /// The available tags.
    tags: Vec<Tag>,
    /// The available options that can be toggled.
    bool_options: Vec<String>,
    /// The available publishers that can be used.
    publishers: Vec<Publisher>,
}

/// A manga product.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct Product {
    /// The UUID of the product.
    uuid: String,
    /// The type of the item
    #[serde(rename = "item_type")]
    r#type: String,
    /// The retail price of the product.
    retail_price: String,
    /// The sale price of the product.
    sale_price: String,
}

/// A chapter range.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize, PartialEq)]
pub struct ChapterRange {
    /// The start of the chapter range.
    start: String,
    /// The end of the chapter range.
    end: String,
}

/// A chapter gap.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize, PartialEq)]
pub struct ChapterGap {
    /// The range of the gap.
    range: ChapterRange,
}

/// A chapter explainer, commonly used in separator.
#[derive(Debug, Clone, Copy, AutoGetter, Serialize, Deserialize, PartialEq)]
pub struct ChapterExplainer {
    /// The number of chapters specified on the separator.
    #[serde(rename = "num_chapters")]
    count: i32,
}

/// A separator for some common chapter explainer.
#[derive(Debug, Clone, Copy, AutoGetter, Serialize, Deserialize, PartialEq)]
pub struct SeparatorChapterExplainer {
    /// The index of the separator.
    #[serde(rename = "list_index")]
    index: i32,
    /// The data of the separator.
    data: ChapterExplainer,
}

/// A separator for chapter gap.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize, PartialEq)]
pub struct SeparatorChapterGap {
    /// The index of the separator.
    #[serde(rename = "list_index")]
    index: i32,
    /// The data of the separator.
    data: ChapterGap,
}

/// A separator for chapters.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum Separator {
    /// A separator for premium notice.
    #[serde(rename = "SEPARATOR_PREMIUM_NOTICE")]
    PremiumNotice(SeparatorChapterExplainer),
    /// A separator for ala carte or volume purchase notice.
    #[serde(rename = "SEPARATOR_ALC_NOTICE")]
    AlaCarteNotice(SeparatorChapterExplainer),
    /// A separator for chapter gap notice.
    #[serde(rename = "SEPARATOR_CHAPTER_GAP")]
    ChapterGap(SeparatorChapterGap),
    /// Unknown separator.
    #[serde(rename = "SEPARATOR_UNKNOWN")]
    Unknown,
}

/// A volume release product.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct Volume {
    /// The UUID of the volume.
    uuid: String,
    /// The manga UUID of the volume.
    #[serde(rename = "manga_uuid")]
    manga: String,
    /// The ISBN of the volume.
    isbn: Option<String>,
    /// The cover image of the volume.
    #[serde(rename = "image")]
    cover: Image,
    /// The title of the volume.
    #[serde(rename = "full_name")]
    title: String,
    /// The short title of the volume.
    #[serde(rename = "short_name")]
    short_title: String,
    /// The volume number of the volume.
    #[serde(rename = "label")]
    volume: String,
    /// Is DRM free
    #[serde(rename = "is_drm_free")]
    drm_free: bool,
    /// The retail/product info of the volume.
    #[serde(rename = "product", default)]
    retail: Option<Product>,
    /// The order of the volume.
    #[serde(rename = "order_number")]
    order: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_separator_premium_notice() {
        let json_test = r#"{
            "type": "SEPARATOR_PREMIUM_NOTICE",
            "list_index": 0,
            "data": {
                "num_chapters": 1
            }
        }"#;

        let separator: Separator = serde_json::from_str(json_test).unwrap();
        match separator {
            Separator::PremiumNotice(data) => {
                assert_eq!(data.index, 0);
                assert_eq!(data.data.count, 1);
            }
            _ => panic!("Invalid separator type"),
        }
    }

    #[test]
    fn test_separator_alacarte_notice() {
        let json_test = r#"{
            "type": "SEPARATOR_ALC_NOTICE",
            "list_index": 0,
            "data": {
                "num_chapters": 1
            }
        }"#;

        let separator: Separator = serde_json::from_str(json_test).unwrap();
        match separator {
            Separator::AlaCarteNotice(data) => {
                assert_eq!(data.index, 0);
                assert_eq!(data.data.count, 1);
            }
            _ => panic!("Invalid separator type"),
        }
    }

    #[test]
    fn test_separator_chapter_gap() {
        let json_test = r#"{
            "type": "SEPARATOR_CHAPTER_GAP",
            "list_index": 0,
            "data": {
                "range": {
                    "start": "1",
                    "end": "2"
                }
            }
        }"#;

        let separator: Separator = serde_json::from_str(json_test).unwrap();
        match separator {
            Separator::ChapterGap(data) => {
                assert_eq!(data.index, 0);
                assert_eq!(data.data.range.start, "1");
                assert_eq!(data.data.range.end, "2");
            }
            _ => panic!("Invalid separator type"),
        }
    }

    #[test]
    fn test_separator_unknown() {
        let json_test = r#"{
            "type": "SEPARATOR_UNKNOWN"
        }"#;

        let separator: Separator = serde_json::from_str(json_test).unwrap();
        assert_eq!(separator, Separator::Unknown)
    }
}
