//! Provides constants used in the library.
//!
//! All the following structs are a lazy static.
//!
//! ```rust
//! use tosho_kmkc::constants::get_constants;
//!
//! let _ = get_constants(2); // Android
//! ```

use std::sync::LazyLock;

use tosho_macros::comptime_b64;

const HASH_HEADER_MOBILE: &str = comptime_b64!("eC1tZ3BrLWhhc2g=");

/// A struct containing constants used in the library.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constants {
    /// The user agent string used for API requests.
    pub(crate) ua: &'static str,
    /// The user agent string used for image requests.
    pub(crate) image_ua: &'static str,
    /// The platform string used for API requests.
    pub(crate) platform: &'static str,
    /// The version string used for API requests.
    pub(crate) version: &'static str,
    /// Display version?
    pub(crate) display_version: Option<&'static str>,
    /// The hash header used for API requests.
    pub(crate) hash: &'static str,
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

/// The constants used for Android devices.
pub static ANDROID_CONSTANTS: LazyLock<Constants> = LazyLock::new(|| Constants {
    ua: "okhttp/4.9.3",
    image_ua: "okhttp/4.9.3",
    platform: "2",
    version: "6.1.0",
    display_version: Some("2.1.5"),
    hash: HASH_HEADER_MOBILE,
});
/// The constants used for iOS devices.
pub static APPLE_CONSTANTS: LazyLock<Constants> = LazyLock::new(|| {
    let hash_header = comptime_b64!("eC1tZ3BrLWhhc2g=");

    let api_ua = comptime_b64!(
        "bWFnZTItZW4vMS4yLjUgKGNvbS5rb2RhbnNoYS5rbWFuZ2E7IGJ1aWxkOjEuMi41OyBpT1MgMTcuMS4yKSBBbGFtb2ZpcmUvMS4yLjU="
    );
    let image_ua = comptime_b64!("bWFnZTItZW4vMS4yLjUgQ0ZOZXR3b3JrLzE0ODUgRGFyd2luLzIzLjEuMA==");

    Constants {
        ua: api_ua,
        image_ua,
        platform: "1",
        version: "5.3.0",
        display_version: None,
        hash: hash_header,
    }
});
/// The constants used for web devices.
pub static WEB_CONSTANTS: LazyLock<Constants> = LazyLock::new(|| {
    let hash_header = comptime_b64!("WC1LbWFuZ2EtSGFzaA==");
    let chrome_ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36";

    Constants {
        ua: chrome_ua,
        image_ua: chrome_ua,
        platform: "3",
        version: "6.0.0",
        display_version: None,
        hash: hash_header,
    }
});

/// The base API used for overall requests.
pub const BASE_API: &str = comptime_b64!("aHR0cHM6Ly9hcGkua21hbmdhLmtvZGFuc2hhLmNvbQ==");
/// The base image URL used for image requests.
pub const BASE_IMG: &str = comptime_b64!("aHR0cHM6Ly9jZG4ua21hbmdhLmtvZGFuc2hhLmNvbQ==");

/// The base host used for overall requests.
pub const BASE_HOST: &str = comptime_b64!("a21hbmdhLmtvZGFuc2hhLmNvbQ==");
/// The API host used for API requests.
pub const API_HOST: &str = comptime_b64!("YXBpLmttYW5nYS5rb2RhbnNoYS5jb20=");
/// The image host used for image requests.
pub const IMAGE_HOST: &str = comptime_b64!("Y2RuLmttYW5nYS5rb2RhbnNoYS5jb20=");

/// The ranking tabs used for the ranking endpoint.
///
/// See: [`crate::KMClient::get_all_rankings`] for more info
pub static RANKING_TABS: LazyLock<Vec<RankingTab>> = LazyLock::new(|| {
    vec![
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
    ]
});

/// Returns the constants for the given device type.
///
/// # Arguments
/// * `device_type` - The device type to get the constants for.
///
/// # Available device types
/// * `1` - Apple/iOS
/// * `2` - Android
/// * `3` - Web
///
/// # Panics
/// Panics if the device type is invalid.
///
/// # Examples
/// ```rust
/// # use tosho_kmkc::constants::get_constants;
/// #
/// let _ = get_constants(2); // Android
/// let _ = get_constants(3); // Web
/// ```
pub fn get_constants(device_type: u8) -> &'static Constants {
    match device_type {
        1 => &APPLE_CONSTANTS,
        2 => &ANDROID_CONSTANTS,
        3 => &WEB_CONSTANTS,
        _ => panic!("Invalid device type"),
    }
}
