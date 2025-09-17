//! Provides constants used in the library.
//!
//! All the following structs are a lazy static.
//!
//! ```rust
//! use tosho_amap::constants::get_constants;
//!
//! let _ = get_constants(1); // Android
//! ```

use std::sync::LazyLock;

use tosho_macros::comptime_b64;

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
}

/// A struct containing the header names used in the library.
#[derive(Debug, Clone)]
pub struct HeaderMapping {
    pub(crate) i: &'static str,
    pub(crate) t: &'static str,
    pub(crate) s: &'static str,
    pub(crate) n: &'static str,
}

/// The name of the application.
pub const APP_NAME: &str = comptime_b64!("QWxwaGFNYW5nYQ==");

/// The constants used for Android devices.
pub static ANDROID_CONSTANTS: LazyLock<Constants> = LazyLock::new(|| {
    let app_version = "3.0.9";

    let main_ua: String = format!(
        "Dalvik/2.1.0 (Linux; U; Android 11; SM-S908E Build/TP1A.220624.014) {}/{}",
        APP_NAME, app_version
    );

    Constants {
        ua: main_ua.clone(),
        image_ua: main_ua,
        platform: "android",
        version: app_version,
    }
});

/// The base API used for overall requests.
pub const BASE_API: &str =
    tosho_macros::comptime_b64!("aHR0cHM6Ly9hcGkuYWxwaGEtbWFuZ2EuY29tL2FwaS9lbg==");
/// The base image URL used for image requests.
pub const BASE_IMG: &str = comptime_b64!("aHR0cHM6Ly9pbWFnZS1lbi5hbHBoYS1tYW5nYS5jb20=");

/// The base host used for overall requests.
pub const BASE_HOST: &str = comptime_b64!("YWxwaGEtbWFuZ2EuY29t");
/// The API host used for API requests.
pub const API_HOST: &str = comptime_b64!("YXBpLmFscGhhLW1hbmdhLmNvbQ==");
/// The image host used for image requests.
pub const IMAGE_HOST: &str = comptime_b64!("aW1hZ2UtZW4uYWxwaGEtbWFuZ2EuY29t");

/// Constants used for header names.
pub(crate) static HEADER_NAMES: LazyLock<HeaderMapping> = LazyLock::new(|| {
    let i = comptime_b64!("YXAtYXV0aC1pZGVudGlmaWVy");
    let t = comptime_b64!("YXAtYXV0aC10b2tlbg==");
    let s = comptime_b64!("YXAtYXV0aC1zZWNyZXQ=");
    let n = comptime_b64!("YXAtYXV0aC1ub25jZQ==");

    HeaderMapping { i, t, s, n }
});

/// The login route used for login requests.
pub(crate) const MASKED_LOGIN: &str = comptime_b64!("bG9naW4vYWxwaGFwb2xpcy5qc29u");

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
/// # use tosho_amap::constants::get_constants;
/// #
/// let _ = get_constants(1); // Android
/// ```
pub fn get_constants(device_type: u8) -> &'static Constants {
    match device_type {
        1 => &ANDROID_CONSTANTS,
        _ => panic!("Invalid device type"),
    }
}
