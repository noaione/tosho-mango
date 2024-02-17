use base64::{engine::general_purpose, Engine as _};
use lazy_static::lazy_static;

/// A struct containing constants used in the library.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constants {
    /// The user agent string used for requests.
    pub(crate) ua: &'static str,
    /// The app name used for Manga requests
    pub(crate) vm_name: String,
    /// The app name used for Jump requests
    pub(crate) sj_name: String,
    /// The app version string used for API requests.
    pub(crate) app_ver: &'static str,
    /// Device ID used for requests
    pub(crate) device_id: &'static str,
    /// Version body name used for requests
    pub(crate) version_body: Option<String>,
}

pub const VM_APP_ID: &str = "1";
pub const SJ_APP_ID: &str = "3";
pub const LIB_VERSION: &str = "9";

lazy_static! {
    /// The constants used for Android devices.
    pub static ref ANDROID_CONSTANTS: Constants = {
        let vm_android_name = String::from_utf8(
            general_purpose::STANDARD
                .decode("Y29tLnZpem1hbmdhLmFuZHJvaWQ=")
                .expect("Failed to decode base64 VM_ANDROID_NAME"),
        )
        .expect("Invalid base64 string (VM_ANDROID_NAME)");
        let sj_android_name = String::from_utf8(
            general_purpose::STANDARD
                .decode("Y29tLnZpei53c2ouYW5kcm9pZA==")
                .expect("Failed to decode base64 SJ_ANDROID_NAME"),
        )
        .expect("Invalid base64 string (SJ_ANDROID_NAME)");
        let android_version_body: String = {
            String::from_utf8(
                general_purpose::STANDARD
                    .decode("YW5kcm9pZF9hcHBfdmVyc2lvbl9jb2Rl")
                    .expect("Failed to decode base64 ANDROID_VERSION_BODY")
            )
            .expect("Invalid base64 string (ANDROID_VERSION_BODY)")
        };

        Constants {
            ua: "Dalvik/2.1.0 (Linux; U; Android 12; SM-G935F Build/SQ3A.220705.004)",
            vm_name: vm_android_name,
            sj_name: sj_android_name,
            app_ver: "143", // 4.4.9
            device_id: "4",
            version_body: Some(android_version_body)
        }
    };
    /// The constants used for Apple devices.
    pub static ref APPLE_CONSTANTS: Constants = {
        let vm_apple_name = String::from_utf8(
            general_purpose::STANDARD
                .decode("Y29tLnZpem1hbmdhLmFwcGxl")
                .expect("Failed to decode base64 VM_APPLE_NAME"),
        )
        .expect("Invalid base64 string (VM_APPLE_NAME)");
        let sj_apple_name = String::from_utf8(
            general_purpose::STANDARD
                .decode("Y29tLnZpei53c2ouYXBwbGU=")
                .expect("Failed to decode base64 SJ_APPLE_NAME"),
        )
        .expect("Invalid base64 string (SJ_APPLE_NAME)");

        Constants {
            ua: "Alamofire/5.7.1/202307211728 CFNetwork/1410.0.3 Darwin/22.6.0",
            vm_name: vm_apple_name,
            sj_name: sj_apple_name,
            app_ver: "143",
            device_id: "1",
            // Might need to add later
            version_body: None
        }
    };
    /// The constants used for Web devices.
    pub static ref WEB_CONSTANTS: Constants = {
        let common_web_name = String::from_utf8(
            general_purpose::STANDARD
                .decode("aHR0cHM6Ly92aXouY29t")
                .expect("Failed to decode base64 COMMON_WEB_NAME"),
        )
        .expect("Invalid base64 string (COMMON_WEB_NAME)");

        Constants {
            ua: "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:122.0) Gecko/20100101 Firefox/122.0",
            vm_name: common_web_name.clone(),
            sj_name: common_web_name,
            app_ver: "143",
            device_id: "3",
            version_body: None
        }
    };
    pub static ref BASE_API: String = {
        String::from_utf8(
            general_purpose::STANDARD
                .decode("aHR0cHM6Ly9hcGkudml6LmNvbQ==")
                .expect("Failed to decode base64 BASE_API")
        )
        .expect("Invalid base64 string (BASE_API)")
    };

    /// The base host used for overall requests.
    pub static ref BASE_HOST: String = {
        String::from_utf8(
            general_purpose::STANDARD
                .decode("dml6LmNvbQ==")
                .expect("Failed to decode base64 BASE_HOST")
        )
        .expect("Invalid base64 string (BASE_HOST)")
    };
    /// The API host used for API requests.
    pub(crate) static ref API_HOST: String = {
        String::from_utf8(
            general_purpose::STANDARD
                .decode("YXBpLnZpei5jb20=")
                .expect("Failed to decode base64 API_HOST")
        )
        .expect("Invalid base64 string (API_HOST)")
    };
    /// Header name for that one piece reference
    pub(crate) static ref HEADER_PIECE: String = {
        String::from_utf8(
            general_purpose::STANDARD
                .decode("eC1kZXZpbC1mcnVpdA==")
                .expect("Failed to decode base64 HEADER_PIECE")
        )
        .expect("Invalid base64 string (HEADER_PIECE)")
    };
    pub(crate) static ref VALUE_PIECE: String = {
        String::from_utf8(
            general_purpose::STANDARD
                .decode("ZmxhbWUtZmxhbWUgZnJ1aXRz")
                .expect("Failed to decode base64 VALUE_PIECE")
        )
        .expect("Invalid base64 string (VALUE_PIECE)")
    };
    /// Data name for specific app ID
    pub(crate) static ref DATA_APP_ID: String = {
        String::from_utf8(
            general_purpose::STANDARD
                .decode("dml6X2FwcF9pZA==")
                .expect("Failed to decode base64 DATA_APP_ID")
        )
        .expect("Invalid base64 string (DATA_APP_ID)")
    };
    /// Expanded VM name
    pub static ref EXPAND_VM_NAME: String = {
        String::from_utf8(
            general_purpose::STANDARD
                .decode("dml6bWFuZ2E=")
                .expect("Failed to decode base64 EXPAND_VM_NAME")
        )
        .expect("Invalid base64 string (EXPAND_VM_NAME)")
    };
    /// Expanded SJ name
    pub static ref EXPAND_SJ_NAME: String = {
        String::from_utf8(
            general_purpose::STANDARD
                .decode("c2hvbmVuanVtcA==")
                .expect("Failed to decode base64 EXPAND_SJ_NAME")
        )
        .expect("Invalid base64 string (EXPAND_SJ_NAME)")
    };
}

/// Returns the constants for the given device type.
///
/// # Arguments
/// * `device_type` - The device type to get the constants for.
///
/// # Available device types
/// * `1` - Android
/// * `2` - Apple
/// * `3` - Web
///
/// # Panics
/// Panics if the device type is invalid.
///
/// # Examples
/// ```
/// use tosho_sjv::constants::get_constants;
///
/// let _ = get_constants(1); // Android
/// ```
pub fn get_constants(device_type: u8) -> &'static Constants {
    match device_type {
        1 => &ANDROID_CONSTANTS,
        2 => &APPLE_CONSTANTS,
        3 => &WEB_CONSTANTS,
        _ => panic!("Invalid device type"),
    }
}
