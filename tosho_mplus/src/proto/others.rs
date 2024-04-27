//! A module containing information related to other models.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

#![allow(clippy::derive_partial_eq_without_eq)]

use super::AvailableLanguages;

/// A service announcement
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceAnnouncement {
    /// The announcement title
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// The announcement body
    #[prost(string, tag = "2")]
    pub body: ::prost::alloc::string::String,
    /// The announcement timestamp
    #[prost(sint64, tag = "3")]
    pub timestamp: i64,
    /// The annnouncement ID
    #[prost(uint64, tag = "4")]
    pub id: u64,
}

/// A list of service announcements
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceAnnouncements {
    /// The list of announcements
    #[prost(message, repeated, tag = "1")]
    pub announcements: ::prost::alloc::vec::Vec<ServiceAnnouncement>,
}

/// A feedback response
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Feedback {
    /// The feedback creation timestamp
    #[prost(sint64, tag = "1")]
    pub timestamp: i64,
    /// The feedback content
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// The feedback type
    #[prost(enumeration = "super::FeedbackType", tag = "3")]
    pub kind: i32,
}

/// A list of feedback
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedbackList {
    /// The list of feedback
    #[prost(message, repeated, tag = "1")]
    pub feedbacks: ::prost::alloc::vec::Vec<Feedback>,
}

/// Publisher news list response
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublisherNewsList {
    /// The publisher ID
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// The publisher name
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// The publisher banner
    #[prost(message, optional, tag = "3")]
    pub banner: ::core::option::Option<super::common::Banner>,
    /// The list of news
    #[prost(message, repeated, tag = "4")]
    pub news: ::prost::alloc::vec::Vec<super::common::PublisherNews>,
}

/// The response for initial view of the app
///
/// The following is `v1` implementation of the initial view response.
///
/// See also: [`InitialView`]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitialView {
    /// Should the user agree to the GDPR or not
    #[prost(bool, tag = "1")]
    pub gdpr_required: bool,
    /// Total of english translated titles
    #[prost(uint64, tag = "2")]
    pub english_titles: u64,
    /// Total of spanish translated titles
    #[prost(uint64, tag = "3")]
    pub spanish_titles: u64,
}

/// The response for intial view of the app
///
/// The following is `v2` implementation of the initial view response.
///
/// See also: [`InitialView`]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitialViewV2 {
    /// Should the user agree to the GDPR or not
    #[prost(bool, tag = "1")]
    pub gdpr_required: bool,
    /// A list of available languages and titles count
    #[prost(message, repeated, tag = "2")]
    pub languages: ::prost::alloc::vec::Vec<AvailableLanguages>,
}
