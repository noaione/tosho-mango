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

use base64::{Engine as _, engine::general_purpose};

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
    pub(crate) i: String,
    pub(crate) t: String,
    pub(crate) s: String,
    pub(crate) n: String,
}

/// The name of the application.
pub static APP_NAME: LazyLock<String> = LazyLock::new(|| {
    String::from_utf8(
        general_purpose::STANDARD
            .decode("QWxwaGFNYW5nYQ==")
            .expect("Failed to decode base64 APP_NAME"),
    )
    .expect("Invalid base64 string (APP_NAME)")
});

/// The constants used for Android devices.
pub static ANDROID_CONSTANTS: LazyLock<Constants> = LazyLock::new(|| {
    let app_version = "3.0.9";

    let main_ua: String = format!(
        "Dalvik/2.1.0 (Linux; U; Android 11; SM-S908E Build/TP1A.220624.014) {}/{}",
        *APP_NAME, app_version
    );

    Constants {
        ua: main_ua.clone(),
        image_ua: main_ua,
        platform: "android",
        version: app_version,
    }
});

/// The base API used for overall requests.
pub static BASE_API: LazyLock<String> = LazyLock::new(|| {
    String::from_utf8(
        general_purpose::STANDARD
            .decode("aHR0cHM6Ly9hcGkuYWxwaGEtbWFuZ2EuY29tL2FwaS9lbg==")
            .expect("Failed to decode base64 BASE_API"),
    )
    .expect("Invalid base64 string (BASE_API)")
});
/// The base image URL used for image requests.
pub static BASE_IMG: LazyLock<String> = LazyLock::new(|| {
    String::from_utf8(
        general_purpose::STANDARD
            .decode("aHR0cHM6Ly9pbWFnZS1lbi5hbHBoYS1tYW5nYS5jb20=")
            .expect("Failed to decode base64 BASE_IMG"),
    )
    .expect("Invalid base64 string (BASE_IMG)")
});

/// The base host used for overall requests.
pub static BASE_HOST: LazyLock<String> = LazyLock::new(|| {
    String::from_utf8(
        general_purpose::STANDARD
            .decode("YWxwaGEtbWFuZ2EuY29t")
            .expect("Failed to decode base64 BASE_HOST"),
    )
    .expect("Invalid base64 string (BASE_HOST)")
});
/// The API host used for API requests.
pub static API_HOST: LazyLock<String> = LazyLock::new(|| {
    String::from_utf8(
        general_purpose::STANDARD
            .decode("YXBpLmFscGhhLW1hbmdhLmNvbQ==")
            .expect("Failed to decode base64 API_HOST"),
    )
    .expect("Invalid base64 string (API_HOST)")
});
/// The image host used for image requests.
pub static IMAGE_HOST: LazyLock<String> = LazyLock::new(|| {
    String::from_utf8(
        general_purpose::STANDARD
            .decode("aW1hZ2UtZW4uYWxwaGEtbWFuZ2EuY29t")
            .expect("Failed to decode base64 IMAGE_HOST"),
    )
    .expect("Invalid base64 string (IMAGE_HOST)")
});

/// Constants used for header names.
pub(crate) static HEADER_NAMES: LazyLock<HeaderMapping> = LazyLock::new(|| {
    let i = String::from_utf8(
        general_purpose::STANDARD
            .decode("YXAtYXV0aC1pZGVudGlmaWVy")
            .expect("Failed to decode base64 HEADER_NAME_I"),
    )
    .expect("Invalid base64 string (HEADER_NAME_I)");

    let t = String::from_utf8(
        general_purpose::STANDARD
            .decode("YXAtYXV0aC10b2tlbg==")
            .expect("Failed to decode base64 HEADER_NAME_T"),
    )
    .expect("Invalid base64 string (HEADER_NAME_T)");

    let s = String::from_utf8(
        general_purpose::STANDARD
            .decode("YXAtYXV0aC1zZWNyZXQ=")
            .expect("Failed to decode base64 HEADER_NAME_S"),
    )
    .expect("Invalid base64 string (HEADER_NAME_S)");

    let n = String::from_utf8(
        general_purpose::STANDARD
            .decode("YXAtYXV0aC1ub25jZQ==")
            .expect("Failed to decode base64 HEADER_NAME_N"),
    )
    .expect("Invalid base64 string (HEADER_NAME_N)");

    HeaderMapping { i, t, s, n }
});

/// The login route used for login requests.
pub(crate) static MASKED_LOGIN: LazyLock<String> = LazyLock::new(|| {
    String::from_utf8(
        general_purpose::STANDARD
            .decode("bG9naW4vYWxwaGFwb2xpcy5qc29u")
            .expect("Failed to decode base64 MASKED_LOGIN"),
    )
    .expect("Invalid base64 string (MASKED_LOGIN)")
});

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
