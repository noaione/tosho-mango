#![allow(clippy::derive_partial_eq_without_eq)]

use tosho_macros::EnumName;

pub const PREFIX: &str = "musq";

/// Device type for MU! by SQ session.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration, EnumName,
)]
pub enum DeviceType {
    /// Android device.
    Android = 1,
    /// iOS device/Apple.
    Apple = 2,
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
    /// The device type of the account/config.
    #[prost(enumeration = "DeviceType", tag = "3")]
    pub r#type: i32,
}

impl Config {
    /// Creates a new config from a session and device type.
    pub fn from_session(session: &str, r#type: DeviceType) -> Self {
        let id = uuid::Uuid::new_v4().to_string();

        Self {
            id,
            session: session.to_string(),
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
        assert_eq!(DeviceType::Android as i32, 1);
        assert_eq!(DeviceType::Apple as i32, 2);
    }

    #[test]
    fn test_numbering_u8_device_type() {
        assert_eq!(DeviceType::Android as u8, 1);
        assert_eq!(DeviceType::Apple as u8, 2);
    }
}
