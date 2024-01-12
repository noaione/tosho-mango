use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{DevicePlatform, EpisodeBadge, GenderType, IntBool};

/// The user point information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPoint {
    /// The paid/purchased point that the user have.
    pub paid_point: u64,
    /// The free point that the user have.
    pub free_point: u64,
    /// The point sale text, currently unknown what it is.
    #[serde(rename = "point_sale_text")]
    pub point_sale: Option<String>,
    /// The point sale finish datetime string.
    #[serde(
        rename = "point_sale_finish_datetime",
        serialize_with = "super::datetime::serialize_opt",
        deserialize_with = "super::datetime::deserialize_opt"
    )]
    pub point_sale_finish: Option<DateTime<Utc>>,
}

impl UserPoint {
    /// Create a new UserPoint object.
    ///
    /// Ignore the point sale text and finish datetime.
    pub fn new(paid_point: u64, free_point: u64) -> Self {
        Self {
            paid_point,
            free_point,
            point_sale: None,
            point_sale_finish: None,
        }
    }

    /// Create a new UserPoint object with point sale text and finish datetime.
    pub fn new_with_sale(
        paid_point: u64,
        free_point: u64,
        point_sale: Option<String>,
        point_sale_finish_datetime: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            paid_point,
            free_point,
            point_sale,
            point_sale_finish: point_sale_finish_datetime,
        }
    }

    /// The total point that the user have.
    pub fn total_point(&self) -> u64 {
        self.paid_point + self.free_point
    }

    /// Check if the user can purchase a chapter.
    ///
    /// # Examples
    /// ```
    /// use tosho_kmkc::models::UserPoint;
    ///
    /// let user_point = UserPoint::new(0, 0);
    ///
    /// assert!(!user_point.can_purchase(1));
    ///
    /// let user_point = UserPoint::new(1, 0);
    /// assert!(user_point.can_purchase(1));
    /// ```
    pub fn can_purchase(&self, price: u64) -> bool {
        self.total_point() >= price
    }

    /// Mutate the [`UserPoint`] to subtract the owned point by the price.
    ///
    /// # Examples
    /// ```
    /// use tosho_kmkc::models::UserPoint;
    ///
    /// let mut user_point = UserPoint::new(10, 10);
    ///
    /// user_point.subtract(5);
    ///
    /// assert_eq!(user_point.paid_point, 10);
    /// assert_eq!(user_point.free_point, 5);
    ///
    /// user_point.subtract(10);
    ///
    /// assert_eq!(user_point.free_point, 0);
    /// assert_eq!(user_point.paid_point, 5);
    /// ```
    pub fn subtract(&mut self, price: u64) {
        if !self.can_purchase(price) {
            // silently fail
            return;
        }

        let fp_min = self.free_point.min(price);
        self.free_point -= fp_min;

        let pp_min = self.paid_point.min((price).saturating_sub(fp_min));
        self.paid_point -= pp_min;
    }

    /// Mutate the [`UserPoint`] to add a bonus point got from a chapter.
    ///
    /// # Examples
    /// ```
    /// use tosho_kmkc::models::UserPoint;
    ///
    /// let mut user_point = UserPoint::new(0, 0);
    ///
    /// user_point.add(10);
    ///
    /// assert_eq!(user_point.free_point, 10);
    /// assert_eq!(user_point.paid_point, 0);
    /// ```
    pub fn add(&mut self, bonus: u64) {
        self.free_point += bonus;
    }
}

/// The user ticket information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTicket {
    /// The ticket that the user have.
    pub total_num: u64,
}

/// Represents the user account point Response.
///
/// You should use it in combination of [`StatusResponse`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPointResponse {
    /// The user point information.
    pub point: UserPoint,
    /// The premium ticket information.
    pub ticket: UserTicket,
}

/// Title that the user favorited
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFavoriteList {
    /// The last updated time of the free episode.
    pub free_episode_updated: String,
    /// The last updated time of the paid episode.
    pub paid_episode_updated: String,
    /// Is there any unread free episode.
    pub is_unread_free_episode: IntBool,
    /// Purchase status of the manga.
    pub purchase_status: EpisodeBadge,
    /// The title ticket recover time.
    pub ticket_recover_time: String,
    /// The title ID.
    pub title_id: i32,
}

/// The device info of a user account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAccountDevice {
    /// The user ID or device ID
    #[serde(rename = "user_id")]
    pub id: u32,
    /// The device name
    #[serde(rename = "device_name")]
    pub name: String,
    /// The device platform
    pub platform: DevicePlatform,
}

/// The user account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAccount {
    /// The account ID
    #[serde(rename = "account_id")]
    pub id: u32,
    /// The account user ID
    pub user_id: u32,
    /// The user name
    #[serde(rename = "nickname")]
    pub name: String,
    /// The user email
    pub email: String,
    /// The user gender
    pub gender: GenderType,
    /// The user birth year
    #[serde(rename = "birthyear")]
    pub birth_year: i32,
    /// The list of registered devices
    #[serde(rename = "device_list")]
    pub devices: Vec<UserAccountDevice>,
    /// Whether the account is registered or not.
    #[serde(rename = "is_registerd")]
    pub registered: IntBool,
    /// The number of days since the account is registered.
    #[serde(rename = "days_since_created")]
    pub registered_days: i64,
}

/// Represents an user account response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountResponse {
    /// The user account information.
    pub account: UserAccount,
}

/// Represents the user information response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfoResponse {
    #[serde(rename = "user_id")]
    pub id: u32,
    /// The user email
    pub email: String,
    /// The user gender
    pub gender: GenderType,
    /// The user hash key
    pub hash_key: String,
}
