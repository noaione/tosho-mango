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
    #[prost(sfixed64, tag = "6")]
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
