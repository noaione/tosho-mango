use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{EpisodeBadge, FavoriteStatus, IntBool, TitleShare};

/// A node of a single episode's information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodeNode {
    /// The episode ID.
    #[serde(rename = "episode_id")]
    pub id: i32,
    /// The episode title.
    #[serde(rename = "episode_name")]
    pub title: String,
    /// The episode index.
    pub index: i32,
    /// The badge for the episode.
    pub badge: EpisodeBadge,
    /// The episode purchase point.
    pub point: i32,
    /// The episode bonus point that will be given if purchased/read.
    pub bonus_point: i32,
    /// The episode use status.
    /// TODO: Change to enum
    pub use_status: i32,
    /// The episode ticket rental status.
    #[serde(rename = "ticket_rental_enabled")]
    pub ticket_rental: IntBool,
    /// The title ID associated with the episode.
    pub title_id: i32,
    /// The episode start time or release time.
    #[serde(with = "super::datetime")]
    pub start_time: DateTime<Utc>,
    /// The episosde rental rest time.
    pub rental_rest_time: Option<String>,
    /// Magazine ID associated with the episode.
    pub magazine_id: Option<i32>,
}

impl EpisodeNode {
    /// Check whether the episode can be purchased via ticket.
    pub fn is_ticketable(&self) -> bool {
        self.ticket_rental == IntBool::True
    }

    /// Check whether the episode is available for reading.
    pub fn is_available(&self) -> bool {
        self.badge != EpisodeBadge::Purchaseable
    }

    /// Set the episode to be available for reading.
    pub fn set_available(&mut self) {
        self.badge = EpisodeBadge::Purchased;
    }
}

/// Represents the episode list response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodesListResponse {
    /// The list of episodes.
    #[serde(rename = "episode_list")]
    pub episodes: Vec<EpisodeNode>,
}

/// The node of a single image page.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImagePageNode {
    pub index: i32,
    #[serde(rename = "image_url")]
    pub url: String,
}

impl ImagePageNode {
    /// The file name of the image.
    ///
    /// # Examples
    /// ```no_run,ignore
    /// use tosho_kmkc::models::ImagePageNode;
    ///
    /// let page: ImagePageNode = ..;
    ///
    /// println!("File name: {}", page.file_name());
    pub fn file_name(&self) -> String {
        let url = self.url.as_str();
        let index = url.rfind('/').unwrap();
        let file_part = &url[index + 1..];
        // remove ?v=...
        let index = file_part.find('?').unwrap_or(file_part.len());
        file_part[..index].to_owned()
    }

    /// The file extension of the image.
    ///
    /// # Examples
    /// ```no_run,ignore
    /// use tosho_kmkc::models::ImagePageNode;
    ///
    /// let page: ImagePageNode = ..;
    ///
    /// println!("File extension: {}", page.extension());
    /// ```
    pub fn extension(&self) -> String {
        let file_name = self.file_name();
        let split: Vec<&str> = file_name.rsplitn(2, '.').collect();

        if split.len() == 2 {
            split[0].to_owned()
        } else {
            String::new()
        }
    }

    /// The file stem of the image.
    ///
    /// # Examples
    /// ```no_run,ignore
    /// use tosho_kmkc::models::ImagePageNode;
    ///
    /// let page: ImagePageNode = ..;
    ///
    /// println!("File stem: {}", page.file_stem());
    /// ```
    pub fn file_stem(&self) -> String {
        let file_name = self.file_name();
        let split: Vec<&str> = file_name.rsplitn(2, '.').collect();

        if split.len() == 2 {
            split[1].to_owned()
        } else {
            file_name
        }
    }
}

/// A simplified string version of the image page node.
#[derive(Debug, Clone)]
pub struct ImagePageNodeStr(String);

impl<'de> Deserialize<'de> for ImagePageNodeStr {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Ok(ImagePageNodeStr(s))
    }
}

impl Serialize for ImagePageNodeStr {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0)
    }
}

impl From<ImagePageNodeStr> for ImagePageNode {
    fn from(value: ImagePageNodeStr) -> Self {
        Self {
            index: 0,
            url: value.0,
        }
    }
}

