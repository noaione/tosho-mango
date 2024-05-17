//! A module containing information related to user account.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

#![allow(clippy::derive_partial_eq_without_eq)]

use std::str::FromStr;

use crate::helper::SubscriptionPlan;

use super::{AvailableLanguages, CommentIcon};

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
    #[prost(int64, tag = "2")]
    pub next_refresh: i64,
}

/// User account subscription information
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserSubscription {
    /// Subscription plan type
    #[prost(string, tag = "1")]
    pub plan: ::prost::alloc::string::String,
    /// Next payment in UNIX timestamp
    #[prost(uint64, tag = "2")]
    pub next_payment: u64,
    /// Is the current subscription a trial?
    #[prost(bool, tag = "3")]
    pub trial: bool,
    /// Is the subscription is currently on the process being downgraded?
    #[prost(bool, tag = "4")]
    pub downgrading: bool,
}

impl UserSubscription {
    /// Get the actual subscriptions plan type
    ///
    /// This will return the actual [`SubscriptionPlan`] type
    /// and fallback to [`SubscriptionPlan::Basic`] if the plan is not recognized.
    pub fn plan(&self) -> SubscriptionPlan {
        match SubscriptionPlan::from_str(&self.plan) {
            Ok(plan) => plan,
            Err(_) => SubscriptionPlan::Basic,
        }
    }
}

/// User settings response
///
/// This is a `v1` implementation of the user settings response.
///
/// See also: [`UserSettingsV2`]
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

/// User settings response
///
/// This is a `v2` implementation of the user settings response.
///
/// See also: [`UserSettings`]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserSettingsV2 {
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
    /// Available languages with title count
    #[prost(message, repeated, tag = "5")]
    pub languages: ::prost::alloc::vec::Vec<AvailableLanguages>,
    /// User subscription information
    #[prost(message, optional, tag = "6")]
    pub subscription: ::core::option::Option<UserSubscription>,
}

/// User profile settings response
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserProfileSettings {
    /// A list of available icons
    #[prost(message, repeated, tag = "1")]
    pub icons: ::prost::alloc::vec::Vec<CommentIcon>,
    /// User display name
    #[prost(string, optional, tag = "2")]
    pub user_name: ::core::option::Option<::prost::alloc::string::String>,
    /// User icon info
    #[prost(message, optional, tag = "3")]
    pub icon: ::core::option::Option<CommentIcon>,
}

/// User profile update result response
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserUpdateProfileResult {
    /// Result of the profile update
    #[prost(enumeration = "super::UpdateProfileResult", tag = "1")]
    pub result: i32,
}

/// Push token response
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushTokenResponse {
    /// The received token
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
    /// The token timestamp
    #[prost(int64, optional, tag = "2")]
    pub timestamp: ::core::option::Option<i64>,
}
