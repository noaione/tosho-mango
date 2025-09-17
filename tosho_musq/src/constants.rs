//! Provides constants used in the library.
//!
//! All the following structs are a lazy static.
//!
//! ```rust
//! use tosho_musq::constants::get_constants;
//!
//! let _ = get_constants(1); // Android
//! ```

use std::sync::LazyLock;

use tosho_macros::comptime_b64;

/// A struct containing constants used in the library.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constants {
    /// The user agent string used for image requests.
    pub(crate) image_ua: String,
    /// The user agent string used for API requests.
    pub(crate) api_ua: String,
    /// The OS version string used for API requests.
    pub(crate) os_ver: &'static str,
    /// The app version string used for API requests.
    pub(crate) app_ver: String,
}

/// The constants used for Android devices.
pub static ANDROID_CONSTANTS: LazyLock<Constants> = LazyLock::new(|| {
    let android_app_ver = "73"; // 2.7.0

    Constants {
        image_ua: "Dalvik/2.1.0 (Linux; U; Android 12; SM-G935F Build/SQ3A.220705.004)".to_string(),
        api_ua: "okhttp/4.12.0".to_string(),
        os_ver: "32", // Android SDK 12
        app_ver: android_app_ver.to_string(),
    }
});
/// The constants used for Apple/iOS devices.
pub static APPLE_CONSTANTS: LazyLock<Constants> = LazyLock::new(|| {
    let ios_app = comptime_b64!("Y29tLnNxdWFyZS1lbml4Lk1hbmdhVVB3");
    let ios_app_pre = comptime_b64!("R2xlbndvb2RfUHJvZA==");
    let ios_app_post = comptime_b64!("QWxhbW9maXJlLzUuNy4x");

    let ios_app_ver = "2.6.1";
    let ios_app_build = "202411251121";

    Constants {
        image_ua: format!("{ios_app_pre}/{ios_app_build} CFNetwork/1410.0.3 Darwin/22.6.0"),
        api_ua: format!(
            "{ios_app_pre}/{ios_app_ver} ({ios_app}; build:{ios_app_build}; iOS 16.7.0) {ios_app_post}"
        ),
        os_ver: "16.7",
        app_ver: ios_app_ver.to_string(),
    }
});
/// The constants used for Web devices.
pub static WEB_CONSTANTS: LazyLock<Constants> = LazyLock::new(|| Constants {
    image_ua: "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:133.0) Gecko/20100101 Firefox/133.0"
        .to_string(),
    api_ua: "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:133.0) Gecko/20100101 Firefox/133.0"
        .to_string(),
    os_ver: "0",
    app_ver: "0".to_string(),
});

/// The base API used for overall requests.
pub const BASE_API: &str = comptime_b64!("aHR0cHM6Ly9nbG9iYWwtYXBpLm1hbmdhLXVwLmNvbS9hcGk=");
/// The base image URL used for image requests.
pub const BASE_IMG: &str = comptime_b64!("aHR0cHM6Ly9nbG9iYWwtaW1nLm1hbmdhLXVwLmNvbQ==");

/// The base host used for overall requests.
pub const BASE_HOST: &str = comptime_b64!("Z2xvYmFsLm1hbmdhLXVwLmNvbQ==");
/// The API host used for API requests.
pub const API_HOST: &str = comptime_b64!("Z2xvYmFsLWFwaS5tYW5nYS11cC5jb20=");
/// The image host used for image requests.
pub const IMAGE_HOST: &str = comptime_b64!("Z2xvYmFsLWltZy5tYW5nYS11cC5jb20=");

/// Returns the constants for the given device type.
///
/// # Arguments
/// * `device_type` - The device type to get the constants for.
///
/// # Available device types
/// * `1` - Android
/// * `2` - Apple/iOS
/// * `3` - Web
///
/// # Panics
/// Panics if the device type is invalid.
///
/// # Examples
/// ```rust
/// # use tosho_musq::constants::get_constants;
/// #
/// let _ = get_constants(1); // Android
/// let _ = get_constants(2); // Apple
/// let _ = get_constants(3); // Web
/// ```
pub fn get_constants(device_type: u8) -> &'static Constants {
    match device_type {
        1 => &ANDROID_CONSTANTS,
        2 => &APPLE_CONSTANTS,
        3 => &WEB_CONSTANTS,
        _ => panic!("Invalid device type"),
    }
}