impl ImagePageNodeStr {
    /// The file name of the image.
    ///
    /// # Examples
    /// ```no_run,ignore
    /// use tosho_kmkc::models::ImagePageNode;
    ///
    /// let page: ImagePageNode = ..;
    ///
    /// println!("File name: {}", page.file_name());
    pub fn file_name(&self) -> String {
        let node = ImagePageNode::from(self.clone());
        node.file_name()
    }

    /// The file extension of the image.
    ///
    /// # Examples
    /// ```no_run,ignore
    /// use tosho_kmkc::models::ImagePageNode;
    ///
    /// let page: ImagePageNode = ..;
    ///
    /// println!("File extension: {}", page.extension());
    /// ```
    pub fn extension(&self) -> String {
        let node = ImagePageNode::from(self.clone());
        node.extension()
    }

    /// The file stem of the image.
    ///
    /// # Examples
    /// ```no_run,ignore
    /// use tosho_kmkc::models::ImagePageNode;
    ///
    /// let page: ImagePageNode = ..;
    ///
    /// println!("File stem: {}", page.file_stem());
    /// ```
    pub fn file_stem(&self) -> String {
        let node = ImagePageNode::from(self.clone());
        node.file_stem()
    }
}

/// Represents the episode view response for mobile viewer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileEpisodeViewerResponse {
    /// The episode ID.
    #[serde(rename = "episode_id")]
    pub id: i32,
    /// The list of pages.
    #[serde(rename = "page_list")]
    pub pages: Vec<ImagePageNode>,
    /// The list of episodes for this titles.
    #[serde(rename = "episode_list")]
    pub episodes: Vec<EpisodeNode>,
    /// The next episode ID.
    #[serde(rename = "next_episode_id", default)]
    pub next_id: Option<i32>,
    /// The previous episode ID.
    #[serde(rename = "prev_episode_id", default)]
    pub prev_id: Option<i32>,
}

/// Represents the episode view response for web viewer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebEpisodeViewerResponse {
    /// The episode ID.
    #[serde(rename = "episode_id")]
    pub id: i32,
    /// The list of pages.
    #[serde(rename = "page_list")]
    pub pages: Vec<ImagePageNodeStr>,
    /// The bonus point of the episode.
    pub bonus_point: i32,
    /// The title ID associated with the episode.
    pub title_id: i32,
    /// The scramble seed for the epsiode
    pub scramble_seed: u32,
}

/// Represents the episode view response.
///
/// This is a combination of both mobile and web viewer response.
#[derive(Debug, Clone)]
pub enum EpisodeViewerResponse {
    /// The mobile viewer response.
    Mobile(MobileEpisodeViewerResponse),
    /// The web viewer response.
    Web(WebEpisodeViewerResponse),
}

/// Represents an episode purchase response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodePurchaseResponse {
    /// The point left on the account
    #[serde(rename = "account_point")]
    pub left: i32,
    /// The point paid for the episode
    #[serde(rename = "paid_point")]
    pub paid: i32,
}

/// Represents a bulk episode purchase response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkEpisodePurchaseResponse {
    /// The point left on the account
    #[serde(rename = "account_point")]
    pub left: i32,
    /// The point paid for the episode
    #[serde(rename = "paid_point")]
    pub paid: i32,
    /// The point earned back from the purchase
    #[serde(rename = "earned_point_back")]
    pub point_back: i32,
}

/// Represents an episode finish response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodeViewerFinishResponse {
    /// The ID of the episode.
    #[serde(rename = "episode_id")]
    pub id: u32,
    /// The title ID of the episode.
    pub title_id: i32,
    /// The favorite status of the titles.
    #[serde(rename = "favorite_status")]
    pub favorite: FavoriteStatus,
    /// The bonus point of the episode.
    pub bonus_point: i32,
    /// The bonus point of the episode.
    #[serde(rename = "view_finish_episode_count")]
    pub view_count: u64,
    #[serde(rename = "title_share_ret")]
    pub share: TitleShare,
}
