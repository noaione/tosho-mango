//! Provides the configuration Struct for the client.
//!
//! ```rust
//! use tosho_kmkc::{KMConfigMobile, KMConfigMobilePlatform};
//!
//! let config = KMConfigMobile {
//!     user_id: "123".to_string(),
//!     hash_key: "abcxyz".to_string(),
//!     platform: KMConfigMobilePlatform::Android,
//! };
//! ```

use reqwest::Url;
use reqwest_cookie_store::{CookieStoreMutex, RawCookie};
use tosho_common::{bail_on_error, make_error, ToshoAuthError, ToshoError, ToshoResult};
use tosho_macros::{EnumName, EnumU32};
use urlencoding::{decode, encode};

use crate::constants::BASE_HOST;

/// Key value mapping for web cookies
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct KMConfigWebKV {
    /// The value of the cookie/key
    pub value: String,
    /// The expiry of the cookie/key
    pub expires: i64,
}

/// Key value mapping for web cookies with [`i64`] as a value
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct KMConfigWebKV64 {
    /// The value of the cookie/key
    pub value: i64,
    /// The expiry of the cookie/key
    pub expires: i64,
}

impl TryFrom<&KMConfigWebKV> for KMConfigWebKV64 {
    type Error = std::num::ParseIntError;

    fn try_from(value: &KMConfigWebKV) -> Result<Self, Self::Error> {
        let parsed = value.value.parse::<i64>()?;

        Ok(Self {
            value: parsed,
            expires: value.expires,
        })
    }
}

impl From<KMConfigWebKV64> for KMConfigWebKV {
    fn from(value: KMConfigWebKV64) -> Self {
        Self {
            value: value.value.to_string(),
            expires: value.expires,
        }
    }
}

impl TryFrom<&str> for KMConfigWebKV64 {
    type Error = ToshoError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let decoded =
            decode(value).map_err(|e| make_error!("Failed to decode urlencoded cookie: {}", e))?;
        let parsed: KMConfigWebKV64 = serde_json::from_str(&decoded)?;
        Ok(parsed)
    }
}

impl Default for KMConfigWebKV {
    fn default() -> Self {
        let current_utc = chrono::Utc::now().timestamp();

        KMConfigWebKV {
            value: "".into(),
            // Expires is current + 1 year
            expires: current_utc + (365 * 24 * 60 * 60),
        }
    }
}

impl TryFrom<&str> for KMConfigWebKV {
    type Error = ToshoError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let data =
            decode(value).map_err(|e| make_error!("Failed to decode urlencoded cookie: {}", e))?;
        let parsed: KMConfigWebKV = serde_json::from_str(&data)
            .map_err(|e| make_error!("Failed to parse cookie as JSON: {}", e))?;
        Ok(parsed)
    }
}

fn i64_to_cookie_time(time: i64) -> ::time::OffsetDateTime {
    ::time::OffsetDateTime::from_unix_timestamp(time).unwrap()
}

impl KMConfigWebKV {
    fn to_cookie(&self, name: String) -> RawCookie<'_> {
        // test if the value is a number
        let binding = match KMConfigWebKV64::try_from(self) {
            Ok(parsed) => encode(&serde_json::to_string(&parsed).unwrap()).to_string(),
            Err(_) => encode(&serde_json::to_string(&self).unwrap()).to_string(),
        };

        RawCookie::build((name, binding))
            .domain(&*BASE_HOST)
            .secure(true)
            .http_only(false)
            .path("/")
            .expires(i64_to_cookie_time(self.expires))
            .build()
    }
}

/// Represents the config/cookies for the web implementation
#[derive(Debug, Clone)]
pub struct KMConfigWeb {
    /// The auth token for KM KC
    pub uwt: String,
    /// Account birthday information.
    pub birthday: KMConfigWebKV,
    /// Account adult ToS aggreement status.
    pub tos_adult: KMConfigWebKV,
    /// Account privacy policy agreement status.
    pub privacy: KMConfigWebKV,
}

impl TryFrom<reqwest_cookie_store::CookieStore> for KMConfigWeb {
    type Error = ToshoError;

