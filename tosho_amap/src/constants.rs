use base64::{engine::general_purpose, Engine as _};
use lazy_static::lazy_static;

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

lazy_static! {
    pub static ref APP_NAME: String = {
        String::from_utf8(
            general_purpose::STANDARD
                .decode("QWxwaGFNYW5nYQ==")
                .expect("Failed to decode base64 APP_NAME")
        )
        .expect("Invalid base64 string (APP_NAME)")
    };

    /// The constants used for Android devices.
    pub static ref ANDROID_CONSTANTS: Constants = {
        let app_version = "3.0.1";

        let ua_base = String::from_utf8(
            general_purpose::STANDARD
                .decode("RGFsdmlrLzIuMS4wIChMaW51eDsgVTsgQW5kcm9pZCAxMTsgU00tUzkwOEUgQnVpbGQvVFAxQS4yMjA2MjQuMDE0KQ==")
                .expect("Failed to decode base64 ANDROID_UA_BASE"),
        )
        .expect("Invalid base64 string");

        let final_ua: String = format!("{} {}/{}", ua_base, *APP_NAME, app_version);

        Constants {
            ua: final_ua.clone(),
            image_ua: final_ua,
            platform: "android",
            version: app_version,
        }
    };
    pub static ref BASE_API: String = {
        String::from_utf8(
            general_purpose::STANDARD
                .decode("aHR0cHM6Ly9hcGkuYWxwaGEtbWFuZ2EuY29tL2FwaS9lbg==")
                .expect("Failed to decode base64 BASE_API")
        )
        .expect("Invalid base64 string (BASE_API)")
    };
    pub static ref BASE_IMG: String = {
        String::from_utf8(
            general_purpose::STANDARD
                .decode("aHR0cHM6Ly9pbWFnZS1lbi5hbHBoYS1tYW5nYS5jb20=")
                .expect("Failed to decode base64 BASE_IMG")
        )
        .expect("Invalid base64 string (BASE_IMG)")
    };

    /// The base host used for overall requests.
    pub static ref BASE_HOST: String = {
        String::from_utf8(
            general_purpose::STANDARD
                .decode("YWxwaGEtbWFuZ2EuY29t")
                .expect("Failed to decode base64 BASE_HOST")
        )
        .expect("Invalid base64 string (BASE_HOST)")
    };
    /// The API host used for API requests.
    pub(crate) static ref API_HOST: String = {
        String::from_utf8(
            general_purpose::STANDARD
                .decode("YXBpLmFscGhhLW1hbmdhLmNvbQ==")
                .expect("Failed to decode base64 API_HOST")
        )
        .expect("Invalid base64 string (API_HOST)")
    };
    /// The image host used for image requests.
    pub(crate) static ref IMAGE_HOST: String = {
        String::from_utf8(
            general_purpose::STANDARD
                .decode("aW1hZ2UtZW4uYWxwaGEtbWFuZ2EuY29t")
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
/// let _ = get_constants(1); // Android
/// ```
pub fn get_constants(device_type: u8) -> &'static Constants {
    match device_type {
        1 => &ANDROID_CONSTANTS,
        _ => panic!("Invalid device type"),
    }
}
