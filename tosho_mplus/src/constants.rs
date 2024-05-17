//! Provides constants used in the library.
//!
//! All the following structs are a lazy static.
//!
//! ```rust
//! use tosho_mplus::constants::get_constants;
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
    pub(crate) app_ver: &'static str,
    /// The OS name string used for API requests.
    pub(crate) os_name: &'static str,
}

lazy_static! {
    /// The constants used for Android devices.
    pub static ref ANDROID_CONSTANTS: Constants = {
        Constants {
            image_ua: "Dalvik/2.1.0 (Linux; U; Android 14; SM-A156E Build/UP1A.231005.007)".to_string(),
            api_ua: "okhttp/4.9.0".to_string(),
            os_ver: "34", // Android SDK 14
            app_ver: "133",
            os_name: "android",
        }
    };

    /// The base API used for overall requests.
    pub static ref BASE_API: String = {
        String::from_utf8(
            general_purpose::STANDARD
                .decode("aHR0cHM6Ly9qdW1wZy1hcGkudG9reW8tY2RuLmNvbS9hcGk=")
                .expect("Failed to decode base64 BASE_API")
        )
        .expect("Invalid base64 string (BASE_API)")
    };
    /// The base image URL used for image requests.
    pub static ref BASE_IMG: String = {
        String::from_utf8(
            general_purpose::STANDARD
                .decode("aHR0cHM6Ly9tYW5nYXBsdXMuc2h1ZWlzaGEuY28uanA=")
                .expect("Failed to decode base64 BASE_IMG")
        )
        .expect("Invalid base64 string (BASE_IMG)")
    };

    /// The list of valid quality formats.
    pub(crate) static ref QUALITY_FORMAT: Vec<&'static str> = vec!["low", "high", "super_high"];
    /// The list of valid weekly codes.
    pub(crate) static ref WEEKLY_CODE: Vec<&'static str> = vec!["mon", "tue", "wed", "thu", "fri", "sat", "sun"];

    /// The base host used for overall requests.
    pub static ref BASE_HOST: String = {
        String::from_utf8(
            general_purpose::STANDARD
                .decode("bWFuZ2FwbHVzLnNodWVpc2hhLmNvLmpw")
                .expect("Failed to decode base64 BASE_HOST")
        )
        .expect("Invalid base64 string (BASE_HOST)")
    };
    /// The API host used for API requests.
    pub(crate) static ref API_HOST: String = {
        String::from_utf8(
            general_purpose::STANDARD
                .decode("anVtcGctYXBpLnRva3lvLWNkbi5jb20=")
                .expect("Failed to decode base64 API_HOST")
        )
        .expect("Invalid base64 string (API_HOST)")
    };
    /// The image host used for image requests.
    pub(crate) static ref IMAGE_HOST: String = {
        String::from_utf8(
            general_purpose::STANDARD
                .decode("bWFuZ2FwbHVzLnNodWVpc2hhLmNvLmpw")
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
/// use tosho_mplus::constants::get_constants;
///
/// let _ = get_constants(1); // Android
/// ```
pub fn get_constants(device_type: u8) -> &'static Constants {
    match device_type {
        1 => &ANDROID_CONSTANTS,
        _ => panic!("Invalid device type"),
    }
}
