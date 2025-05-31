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

use base64::{Engine as _, engine::general_purpose};

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
    let ios_app = String::from_utf8(
        general_purpose::STANDARD
            .decode("Y29tLnNxdWFyZS1lbml4Lk1hbmdhVVB3")
            .expect("Failed to decode base64 IOS_APP"),
    )
    .expect("Invalid base64 string (IOS_APP)");
    let ios_app_pre = String::from_utf8(
        general_purpose::STANDARD
            .decode("R2xlbndvb2RfUHJvZA==")
            .expect("Failed to decode base64 IOS_APP_PRE"),
    )
    .expect("Invalid base64 string (IOS_APP_PRE)");
    let ios_app_post = String::from_utf8(
        general_purpose::STANDARD
            .decode("QWxhbW9maXJlLzUuNy4x")
            .expect("Failed to decode base64 IOS_APP_POST"),
    )
    .expect("Invalid base64 string (IOS_APP_POST)");

    let ios_app_ver = "2.6.1";
    let ios_app_build = "202411251121";

    Constants {
        image_ua: format!(
            "{}/{} CFNetwork/1410.0.3 Darwin/22.6.0",
            ios_app_pre, ios_app_build
        ),
        api_ua: format!(
            "{}/{} ({}; build:{}; iOS 16.7.0) {}",
            ios_app_pre, ios_app_ver, ios_app, ios_app_build, ios_app_post
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
pub static BASE_API: LazyLock<String> = LazyLock::new(|| {
    String::from_utf8(
        general_purpose::STANDARD
            .decode("aHR0cHM6Ly9nbG9iYWwtYXBpLm1hbmdhLXVwLmNvbS9hcGk=")
            .expect("Failed to decode base64 BASE_API"),
    )
    .expect("Invalid base64 string (BASE_API)")
});
/// The base image URL used for image requests.
pub static BASE_IMG: LazyLock<String> = LazyLock::new(|| {
    String::from_utf8(
        general_purpose::STANDARD
            .decode("aHR0cHM6Ly9nbG9iYWwtaW1nLm1hbmdhLXVwLmNvbQ==")
            .expect("Failed to decode base64 BASE_IMG"),
    )
    .expect("Invalid base64 string (BASE_IMG)")
});

/// The base host used for overall requests.
pub static BASE_HOST: LazyLock<String> = LazyLock::new(|| {
    String::from_utf8(
        general_purpose::STANDARD
            .decode("Z2xvYmFsLm1hbmdhLXVwLmNvbQ==")
            .expect("Failed to decode base64 BASE_HOST"),
    )
    .expect("Invalid base64 string (BASE_HOST)")
});
/// The API host used for API requests.
pub static API_HOST: LazyLock<String> = LazyLock::new(|| {
    String::from_utf8(
        general_purpose::STANDARD
            .decode("Z2xvYmFsLWFwaS5tYW5nYS11cC5jb20=")
            .expect("Failed to decode base64 API_HOST"),
    )
    .expect("Invalid base64 string (API_HOST)")
});
/// The image host used for image requests.
pub static IMAGE_HOST: LazyLock<String> = LazyLock::new(|| {
    String::from_utf8(
        general_purpose::STANDARD
            .decode("Z2xvYmFsLWltZy5tYW5nYS11cC5jb20=")
            .expect("Failed to decode base64 IMAGE_HOST"),
    )
    .expect("Invalid base64 string (IMAGE_HOST)")
});

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
