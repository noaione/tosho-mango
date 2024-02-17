//! Provides constants used in the library.
//!
//! All the following structs are a lazy static.
//!
//! ```rust
//! use tosho_musq::constants::get_constants;
//!
//! let _ = get_constants(1); // Android
//! ```

use base64::{engine::general_purpose, Engine as _};
use lazy_static::lazy_static;

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

lazy_static! {
    /// The constants used for Android devices.
    pub static ref ANDROID_CONSTANTS: Constants = {
        let android_app = String::from_utf8(
            general_purpose::STANDARD
                .decode("Y29tLnNxdWFyZV9lbml4LmFuZHJvaWRfZ29vZ2xlcGxheS5tYW5nYXVwX2dsb2JhbA==")
                .expect("Failed to decode base64 ANDROID_APP"),
        )
        .expect("Invalid base64 string");

        let android_app_ver = "45"; // 2.0.0

        Constants {
            image_ua: "Dalvik/2.1.0 (Linux; U; Android 12; SM-G935F Build/SQ3A.220705.004)".to_string(),
            api_ua: format!("{}/{} (Linux; U; Android 12; en_US; SM-G935F; Build/SQ3A.220705.004; Cronet/114.0.5735.33)", android_app, android_app_ver),
            os_ver: "32", // Android SDK 12
            app_ver: android_app_ver.to_string(),
        }
    };
    /// The constants used for iOS devices.
    pub static ref IOS_CONSTANTS: Constants = {
        let ios_app = String::from_utf8(
            general_purpose::STANDARD
                .decode("Y29tLnNxdWFyZS1lbml4Lk1hbmdhVVB3")
                .expect("Failed to decode base64 IOS_APP"),
        )
        .expect("Invalid base64 string (IOS_APP)");
        let ios_app_pre = String::from_utf8(
            general_purpose::STANDARD
                .decode("R2xlbndvb2RfUHJvZA==")
                .expect("Failed to decode base64 IOS_APP_PRE")
        )
        .expect("Invalid base64 string (IOS_APP_PRE)");
        let ios_app_post = String::from_utf8(
            general_purpose::STANDARD
                .decode("QWxhbW9maXJlLzUuNy4x")
                .expect("Failed to decode base64 IOS_APP_POST")
        )
        .expect("Invalid base64 string (IOS_APP_POST)");

        let ios_app_ver = "2.0.1";
        let ios_app_build = "202307211728";

        Constants {
            image_ua: format!("{}/{} CFNetwork/1410.0.3 Darwin/22.6.0", ios_app_pre, ios_app_build),
            api_ua: format!("{}/{} ({}; build:{}; iOS 16.7.0) {}", ios_app_pre, ios_app_ver, ios_app, ios_app_build, ios_app_post),
            os_ver: "16.7",
            app_ver: ios_app_ver.to_string(),
        }
    };

    /// The base API used for overall requests.
    pub static ref BASE_API: String = {
        String::from_utf8(
            general_purpose::STANDARD
                .decode("aHR0cHM6Ly9nbG9iYWwtYXBpLm1hbmdhLXVwLmNvbS9hcGk=")
                .expect("Failed to decode base64 BASE_API")
        )
        .expect("Invalid base64 string (BASE_API)")
    };
    /// The base image URL used for image requests.
    pub static ref BASE_IMG: String = {
        String::from_utf8(
            general_purpose::STANDARD
                .decode("aHR0cHM6Ly9nbG9iYWwtaW1nLm1hbmdhLXVwLmNvbQ==")
                .expect("Failed to decode base64 BASE_IMG")
        )
        .expect("Invalid base64 string (BASE_IMG)")
    };

    /// The list of valid quality formats.
    pub(crate) static ref QUALITY_FORMAT: Vec<&'static str> = vec!["middle", "high"];
    /// The list of valid weekly codes.
    pub(crate) static ref WEEKLY_CODE: Vec<&'static str> = vec!["mon", "tue", "wed", "thu", "fri", "sat", "sun"];

    /// The base host used for overall requests.
    pub static ref BASE_HOST: String = {
        String::from_utf8(
            general_purpose::STANDARD
                .decode("Z2xvYmFsLm1hbmdhLXVwLmNvbQ==")
                .expect("Failed to decode base64 BASE_HOST")
        )
        .expect("Invalid base64 string (BASE_HOST)")
    };
    /// The API host used for API requests.
    pub(crate) static ref API_HOST: String = {
        String::from_utf8(
            general_purpose::STANDARD
                .decode("Z2xvYmFsLWFwaS5tYW5nYS11cC5jb20=")
                .expect("Failed to decode base64 API_HOST")
        )
        .expect("Invalid base64 string (API_HOST)")
    };
    /// The image host used for image requests.
    pub(crate) static ref IMAGE_HOST: String = {
        String::from_utf8(
            general_purpose::STANDARD
                .decode("Z2xvYmFsLWltZy5tYW5nYS11cC5jb20=")
                .expect("Failed to decode base64 IMAGE_HOST")
        )
        .expect("Invalid base64 string (IMAGE_HOST)")
    };
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
/// use tosho_musq::constants::get_constants;
///
/// let _ = get_constants(1); // Android
/// let _ = get_constants(2); // iOS
/// ```
pub fn get_constants(device_type: u8) -> &'static Constants {
    match device_type {
        1 => &ANDROID_CONSTANTS,
        2 => &IOS_CONSTANTS,
        _ => panic!("Invalid device type"),
    }
}
