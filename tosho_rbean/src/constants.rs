//! Provides constants used in the library.
//!
//! All the following structs are a lazy static.
//!
//! ```rust
//! use tosho_rbean::constants::get_constants;
//!
//! let _ = get_constants(1); // Web
//! ```

use base64::{engine::general_purpose, Engine as _};
use lazy_static::lazy_static;

/// A struct containing constants used in the library.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constants {
    /// The user agent string used for API requests.
    pub(crate) ua: &'static str,
    /// The user agent string used for image requests.
    pub(crate) image_ua: &'static str,
    /// Public key used for authentication.
    pub(crate) public: String,
}

lazy_static! {
    /// Token key used for authentication.
    pub(crate) static ref TOKEN_AUTH: String = {
        String::from_utf8(
            general_purpose::STANDARD
                .decode("QUl6YVN5RHR5bjg0U2J1ZFptOWFoZkgwNnYtaUppV0JZWVp1c1lrDQo=")
                .expect("Failed to decode base64 TOKEN_AUTH")
        )
        .expect("Invalid base64 string (TOKEN_AUTH)")
    };

    /// The constants used for Android devices.
    pub static ref ANDROID_CONSTANTS: Constants = {
        let public = String::from_utf8(
            general_purpose::STANDARD
                .decode("TVA2d2J1WkF3Mm5UTTlQUVQ4R2ZGNGZz")
                .expect("Failed to decode base64 ANDROID_PUBLIC")
        )
        .expect("Invalid base64 string (ANDROID_PUBLIC)");

        Constants {
            ua: "okhttp/4.9.0",
            image_ua: "Dalvik/2.1.0 (Linux; U; Android 12; SM-G935F Build/SQ3A.220705.004)",
            public,
        }
    };

    /// The base API used for overall requests.
    pub static ref BASE_API: String = {
        String::from_utf8(
            general_purpose::STANDARD
                .decode("aHR0cHM6Ly9wcm9kdWN0aW9uLmFwaS5henVraS5jbw==")
                .expect("Failed to decode base64 BASE_API")
        )
        .expect("Invalid base64 string (BASE_API)")
    };
    /// The base image URL used for image requests.
    pub static ref BASE_IMG: String = {
        String::from_utf8(
            general_purpose::STANDARD
                .decode("aHR0cHM6Ly9wcm9kdWN0aW9uLmltYWdlLWNvbnRlbnQuYXp1a2kuY28=")
                .expect("Failed to decode base64 BASE_IMG")
        )
        .expect("Invalid base64 string (BASE_IMG)")
    };

    /// The base host used for overall requests.
    pub static ref BASE_HOST: String = {
        String::from_utf8(
            general_purpose::STANDARD
                .decode("d3d3LmF6dWtpLmNv")
                .expect("Failed to decode base64 BASE_HOST")
        )
        .expect("Invalid base64 string (BASE_HOST)")
    };
    /// The API host used for API requests.
    pub(crate) static ref API_HOST: String = {
        String::from_utf8(
            general_purpose::STANDARD
                .decode("cHJvZHVjdGlvbi5hcGkuYXp1a2kuY28=")
                .expect("Failed to decode base64 API_HOST")
        )
        .expect("Invalid base64 string (API_HOST)")
    };
    /// The image host used for image requests.
    pub(crate) static ref IMAGE_HOST: String = {
        String::from_utf8(
            general_purpose::STANDARD
                .decode("cHJvZHVjdGlvbi5pbWFnZS1jb250ZW50LmF6dWtpLmNv")
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
/// use tosho_rbean::constants::get_constants;
///
/// let _ = get_constants(1); // Android
/// ```
pub fn get_constants(device_type: u8) -> &'static Constants {
    match device_type {
        1 => &ANDROID_CONSTANTS,
        _ => panic!("Invalid device type"),
    }
}
