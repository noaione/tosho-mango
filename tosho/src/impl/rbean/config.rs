#![allow(clippy::derive_partial_eq_without_eq)]

use clap::ValueEnum;
use tosho_macros::EnumName;
use tosho_rbean::{RBConfig, RBLoginResponse, RBPlatform};

pub const PREFIX: &str = "rbean";

/// Device type for 小豆 by KRKR session.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration, EnumName,
)]
pub enum DeviceType {
    /// Android device.
    Android = 1,
    /// iOS device/Apple.
    Apple = 2,
    /// Web device.
    Web = 3,
}

impl ValueEnum for DeviceType {
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Self::Android => Some(clap::builder::PossibleValue::new("android")),
            Self::Apple => Some(clap::builder::PossibleValue::new("apple")),
            Self::Web => Some(clap::builder::PossibleValue::new("web")),
        }
    }

    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Android, Self::Apple, Self::Web]
    }

    fn from_str(input: &str, ignore_case: bool) -> Result<Self, String> {
        let input = if ignore_case {
            input.to_lowercase()
        } else {
            input.to_string()
        };
        match input.as_str() {
            "android" => Ok(Self::Android),
            "apple" => Ok(Self::Apple),
            "web" => Ok(Self::Web),
            _ => Err(format!("Invalid RB device mode: {}", input)),
        }
    }
}

/// Represents the main config file for the 小豆 by KRKR app.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Config {
    /// The UUID of the account/config.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The username of the account/config.
    #[prost(string, tag = "2")]
    pub username: ::prost::alloc::string::String,
    /// The UUID of the user.
    #[prost(string, tag = "3")]
    pub user_id: ::prost::alloc::string::String,
    /// The email of the user.
    #[prost(string, tag = "4")]
    pub email: ::prost::alloc::string::String,
    /// Access token for the account.
    #[prost(string, tag = "5")]
    pub access_token: ::prost::alloc::string::String,
    /// Refresh token for the account.
    #[prost(string, tag = "6")]
    pub refresh_token: ::prost::alloc::string::String,
    /// Last expiry time of the access token.
    #[prost(int64, tag = "7")]
    pub expiry: i64,
    /// The device type of the account/config.
    #[prost(enumeration = "DeviceType", tag = "10")]
    pub platform: i32,
}

impl From<RBLoginResponse> for Config {
    fn from(value: RBLoginResponse) -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        let platform_id = match value.platform {
            RBPlatform::Android => DeviceType::Android,
            RBPlatform::Apple => DeviceType::Apple,
            RBPlatform::Web => DeviceType::Web,
        };

        let username = value.user.username.unwrap_or("[no username]".to_string());

        Self {
            id,
            username,
            user_id: value.user.uuid,
            email: value.user.email,
            access_token: value.token,
            refresh_token: value.refresh_token,
            expiry: value.expiry,
            platform: platform_id as i32,
        }
    }
}

impl From<Config> for RBConfig {
    fn from(value: Config) -> Self {
        Self {
            token: value.access_token.clone(),
            refresh_token: value.refresh_token.clone(),
            platform: match value.platform() {
                DeviceType::Android => RBPlatform::Android,
                DeviceType::Apple => RBPlatform::Apple,
                DeviceType::Web => RBPlatform::Web,
            },
        }
    }
}

impl Config {
    /// Apply the old ID to the new config.
    pub fn with_id(self, id: &str) -> Self {
        Self {
            id: id.to_string(),
            ..self
        }
    }
}
