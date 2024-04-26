//! A module containing all the models/proto mapping used in the library.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

#![allow(clippy::derive_partial_eq_without_eq)]

pub mod accounts;
pub mod chapters;
pub mod comments;
pub mod common;
pub mod enums;
pub mod home_view;
pub mod titles;

pub use accounts::*;
pub use chapters::*;
pub use comments::*;
pub use common::*;
pub use enums::*;
pub use home_view::*;
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
