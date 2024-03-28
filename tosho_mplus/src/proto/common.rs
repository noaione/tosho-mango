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
