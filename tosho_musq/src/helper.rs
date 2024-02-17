//! Provides a collection of helper Structs that can be used.
//!
//! ```rust
//! use tosho_musq::{WeeklyCode, ImageQuality, ConsumeCoin};
//!
//! let today = WeeklyCode::today();
//! let hq_img = ImageQuality::High;
//! let coins = ConsumeCoin::new(1, 2, 3, 4);
//!
//! assert!(coins.is_possible());
//! ```

use std::str::FromStr;

use chrono::Datelike;
use serde::{Deserialize, Serialize};

/// Weekly code for manga updates.
///
/// Used with [`crate::MUClient::get_weekly_titles`] to get manga updates for each week.
///
/// # Example
/// ```no_run
/// use tosho_musq::{constants::get_constants, MUClient};
///
/// let client = MUClient::new("123456", get_constants(1));
///
/// let weekly_titles = client.get_weekly_titles(tosho_musq::WeeklyCode::today());
/// ```
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    tosho_macros::SerializeEnum,
    tosho_macros::DeserializeEnum,
    tosho_macros::EnumName,
)]
pub enum WeeklyCode {
    /// Monday
    Monday,
    /// Tuesday
    Tuesday,
    /// Wednesday
    Wednesday,
    /// Thursday
    Thursday,
    /// Friday
    Friday,
    /// Saturday
    Saturday,
    /// Sunday
    Sunday,
}

tosho_macros::enum_error!(WeeklyCodeFromStrError);

impl FromStr for WeeklyCode {
    type Err = WeeklyCodeFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "monday" | "mon" => Ok(WeeklyCode::Monday),
            "tuesday" | "tue" => Ok(WeeklyCode::Tuesday),
            "wednesday" | "wed" => Ok(WeeklyCode::Wednesday),
            "thursday" | "thu" => Ok(WeeklyCode::Thursday),
            "friday" | "fri" => Ok(WeeklyCode::Friday),
            "saturday" | "sat" => Ok(WeeklyCode::Saturday),
            "sunday" | "sun" => Ok(WeeklyCode::Sunday),
            _ => Err(WeeklyCodeFromStrError {
                original: s.to_string(),
            }),
        }
    }
}

impl ToString for WeeklyCode {
    fn to_string(&self) -> String {
        match self {
            WeeklyCode::Monday => "mon".to_string(),
            WeeklyCode::Tuesday => "tue".to_string(),
            WeeklyCode::Wednesday => "wed".to_string(),
            WeeklyCode::Thursday => "thu".to_string(),
            WeeklyCode::Friday => "fri".to_string(),
            WeeklyCode::Saturday => "sat".to_string(),
            WeeklyCode::Sunday => "sun".to_string(),
        }
    }
}

impl WeeklyCode {
    /// Get the current day of the week and return
    /// the corresponding [``WeeklyCode``] enum.
    ///
    /// # Example
    /// ```
    /// use tosho_musq::helper::WeeklyCode;
    ///
    /// let today = WeeklyCode::today();
    /// ```
    pub fn today() -> WeeklyCode {
        let today = chrono::Local::now().weekday();

        match today {
            chrono::Weekday::Mon => WeeklyCode::Monday,
            chrono::Weekday::Tue => WeeklyCode::Tuesday,
            chrono::Weekday::Wed => WeeklyCode::Wednesday,
            chrono::Weekday::Thu => WeeklyCode::Thursday,
            chrono::Weekday::Fri => WeeklyCode::Friday,
            chrono::Weekday::Sat => WeeklyCode::Saturday,
            chrono::Weekday::Sun => WeeklyCode::Sunday,
        }
    }

    /// Get the zero-based index of the day of the week.
    ///
    /// # Example
    /// ```
    /// use tosho_musq::helper::WeeklyCode;
    ///
    /// let monday = WeeklyCode::Monday;
    /// assert_eq!(monday.get_index(), 0);
    /// ```
    pub fn get_index(&self) -> usize {
        match self {
            WeeklyCode::Monday => 0,
            WeeklyCode::Tuesday => 1,
            WeeklyCode::Wednesday => 2,
            WeeklyCode::Thursday => 3,
            WeeklyCode::Friday => 4,
            WeeklyCode::Saturday => 5,
            WeeklyCode::Sunday => 6,
        }
    }
}

