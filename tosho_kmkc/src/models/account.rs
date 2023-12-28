use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{EpisodeBadge, IntBool};

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
    #[serde(rename = "point_sale_finish_datetime")]
    point_sale_finish: Option<String>,
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
        point_sale_finish_datetime: Option<String>,
    ) -> Self {
        Self {
            paid_point,
            free_point,
            point_sale,
            point_sale_finish: point_sale_finish_datetime,
        }
    }

    /// The point sale finish datetime.
    ///
    /// # Examples
    /// ```
    /// use tosho_kmkc::models::UserPoint;
    /// use chrono::Datelike;
    ///
    /// let user_point = UserPoint::new(0, 0);
    ///
    /// assert!(user_point.point_sale_finish().is_none());
    ///
    /// let user_point = UserPoint::new_with_sale(
    ///     0,
    ///     0,
    ///     None,
    ///     Some("2021-01-01 00:00:00".to_string()),
    /// );
    ///
    /// assert_eq!(user_point.point_sale_finish().unwrap().year(), 2021);
    /// ```
    pub fn point_sale_finish(&self) -> Option<DateTime<Utc>> {
        // Formatted YYYY-MM-DD HH:MM:SS
        // Assume UTC
        match &self.point_sale_finish {
            Some(s) => {
                let naive_dt = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").ok()?;
                let utc_dt = naive_dt.and_utc();
                Some(utc_dt)
            }
            None => None,
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
#[derive(Debug, Serialize, Deserialize)]
pub struct UserTicket {
    /// The ticket that the user have.
    pub total_num: u64,
}

/// Represents the user account point Response.
///
/// You should use it in combination of [`StatusResponse`].
#[derive(Debug, Serialize, Deserialize)]
pub struct UserPointResponse {
    /// The user point information.
    pub point: UserPoint,
    /// The premium ticket information.
    pub ticket: UserTicket,
}

/// Title that the user favorited
#[derive(Debug, Serialize, Deserialize)]
pub struct UserFavoriteList {
    /// The last updated time of the free episode.
    free_episode_updated: String,
    /// The last updated time of the paid episode.
    paid_episode_updated: String,
    /// Is there any unread free episode.
    is_unread_free_episode: IntBool,
    /// Purchase status of the manga.
    purchase_status: EpisodeBadge,
    /// The title ticket recover time.
    ticket_recover_time: String,
    /// The title ID.
    title_id: i32,
}
