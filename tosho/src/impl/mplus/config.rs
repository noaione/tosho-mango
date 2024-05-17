#![allow(clippy::derive_partial_eq_without_eq)]

use tosho_macros::EnumName;

pub const PREFIX: &str = "mplus";

/// Device type for M+ by S session.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration, EnumName,
)]
pub enum DeviceType {
    /// Android device.
    Android = 1,
    /// iOS device/Apple.
    Apple = 2,
    /// Web browser.
    Web = 3,
}

/// Represents the main config file for the MU! by SQ app.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Config {
    /// The UUID of the account/config.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The session ID of the account/config.
    #[prost(string, tag = "2")]
    pub session: ::prost::alloc::string::String,
    /// Username of the account.
    #[prost(string, optional, tag = "3")]
    pub username: ::core::option::Option<::prost::alloc::string::String>,
    /// The device type of the account/config.
    #[prost(enumeration = "DeviceType", tag = "9")]
    pub r#type: i32,
}

impl Config {
    /// Creates a new config from a session and device type.
    pub fn from_session(session: &str, r#type: DeviceType) -> Self {
        let id = uuid::Uuid::new_v4().to_string();

        Self {
            id,
            username: None,
            session: session.to_string(),
            r#type: r#type as i32,
        }
    }

    /// Create new config with ID
    pub fn with_id(&self, id: &str) -> Self {
        Self {
            id: id.to_string(),
            session: self.session.clone(),
            username: self.username.clone(),
            r#type: self.r#type,
        }
    }

    /// Create new config with username
    pub fn with_username(&self, username: &str) -> Self {
        Self {
            id: self.id.clone(),
            session: self.session.clone(),
            username: Some(username.to_string()),
            r#type: self.r#type,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numbering_device_type() {
        assert_eq!(DeviceType::Android as i32, 1);
        assert_eq!(DeviceType::Apple as i32, 2);
    }

    #[test]
    fn test_numbering_u8_device_type() {
        assert_eq!(DeviceType::Android as u8, 1);
        assert_eq!(DeviceType::Apple as u8, 2);
    }
}
