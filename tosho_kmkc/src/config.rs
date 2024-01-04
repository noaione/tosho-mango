use core::panic;

use cookie_store::{Cookie, RawCookie};
use reqwest::Url;
use reqwest_cookie_store::CookieStoreMutex;
use serde::ser::SerializeStruct;
use time::OffsetDateTime;
use urlencoding::{decode, encode};

use crate::constants::BASE_HOST;

#[derive(Debug, serde::Deserialize, Clone)]
pub struct KMConfigWebKV {
    /// The value of the cookie/key
    pub value: String,
    /// The expiry of the cookie/key
    pub expires: i64,
}

impl Default for KMConfigWebKV {
    fn default() -> Self {
        let current_utc = OffsetDateTime::now_utc().unix_timestamp();

        KMConfigWebKV {
            value: String::new(),
            // Expires is current + 1 year
            expires: current_utc + (365 * 24 * 60 * 60),
        }
    }
}

impl serde::Serialize for KMConfigWebKV {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let mut state = serializer.serialize_struct("KMConfigWebKV", 2)?;
        // Check if value is a str digit
        let val_as_i64 = self.value.parse::<i64>();
        match val_as_i64 {
            Ok(val) => {
                state.serialize_field("value", &val)?;
            }
            Err(_) => {
                state.serialize_field("value", &self.value)?;
            }
        }
        state.end()
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

impl From<reqwest::cookie::Cookie<'_>> for KMConfigWebKV {
    fn from(value: reqwest::cookie::Cookie) -> Self {
        // unquote the value
        let binding = value.value().to_string();
        let data = decode(&binding).unwrap();
        let parsed: KMConfigWebKV = serde_json::from_str(&data).unwrap();
        parsed
    }
}

fn i64_to_cookie_time(time: i64) -> OffsetDateTime {
    OffsetDateTime::from_unix_timestamp(time).unwrap()
}

impl KMConfigWebKV {
    fn to_cookie(&self, name: String) -> RawCookie<'_> {
        let binding = encode(&serde_json::to_string(&self).unwrap()).to_string();
        RawCookie::build(name, binding)
            .domain(BASE_HOST.as_str())
            .secure(true)
            .http_only(false)
            .path("/")
            .expires(i64_to_cookie_time(self.expires))
            .finish()
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

impl From<&reqwest::Response> for KMConfigWeb {
    fn from(value: &reqwest::Response) -> Self {
        let mut uwt = String::new();
        let mut birthday = KMConfigWebKV::default();
        let mut tos_adult = KMConfigWebKV::default();
        let mut privacy = KMConfigWebKV::default();

        for cookie in value.cookies() {
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

impl From<KMConfigWeb> for CookieStoreMutex {
    fn from(value: KMConfigWeb) -> Self {
        let store = CookieStoreMutex::default();

        let birthday_cookie = value.birthday.to_cookie("birthday".to_string());
        let tos_adult_cookie = value
            .tos_adult
            .to_cookie("terms_of_service_adult".to_string());
        let privacy_cookie = value.privacy.to_cookie("privacy_policy".to_string());

        let uwt = RawCookie::build("uwt", value.uwt)
            .domain(BASE_HOST.as_str())
            .secure(true)
            .http_only(true)
            .path("/")
            .expires(i64_to_cookie_time(value.birthday.expires))
            .finish();

        let base_host_url = Url::parse(&format!("https://{}", BASE_HOST.as_str())).unwrap();
        store
            .lock()
            .unwrap()
            .insert_raw(&uwt, &base_host_url)
            .unwrap();
        store
            .lock()
            .unwrap()
            .insert_raw(&birthday_cookie, &base_host_url)
            .unwrap();
        store
            .lock()
            .unwrap()
            .insert_raw(&tos_adult_cookie, &base_host_url)
            .unwrap();
        store
            .lock()
            .unwrap()
            .insert_raw(&privacy_cookie, &base_host_url)
            .unwrap();

        store
    }
}

impl Default for KMConfigWeb {
    /// Create a default [`KMConfigWeb`]
    ///
    /// Default will make an empty uwt, with a birthday of 1998-01, and tos_adult and privacy of 1
    fn default() -> Self {
        let mut birthday = KMConfigWebKV::default();
        birthday.value = "1998-01".to_string();

        let mut tos_toggle = KMConfigWebKV::default();
        tos_toggle.value = "1".to_string();

        Self {
            uwt: String::new(),
            birthday,
            tos_adult: tos_toggle.clone(),
            privacy: tos_toggle,
        }
    }
}

/// Represents the mobile config
#[derive(Debug, Clone)]
pub struct KMConfigMobile {
    pub user_id: String,
    pub hash_key: String,
}

/// Represents the config for the KM KC
#[derive(Debug, Clone)]
pub enum KMConfig {
    /// Web configuration
    Web(KMConfigWeb),
    /// Mobile configuration
    Mobile(KMConfigMobile),
}
