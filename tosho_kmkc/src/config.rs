use core::panic;

use cookie_store::Cookie;
use reqwest_cookie_store::CookieStoreMutex;
use urlencoding::decode;

#[derive(Debug, serde::Deserialize)]
pub struct KMConfigWebKV {
    /// The value of the cookie/key
    pub value: String,
    /// The expiry of the cookie/key
    pub expires: i64,
}

impl Default for KMConfigWebKV {
    fn default() -> Self {
        KMConfigWebKV {
            value: String::new(),
            expires: 0,
        }
    }
}

impl From<&Cookie<'_>> for KMConfigWebKV {
    fn from(value: &Cookie<'_>) -> Self {
        // unquote the value
        let binding = value.value().to_string();
        let data = decode(&binding).unwrap();
        let parsed: KMConfigWebKV = serde_json::from_str(&data).unwrap();
        parsed
    }
}

/// Represents the config/cookies for the web implementation
#[derive(Debug)]
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

impl From<CookieStoreMutex> for KMConfigWeb {
    fn from(value: CookieStoreMutex) -> Self {
        let mut uwt = String::new();
        let mut birthday = KMConfigWebKV::default();
        let mut tos_adult = KMConfigWebKV::default();
        let mut privacy = KMConfigWebKV::default();

        for cookie in value.lock().unwrap().iter_any() {
            match cookie.name() {
                "uwt" => uwt = cookie.value().to_string(),
                "birthday" => birthday = KMConfigWebKV::from(cookie),
                "terms_of_service_adult" => tos_adult = KMConfigWebKV::from(cookie),
                "privacy_policy" => privacy = KMConfigWebKV::from(cookie),
                _ => (),
            }
        }

        if uwt.is_empty() {
            panic!("uwt cookie not found");
        }

        KMConfigWeb {
            uwt,
            birthday,
            tos_adult,
            privacy,
        }
    }
}

/// Represents the mobile config
#[derive(Debug)]
pub struct KMConfigMobile {
    pub user_id: String,
    pub user_token: String,
}

/// Represents the config for the KM KC
#[derive(Debug)]
pub enum KMConfig {
    /// Web configuration
    Web(KMConfigWeb),
    /// Mobile configuration
    Mobile(KMConfigMobile),
}
