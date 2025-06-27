//! A module containing some common models.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

#![allow(clippy::derive_partial_eq_without_eq)]

use std::str::FromStr;

use tosho_macros::AutoGetter;

use crate::helper::SubscriptionPlan;

use super::enums::{ErrorAction, Language};

/// A popup button action
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PopupButton {
    /// The button action.
    #[prost(string, optional, tag = "1")]
    text: ::core::option::Option<::prost::alloc::string::String>,
    // There is also a `action` but it's tied to system.
    // So we don't need to implement it.
}

/// A default Popup response.
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct PopupMessage {
    /// The message subject.
    #[prost(string, optional, tag = "1")]
    #[skip_field]
    subject: ::core::option::Option<::prost::alloc::string::String>,
    /// The message body.
    #[prost(string, optional, tag = "2")]
    #[skip_field]
    body: ::core::option::Option<::prost::alloc::string::String>,
    /// The OK button action.
    #[prost(message, optional, tag = "3")]
    ok_button: ::core::option::Option<PopupButton>,
    /// The neutral button action.
    #[prost(message, optional, tag = "4")]
    neutral_button: ::core::option::Option<PopupButton>,
    /// The cancel button action.
    #[prost(message, optional, tag = "5")]
    cancel_button: ::core::option::Option<PopupButton>,
    /// The language of the message.
    #[prost(enumeration = "Language", tag = "6")]
    #[skip_field]
    language: i32,
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
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct ErrorResponse {
    /// The error action or error code of the request.
    #[prost(enumeration = "ErrorAction", tag = "1")]
    #[skip_field]
    action: i32,
    /// English popup message
    #[prost(message, optional, tag = "2")]
    english_popup: ::core::option::Option<PopupMessage>,
    /// Spanish popup message
    #[prost(message, optional, tag = "3")]
    spanish_popup: ::core::option::Option<PopupMessage>,
    /// Debug message
    #[prost(string, optional, tag = "4")]
    #[skip_field]
    debug_message: ::core::option::Option<::prost::alloc::string::String>,
    /// Array of other popup messages
    #[prost(message, repeated, tag = "5")]
    other_popups: ::prost::alloc::vec::Vec<PopupMessage>,
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
                    message.push_str(&format!(": {popup_message}"));
                }
                if let Some(debug_message) = &self.debug_message {
                    message.push_str(&format!("\nDebug: {debug_message}"));
                }
                message
            }
            ErrorAction::Unauthorized => {
                let mut message = String::new();
                message.push_str("You are not authorized to access this resource");
                if let Some(debug_message) = &self.debug_message {
                    message.push_str(&format!("\nDebug: {debug_message}"));
                }
                message
            }
            ErrorAction::Maintenance => {
                let mut message = String::new();
                message.push_str("Server is under maintenance");
                if let Some(debug_message) = &self.debug_message {
                    message.push_str(&format!("\nDebug: {debug_message}"));
                }
                message
            }
            ErrorAction::GeoIPBlocked => {
                let mut message = String::new();
                message.push_str("Your request is blocked by GeoIP");
                if let Some(debug_message) = &self.debug_message {
                    message.push_str(&format!("\nDebug: {debug_message}"));
                }
                message
            }
            ErrorAction::Unrecognized => {
                let mut message = String::new();
                message.push_str("An unknown error occurred");
                if let Some(debug_message) = &self.debug_message {
                    message.push_str(&format!("\nDebug: {debug_message}"));
                }
                message
            }
        }
    }
}

/// The banner data.
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct Banner {
    /// The banner image.
    #[prost(string, tag = "1")]
    image: ::prost::alloc::string::String,
    /// The associated ID.
    #[prost(uint64, optional, tag = "3")]
    #[skip_field]
    id: Option<u64>,
    /// The banner width.
    #[prost(uint32, optional, tag = "4")]
    #[skip_field]
    width: Option<u32>,
    /// The banner height.
    #[prost(uint32, optional, tag = "5")]
    #[skip_field]
    height: Option<u32>,
}

/// A tag information
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct Tag {
    /// Tag name
    #[prost(string, tag = "1")]
    name: ::prost::alloc::string::String,
    /// Tag slug
    #[prost(string, tag = "2")]
    slug: ::prost::alloc::string::String,
}

/// A label information
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct Label {
    /// Label ID
    #[prost(uint64, tag = "1")]
    id: u64,
    /// Label description
    #[prost(string, tag = "2")]
    description: ::prost::alloc::string::String,
}

