#![allow(clippy::derive_partial_eq_without_eq)]

pub const PREFIX: &'static str = "kmkc";

/// Device type for MU! by SQ session.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
pub enum DeviceType {
    /// Mobile device.
    Mobile = 1,
    /// Web app/platform
    Web = 2,
}

/// Mobile platform for MU! by SQ session.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
pub enum MobilePlatform {
    /// Android device.
    Android = 1,
    /// iOS device/Apple.
    Apple = 2,
}

/// Represents the basic/simple config file for the KM by KC app.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigBase {
    /// The UUID of the account/config.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The device type of the account/config.
    #[prost(enumeration = "DeviceType", tag = "2")]
    pub r#type: i32,
}

/// Represents the mobile config file for the KM by KC app.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigMobile {
    /// The UUID of the account/config.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The device type of the account/config.
    #[prost(enumeration = "DeviceType", tag = "2")]
    pub r#type: i32,
    /// The username of the account/config.
    #[prost(string, tag = "3")]
    pub username: ::prost::alloc::string::String,
    /// The email of the account/config.
    #[prost(string, tag = "4")]
    pub email: ::prost::alloc::string::String,
    /// The account ID of the account/config.
    #[prost(uint32, tag = "5")]
    pub account_id: u32,
    /// The device ID of the account/config.
    #[prost(uint32, tag = "6")]
    pub device_id: u32,
    /// The user ID of the account/config.
    #[prost(string, tag = "100")]
    pub user_id: ::prost::alloc::string::String,
    /// The user secret/token of the account/config.
    #[prost(string, tag = "101")]
    pub user_secret: ::prost::alloc::string::String,
    /// The platform of the account/config.
    #[prost(enumeration = "MobilePlatform", optional, tag = "102")]
    pub platform: ::core::option::Option<i32>,
}

impl From<ConfigMobile> for tosho_kmkc::KMConfigMobile {
    fn from(value: ConfigMobile) -> Self {
        tosho_kmkc::config::KMConfigMobile {
            user_id: value.user_id.clone(),
            hash_key: value.user_secret.clone(),
        }
    }
}

/// Represents the key-value cookies pair for the KM by KC app.
///
/// Used in the [`ConfigWeb`] message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigWebKeyValue {
    /// The key of the cookie.
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
    /// The value of the cookie.
    #[prost(uint64, tag = "2")]
    pub expires: u64,
}

impl From<tosho_kmkc::config::KMConfigWebKV> for ConfigWebKeyValue {
    fn from(value: tosho_kmkc::config::KMConfigWebKV) -> Self {
        ConfigWebKeyValue {
            value: value.value.clone(),
            expires: value.expires.clone().try_into().unwrap_or(0),
        }
    }
}

impl From<ConfigWebKeyValue> for tosho_kmkc::KMConfigWebKV {
    fn from(value: ConfigWebKeyValue) -> Self {
        tosho_kmkc::KMConfigWebKV {
            value: value.value.clone(),
            expires: value.expires.clone().try_into().unwrap_or(0),
        }
    }
}

/// Represents the web config file for the KM by KC app.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigWeb {
    /// The UUID of the account/config.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The device type of the account/config.
    #[prost(enumeration = "DeviceType", tag = "2")]
    pub r#type: i32,
    /// The username of the account/config.
    #[prost(string, tag = "3")]
    pub username: ::prost::alloc::string::String,
    /// The email of the account/config.
    #[prost(string, tag = "4")]
    pub email: ::prost::alloc::string::String,
    /// The account ID of the account/config.
    #[prost(uint32, tag = "5")]
    pub account_id: u32,
    /// The device ID of the account/config.
    #[prost(uint32, tag = "6")]
    pub device_id: u32,
    /// The auth token/uwt token.
    #[prost(string, tag = "100")]
    pub uwt: ::prost::alloc::string::String,
    /// Account birthday information.
    #[prost(message, tag = "101")]
    pub birthday: ::core::option::Option<ConfigWebKeyValue>,
    /// Account adult ToS aggreement status.
    #[prost(message, tag = "102")]
    pub tos_adult: ::core::option::Option<ConfigWebKeyValue>,
    /// Account privacy policy agreement status.
    #[prost(message, tag = "103")]
    pub privacy: ::core::option::Option<ConfigWebKeyValue>,
}

impl From<ConfigWeb> for tosho_kmkc::KMConfigWeb {
    fn from(value: ConfigWeb) -> Self {
        tosho_kmkc::KMConfigWeb {
            uwt: value.uwt.clone(),
            birthday: value.birthday.clone().unwrap().into(),
            tos_adult: value.tos_adult.clone().unwrap().into(),
            privacy: value.privacy.clone().unwrap().into(),
        }
    }
}

impl From<tosho_kmkc::KMConfigWeb> for ConfigWeb {
    fn from(value: tosho_kmkc::KMConfigWeb) -> Self {
        ConfigWeb {
            id: String::new(),
            r#type: DeviceType::Web as i32,
            username: String::new(),
            email: String::from("temp@kmkc.xyz"),
            account_id: 0,
            device_id: 0,
            uwt: value.uwt.clone(),
            birthday: Some(value.birthday.clone().into()),
            tos_adult: Some(value.tos_adult.clone().into()),
            privacy: Some(value.privacy.clone().into()),
        }
    }
}

/// Represents the config file for the KM by KC app.
#[derive(Clone, Debug)]
pub enum Config {
    /// The mobile config file.
    Mobile(ConfigMobile),
    /// The web config file.
    Web(ConfigWeb),
}

impl From<ConfigMobile> for Config {
    fn from(value: ConfigMobile) -> Self {
        Config::Mobile(value)
    }
}
impl From<ConfigWeb> for Config {
    fn from(value: ConfigWeb) -> Self {
        Config::Web(value)
    }
}
