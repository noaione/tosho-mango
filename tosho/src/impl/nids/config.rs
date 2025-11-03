#![allow(clippy::derive_partial_eq_without_eq)]

use tosho_macros::{AutoGetter, EnumName};

pub const PREFIX: &str = "nids";

/// Device type for NI by DS session.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration, EnumName,
)]
pub enum DeviceType {
    /// Web browser.
    Web = 1,
}

/// Represents the main config file for the NI by DS app.
#[derive(Clone, PartialEq, AutoGetter, ::prost::Message)]
pub struct Config {
    /// The UUID of the account/config.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The session token of the account/config.
    #[prost(string, tag = "2")]
    pub session: ::prost::alloc::string::String,
    /// The email of the account/config.
    #[prost(string, tag = "3")]
    pub email: ::prost::alloc::string::String,
    /// The username of the account/config.
    #[prost(string, optional, tag = "4")]
    #[skip_field]
    pub username: Option<::prost::alloc::string::String>,
    /// The refresh token of the account/config.
    #[prost(string, optional, tag = "5")]
    #[skip_field]
    pub refresh_token: Option<::prost::alloc::string::String>,
    /// The device type of the account/config.
    #[prost(enumeration = "DeviceType", tag = "10")]
    #[skip_field]
    pub r#type: i32,
}

impl Config {
    /// Get the ID of the config.
    pub fn get_id(&self) -> &str {
        &self.id
    }

    /// Creates a new config from a session and device type.
    pub fn from_session(session: &str, r#type: DeviceType) -> Self {
        let id = uuid::Uuid::new_v4().to_string();

        Self {
            id,
            session: session.to_string(),
            refresh_token: None,
            email: String::new(),
            username: None,
            r#type: r#type as i32,
        }
    }

    /// Apply the old ID to the new config.
    pub fn apply_id(&mut self, old_id: &str) {
        self.id = old_id.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numbering_device_type() {
        assert_eq!(DeviceType::Web as i32, 1);
    }

    #[test]
    fn test_numbering_u8_device_type() {
        assert_eq!(DeviceType::Web as u8, 1);
    }
}
