#![allow(clippy::derive_partial_eq_without_eq)]

use tosho_amap::{models::AccountUserInfo, AMConfig};
use tosho_macros::EnumName;

pub const PREFIX: &str = "amap";

/// Device type for AM by AP session.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration, EnumName,
)]
pub enum DeviceType {
    /// Android device.
    Android = 1,
}

/// Represents the main config file for the AM by AP app.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Config {
    /// The UUID of the account/config.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The email of the account/config.
    #[prost(string, tag = "2")]
    pub email: ::prost::alloc::string::String,
    /// The user ID of the account/config.
    #[prost(uint64, tag = "3")]
    pub user_id: u64,
    /// The token of the account/config.
    #[prost(string, tag = "4")]
    pub token: ::prost::alloc::string::String,
    /// THe identifier of the account/config.
    #[prost(string, tag = "5")]
    pub identifier: ::prost::alloc::string::String,
    /// The session of the account/config.
    #[prost(string, tag = "6")]
    pub session: ::prost::alloc::string::String,
    /// The device type of the account/config.
    #[prost(enumeration = "DeviceType", tag = "10")]
    pub r#type: i32,
}

impl From<AMConfig> for Config {
    fn from(value: AMConfig) -> Self {
        let new_uuid = uuid::Uuid::new_v4().to_string();
        Self {
            id: new_uuid.clone(),
            email: format!("{}@amap.xyz", new_uuid),
            user_id: 0,
            token: value.token,
            identifier: value.identifier,
            session: value.session_v2,
            r#type: DeviceType::Android as i32,
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

    /// Apply the account info response to the new config.
    pub fn with_account_info(self, info: &AccountUserInfo) -> Self {
        Self {
            user_id: info.id,
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
}
