//! Provides the configuration Struct for the client.
//!
//! ```rust
//! use tosho_amap::AMConfig;
//!
//! let config = AMConfig::new("123", "abcxyz", "xyz987abc");
//! ```

use std::sync::LazyLock;

use base64::{engine::general_purpose, Engine as _};
use reqwest::Url;
use reqwest_cookie_store::{CookieStoreMutex, RawCookie};
use tosho_common::{make_error, ToshoError};
use tosho_macros::AutoGetter;

use crate::constants::BASE_HOST;

/// The cookie name used for session_v2, lazy static.
pub static SESSION_COOKIE_NAME: LazyLock<String> = LazyLock::new(|| {
    String::from_utf8(
        general_purpose::STANDARD
            .decode("YWxwbF92Ml9lbl9zZXNzaW9u")
            .expect("Failed to decode base64 SESSION_COOKIE_NAME"),
    )
    .expect("Invalid base64 string (SESSION_COOKIE_NAME)")
});

/// Represents the configuration for the client.
#[derive(Debug, Clone, AutoGetter)]
pub struct AMConfig {
    /// The token of the account
    token: String,
    /// The identifier (guest ID) of the account, tied to token.
    identifier: String,
    /// The cookie of session_v2
    session_v2: String,
}

impl AMConfig {
    /// Create a new config instance of [`AMConfig`]
    pub fn new(
        token: impl Into<String>,
        identifier: impl Into<String>,
        session_v2: impl Into<String>,
    ) -> Self {
        Self {
            token: token.into(),
            identifier: identifier.into(),
            session_v2: session_v2.into(),
        }
    }
}

impl TryFrom<AMConfig> for reqwest_cookie_store::CookieStore {
    type Error = ToshoError;

    fn try_from(value: AMConfig) -> Result<Self, Self::Error> {
        let mut store = reqwest_cookie_store::CookieStore::default();
        let base_host_url = Url::parse(&format!("https://{}", &*BASE_HOST)).map_err(|e| {
            make_error!(
                "Failed to parse base host URL of https://{}: {}",
                &*BASE_HOST,
                e
            )
        })?;

        let session_cookie = RawCookie::build((&*SESSION_COOKIE_NAME, value.session_v2))
            .domain(&*BASE_HOST)
            .secure(true)
            .path("/")
            .build();

        store
            .insert_raw(&session_cookie, &base_host_url)
            .map_err(|e| {
                make_error!(
                    "Failed to insert session cookie of `{}` in `{}` into store: {}",
                    &session_cookie,
                    &base_host_url,
                    e
                )
            })?;
        Ok(store)
    }
}

impl TryFrom<AMConfig> for CookieStoreMutex {
    type Error = ToshoError;

    fn try_from(value: AMConfig) -> Result<Self, Self::Error> {
        let store: reqwest_cookie_store::CookieStore = value.try_into()?;
        Ok(CookieStoreMutex::new(store))
    }
}
