//! Provides constants used in the library.
//!
//! All the following structs are a lazy static.
//!
//! ```rust
//! use tosho_sjv::constants::get_constants;
//!
//! let _ = get_constants(1); // Android
//! ```

use std::sync::LazyLock;

use tosho_macros::comptime_b64;

/// A struct containing constants used in the library.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constants {
    /// The user agent string used for requests.
    pub(crate) ua: &'static str,
    /// The app name used for Manga requests
    pub(crate) vm_name: &'static str,
    /// The app name used for Jump requests
    pub(crate) sj_name: &'static str,
    /// The app version string used for API requests.
    pub(crate) app_ver: &'static str,
    /// Device ID used for requests
    pub(crate) device_id: &'static str,
    /// Version body name used for requests
    pub(crate) version_body: Option<&'static str>,
}

/// App ID for VM
pub const VM_APP_ID: &str = "1";
/// App ID for SJ
pub const SJ_APP_ID: &str = "3";
/// API library version
pub const LIB_VERSION: &str = "9";

/// The constants used for Android devices.
pub static ANDROID_CONSTANTS: LazyLock<Constants> = LazyLock::new(|| {
    let vm_android_name = comptime_b64!("Y29tLnZpem1hbmdhLmFuZHJvaWQ=");
    let sj_android_name = comptime_b64!("Y29tLnZpei53c2ouYW5kcm9pZA==");
    let android_version_body = comptime_b64!("YW5kcm9pZF9hcHBfdmVyc2lvbl9jb2Rl");

    Constants {
        ua: "Dalvik/2.1.0 (Linux; U; Android 12; SM-G935F Build/SQ3A.220705.004)",
        vm_name: vm_android_name,
        sj_name: sj_android_name,
        app_ver: "180", // 4.8.2
        device_id: "4",
        version_body: Some(android_version_body),
    }
});
/// The constants used for Apple devices.
pub static APPLE_CONSTANTS: LazyLock<Constants> = LazyLock::new(|| {
    let vm_apple_name = comptime_b64!("Y29tLnZpem1hbmdhLmFwcGxl");
    let sj_apple_name = comptime_b64!("Y29tLnZpei53c2ouYXBwbGU=");

    Constants {
        ua: "Alamofire/5.7.1/202307211728 CFNetwork/1410.0.3 Darwin/22.6.0",
        vm_name: vm_apple_name,
        sj_name: sj_apple_name,
        app_ver: "180",
        device_id: "1",
        // Might need to add later
        version_body: None,
    }
});
/// The constants used for Web devices.
pub static WEB_CONSTANTS: LazyLock<Constants> = LazyLock::new(|| {
    let common_web_name = comptime_b64!("aHR0cHM6Ly92aXouY29t");

    Constants {
        ua: "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:122.0) Gecko/20100101 Firefox/122.0",
        vm_name: common_web_name,
        sj_name: common_web_name,
        app_ver: "180",
        device_id: "3",
        version_body: None,
    }
});

/// The base API used for overall requests.
pub const BASE_API: &str = comptime_b64!("aHR0cHM6Ly9hcGkudml6LmNvbQ==");

/// The base host used for overall requests.
pub const BASE_HOST: &str = comptime_b64!("dml6LmNvbQ==");
/// The API host used for API requests.
pub const API_HOST: &str = comptime_b64!("YXBpLnZpei5jb20=");

/// The header name for the one piece reference.
pub(crate) const HEADER_PIECE: &str = comptime_b64!("eC1kZXZpbC1mcnVpdA==");
/// The header value for the one piece reference.
pub(crate) const VALUE_PIECE: &str = comptime_b64!("ZmxhbWUtZmxhbWUgZnJ1aXRz");

/// Data name for specific app ID
pub(crate) const DATA_APP_ID: &str = comptime_b64!("dml6X2FwcF9pZA==");

/// Expanded `VM` app name
pub const EXPAND_VM_NAME: &str = comptime_b64!("dml6bWFuZ2E=");
/// Expanded `SJ` app name
pub const EXPAND_SJ_NAME: &str = comptime_b64!("c2hvbmVuanVtcA==");

/// Returns the constants for the given device type.
///
/// # Arguments
/// * `device_type` - The device type to get the constants for.
///
/// # Available device types
/// * `1` - Android
/// * `2` - Apple/iOS
/// * `3` - Web
///
/// # Panics
/// Panics if the device type is invalid.
///
/// # Examples
/// ```rust
/// # use tosho_sjv::constants::get_constants;
/// #
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
