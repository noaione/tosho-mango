//! A module containing all the models/proto mapping used in the library.
//!
//! The following module contains almost all the proto models used in the original app.
//! Including all the outdated/unused/deprecated models for preservation purpose.
//!
//! This module is manually created, if something is missing or broken, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose)
//! or a [pull request](https://github.com/noaione/tosho-mango/compare).

#![allow(clippy::derive_partial_eq_without_eq)]

pub mod accounts;
pub mod chapters;
pub mod comments;
pub mod common;
pub mod enums;
pub mod home_view;
pub mod others;
pub mod titles;

pub use accounts::*;
pub use chapters::*;
pub use comments::*;
pub use common::*;
pub use enums::*;
pub use home_view::*;
pub use others::*;
pub use titles::*;

/// Indicate a success response
///
/// Depending on the request type, not all field will be available.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuccessResponse {
    #[prost(bool, optional, tag = "1")]
    pub featured: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "2")]
    pub registration: ::core::option::Option<RegistrationData>,
    #[prost(message, optional, tag = "3")]
    pub home_view: ::core::option::Option<HomeView>,
    #[prost(message, optional, tag = "4")]
    pub featured_titles: ::core::option::Option<FeaturedTitles>,
    #[prost(message, optional, tag = "5")]
    pub all_titles: ::core::option::Option<TitleListOnly>,
    #[prost(message, optional, tag = "6")]
    pub title_ranking: ::core::option::Option<TitleListOnly>,
    #[prost(message, optional, tag = "7")]
    pub subscribed_titles: ::core::option::Option<TitleListOnly>,
    #[prost(message, optional, tag = "8")]
    pub title_detail: ::core::option::Option<TitleDetail>,
    #[prost(message, optional, tag = "9")]
    pub comment_list: ::core::option::Option<CommentList>,
    #[prost(message, optional, tag = "10")]
    pub chapter_viewer: ::core::option::Option<ChapterViewer>,
    #[prost(message, optional, tag = "11")]
    pub web_home_view: ::core::option::Option<WebHomeView>,
    #[prost(message, optional, tag = "12")]
    pub user_settings: ::core::option::Option<UserSettings>,
    #[prost(message, optional, tag = "13")]
    pub user_profile_settings: ::core::option::Option<UserProfileSettings>,
    #[prost(message, optional, tag = "14")]
    pub update_profile_result: ::core::option::Option<UserUpdateProfileResult>,
    #[prost(message, optional, tag = "15")]
    pub service_announcements: ::core::option::Option<ServiceAnnouncements>,
    #[prost(message, optional, tag = "16")]
    pub initial_view: ::core::option::Option<InitialView>,
    #[prost(message, optional, tag = "17")]
    pub feedback_view: ::core::option::Option<FeedbackList>,
    #[prost(message, optional, tag = "18")]
    pub publisher_news_list: ::core::option::Option<PublisherNewsList>,
    #[prost(message, optional, tag = "19")]
    pub questionnaire: ::core::option::Option<QuestionnaireResponse>,
    #[prost(message, optional, tag = "20")]
    pub title_updates: ::core::option::Option<TitleUpdates>,
    #[prost(message, optional, tag = "21")]
    pub home_view_v2: ::core::option::Option<HomeViewV2>,
    #[prost(message, optional, tag = "22")]
    pub updated_titles: ::core::option::Option<UpdatedTitleList>,
    #[prost(message, optional, tag = "23")]
    pub title_tickets: ::core::option::Option<TitleTicketList>,
    #[prost(message, optional, tag = "24")]
    pub home_view_v3: ::core::option::Option<HomeViewV3>,
    #[prost(message, optional, tag = "25")]
    pub all_titles_v2: ::core::option::Option<TitleListOnlyV2>,
    #[prost(message, optional, tag = "26")]
    pub user_settings_v2: ::core::option::Option<UserSettingsV2>,
    #[prost(message, optional, tag = "27")]
    pub title_updates_v2: ::core::option::Option<TitleUpdatesV2>,
    #[prost(message, optional, tag = "28")]
    pub initial_view_v2: ::core::option::Option<InitialViewV2>,
    #[prost(message, optional, tag = "29")]
    pub languages: ::core::option::Option<Languages>,
    #[prost(message, optional, tag = "30")]
    pub web_home_view_v2: ::core::option::Option<WebHomeViewV2>,
    #[prost(message, optional, tag = "31")]
    pub web_home_view_v3: ::core::option::Option<WebHomeViewV3>,
    #[prost(message, optional, tag = "32")]
    pub push_token: ::core::option::Option<PushTokenResponse>,
    #[prost(message, optional, tag = "33")]
    pub free_titles: ::core::option::Option<FreeTitles>,
    #[prost(message, optional, tag = "34")]
    pub labelled_titles: ::core::option::Option<LabelledTitles>,
    #[prost(message, optional, tag = "35")]
    pub search_results: ::core::option::Option<SearchResults>,
    #[prost(message, optional, tag = "36")]
    pub subscriptions: ::core::option::Option<SubscriptionResponse>,
    #[prost(message, optional, tag = "37")]
    pub title_ranking_v2: ::core::option::Option<TitleRankingList>,
    #[prost(message, optional, tag = "38")]
    pub web_home_view_v4: ::core::option::Option<WebHomeViewV4>,
    #[prost(message, optional, tag = "39")]
    pub featured_titles_v2: ::core::option::Option<FeaturedTitlesV2>,
}

/// A success or error response enum
///
/// This is used like `oneOf` in Protobuf so only one of them will be available.
#[derive(Clone, PartialEq, ::prost::Oneof)]
pub enum SuccessOrError {
    #[prost(message, tag = "1")]
    Success(Box<SuccessResponse>),
    #[prost(message, tag = "2")]
    Error(Box<ErrorResponse>),
}

/// Proto response from the API, wrap a simple `oneOf` of [`SuccessOrError`]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(oneof = "SuccessOrError", tags = "1, 2")]
    pub response: ::core::option::Option<SuccessOrError>,
}