/// A publisher news information
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct PublisherNews {
    /// Publisher news ID
    #[prost(uint64, tag = "1")]
    news_id: u64,
    /// Publisher ID
    #[prost(uint64, tag = "2")]
    publisher_id: u64,
    /// Publisher name
    #[prost(string, tag = "3")]
    name: ::prost::alloc::string::String,
    /// Publisher news title
    #[prost(string, tag = "4")]
    title: ::prost::alloc::string::String,
    /// Publisher news content
    #[prost(string, tag = "5")]
    content: ::prost::alloc::string::String,
    /// Time of publication in UNIX timestamp
    #[prost(uint64, tag = "6")]
    published_at: u64,
}

/// A publisher information
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct PublisherItem {
    /// The publisher banner info
    #[prost(message, optional, tag = "1")]
    banner: ::core::option::Option<Banner>,
    /// The publisher news
    #[prost(message, optional, tag = "2")]
    news: ::core::option::Option<PublisherNews>,
}

/// Available languages in the source.
#[derive(Clone, AutoGetter, PartialEq, Copy, ::prost::Message)]
pub struct AvailableLanguages {
    /// The language
    #[prost(enumeration = "Language", tag = "1")]
    #[skip_field]
    language: i32,
    /// Total count of titles available in the language
    #[prost(uint64, tag = "2")]
    titles_count: u64,
}

/// A information about the current languages
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct Languages {
    /// The current UI language
    #[prost(enumeration = "Language", tag = "1")]
    #[skip_field]
    language: i32,
    /// The content language
    #[prost(enumeration = "Language", optional, tag = "2")]
    #[skip_field]
    content_language: ::core::option::Option<i32>,
    /// The secondary content language
    ///
    /// This will take priority over the first content language.
    #[prost(enumeration = "Language", optional, tag = "3")]
    #[skip_field]
    content_language_secondary: ::core::option::Option<i32>,
    /// The tertiary content language
    ///
    /// This will take priority over the first and second content language.
    #[prost(enumeration = "Language", optional, tag = "4")]
    #[skip_field]
    content_language_tertiary: ::core::option::Option<i32>,
    /// The available languages
    #[prost(message, repeated, tag = "5")]
    availables: ::prost::alloc::vec::Vec<AvailableLanguages>,
}

impl Languages {
    /// Get the current active content language.
    pub fn content_languages(&self) -> Language {
        match (
            self.content_language_tertiary,
            self.content_language_secondary,
            self.content_language,
        ) {
            (Some(_), _, _) => self.content_language_tertiary(),
            (_, Some(_), _) => self.content_language_secondary(),
            (_, _, Some(_)) => self.content_language(),
            _ => Language::English,
        }
    }
}

/// A subscription offer for Android device
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct SubscriptionOfferAndroid {
    /// The offer tag
    #[prost(string, tag = "1")]
    tag: ::prost::alloc::string::String,
}

/// A subscription offer for Apple device
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct SubscriptionOfferApple {
    /// The offer type
    #[prost(enumeration = "super::PlanOfferType", tag = "1")]
    #[skip_field]
    kind: i32,
    /// The signature info
    #[prost(string, tag = "2")]
    signature: ::prost::alloc::string::String,
    /// The apple key info
    #[prost(string, tag = "3")]
    key: ::prost::alloc::string::String,
    /// The nonce info
    #[prost(string, tag = "4")]
    nonce: ::prost::alloc::string::String,
    /// The timestamp info
    #[prost(string, tag = "5")]
    timestamp: ::prost::alloc::string::String,
    /// The identifier info
    #[prost(string, tag = "6")]
    identifier: ::prost::alloc::string::String,
}

/// A plan information
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct Plan {
    /// The plan name/ID
    #[prost(string, tag = "1")]
    #[skip_field]
    name: ::prost::alloc::string::String,
    /// The plan description
    #[prost(string, tag = "2")]
    description: ::prost::alloc::string::String,
    /// The plan product ID
    #[prost(string, tag = "3")]
    product_id: ::prost::alloc::string::String,
    /// The subscription offer for apple devices
    #[prost(message, optional, tag = "4")]
    apple_offer: ::core::option::Option<SubscriptionOfferApple>,
    /// The subscription offer for android devices
    #[prost(message, repeated, tag = "5")]
    android_offer: ::prost::alloc::vec::Vec<SubscriptionOfferAndroid>,
}

impl Plan {
    /// Get the actual subscriptions plan type
    ///
    /// This will return the actual [`SubscriptionPlan`] type
    /// and fallback to [`SubscriptionPlan::Basic`] if the plan is not recognized.
    pub fn name(&self) -> SubscriptionPlan {
        match SubscriptionPlan::from_str(&self.name) {
            Ok(plan) => plan,
            Err(_) => SubscriptionPlan::Basic,
        }
    }
}
