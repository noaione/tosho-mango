#![allow(clippy::derive_partial_eq_without_eq)]

use clap::ValueEnum;
use tosho_macros::EnumName;
use tosho_sjv::{SJConfig, SJMode};

pub const PREFIX: &str = "sjv";

/// Device type for AM by AP session.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration, EnumName,
)]
pub enum DeviceType {
    /// Android device.
    Android = 4,
}

/// Device type for AM by AP session.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration, EnumName,
)]
pub enum SJDeviceMode {
    /// VM (Manga) mode.
    VM = 1,
    /// SJ (Jump) mode.
    SJ = 3,
}

impl From<SJMode> for SJDeviceMode {
    fn from(value: SJMode) -> Self {
        match value {
            SJMode::VM => Self::VM,
            SJMode::SJ => Self::SJ,
        }
    }
}

impl ValueEnum for SJDeviceMode {
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Self::VM => Some(clap::builder::PossibleValue::new("vm")),
            Self::SJ => Some(clap::builder::PossibleValue::new("sj")),
        }
    }

    fn value_variants<'a>() -> &'a [Self] {
        &[Self::VM, Self::SJ]
    }

    fn from_str(input: &str, ignore_case: bool) -> Result<Self, String> {
        let input = if ignore_case {
            input.to_lowercase()
        } else {
            input.to_string()
        };
        match input.as_str() {
            "vm" => Ok(Self::VM),
            "sj" => Ok(Self::SJ),
            _ => Err(format!("Invalid SJ device mode: {}", input)),
        }
    }
}

/// Represents the main config file for the SJ/M by V app.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Config {
    /// The UUID of the account/config.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The email of the account/config.
    #[prost(string, tag = "2")]
    pub email: ::prost::alloc::string::String,
    /// The username of the account/config.
    #[prost(string, tag = "3")]
    pub username: ::prost::alloc::string::String,
    /// The user ID of the account/config.
    #[prost(uint32, tag = "4")]
    pub user_id: u32,
    /// The token of the account/config.
    #[prost(string, tag = "5")]
    pub token: ::prost::alloc::string::String,
    /// THe instance ID of the account/config.
    #[prost(string, tag = "6")]
    pub instance: ::prost::alloc::string::String,
    /// The device type of the account/config.
    #[prost(enumeration = "DeviceType", tag = "10")]
    pub r#type: i32,
    /// The mode of the account/config.
    #[prost(enumeration = "SJDeviceMode", tag = "11")]
    pub mode: i32,
}

impl From<SJConfig> for Config {
    fn from(value: SJConfig) -> Self {
        let new_uuid = uuid::Uuid::new_v4().to_string();
        Self {
            id: new_uuid.clone(),
            email: format!("{}@sjv.xyz", new_uuid),
            username: "sjv_temp".to_string(),
            user_id: value.user_id,
            token: value.token,
            instance: value.instance,
            r#type: DeviceType::Android as i32,
            mode: SJDeviceMode::SJ as i32,
        }
    }
}

impl From<Config> for SJConfig {
    fn from(value: Config) -> Self {
        Self {
            user_id: value.user_id,
            token: value.token,
            instance: value.instance,
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

    /// Apply email to the new config.
    pub fn with_email(self, email: &str) -> Self {
        Self {
            email: email.to_string(),
            ..self
        }
    }

    /// Apply username to the new config.
    pub fn with_username(self, username: &str) -> Self {
        Self {
            username: username.to_string(),
            ..self
        }
    }

    /// Apply mode to the new config.
    pub fn with_mode(self, mode: SJDeviceMode) -> Self {
        Self {
            mode: mode as i32,
            ..self
        }
    }
}