    fn try_from(value: reqwest_cookie_store::CookieStore) -> Result<Self, Self::Error> {
        let mut uwt = String::new();
        let mut birthday = KMConfigWebKV::default();
        let mut tos_adult = KMConfigWebKV::default();
        let mut privacy = KMConfigWebKV::default();

        for cookie in value.iter_any() {
            match cookie.name() {
                "uwt" => uwt = cookie.value().to_string(),
                "birthday" => birthday = KMConfigWebKV::try_from(cookie.value())?,
                "terms_of_service_adult" => {
                    tos_adult = match KMConfigWebKV64::try_from(cookie.value()) {
                        Ok(parsed) => KMConfigWebKV::from(parsed),
                        Err(_) => KMConfigWebKV::try_from(cookie.value())?,
                    }
                }
                "privacy_policy" => {
                    privacy = match KMConfigWebKV64::try_from(cookie.value()) {
                        Ok(parsed) => KMConfigWebKV::from(parsed),
                        Err(_) => KMConfigWebKV::try_from(cookie.value())?,
                    }
                }
                _ => (),
            }
        }

        if uwt.is_empty() {
            return Err(ToshoAuthError::UnknownSession.into());
        }

        Ok(KMConfigWeb {
            uwt,
            birthday,
            tos_adult,
            privacy,
        })
    }
}

impl TryFrom<KMConfigWeb> for reqwest_cookie_store::CookieStore {
    type Error = ToshoError;

    fn try_from(value: KMConfigWeb) -> Result<Self, Self::Error> {
        let mut store = reqwest_cookie_store::CookieStore::default();
        let base_host_url = Url::parse(&format!("https://{}", &*BASE_HOST))
            .map_err(|e| make_error!("Failed to parse base host url of {}: {}", &*BASE_HOST, e))?;

        let birthday_cookie = value.birthday.to_cookie("birthday".to_string());
        let tos_adult_cookie = value
            .tos_adult
            .to_cookie("terms_of_service_adult".to_string());
        let privacy_cookie = value.privacy.to_cookie("privacy_policy".to_string());

        store
            .insert_raw(&birthday_cookie, &base_host_url)
            .map_err(|e| {
                make_error!(
                    "Failed to insert birthday cookie of `{}` in `{}` into store: {}",
                    &birthday_cookie,
                    &base_host_url,
                    e
                )
            })?;
        store
            .insert_raw(&tos_adult_cookie, &base_host_url)
            .map_err(|e| {
                make_error!(
                    "Failed to insert ToS cookie of `{}` in `{}` into store: {}",
                    &tos_adult_cookie,
                    &base_host_url,
                    e
                )
            })?;
        store
            .insert_raw(&privacy_cookie, &base_host_url)
            .map_err(|e| {
                make_error!(
                    "Failed to insert privacy cookie of `{}` in `{}` into store: {}",
                    &privacy_cookie,
                    &base_host_url,
                    e
                )
            })?;

        if !value.uwt.is_empty() {
            let uwt = RawCookie::build(("uwt", value.uwt))
                .domain(&*BASE_HOST)
                .secure(true)
                .http_only(true)
                .path("/")
                .expires(i64_to_cookie_time(value.birthday.expires))
                .build();
            store.insert_raw(&uwt, &base_host_url).map_err(|e| {
                make_error!(
                    "Failed to insert uwt cookie of `{}` in `{}` into store: {}",
                    &uwt,
                    &base_host_url,
                    e
                )
            })?;
        }

        Ok(store)
    }
}

// impl From<KMConfigWeb> for reqwest_cookie_store::CookieStore {
//     fn from(value: KMConfigWeb) -> Self {}
// }

impl TryFrom<KMConfigWeb> for CookieStoreMutex {
    type Error = ToshoError;

    fn try_from(value: KMConfigWeb) -> Result<Self, Self::Error> {
        let store: reqwest_cookie_store::CookieStore = value.try_into()?;
        Ok(CookieStoreMutex::new(store))
    }
}

