//! A module containing all the models/proto mapping used in the library.
//!
//! The following module contains almost all the proto models used in the original app.
//! Including all the outdated/unused/deprecated models for preservation purpose.
//!
//! This module is manually created, if something is missing or broken, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose)
//! or a [pull request](https://github.com/noaione/tosho-mango/compare).

#![allow(clippy::derive_partial_eq_without_eq)]
#![warn(clippy::missing_docs_in_private_items)]

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
use tosho_macros::AutoGetter;

/// Indicate a success response
///
/// Depending on the request type, not all field will be available.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuccessResponse {
    /// Is this a featured manga request or this have featured manga list
    #[prost(bool, optional, tag = "1")]
    // #[auto_getters(skip_field)]
    pub featured: ::core::option::Option<bool>,
    /// Registration response information
    #[prost(message, optional, tag = "2")]
    pub registration: ::core::option::Option<RegistrationData>,
    /// The homepage view, v1 implementation
    #[prost(message, optional, tag = "3")]
    pub home_view: ::core::option::Option<HomeView>,
    /// The list of featured titles
    #[prost(message, optional, tag = "4")]
    pub featured_titles: ::core::option::Option<FeaturedTitles>,
    /// The list of all titles
    #[prost(message, optional, tag = "5")]
    pub all_titles: ::core::option::Option<TitleListOnly>,
    /// The list of titles ranked depending on the request
    #[prost(message, optional, tag = "6")]
    pub title_ranking: ::core::option::Option<TitleListOnly>,
    /// The current user subscribed ro favorited titles
    #[prost(message, optional, tag = "7")]
    pub subscribed_titles: ::core::option::Option<TitleListOnly>,
    /// Information about a title
    #[prost(message, optional, tag = "8")]
    pub title_detail: ::core::option::Option<TitleDetail>,
    /// The list of comments on a chapter
    #[prost(message, optional, tag = "9")]
    pub comment_list: ::core::option::Option<CommentList>,
    /// The chapter viewer information
    #[prost(message, optional, tag = "10")]
    pub chapter_viewer: ::core::option::Option<ChapterViewer>,
    /// The web version of homepage view, v1 implementation
    #[prost(message, optional, tag = "11")]
    pub web_home_view: ::core::option::Option<WebHomeView>,
    /// The user settings information
    #[prost(message, optional, tag = "12")]
    pub user_settings: ::core::option::Option<UserSettings>,
    /// The user profile settings information
    #[prost(message, optional, tag = "13")]
    pub user_profile_settings: ::core::option::Option<UserProfileSettings>,
    /// The update profile result
    #[prost(message, optional, tag = "14")]
    pub update_profile_result: ::core::option::Option<UserUpdateProfileResult>,
    /// The current service announcement, usually used for maintenance notice
    #[prost(message, optional, tag = "15")]
    pub service_announcements: ::core::option::Option<ServiceAnnouncements>,
    /// Initial view response, used when booting up the app
    #[prost(message, optional, tag = "16")]
    pub initial_view: ::core::option::Option<InitialView>,
    /// The feedback response
    #[prost(message, optional, tag = "17")]
    pub feedback_view: ::core::option::Option<FeedbackList>,
    /// The publisher news list
    #[prost(message, optional, tag = "18")]
    pub publisher_news_list: ::core::option::Option<PublisherNewsList>,
    /// Questionnaire response
    #[prost(message, optional, tag = "19")]
    pub questionnaire: ::core::option::Option<QuestionnaireResponse>,
    /// The current title updates
    #[prost(message, optional, tag = "20")]
    pub title_updates: ::core::option::Option<TitleUpdates>,
    /// The homepage view, v2 implementation
    #[prost(message, optional, tag = "21")]
    pub home_view_v2: ::core::option::Option<HomeViewV2>,
    /// The updated titles list
    #[prost(message, optional, tag = "22")]
    pub updated_titles: ::core::option::Option<UpdatedTitleList>,
    /// Title that can be read with tickets
    #[prost(message, optional, tag = "23")]
    pub title_tickets: ::core::option::Option<TitleTicketList>,
    /// The homepage view, v3 implementation
    ///
    /// Currently used in the app
    #[prost(message, optional, tag = "24")]
    pub home_view_v3: ::core::option::Option<HomeViewV3>,
    /// The list of all titles, v2 implementation
    ///
    /// Currently used in the app
    #[prost(message, optional, tag = "25")]
    pub all_titles_v2: ::core::option::Option<TitleListOnlyV2>,
    /// User settings information, v2 implementation
    ///
    /// Currently used in the app
    #[prost(message, optional, tag = "26")]
    pub user_settings_v2: ::core::option::Option<UserSettingsV2>,
    /// The latest title updates, v2 implementation
    ///
    /// Currently used in the app
    #[prost(message, optional, tag = "27")]
    pub title_updates_v2: ::core::option::Option<TitleUpdatesV2>,
    /// Initial view response, used when booting up the app, v2 implementation
    ///
    /// Currently used in the app
    #[prost(message, optional, tag = "28")]
    pub initial_view_v2: ::core::option::Option<InitialViewV2>,
    /// The list of available languages
    #[prost(message, optional, tag = "29")]
    pub languages: ::core::option::Option<Languages>,
    /// The web version of homepage view, v2 implementation
    #[prost(message, optional, tag = "30")]
    pub web_home_view_v2: ::core::option::Option<WebHomeViewV2>,
    /// The web version of homepage view, v3 implementation
    #[prost(message, optional, tag = "31")]
    pub web_home_view_v3: ::core::option::Option<WebHomeViewV3>,
    /// Push token information, used for notification
    #[prost(message, optional, tag = "32")]
    pub push_token: ::core::option::Option<PushTokenResponse>,
    /// The list of available or free titles to read
    #[prost(message, optional, tag = "33")]
    pub free_titles: ::core::option::Option<FreeTitles>,
    /// A list of labelled titles
    ///
    /// Currently unknown where this is used
    #[prost(message, optional, tag = "34")]
    pub labelled_titles: ::core::option::Option<LabelledTitles>,
    /// The search results response
    #[prost(message, optional, tag = "35")]
    pub search_results: ::core::option::Option<SearchResults>,
    /// The user subscription information and all the available subscription
    #[prost(message, optional, tag = "36")]
    pub subscriptions: ::core::option::Option<SubscriptionResponse>,
    /// Title ranking information, v2 implementation
    ///
    /// Currently used in the app
    #[prost(message, optional, tag = "37")]
    pub title_ranking_v2: ::core::option::Option<TitleRankingList>,
    /// The web version of homepage view, v4 implementation
    ///
    /// Currently used in the web version
    #[prost(message, optional, tag = "38")]
    pub web_home_view_v4: ::core::option::Option<WebHomeViewV4>,
    /// The list of featured titles, v2 implementation
    ///
    /// Currently used in the app
    #[prost(message, optional, tag = "39")]
    pub featured_titles_v2: ::core::option::Option<FeaturedTitlesV2>,
}

/// A success or error response enum
///
/// This is used like `oneOf` in Protobuf so only one of them will be available.
#[derive(Clone, PartialEq, ::prost::Oneof)]
pub enum SuccessOrError {
    /// A [`Box`]-ed [`SuccessResponse`]
    ///
    /// Depending on the API request, only some of them
    /// would be available to be used.
    #[prost(message, tag = "1")]
    Success(Box<SuccessResponse>),
    /// A [`Box`]-ed [`ErrorResponse`]
    #[prost(message, tag = "2")]
    Error(Box<ErrorResponse>),
}

/// Proto response from the API, wrap a simple `oneOf` of [`SuccessOrError`]
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct Response {
    /// The one-of API response data
    #[prost(oneof = "SuccessOrError", tags = "1, 2")]
    pub response: ::core::option::Option<SuccessOrError>,
}
