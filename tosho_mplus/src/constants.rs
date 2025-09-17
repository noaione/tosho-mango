//! Provides constants used in the library.
//!
//! All the following structs are a lazy static.
//!
//! ```rust
//! use tosho_mplus::constants::get_constants;
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
    pub(crate) app_ver: &'static str,
    /// The OS name string used for API requests.
    pub(crate) os_name: &'static str,
}

/// The constants used for Android devices.
pub static ANDROID_CONSTANTS: LazyLock<Constants> = LazyLock::new(|| {
    Constants {
        image_ua: "Dalvik/2.1.0 (Linux; U; Android 14; SM-A156E Build/UP1A.231005.007)".to_string(),
        api_ua: "okhttp/4.9.0".to_string(),
        os_ver: "34", // Android SDK 14
        app_ver: "1024",
        os_name: "android",
    }
});

/// The base API used for overall requests.
pub const BASE_API: &str = comptime_b64!("aHR0cHM6Ly9qdW1wZy1hcGkudG9reW8tY2RuLmNvbS9hcGk=");
/// The base image URL used for image requests.
pub const BASE_IMG: &str = comptime_b64!("aHR0cHM6Ly9qdW1wZy1hc3NldHMudG9reW8tY2RuLmNvbQ==");

/// The base host used for overall requests.
pub const BASE_HOST: &str = comptime_b64!("bWFuZ2FwbHVzLnNodWVpc2hhLmNvLmpw");
/// The API host used for API requests.
pub const API_HOST: &str = comptime_b64!("anVtcGctYXBpLnRva3lvLWNkbi5jb20=");
/// The image host used for image requests.
pub const IMAGE_HOST: &str = comptime_b64!("anVtcGctYXNzZXRzLnRva3lvLWNkbi5jb20=");

/// Returns the constants for the given device type.
///
/// # Arguments
/// * `device_type` - The device type to get the constants for.
///
/// # Available device types
/// * `1` - Android
///
/// # Panics
/// Panics if the device type is invalid.
///
/// # Examples
/// ```rust
/// # use tosho_mplus::constants::get_constants;
/// #
/// let _ = get_constants(1); // Android
/// ```
pub fn get_constants(device_type: u8) -> &'static Constants {
    match device_type {
        1 => &ANDROID_CONSTANTS,
        _ => panic!("Invalid device type"),
    }
}
