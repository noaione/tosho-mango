//! A module containing information related to titles/manga.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tosho_macros::AutoGetter;

use super::{FavoriteStatus, MagazineCategory, PublishCategory, SupportStatus};

/// A single title's information.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct TitleNode {
    /// The title ID.
    #[serde(rename = "title_id")]
    id: i32,
    /// The title name.
    #[serde(rename = "title_name")]
    title: String,
    /// The title thumbnail URL.
    #[serde(rename = "thumbnail_image_url")]
    thumbnail_url: String,
    /// The title square thumbnail URL.
    #[serde(rename = "thumbnail_rect_image_url")]
    square_thumbnail_url: String,
    /// The title feature/banner image URL
    #[serde(rename = "feature_image_url")]
    banner_url: String,
    /// The current active campaign text.
    campaign_text: String,
    /// The current notice for the title.
    #[serde(rename = "notice_text")]
    notice: String,
    /// The first episode ID.
    first_episode_id: i32,
    /// The next update text for the title.
    #[serde(rename = "next_updated_text")]
    next_update: Option<String>,
    /// The author of the title.
    #[serde(rename = "author_text")]
    author: String,
    /// The authors of the title.
    author_list: Vec<String>,
    /// The title's description.
    #[serde(rename = "introduction_text")]
    description: String,
    /// The title's summary or tagline
    #[serde(rename = "short_introduction_text")]
    summary: String,
    /// The update cycle for when new episodes are released.
    #[serde(rename = "new_episode_update_cycle_text")]
    update_cycle: String,
    /// The update cycle for when new free episodes are released.
    #[serde(rename = "free_episode_update_cycle_text")]
    free_update_cycle: String,
    /// The order of the episode
    episode_order: i32,
    /// The list of episode IDs.
    #[serde(rename = "episode_id_list")]
    episode_ids: Vec<i32>,
    /// The latest paid episode ID.
    #[serde(rename = "latest_paid_episode_id")]
    latest_episode_ids: Vec<i32>,
    /// The latest free episode ID.
    latest_free_episode_id: Option<i32>,
    /// The list of genre IDs.
    #[serde(rename = "genre_id_list")]
    genre_ids: Vec<i32>,
    /// The favorite status of the titles.
    #[serde(rename = "favorite_status")]
    #[copyable]
    favorite: FavoriteStatus,
    /// The support status of the title.
    #[serde(rename = "support_status")]
    #[copyable]
    support: SupportStatus,
    /// The publish category of the title.
    #[serde(rename = "publish_category")]
    #[copyable]
    publishing: PublishCategory,
    /// The magazine category of the title.
    #[serde(rename = "magazine_category", default)]
    #[copyable]
    magazine: MagazineCategory,
}

/// Represents the title list response.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct TitleListResponse {
    /// The list of titles.
    #[serde(rename = "title_list")]
    titles: Vec<TitleNode>,
}

/// Represents a search response results
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct SearchResponse {
    /// The list of titles.
    #[serde(rename = "title_list")]
    titles: Vec<TitleNode>,
    /// The list of title IDs.
    #[serde(rename = "title_id_list")]
    title_ids: Vec<i32>,
}

/// The premium ticket of a title
#[derive(Debug, Clone, Copy, AutoGetter, Serialize, Deserialize)]
pub struct PremiumTicketInfo {
    /// The number of owned premium tickets.
    #[serde(rename = "own_ticket_num")]
    owned: u64,
    /// The type of premium ticket.
    /// Using integer instead of enum because it does not have enough information.
    #[serde(rename = "ticket_type")]
    r#type: i32,
    /// The rental time of the premium ticket in seconds.
    #[serde(rename = "rental_second")]
    duration: i32,
}

/// The title ticket of a title
#[derive(Debug, Clone, Copy, AutoGetter, Serialize, Deserialize)]
pub struct TitleTicketInfo {
    /// The number of owned title tickets.
    #[serde(rename = "own_ticket_num")]
    owned: u64,
    /// The rental time of the title ticket in seconds.
    #[serde(rename = "rental_second")]
    duration: i32,
    /// The type of title ticket.
    /// Using integer instead of enum because it does not have enough information.
    #[serde(rename = "ticket_type")]
    r#type: i32,
    /// The ticket vversion of the title ticket.
    #[serde(rename = "ticket_version")]
    version: i32,
    /// The maximum title ticket you can own
    #[serde(rename = "max_ticket_num")]
    max_owned: u64,
    /// The recover time left of the title ticket.
    #[serde(rename = "recover_second")]
    recover_time: i32,
    /// The end time of the title ticket, if used.
    #[serde(rename = "finish_time")]
    end_time: Option<i32>,
    /// The next ticket recover time left of the title ticket.
    #[serde(rename = "next_ticket_recover_second")]
    next_recover_time: i32,
}

