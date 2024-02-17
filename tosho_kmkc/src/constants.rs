//! Provides constants used in the library.
//!
//! All the following structs are a lazy static.
//!
//! ```rust
//! use tosho_kmkc::constants::get_constants;
//!
//! let _ = get_constants(2); // Android
//! ```

use base64::{engine::general_purpose, Engine as _};
use lazy_static::lazy_static;

/// A struct containing constants used in the library.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constants {
    /// The user agent string used for API requests.
    pub(crate) ua: String,
    /// The user agent string used for image requests.
    pub(crate) image_ua: String,
    /// The platform string used for API requests.
    pub(crate) platform: &'static str,
    /// The version string used for API requests.
    pub(crate) version: &'static str,
    /// The hash header used for API requests.
    pub(crate) hash: String,
}

/// A ranking tab for KM.
#[derive(Debug, Clone)]
pub struct RankingTab {
    /// The ID of the ranking tab.
    pub id: u32,
    /// The name of the ranking tab.
    pub name: &'static str,
    /// The tab name used in the choice list.
    pub tab: &'static str,
}

impl RankingTab {
    fn new(id: u32, name: &'static str, tab: &'static str) -> Self {
        Self { id, name, tab }
    }
}

lazy_static! {
    /// The constants used for Android devices.
    pub static ref ANDROID_CONSTANTS: Constants = {
        let hash_header = String::from_utf8(
            general_purpose::STANDARD
                .decode("eC1tZ3BrLWhhc2g=")
                .expect("Failed to decode base64 ANDROID_HASH_HEADER"),
        )
        .expect("Invalid base64 string");

        Constants {
            ua: "okhttp/4.9.3".to_string(),
            image_ua: "okhttp/4.9.3".to_string(),
            platform: "2",
            version: "5.8.0",
            hash: hash_header,
        }
    };
    /// The constants used for iOS devices.
    pub static ref IOS_CONSTANTS: Constants = {
        let hash_header = String::from_utf8(
            general_purpose::STANDARD
                .decode("eC1tZ3BrLWhhc2g=")
                .expect("Failed to decode base64 IOS_HASH_HEADER"),
        )
        .expect("Invalid base64 string");

        let api_ua = String::from_utf8(
            general_purpose::STANDARD
                .decode("bWFnZTItZW4vMS4yLjUgKGNvbS5rb2RhbnNoYS5rbWFuZ2E7IGJ1aWxkOjEuMi41OyBpT1MgMTcuMS4yKSBBbGFtb2ZpcmUvMS4yLjU=")
                .expect("Failed to decode base64 IOS_API_UA"),
        )
        .expect("Invalid base64 string");
        let image_ua = String::from_utf8(
            general_purpose::STANDARD
                .decode("bWFnZTItZW4vMS4yLjUgQ0ZOZXR3b3JrLzE0ODUgRGFyd2luLzIzLjEuMA==")
                .expect("Failed to decode base64 IOS_IMAGE_UA"),
        )
        .expect("Invalid base64 string");

        Constants {
            ua: api_ua,
            image_ua,
            platform: "1",
            version: "5.3.0",
            hash: hash_header,
        }
    };
    /// The constants used for web devices.
    pub static ref WEB_CONSTANTS: Constants = {
        let hash_header = String::from_utf8(
            general_purpose::STANDARD
                .decode("WC1LbWFuZ2EtSGFzaA==")
                .expect("Failed to decode base64 WEB_HASH_HEADER"),
        )
        .expect("Invalid base64 string");

        let chrome_ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36".to_string();

        Constants {
            ua: chrome_ua.clone(),
            image_ua: chrome_ua,
            platform: "3",
            version: "6.0.0",
            hash: hash_header,
        }
    };

    /// The base API used for overall requests.
    pub static ref BASE_API: String = {
        String::from_utf8(
            general_purpose::STANDARD
                .decode("aHR0cHM6Ly9hcGkua21hbmdhLmtvZGFuc2hhLmNvbQ==")
                .expect("Failed to decode base64 BASE_API")
        )
        .expect("Invalid base64 string (BASE_API)")
    };
    /// The base image URL used for image requests.
    pub static ref BASE_IMG: String = {
        String::from_utf8(
            general_purpose::STANDARD
                .decode("aHR0cHM6Ly9jZG4ua21hbmdhLmtvZGFuc2hhLmNvbQ==")
                .expect("Failed to decode base64 BASE_IMG")
        )
        .expect("Invalid base64 string (BASE_IMG)")
    };

    /// The base host used for overall requests.
    pub static ref BASE_HOST: String = {
        String::from_utf8(
            general_purpose::STANDARD
                .decode("a21hbmdhLmtvZGFuc2hhLmNvbQ==")
                .expect("Failed to decode base64 BASE_HOST")
        )
        .expect("Invalid base64 string (BASE_HOST)")
    };
    /// The API host used for API requests.
    pub(crate) static ref API_HOST: String = {
        String::from_utf8(
            general_purpose::STANDARD
                .decode("YXBpLmttYW5nYS5rb2RhbnNoYS5jb20=")
                .expect("Failed to decode base64 API_HOST")
        )
        .expect("Invalid base64 string (API_HOST)")
    };
    /// The image host used for image requests.
    pub(crate) static ref IMAGE_HOST: String = {
        String::from_utf8(
            general_purpose::STANDARD
                .decode("Y2RuLmttYW5nYS5rb2RhbnNoYS5jb20=")
                .expect("Failed to decode base64 IMAGE_HOST")
        )
        .expect("Invalid base64 string (IMAGE_HOST)")
    };

    /// The ranking tabs used for the ranking endpoint.
    ///
    /// See: [`crate::KMClient::get_all_rankings`] for more info
    pub static ref RANKING_TABS: Vec<RankingTab> = vec![
        RankingTab::new(3, "Action", "action"),
        RankingTab::new(4, "Sports", "sports"),
        RankingTab::new(5, "Romance", "romance"),
        RankingTab::new(6, "Isekai", "isekai"),
        RankingTab::new(7, "Suspense", "romance"),
        RankingTab::new(8, "Outlaws", "outlaws"),
        RankingTab::new(9, "Drama", "drama"),
        RankingTab::new(10, "Fantasy", "fantasy"),
        RankingTab::new(11, "Slice of Life", "sol"),
        RankingTab::new(12, "All", "all"),
        RankingTab::new(13, "Today's Specials", "specials"),
    ];
}

/// Returns the constants for the given device type.
///
/// # Arguments
/// * `device_type` - The device type to get the constants for.
///
/// # Panics
/// Panics if the device type is invalid.
///
/// # Examples
/// ```
/// use tosho_kmkc::constants::get_constants;
///
/// let _ = get_constants(2); // Android
/// let _ = get_constants(3); // Web
/// ```
pub fn get_constants(device_type: u8) -> &'static Constants {
    match device_type {
        1 => &IOS_CONSTANTS,
        2 => &ANDROID_CONSTANTS,
        3 => &WEB_CONSTANTS,
        _ => panic!("Invalid device type"),
    }
}
