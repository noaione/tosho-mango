use serde::{Deserialize, Serialize};

use super::{FavoriteStatus, MagazineCategory, PublishCategory, SupportStatus};

/// A single title's information.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub latest_free_episode_id: i32,
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleListResponse {
    /// The list of titles.
    #[serde(rename = "title_list")]
    pub titles: Vec<TitleNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    /// The list of titles.
    #[serde(rename = "title_list")]
    pub titles: Vec<TitleNode>,
    /// The list of title IDs.
    #[serde(rename = "title_id_list")]
    pub title_ids: Vec<i32>,
}

/// The premium ticket of a title
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PremiumTicketInfo {
    /// The number of owned premium tickets.
    #[serde(rename = "own_ticket_num")]
    pub owned: u64,
    /// The type of premium ticket.
    /// Using integer instead of enum because it does not have enough information.
    #[serde(rename = "ticket_type")]
    pub r#type: i32,
    /// The rental time of the premium ticket in seconds.
    #[serde(rename = "rental_second")]
    pub duration: i32,
}

/// The title ticket of a title
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleTicketInfo {
    /// The number of owned title tickets.
    #[serde(rename = "own_ticket_num")]
    pub owned: u64,
    /// The rental time of the title ticket in seconds.
    #[serde(rename = "rental_second")]
    pub duration: i32,
    /// The type of title ticket.
    /// Using integer instead of enum because it does not have enough information.
    #[serde(rename = "ticket_type")]
    pub r#type: i32,
    /// The ticket vversion of the title ticket.
    #[serde(rename = "ticket_version")]
    pub version: i32,
    /// The maximum title ticket you can own
    #[serde(rename = "max_ticket_num")]
    pub max_owned: u64,
    /// The recover time left of the title ticket.
    #[serde(rename = "recover_second")]
    pub recover_time: i32,
    /// The end time of the title ticket, if used.
    #[serde(rename = "finish_time")]
    pub end_time: Option<i32>,
    /// The next ticket recover time left of the title ticket.
    #[serde(rename = "next_ticket_recover_second")]
    pub next_recover_time: i32,
}

/// A ticket info for a title (either premium or title ticket).
#[derive(Debug, Clone)]
pub enum TicketInfoType {
    /// The premium ticket info.
    Premium(PremiumTicketInfo),
    /// The title ticket info.
    Title(TitleTicketInfo),
}

/// A ticket info for a title.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketInfo {
    /// The premium ticket info.
    #[serde(rename = "premium_ticket_info")]
    pub premium: Option<PremiumTicketInfo>,
    /// The title ticket info.
    #[serde(rename = "title_ticket_info")]
    pub title: Option<TitleTicketInfo>,
    /// The list of applicable title IDs.
    #[serde(rename = "target_episode_id_list")]
    pub title_ids: Option<Vec<i32>>,
}

/// The title ticket list entry of a title.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleTicketListNode {
    /// The title ID.
    #[serde(rename = "title_id")]
    pub id: i32,
    /// The ticket info
    #[serde(rename = "ticket_info")]
    pub info: TicketInfo,
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleTicketListResponse {
    /// The list of title ticket list entries.
    #[serde(rename = "title_ticket_list")]
    pub tickets: Vec<TitleTicketListNode>,
}

/// A title node from a title purchase response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitlePurchaseNode {
    /// The title ID.
    #[serde(rename = "title_id")]
    pub id: i32,
    /// The title name.
    #[serde(rename = "title_name")]
    pub title: String,
    /// The title thumbnail URL.
    #[serde(rename = "thumbnail_image_url")]
    pub thumbnail_url: String,
    /// The first episode ID.
    pub first_episode_id: i32,
    /// The ID of the recently purchased episode.
    #[serde(rename = "recently_purchased_episode_id")]
    pub recent_purchase_id: i32,
}

/// Represents the title purchase response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitlePurchaseResponse {
    /// The list of title nodes.
    #[serde(rename = "title_list")]
    pub titles: Vec<TitlePurchaseNode>,
}

/// Title sharing data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleShare {
    #[serde(rename = "title_name")]
    title: String,
    #[serde(rename = "twitter_post_text")]
    post_text: String,
    url: String,
}