/// The image quality to be downloaded.
///
/// Used with [`crate::MUClient::get_chapter_images`] to select image quality.
///
/// # Example
/// ```no_run
/// use tosho_musq::{constants::get_constants, MUClient, ImageQuality};
///
/// let client = MUClient::new("123456", get_constants(1));
///
/// let chapter_images = client.get_chapter_images(12345, ImageQuality::Normal, None);
/// ```
#[derive(
    Debug, Clone, Copy, PartialEq, tosho_macros::SerializeEnum, tosho_macros::DeserializeEnum,
)]
pub enum ImageQuality {
    /// Normal quality images
    Normal,
    /// High quality images
    High,
}

impl FromStr for ImageQuality {
    type Err = WeeklyCodeFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "normal" | "middle" | "standard" => Ok(ImageQuality::Normal),
            "high" => Ok(ImageQuality::High),
            _ => Err(WeeklyCodeFromStrError {
                original: s.to_string(),
            }),
        }
    }
}

impl ToString for ImageQuality {
    fn to_string(&self) -> String {
        match self {
            ImageQuality::Normal => "middle".to_string(),
            ImageQuality::High => "high".to_string(),
        }
    }
}

/// A custom struct to store and handle the coins needed to purchase chapter.
///
/// Every attribute will default to 0.
///
/// # Example
/// ```
/// use tosho_musq::ConsumeCoin;
///
/// let coins = ConsumeCoin::new(1, 2, 3, 4);
/// assert_eq!(coins.is_possible(), true);
/// ```
#[derive(Debug, Clone)]
pub struct ConsumeCoin {
    /// The free coins used to get this chapter.
    free: u64,
    /// The event coins used to get this chapter.
    event: u64,
    /// The paid coins used to get this chapter.
    paid: u64,
    /// The total coins needed to get this chapter.
    need: u64,
}

impl ConsumeCoin {
    pub fn new(free: u64, event: u64, paid: u64, need: u64) -> Self {
        Self {
            free,
            event,
            paid,
            need,
        }
    }

    /// Check if the chapter can be purchased.
    ///
    /// # Example
    /// ```
    /// use tosho_musq::ConsumeCoin;
    ///
    /// let coins = ConsumeCoin::new(1, 2, 3, 4);
    /// assert_eq!(coins.is_possible(), true);
    ///
    /// let coins = ConsumeCoin::new(1, 2, 3, 10);
    /// assert_eq!(coins.is_possible(), false);
    /// ```
    pub fn is_possible(&self) -> bool {
        self.free + self.event + self.paid >= self.need
    }

    /// Check if the chapter is free.
    ///
    /// # Example
    /// ```
    /// use tosho_musq::ConsumeCoin;
    ///
    /// let coins = ConsumeCoin::new(1, 2, 3, 4);
    /// assert_eq!(coins.is_free(), false);
    ///
    /// let coins = ConsumeCoin::new(4, 0, 0, 0);
    /// assert_eq!(coins.is_free(), true);
    /// ```
    pub fn is_free(&self) -> bool {
        self.need == 0
    }

    // Accessors

    /// Get the free coins used to get this chapter.
    pub fn get_free(&self) -> u64 {
        self.free
    }

    /// Get the event coins used to get this chapter.
    pub fn get_event(&self) -> u64 {
        self.event
    }

    /// Get the paid coins used to get this chapter.
    pub fn get_paid(&self) -> u64 {
        self.paid
    }

    /// Get the total coins needed to get this chapter.
    pub fn get_need(&self) -> u64 {
        self.need
    }

    // Mutators

    /// Get a mutable reference to the free coins used to get this chapter.
    pub fn get_free_mut(&mut self) -> &mut u64 {
        &mut self.free
    }

    /// Get a mutable reference to the event coins used to get this chapter.
    pub fn get_event_mut(&mut self) -> &mut u64 {
        &mut self.event
    }

    /// Get a mutable reference to the paid coins used to get this chapter.
    pub fn get_paid_mut(&mut self) -> &mut u64 {
        &mut self.paid
    }

    /// Get a mutable reference to the total coins needed to get this chapter.
    pub fn get_need_mut(&mut self) -> &mut u64 {
        &mut self.need
    }
}

impl Default for ConsumeCoin {
    fn default() -> Self {
        Self::new(0, 0, 0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consume_free() {
        let cc = ConsumeCoin::default();
        assert!(cc.is_free());
        assert!(cc.is_possible());
    }

    #[test]
    fn test_consume_possible() {
        let cc = ConsumeCoin::new(20, 0, 0, 10);
        assert!(cc.is_possible());
    }

    #[test]
    fn test_consume_impossible() {
        let cc = ConsumeCoin::new(0, 0, 0, 10);
        assert!(!cc.is_possible());
    }
}
