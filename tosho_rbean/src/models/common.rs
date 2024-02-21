//! A module containing some common models.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

use serde::{Deserialize, Serialize};

use super::Image;

/// Creator or author of a manga.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Creator {
    /// The name of the creator.
    pub name: String,
    /// The UUID of the creator.
    pub uuid: String,
}

/// Publisher of a manga.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Publisher {
    /// The name of the publisher.
    pub name: String,
    /// The UUID of the publisher.
    pub uuid: String,
    /// The URL slug of the publisher.
    pub slug: String,
}

/// A label of a manga with UUID.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    /// The name of the label.
    #[serde(rename = "label")]
    pub name: String,
    /// The UUID of the label.
    pub uuid: String,
}

/// A sort options filters for searching.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortOptions {
    /// The sort type.
    pub r#type: String,
    /// The sort name.
    pub name: String,
}

/// Tags available for searching.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    /// The name of the tag.
    pub name: String,
    /// The slug of the tag.
    pub slug: String,
}

/// Genres available from [`crate::models::HomeResponse`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomeGenre {
    /// The name of the genre.
    pub name: String,
    /// The tag of the genre.
    pub tag: String,
}

/// A collection of manga filters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaFilters {
    /// The sort options.
    pub sort_options: Vec<SortOptions>,
    /// The available tags.
    pub tags: Vec<Tag>,
    /// The available options that can be toggled.
    pub bool_options: Vec<String>,
    /// The available publishers that can be used.
    pub publishers: Vec<Publisher>,
}

/// A manga product.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    /// The UUID of the product.
    pub uuid: String,
    /// The type of the item
    #[serde(rename = "item_type")]
    pub r#type: String,
    /// The retail price of the product.
    pub retail_price: String,
    /// The sale price of the product.
    pub sale_price: String,
}

/// A chapter range.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChapterRange {
    /// The start of the chapter range.
    pub start: String,
    /// The end of the chapter range.
    pub end: String,
}

/// A chapter gap.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChapterGap {
    /// The range of the gap.
    pub range: ChapterRange,
}

/// A chapter explainer, commonly used in separator.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChapterExplainer {
    #[serde(rename = "num_chapters")]
    pub count: i32,
}

/// A separator for some common chapter explainer.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SeparatorChapterExplainer {
    /// The index of the separator.
    #[serde(rename = "list_index")]
    pub index: i32,
    /// The data of the separator.
    pub data: ChapterExplainer,
}

/// A separator for chapter gap.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SeparatorChapterGap {
    /// The index of the separator.
    #[serde(rename = "list_index")]
    pub index: i32,
    /// The data of the separator.
    pub data: ChapterGap,
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Volume {
    /// The UUID of the volume.
    pub uuid: String,
    /// The manga UUID of the volume.
    #[serde(rename = "manga_uuid")]
    pub manga: String,
    /// The ISBN of the volume.
    pub isbn: Option<String>,
    /// The cover image of the volume.
    #[serde(rename = "image")]
    pub cover: Image,
    /// The title of the volume.
    #[serde(rename = "full_name")]
    pub title: String,
    /// The short title of the volume.
    #[serde(rename = "short_name")]
    pub short_title: String,
    /// The volume number of the volume.
    #[serde(rename = "label")]
    pub volume: String,
    /// Is DRM free
    #[serde(rename = "is_drm_free")]
    pub drm_free: bool,
    /// The retail/product info of the volume.
    #[serde(rename = "product", default)]
    pub retail: Option<Product>,
    /// The order of the volume.
    #[serde(rename = "order_number")]
    pub order: i32,
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
