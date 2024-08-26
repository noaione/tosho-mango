use std::sync::LazyLock;

use base64::{engine::general_purpose, Engine as _};

/// The user agent used for requests.
///
/// This is Firefox 129.0 on Windows 10.
pub const USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:129.0) Gecko/20100101 Firefox/129.0";

/// The base API used for overall requests.
pub static BASE_API: LazyLock<String> = LazyLock::new(|| {
    // This include the /api path (without the trailing slash)
    String::from_utf8(
        general_purpose::STANDARD
            .decode("aHR0cHM6Ly9kb3VqaW4uaW8vYXBp")
            .expect("Failed to decode base64 BASE_API"),
    )
    .expect("Invalid base64 string (BASE_API)")
});
/// The base image URL used for image requests.
pub static BASE_IMG: LazyLock<String> = LazyLock::new(|| {
    String::from_utf8(
        general_purpose::STANDARD
            .decode("aHR0cHM6Ly9kb3VqaW4taW8tbWFuZ2EuYi1jZG4ubmV0")
            .expect("Failed to decode base64 BASE_IMG"),
    )
    .expect("Invalid base64 string (BASE_IMG)")
});

/// The base host used for overall requests.
pub static BASE_HOST: LazyLock<String> = LazyLock::new(|| {
    String::from_utf8(
        general_purpose::STANDARD
            .decode("ZG91amluLmlv")
            .expect("Failed to decode base64 BASE_HOST"),
    )
    .expect("Invalid base64 string (BASE_HOST)")
});
/// The API host used for API requests.
pub static API_HOST: LazyLock<String> = LazyLock::new(|| {
    String::from_utf8(
        general_purpose::STANDARD
            .decode("ZG91amluLmlv")
            .expect("Failed to decode base64 API_HOST"),
    )
    .expect("Invalid base64 string (API_HOST)")
});
/// The image host used for image requests.
pub static IMAGE_HOST: LazyLock<String> = LazyLock::new(|| {
    String::from_utf8(
        general_purpose::STANDARD
            .decode("ZG91amluLWlvLW1hbmdhLmItY2RuLm5ldA==")
            .expect("Failed to decode base64 IMAGE_HOST"),
    )
    .expect("Invalid base64 string (IMAGE_HOST)")
});
