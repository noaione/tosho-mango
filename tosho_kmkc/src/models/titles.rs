use serde::{Deserialize, Serialize};

use super::{FavoriteStatus, MagazineCategory, PublishCategory, SupportStatus};

/// A single title's information.
#[derive(Debug, Serialize, Deserialize)]
pub struct TitleNode {
    /// The title ID.
    #[serde(rename = "title_id")]
    pub id: i32,
    /// The title name.
    #[serde(rename = "title_name")]
    pub title: String,
    /// The title thumbnail URL.
    #[serde(rename = "thumbnail_image_url")]
    pub thumbnail_url: String,
    /// The title square thumbnail URL.
    #[serde(rename = "thumbnail_rect_image_url")]
    pub square_thumbnail_url: String,
    /// The title feature/banner image URL
    #[serde(rename = "feature_image_url")]
    pub banner_url: String,
    /// The current active campaign text.
    pub campaign_text: String,
    /// The current notice for the title.
    #[serde(rename = "notice_text")]
    pub notice: String,
    /// The first episode ID.
    pub first_episode_id: i32,
    /// The next update text for the title.
    #[serde(rename = "next_updated_text")]
    pub next_update: Option<String>,
    /// The author of the title.
    #[serde(rename = "author_text")]
    pub author: String,
    /// The authors of the title.
    pub author_list: Vec<String>,
    /// The title's description.
    #[serde(rename = "introduction_text")]
    pub description: String,
    /// The title's summary or tagline
    #[serde(rename = "short_introduction_text")]
    pub summary: String,
    /// The update cycle for when new episodes are released.
    #[serde(rename = "new_episode_update_cycle_text")]
    pub update_cycle: String,
    /// The update cycle for when new free episodes are released.
    #[serde(rename = "free_episode_update_cycle_text")]
    pub free_update_cycle: String,
    /// The order of the episode
    pub episode_order: i32,
    /// The list of episode IDs.
    #[serde(rename = "episode_id_list")]
    pub episode_ids: Vec<i32>,
    /// The latest paid episode ID.
    #[serde(rename = "latest_paid_episode_id")]
    pub latest_episode_ids: Vec<i32>,
    /// The latest free episode ID.
    pub latest_free_episode_ids: i32,
    /// The list of genre IDs.
    #[serde(rename = "genre_id_list")]
    pub genre_ids: Vec<i32>,
    /// The favorite status of the titles.
    #[serde(rename = "favorite_status")]
    pub favorite: FavoriteStatus,
    /// The support status of the title.
    #[serde(rename = "support_status")]
    pub support: SupportStatus,
    /// The publish category of the title.
    #[serde(rename = "publish_category")]
    pub publishing: PublishCategory,
    /// The magazine category of the title.
    #[serde(rename = "magazine_category", default)]
    pub magazine: MagazineCategory,
}

/// Represents the title list response.
#[derive(Debug, Serialize, Deserialize)]
pub struct TitleListResponse {
    /// The list of titles.
    #[serde(rename = "title_list")]
    pub titles: Vec<TitleNode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    /// The list of titles.
    #[serde(rename = "title_list")]
    pub titles: Vec<TitleNode>,
    /// The list of title IDs.
    #[serde(rename = "title_id_list")]
    pub title_ids: Vec<i32>,
}
