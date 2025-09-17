//! Provides constants used in the library.
//!
//! All the following structs are a lazy static.
//!
//! ```rust
//! use tosho_rbean::constants::get_constants;
//!
//! let _ = get_constants(1); // Web
//! ```

use std::sync::LazyLock;

use tosho_macros::comptime_b64;

/// A struct containing constants used in the library.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constants {
    /// The user agent string used for API requests.
    pub(crate) ua: &'static str,
    /// The user agent string used for image requests.
    pub(crate) image_ua: &'static str,
    /// Public key used for authentication.
    pub(crate) public: &'static str,
}

/// Token used for authentication.
pub(crate) const TOKEN_AUTH: &str =
    comptime_b64!("QUl6YVN5RHR5bjg0U2J1ZFptOWFoZkgwNnYtaUppV0JZWVp1c1lr");

/// The constants used for Android devices.
pub static ANDROID_CONSTANTS: LazyLock<Constants> = LazyLock::new(|| {
    let public = comptime_b64!("TVA2d2J1WkF3Mm5UTTlQUVQ4R2ZGNGZz");

    Constants {
        ua: "okhttp/4.9.0",
        image_ua: "Dalvik/2.1.0 (Linux; U; Android 12; SM-G935F Build/SQ3A.220705.004)",
        public,
    }
});

/// The base API used for overall requests.
pub const BASE_API: &str = comptime_b64!("aHR0cHM6Ly9wcm9kdWN0aW9uLmFwaS5henVraS5jbw==");
/// The base image URL used for image requests.
pub const BASE_IMG: &str =
    comptime_b64!("aHR0cHM6Ly9wcm9kdWN0aW9uLmltYWdlLWNvbnRlbnQuYXp1a2kuY28=");

/// The base host used for overall requests.
pub const BASE_HOST: &str = comptime_b64!("d3d3LmF6dWtpLmNv");
/// The API host used for API requests.
pub const API_HOST: &str = comptime_b64!("cHJvZHVjdGlvbi5hcGkuYXp1a2kuY28=");
/// The image host used for image requests.
pub const IMAGE_HOST: &str = comptime_b64!("cHJvZHVjdGlvbi5pbWFnZS1jb250ZW50LmF6dWtpLmNv");

/// The X-DRM-HEADER used for API requests.
pub(crate) const X_DRM_HEADER: &str = comptime_b64!("WC1BWlVLSS1EUk0=");

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
/// # use tosho_rbean::constants::get_constants;
/// #
/// let _ = get_constants(1); // Android
/// ```
pub fn get_constants(device_type: u8) -> &'static Constants {
    match device_type {
        1 => &ANDROID_CONSTANTS,
        _ => panic!("Invalid device type"),
    }
}
