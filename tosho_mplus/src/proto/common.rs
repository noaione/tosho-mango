//! A module containing some common models.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

#![allow(clippy::derive_partial_eq_without_eq)]

use super::enums::{ErrorAction, Language};

/// A popup button action
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PopupButton {
    /// The button action.
    #[prost(string, optional, tag = "1")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
    // There is also a `action` but it's tied to system.
    // So we don't need to implement it.
}

/// A default Popup response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PopupMessage {
    /// The message subject.
    #[prost(string, optional, tag = "1")]
    pub subject: ::core::option::Option<::prost::alloc::string::String>,
    /// The message body.
    #[prost(string, optional, tag = "2")]
    pub body: ::core::option::Option<::prost::alloc::string::String>,
    /// The OK button action.
    #[prost(message, optional, tag = "3")]
    pub ok_button: ::core::option::Option<PopupButton>,
    /// The neutral button action.
    #[prost(message, optional, tag = "4")]
    pub neutral_button: ::core::option::Option<PopupButton>,
    /// The cancel button action.
    #[prost(message, optional, tag = "5")]
    pub cancel_button: ::core::option::Option<PopupButton>,
    /// The language of the message.
    #[prost(enumeration = "Language", tag = "6")]
    pub language: i32,
}

impl PopupMessage {
    /// Format the popup message into a string.
    pub fn as_string(&self) -> String {
        let mut message = String::new();
        if let Some(subject) = &self.subject {
            message.push_str(subject);
            message.push_str(": ");
        }
        if let Some(body) = &self.body {
            message.push_str(body);
        }
        message
    }
}

/// An error response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorResponse {
    /// The error action or error code of the request.
    #[prost(enumeration = "ErrorAction", tag = "1")]
    pub action: i32,
    /// English popup message
    #[prost(message, optional, tag = "2")]
    pub english_popup: ::core::option::Option<PopupMessage>,
    /// Spanish popup message
    #[prost(message, optional, tag = "3")]
    pub spanish_popup: ::core::option::Option<PopupMessage>,
    /// Debug message
    #[prost(string, optional, tag = "4")]
    pub debug_message: ::core::option::Option<::prost::alloc::string::String>,
    /// Array of other popup messages
    #[prost(message, repeated, tag = "5")]
    pub other_popups: ::prost::alloc::vec::Vec<PopupMessage>,
}

impl ErrorResponse {
    /// Format the error response into a string.
    pub fn as_string(&self) -> String {
        let popup_message = self.english_popup.as_ref().map(|popup| popup.as_string());
        match self.action() {
            ErrorAction::Default => {
                let mut message = String::new();
                message.push_str("An error occurred");
                if let Some(popup_message) = popup_message {
                    message.push_str(&format!(": {}", popup_message));
                }
                if let Some(debug_message) = &self.debug_message {
                    message.push_str(&format!("\nDebug: {}", debug_message));
                }
                message
            }
            ErrorAction::Unauthorized => {
                let mut message = String::new();
                message.push_str("You are not authorized to access this resource");
                if let Some(debug_message) = &self.debug_message {
                    message.push_str(&format!("\nDebug: {}", debug_message));
                }
                message
            }
            ErrorAction::Maintenance => {
                let mut message = String::new();
                message.push_str("Server is under maintenance");
                if let Some(debug_message) = &self.debug_message {
                    message.push_str(&format!("\nDebug: {}", debug_message));
                }
                message
            }
            ErrorAction::GeoIPBlocked => {
                let mut message = String::new();
                message.push_str("Your request is blocked by GeoIP");
                if let Some(debug_message) = &self.debug_message {
                    message.push_str(&format!("\nDebug: {}", debug_message));
                }
                message
            }
            ErrorAction::Unrecognized => {
                let mut message = String::new();
                message.push_str("An unknown error occurred");
                if let Some(debug_message) = &self.debug_message {
                    message.push_str(&format!("\nDebug: {}", debug_message));
                }
                message
            }
        }
    }
}

/// The banner data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Banner {
    /// The banner image.
    #[prost(string, tag = "1")]
    pub image: ::prost::alloc::string::String,
    /// The associated ID.
    #[prost(uint64, optional, tag = "3")]
    pub id: Option<u64>,
    /// The banner width.
    #[prost(uint32, optional, tag = "4")]
    pub width: Option<u32>,
    /// The banner height.
    #[prost(uint32, optional, tag = "5")]
    pub height: Option<u32>,
}

/// A tag information
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tag {
    /// Tag ID
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// Tag slug
    #[prost(string, tag = "2")]
    pub slug: ::prost::alloc::string::String,
}

/// A label information
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Label {
    /// Label ID
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// Label description
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
}

/// A publisher news information
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublisherNews {
    /// Publisher news ID
    #[prost(uint64, tag = "1")]
    pub news_id: u64,
    /// Publisher ID
    #[prost(uint64, tag = "2")]
    pub publisher_id: u64,
    /// Publisher name
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Publisher news title
    #[prost(string, tag = "4")]
    pub title: ::prost::alloc::string::String,
    /// Publisher news content
    #[prost(string, tag = "5")]
    pub content: ::prost::alloc::string::String,
    /// Time of publication in UNIX timestamp
    #[prost(sint64, tag = "6")]
    pub published_at: i64,
}

/// A publisher information
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublisherItem {
    /// The publisher banner info
    #[prost(message, optional, tag = "1")]
    pub banner: ::core::option::Option<Banner>,
    /// The publisher news
    #[prost(message, optional, tag = "2")]
    pub news: ::core::option::Option<PublisherNews>,
}

/// Available languages in the source.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AvailableLanguages {
    /// The language
    #[prost(enumeration = "Language", tag = "1")]
    pub language: i32,
    /// Total count of titles available in the language
    #[prost(uint64, tag = "2")]
    pub titles_count: u64,
}