impl Default for KMConfigWeb {
    /// Create a default [`KMConfigWeb`]
    ///
    /// Default will make an empty uwt, with a birthday of 1998-01, and tos_adult and privacy of 1
    fn default() -> Self {
        let birthday = KMConfigWebKV {
            value: "1998-01".into(),
            ..Default::default()
        };
        let tos_toggle = KMConfigWebKV {
            value: "1".into(),
            ..Default::default()
        };

        Self {
            uwt: String::new(),
            birthday,
            tos_adult: tos_toggle.clone(),
            privacy: tos_toggle,
        }
    }
}

/// The mobile platform to use
#[derive(Debug, Clone, EnumName, EnumU32)]
#[repr(u8)]
pub enum KMConfigMobilePlatform {
    /// Apple/iOS
    Apple = 1,
    /// Android
    Android = 2,
}

/// Represents the mobile config
#[derive(Debug, Clone)]
pub struct KMConfigMobile {
    pub user_id: String,
    pub hash_key: String,
    pub platform: KMConfigMobilePlatform,
}

/// Represents the config for the KM KC
#[derive(Debug, Clone)]
pub enum KMConfig {
    /// Web configuration
    Web(KMConfigWeb),
    /// Mobile configuration
    Mobile(KMConfigMobile),
}

impl From<KMConfigWeb> for KMConfig {
    fn from(value: KMConfigWeb) -> Self {
        Self::Web(value)
    }
}

impl From<KMConfigMobile> for KMConfig {
    fn from(value: KMConfigMobile) -> Self {
        Self::Mobile(value)
    }
}

fn parse_cookie_as_str_kv(cookie_value: &str) -> ToshoResult<KMConfigWebKV> {
    let kv64 = KMConfigWebKV64::try_from(cookie_value);

    match kv64 {
        Ok(parsed) => {
            // Parse first from kv64 since number will fails on string KV
            Ok(KMConfigWebKV::from(parsed))
        }
        Err(_) => KMConfigWebKV::try_from(cookie_value),
    }
}

impl TryFrom<String> for KMConfigWeb {
    type Error = ToshoError;

    /// Parse a netscape cookie string into a [`KMConfigWeb`]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut uwt = String::new();
        let mut birthday = KMConfigWebKV::default();
        let mut tos_adult = KMConfigWebKV::default();
        let mut privacy = KMConfigWebKV::default();

        for cookie_line in value.lines() {
            if cookie_line.starts_with('#') && !cookie_line.starts_with("#HttpOnly_") {
                continue;
            }

            // cookie is separated by tabs
            // domain, include subdomain, path, secure, expiration, name, value
            let cookie_parts: Vec<&str> = cookie_line.split('\t').collect();
            if cookie_parts.len() != 7 {
                bail_on_error!("Failed to parse cookie line: {}", cookie_line.to_string());
            }

            let cookie_name = cookie_parts[5];
            let cookie_value = cookie_parts[6];

            match cookie_name {
                "uwt" => uwt = cookie_value.to_string(),
                "birthday" => birthday = parse_cookie_as_str_kv(cookie_value)?,
                "terms_of_service_adult" => tos_adult = parse_cookie_as_str_kv(cookie_value)?,
                "privacy_policy" => privacy = parse_cookie_as_str_kv(cookie_value)?,
                _ => (),
            }
        }

        Ok(KMConfigWeb {
            uwt,
            birthday,
            tos_adult,
            privacy,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kv_serde_str() {
        let kv = KMConfigWebKV {
            value: "test".into(),
            expires: 123,
        };

        let serde = serde_json::to_string(&kv).unwrap();
        assert_eq!(serde, "{\"value\":\"test\",\"expires\":123}");
    }

    #[test]
    fn test_cookie_i64() {
        let kv = KMConfigWebKV {
            value: "123".into(),
            expires: 123,
        };

        let cookie = kv.to_cookie("test".into());
        let decoded_cookie = decode(cookie.value()).unwrap();
        assert_eq!(decoded_cookie, "{\"value\":123,\"expires\":123}");
    }

    #[test]
    fn test_mobile_platform_u8() {
        assert_eq!(KMConfigMobilePlatform::Apple as u8, 1);
        assert_eq!(KMConfigMobilePlatform::Android as u8, 2);
    }
}