/// A ticket info for a title (either premium or title ticket).
#[derive(Debug, Clone, Copy)]
pub enum TicketInfoType {
    /// The premium ticket info.
    Premium(PremiumTicketInfo),
    /// The title ticket info.
    Title(TitleTicketInfo),
}

/// A ticket info for a title.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct TicketInfo {
    /// The premium ticket info.
    #[serde(rename = "premium_ticket_info")]
    #[copyable]
    premium: Option<PremiumTicketInfo>,
    /// The title ticket info.
    #[serde(rename = "title_ticket_info")]
    #[copyable]
    title: Option<TitleTicketInfo>,
    /// The list of applicable title IDs.
    #[serde(rename = "target_episode_id_list")]
    title_ids: Option<Vec<i32>>,
}

/// The title ticket list entry of a title.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct TitleTicketListNode {
    /// The title ID.
    #[serde(rename = "title_id")]
    id: i32,
    /// The ticket info
    #[serde(rename = "ticket_info")]
    info: TicketInfo,
}

impl TitleTicketListNode {
    /// Whether the title ticket is available.
    pub fn is_title_available(&self) -> bool {
        match self.info.title {
            Some(ref info) => info.owned > 0,
            None => false,
        }
    }

    /// Whether the premium ticket is available.
    pub fn is_premium_available(&self) -> bool {
        match self.info.premium {
            Some(ref info) => info.owned > 0,
            None => false,
        }
    }

    /// Subtract or use a title ticket.
    pub fn subtract_title(&mut self) {
        if let Some(ref mut info) = self.info.title {
            info.owned = info.owned.saturating_sub(1);
        }
    }

    /// Subtract or use a premium ticket.
    pub fn subtract_premium(&mut self) {
        if let Some(ref mut info) = self.info.premium {
            info.owned = info.owned.saturating_sub(1);
        }
    }

    /// Whether the title has any ticket type available.
    pub fn has_ticket(&self) -> bool {
        self.is_title_available() || self.is_premium_available()
    }
}

/// Represents the title ticket list response.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct TitleTicketListResponse {
    /// The list of title ticket list entries.
    #[serde(rename = "title_ticket_list")]
    tickets: Vec<TitleTicketListNode>,
}

/// A title node from a title purchase response.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct TitlePurchaseNode {
    /// The title ID.
    #[serde(rename = "title_id")]
    id: i32,
    /// The title name.
    #[serde(rename = "title_name")]
    title: String,
    /// The title thumbnail URL.
    #[serde(rename = "thumbnail_image_url")]
    thumbnail_url: String,
    /// The first episode ID.
    first_episode_id: i32,
    /// The ID of the recently purchased episode.
    #[serde(rename = "recently_purchased_episode_id")]
    recent_purchase_id: i32,
}

/// Represents the title purchase response.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct TitlePurchaseResponse {
    /// The list of title nodes.
    #[serde(rename = "title_list")]
    titles: Vec<TitlePurchaseNode>,
}

/// Title sharing data
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct TitleShare {
    /// The title name.
    #[serde(rename = "title_name")]
    title: String,
    /// The title social media post text.
    #[serde(rename = "twitter_post_text")]
    post_text: String,
    /// The share URL.
    url: String,
}

/// Favorite title node
#[derive(Debug, Clone, Copy, AutoGetter, Serialize, Deserialize)]
#[auto_getters(unref = true)]
pub struct TitleFavoriteNode {
    /// The title ID.
    #[serde(rename = "title_id")]
    id: i32,
    /// The update cycle for when new episodes are released.
    #[serde(
        rename = "paid_episode_updated",
        serialize_with = "super::datetime::serialize_opt",
        deserialize_with = "super::datetime::deserialize_opt"
    )]
    update_cycle: Option<DateTime<Utc>>,
    /// The update cycle for when new free episodes are released.
    #[serde(
        rename = "free_episode_updated",
        serialize_with = "super::datetime::serialize_opt",
        deserialize_with = "super::datetime::deserialize_opt"
    )]
    free_update_cycle: Option<DateTime<Utc>>,
    /// The title purchase status.
    purchase_status: i32,
    /// The title ticket recover time.
    #[serde(
        serialize_with = "super::datetime::serialize_opt",
        deserialize_with = "super::datetime::deserialize_opt"
    )]
    ticket_recover_time: Option<DateTime<Utc>>,
}

/// Represents the user favorite title list response.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct TitleFavoriteResponse {
    /// The list of favorite title nodes.
    #[serde(rename = "favorite_title_list")]
    favorites: Vec<TitleFavoriteNode>,
    /// The list of title nodes.
    #[serde(rename = "title_list")]
    titles: Vec<TitleNode>,
    /// Favorite count
    #[serde(rename = "favorite_num")]
    count: u64,
    /// Maximum favorite count
    #[serde(rename = "max_favorite_num")]
    max_count: u64,
}
