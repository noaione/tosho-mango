//! A module containing information related to user account.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tosho_macros::AutoGetter;

use super::{DevicePlatform, EpisodeBadge, GenderType, IntBool};

/// The user point information
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct UserPoint {
    /// The paid/purchased point that the user have.
    paid_point: u64,
    /// The free point that the user have.
    free_point: u64,
    /// The point sale text, currently unknown what it is.
    #[serde(rename = "point_sale_text")]
    point_sale: Option<String>,
    /// The point sale finish datetime string.
    #[serde(
        rename = "point_sale_finish_datetime",
        serialize_with = "super::datetime::serialize_opt",
        deserialize_with = "super::datetime::deserialize_opt"
    )]
    #[copyable]
    point_sale_finish: Option<DateTime<Utc>>,
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
    /// ```rust
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
    /// ```rust
    /// use tosho_kmkc::models::UserPoint;
    ///
    /// let mut user_point = UserPoint::new(10, 10);
    ///
    /// user_point.subtract(5);
    ///
    /// assert_eq!(user_point.paid_point(), 10);
    /// assert_eq!(user_point.free_point(), 5);
    ///
    /// user_point.subtract(10);
    ///
    /// assert_eq!(user_point.free_point(), 0);
    /// assert_eq!(user_point.paid_point(), 5);
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
    /// ```rust
    /// use tosho_kmkc::models::UserPoint;
    ///
    /// let mut user_point = UserPoint::new(0, 0);
    ///
    /// user_point.add(10);
    ///
    /// assert_eq!(user_point.free_point(), 10);
    /// assert_eq!(user_point.paid_point(), 0);
    /// ```
    pub fn add(&mut self, bonus: u64) {
        self.free_point += bonus;
    }
}

/// The user ticket information.
#[derive(Debug, Clone, Copy, AutoGetter, Serialize, Deserialize)]
pub struct UserTicket {
    /// The ticket that the user have.
    total_num: u64,
}

/// Represents the user account point Response.
///
/// You should use it in combination of [`crate::models::StatusResponse`].
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct UserPointResponse {
    /// The user point information.
    point: UserPoint,
    /// The premium ticket information.
    #[copyable]
    ticket: UserTicket,
}

/// Title that the user favorited
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct UserFavoriteList {
    /// The last updated time of the free episode.
    free_episode_updated: String,
    /// The last updated time of the paid episode.
    paid_episode_updated: String,
    /// Is there any unread free episode.
    #[copyable]
    is_unread_free_episode: IntBool,
    /// Purchase status of the manga.
    #[copyable]
    purchase_status: EpisodeBadge,
    /// The title ticket recover time.
    ticket_recover_time: String,
    /// The title ID.
    title_id: i32,
}

/// The device info of a user account
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct UserAccountDevice {
    /// The user ID or device ID
    #[serde(rename = "user_id")]
    id: u32,
    /// The device name
    #[serde(rename = "device_name")]
    name: String,
    /// The device platform
    #[copyable]
    platform: DevicePlatform,
}

/// The user account information
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct UserAccount {
    /// The account ID
    #[serde(rename = "account_id")]
    id: u32,
    /// The account user ID
    user_id: u32,
    /// The user name
    #[serde(rename = "nickname")]
    name: Option<String>,
    /// The user email
    email: String,
    /// The user gender
    #[copyable]
    gender: Option<GenderType>,
    /// The user birth year
    #[serde(rename = "birthyear")]
    birth_year: Option<i32>,
    /// The list of registered devices
    #[serde(rename = "device_list")]
    devices: Vec<UserAccountDevice>,
    /// Whether the account is registered or not.
    #[serde(rename = "is_registerd")]
    #[copyable]
    registered: IntBool,
    /// The number of days since the account is registered.
    #[serde(rename = "days_since_created")]
    registered_days: i64,
}

/// Represents an user account response.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct AccountResponse {
    /// The user account information.
    account: UserAccount,
}

/// Represents the user information response.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct UserInfoResponse {
    /// The user ID
    #[serde(rename = "user_id")]
    id: u32,
    /// The user email
    email: String,
    /// The user gender
    #[copyable]
    gender: Option<GenderType>,
    /// The user hash key
    hash_key: String,
    /// The user UUID
    original_uuid: Option<String>,
}
