//! A module containing information related to banners.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

#![allow(clippy::derive_partial_eq_without_eq)]

use super::{Banner, PopupMessage};

/// Home view response
///
/// The following is `v1` implementation of the home view response.
///
/// See also: [`HomeViewV2`] and [`HomeViewV3`]
///
/// And the web version: [`WebHomeView`]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HomeView {
    /// The collection of top banner in the home page.
    #[prost(message, repeated, tag = "1")]
    pub top_banners: ::prost::alloc::vec::Vec<Banner>,
    /// The list of featured/updated titles.
    #[prost(message, repeated, tag = "2")]
    pub titles: ::prost::alloc::vec::Vec<super::titles::UpdatedTitleGroup>,
    /// Popup banner information.
    #[prost(message, optional, tag = "9")]
    pub popup_banner: ::core::option::Option<PopupMessage>,
    /// Should the popup banner be shown or not.
    #[prost(bool, tag = "10")]
    pub show_popup: bool,
}

/// Home view response
///
/// The following is `v2` implementation of the home view response.
///
/// See also: [`HomeView`] and [`HomeViewV3`]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HomeViewV2 {
    /// The collection of top banner in the home page.
    #[prost(message, repeated, tag = "1")]
    pub top_banners: ::prost::alloc::vec::Vec<Banner>,
    /// The updated titles.
    #[prost(message, repeated, tag = "2")]
    pub updated_titles: ::prost::alloc::vec::Vec<super::titles::UpdatedTitleGroupOriginal>,
    /// History of recently read titles.
    #[prost(message, repeated, tag = "3")]
    pub history: ::prost::alloc::vec::Vec<super::titles::SubscribedTitle>,
    /// Upcoming chapter of the titles.
    #[prost(message, repeated, tag = "4")]
    pub upcoming: ::prost::alloc::vec::Vec<super::titles::UpcomingChapterTitle>,
    /// A title that is highlighted.
    #[prost(message, optional, tag = "5")]
    pub highlighted: ::core::option::Option<super::titles::HighlightedTitle>,
    /// Popup banner information.
    #[prost(message, optional, tag = "9")]
    pub popup_banner: ::core::option::Option<PopupMessage>,
    /// Should the popup banner be shown or not.
    #[prost(bool, tag = "10")]
    pub show_popup: bool,
}

/// Home view response
///
/// The following is `v3` implementation of the home view response.
///
/// See also: [`HomeView`] and [`HomeViewV2`]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HomeViewV3 {
    /// The collection of top banner in the home page.
    #[prost(message, repeated, tag = "1")]
    pub top_banners: ::prost::alloc::vec::Vec<Banner>,
    /// The updated title groups
    #[prost(message, repeated, tag = "2")]
    pub groups: ::prost::alloc::vec::Vec<super::titles::UpdatedTitleGroupV2>,
    /// Popup banner information.
    #[prost(message, optional, tag = "9")]
    pub popup_banner: ::core::option::Option<PopupMessage>,
    /// Should the popup banner be shown or not.
    #[prost(bool, tag = "10")]
    pub show_popup: bool,
    /// The user subscription information
    #[prost(message, optional, tag = "11")]
    pub subscription: ::core::option::Option<super::accounts::UserSubscription>,
    /// Service announcements
    #[prost(message, repeated, tag = "12")]
    pub announcement: ::prost::alloc::vec::Vec<super::others::ServiceAnnouncement>,
}

/// Web version of home view
///
/// The following is `v1` implementation of the web home view response.
///
/// And the app version: [`HomeView`]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebHomeView {
    /// The collection of top banner in the home page.
    #[prost(message, repeated, tag = "1")]
    pub top_banners: ::prost::alloc::vec::Vec<Banner>,
    /// The list of featured/updated titles.
    #[prost(message, repeated, tag = "2")]
    pub titles: ::prost::alloc::vec::Vec<super::titles::UpdatedTitleGroup>,
    /// The current titles ranking.
    #[prost(message, repeated, tag = "3")]
    pub rankings: ::prost::alloc::vec::Vec<super::titles::Title>,
    /// Popup banner information.
    #[prost(message, optional, tag = "4")]
    pub popup_banner: ::core::option::Option<PopupMessage>,
    /// Featured titles.
    #[prost(message, repeated, tag = "5")]
    pub featured_titles: ::prost::alloc::vec::Vec<super::titles::TitleList>,
    /// Should the popup banner be shown or not.
    #[prost(bool, tag = "10")]
    pub show_popup: bool,
}
