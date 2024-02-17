//! A module containing information related to other parts of the API.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

use serde::{Deserialize, Serialize};

use super::{IntBool, SimpleId, TitleNode};

/// The weekly list contents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeeklyListContent {
    /// The weekday index (1-7)
    #[serde(rename = "weekday_index")]
    pub weekday: i32,
    /// The list of titles.
    #[serde(rename = "title_id_list")]
    pub titles: Vec<i32>,
    /// The featured title ID.
    #[serde(rename = "feature_title_id")]
    pub featured_id: i32,
    /// The list of title with bonus point.
    #[serde(rename = "bonus_point_title_id")]
    pub bonus_titles: Vec<i32>,
    /// The list of popular titles.
    #[serde(rename = "popular_title_id_list")]
    pub popular_titles: Vec<i32>,
    /// The list of new titles.
    #[serde(rename = "new_title_id_list")]
    pub new_titles: Vec<i32>,
}

/// Represents the weekly list response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeeklyListResponse {
    /// The list of weekly list contents.
    #[serde(rename = "weekly_list")]
    pub contents: Vec<WeeklyListContent>,
    /// The list of titles associated with the weekly list.
    #[serde(rename = "title_list")]
    pub titles: Vec<TitleNode>,
}

/// Magazine category information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MagazineCategoryInfo {
    /// The magazine category ID.
    #[serde(rename = "magazine_category_id")]
    pub id: u32,
    /// The magazine category name.
    #[serde(rename = "magazine_category_name_text")]
    pub name: String,
    /// Whether the magazine is purchased or not.
    #[serde(rename = "is_purchase")]
    pub purchased: IntBool,
    /// Whether the magazine is searchable or not.
    #[serde(rename = "is_search")]
    pub searchable: IntBool,
    /// Whether the magazine is subscribable or not.
    #[serde(rename = "is_subscription")]
    pub subscribable: IntBool,
    /// The image URL of the magazine category.
    #[serde(rename = "subscription_image_url", default)]
    pub image_url: Option<String>,
}

/// Represents the magazine category response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MagazineCategoryResponse {
    /// The list of magazine categories.
    #[serde(rename = "magazine_category_list")]
    pub categories: Vec<MagazineCategoryInfo>,
}

/// A genre node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenreNode {
    /// The genre ID.
    #[serde(rename = "genre_id")]
    pub id: i32,
    /// The genre name.
    #[serde(rename = "genre_name")]
    pub name: String,
    /// The genre image URL.
    pub image_url: String,
}

/// Represents the genre search response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenreSearchResponse {
    /// The list of genres.
    #[serde(rename = "genre_list")]
    pub genres: Vec<GenreNode>,
}

/// Represents a ranking list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankingListResponse {
    /// The list of titles.
    pub titles: Vec<SimpleId>,
}
