//! Provides constants used in the library.
//!
//! All the following structs are a lazy static.
//!
//! ```rust
//! use tosho_amap::constants::get_constants;
//!
//! let _ = get_constants(1); // Web
//! ```

use base64::{engine::general_purpose, Engine as _};
use lazy_static::lazy_static;

/// A struct containing constants used in the library.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constants {
    /// The user agent string used for all requests.
    pub(crate) ua: String,
}

lazy_static! {
    /// The constants used for Web devices.
    pub static ref WEB_CONSTANTS: Constants = {
        Constants {
            ua: "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:122.0) Gecko/20100101 Firefox/122.0".to_string(),
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
/// use tosho_amap::constants::get_constants;
///
/// let _ = get_constants(1); // Web
/// ```
pub fn get_constants(device_type: u8) -> &'static Constants {
    match device_type {
        1 => &WEB_CONSTANTS,
        _ => panic!("Invalid device type"),
    }
}
