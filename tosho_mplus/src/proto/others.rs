//! A module containing information related to other models.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

#![allow(clippy::derive_partial_eq_without_eq)]

use tosho_macros::AutoGetter;

use super::AvailableLanguages;

/// A service announcement
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct ServiceAnnouncement {
    /// The announcement title
    #[prost(string, tag = "1")]
    title: ::prost::alloc::string::String,
    /// The announcement body
    #[prost(string, tag = "2")]
    body: ::prost::alloc::string::String,
    /// The announcement timestamp
    #[prost(int64, tag = "3")]
    timestamp: i64,
    /// The annnouncement ID
    #[prost(uint64, tag = "4")]
    id: u64,
}

/// A list of service announcements
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct ServiceAnnouncements {
    /// The list of announcements
    #[prost(message, repeated, tag = "1")]
    announcements: ::prost::alloc::vec::Vec<ServiceAnnouncement>,
}

/// A feedback response
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct Feedback {
    /// The feedback creation timestamp
    #[prost(int64, tag = "1")]
    timestamp: i64,
    /// The feedback content
    #[prost(string, tag = "2")]
    title: ::prost::alloc::string::String,
    /// The feedback type
    #[prost(enumeration = "super::FeedbackType", tag = "3")]
    #[skip_field]
    kind: i32,
}

/// A list of feedback
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct FeedbackList {
    /// The list of feedback
    #[prost(message, repeated, tag = "1")]
    feedbacks: ::prost::alloc::vec::Vec<Feedback>,
}

/// Publisher news list response
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct PublisherNewsList {
    /// The publisher ID
    #[prost(uint64, tag = "1")]
    id: u64,
    /// The publisher name
    #[prost(string, tag = "2")]
    name: ::prost::alloc::string::String,
    /// The publisher banner
    #[prost(message, optional, tag = "3")]
    banner: ::core::option::Option<super::common::Banner>,
    /// The list of news
    #[prost(message, repeated, tag = "4")]
    news: ::prost::alloc::vec::Vec<super::common::PublisherNews>,
}

/// A single questionnaire
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct Questionnaire {
    /// The question
    #[prost(string, tag = "1")]
    question: ::prost::alloc::string::String,
    /// The selection of answers
    #[prost(string, repeated, tag = "2")]
    answers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Total number of choices
    #[prost(uint64, tag = "3")]
    total_choices: u64,
    /// Hide free form answer
    #[prost(bool, tag = "4")]
    hide_free_form: bool,
    /// Free form answer
    #[prost(string, optional, tag = "5")]
    #[skip_field]
    free_form: ::core::option::Option<::prost::alloc::string::String>,
    /// Can skip the question or not
    #[prost(bool, tag = "6")]
    can_skip: bool,
}

/// A questionnaire response
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct QuestionnaireResponse {
    /// Is this questionnaire answered already?
    #[prost(bool, tag = "1")]
    answered: bool,
    /// The subject of the questionnaire
    #[prost(string, tag = "2")]
    subject: ::prost::alloc::string::String,
    /// The list of questions
    #[prost(message, repeated, tag = "3")]
    questions: ::prost::alloc::vec::Vec<Questionnaire>,
    /// The language of the questionnaire
    #[prost(enumeration = "super::Language", tag = "4")]
    #[skip_field]
    language: i32,
}

/// The response for initial view of the app
///
/// The following is `v1` implementation of the initial view response.
///
/// See also: [`InitialView`]
#[derive(Clone, Copy, AutoGetter, PartialEq, ::prost::Message)]
pub struct InitialView {
    /// Should the user agree to the GDPR or not
    #[prost(bool, tag = "1")]
    gdpr_required: bool,
    /// Total of english translated titles
    #[prost(uint64, tag = "2")]
    english_titles: u64,
    /// Total of spanish translated titles
    #[prost(uint64, tag = "3")]
    spanish_titles: u64,
}

/// The response for intial view of the app
///
/// The following is `v2` implementation of the initial view response.
///
/// See also: [`InitialView`]
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct InitialViewV2 {
    /// Should the user agree to the GDPR or not
    #[prost(bool, tag = "1")]
    gdpr_required: bool,
    /// A list of available languages and titles count
    #[prost(message, repeated, tag = "2")]
    languages: ::prost::alloc::vec::Vec<AvailableLanguages>,
}

/// The search contents
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct SearchContents {
    /// The banner for search
    #[prost(message, optional, tag = "1")]
    banner: ::core::option::Option<super::common::Banner>,
    /// The list titles that match the search
    #[prost(message, repeated, tag = "2")]
    titles: ::prost::alloc::vec::Vec<super::titles::TitleList>,
    /// The ranked titles
    #[prost(message, repeated, tag = "3")]
    rankings: ::prost::alloc::vec::Vec<super::titles::TitleRankingGroup>,
    /// All the labels for this content
    #[prost(message, repeated, tag = "4")]
    labels: ::prost::alloc::vec::Vec<super::common::Label>,
}

/// The response for search result
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct SearchResults {
    /// The top search banners
    #[prost(message, repeated, tag = "1")]
    top_banners: ::prost::alloc::vec::Vec<super::common::Banner>,
    /// The list of tags that can be used on the search
    #[prost(message, repeated, tag = "2")]
    tags: ::prost::alloc::vec::Vec<super::common::Tag>,
    /// The list of titles that match the search
    #[prost(message, repeated, tag = "3")]
    titles: ::prost::alloc::vec::Vec<super::titles::TitleListV2>,
    /// The list of search results contents
    #[prost(message, repeated, tag = "5")]
    contents: ::prost::alloc::vec::Vec<SearchContents>,
}

/// The subscriptions list response
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct SubscriptionResponse {
    /// The user subscription
    #[prost(message, optional, tag = "1")]
    subscription: ::core::option::Option<super::accounts::UserSubscription>,
    /// The available plans
    #[prost(message, repeated, tag = "2")]
    plans: ::prost::alloc::vec::Vec<super::common::Plan>,
    /// The available titles for each plans
    #[prost(message, repeated, tag = "3")]
    titles: ::prost::alloc::vec::Vec<super::titles::SubscriptionTitles>,
    /// User has already used the free trial
    #[prost(bool, tag = "4")]
    free_trial_used: bool,
}
