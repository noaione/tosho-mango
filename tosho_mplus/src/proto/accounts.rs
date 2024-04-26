//! A module containing information related to user account.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

#![allow(clippy::derive_partial_eq_without_eq)]

use super::CommentIcon;

/// Registration data response
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegistrationData {
    /// Device secret
    #[prost(string, tag = "1")]
    pub secret: ::prost::alloc::string::String,
}

/// User account ticket information
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserTickets {
    /// Current ticket amount
    #[prost(uint64, tag = "1")]
    pub ticket: u64,
    /// Next ticket refresh in UNIX timestamp
    #[prost(sint64, tag = "2")]
    pub next_refresh: i64,
}

/// User account subscription information
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserSubscription {
    /// Subscription plan type
    ///
    /// TODO: Move into enum when all plans are known
    #[prost(string, tag = "1")]
    pub plan: ::prost::alloc::string::String,
    /// Next payment in UNIX timestamp
    #[prost(sint64, tag = "2")]
    pub next_payment: i64,
    /// Is the current subscription a trial?
    #[prost(bool, tag = "3")]
    pub trial: bool,
    /// Is the subscription is currently on the process being downgraded?
    #[prost(bool, tag = "4")]
    pub downgrading: bool,
}

/// User settings response
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserSettings {
    /// User icon info
    #[prost(message, optional, tag = "1")]
    pub icon: ::core::option::Option<CommentIcon>,
    /// User display name
    #[prost(string, tag = "2")]
    pub user_name: ::prost::alloc::string::String,
    /// User wants notification for news and events
    #[prost(bool, tag = "3")]
    pub news_notification: bool,
    /// User wants notification for chapter updates
    #[prost(bool, tag = "4")]
    pub chapter_notification: bool,
    /// English title count
    #[prost(uint64, tag = "5")]
    pub english_title_count: u64,
}

/// User profile settings response
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserProfileSettings {
    /// A list of available icons
    #[prost(message, repeated, tag = "1")]
    pub icons: ::prost::alloc::vec::Vec<CommentIcon>,
    /// User display name
    #[prost(string, tag = "2")]
    pub user_name: ::prost::alloc::string::String,
    /// User icon info
    #[prost(message, optional, tag = "3")]
    pub icon: ::core::option::Option<CommentIcon>,
}
