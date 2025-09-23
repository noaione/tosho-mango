//! Provides constants used in the library.
//!
//! All the following structs are a lazy static.
//!
//! ```rust
//! use tosho_nids::constants::get_constants;
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
}

/// The constants used for Web devices.
pub static WEB_CONSTANTS: LazyLock<Constants> = LazyLock::new(|| Constants {
    ua: "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:143.0) Gecko/20100101 Firefox/143.0",
    image_ua: "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:143.0) Gecko/20100101 Firefox/143.0",
});

/// The base API used for overall requests.
pub const BASE_API: &str = comptime_b64!("aHR0cHM6Ly9td2FwaS5uZW9uaWNoaWJhbi5jb20=");
/// The base image URL used for image requests.
pub const BASE_IMG: &str = comptime_b64!("aHR0cHM6Ly9hc3NldHMubmVvbmljaGliYW4uY29t");
/// The base website URL used for website requests.
pub const BASE_WEB: &str = comptime_b64!("aHR0cHM6Ly9uZW9uaWNoaWJhbi5jb20=");
/// The base secure image URL used for protected image requests.
pub const BASE_SECURE_IMG: &str =
    comptime_b64!("aHR0cHM6Ly9zZWN1cmUtYXNzZXRzLm5lb25pY2hpYmFuLmNvbQ==");

/// The base host used for overall requests.
pub const BASE_HOST: &str = comptime_b64!("bmVvbmljaGliYW4uY29t");
/// The API host used for API requests.
pub const API_HOST: &str = comptime_b64!("bXdhcGkubmVvbmljaGliYW4uY29t");
/// The image host used for image requests.
pub const IMAGE_HOST: &str = comptime_b64!("YXNzZXRzLm5lb25pY2hpYmFuLmNvbQ==");
/// The secure image host used for protected image requests.
pub const SECURE_IMAGE_HOST: &str = comptime_b64!("c2VjdXJlLWFzc2V0cy5uZW9uaWNoaWJhbi5jb20=");

/// Returns the constants for the given device type.
///
/// # Arguments
/// * `device_type` - The device type to get the constants for.
///
/// # Available device types
/// * `1` - Web
///
/// # Panics
/// Panics if the device type is invalid.
///
/// # Examples
/// ```rust
/// # use tosho_nids::constants::get_constants;
/// #
/// let _ = get_constants(1); // Web
/// ```
pub fn get_constants(device_type: u8) -> &'static Constants {
    match device_type {
        1 => &WEB_CONSTANTS,
        _ => panic!("Invalid device type"),
    }
}
