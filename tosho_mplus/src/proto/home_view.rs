//! A module containing information related to banners.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

#![allow(clippy::derive_partial_eq_without_eq)]

use tosho_macros::AutoGetter;

use super::{Banner, PopupMessage};

/// Home view response
///
/// The following is `v1` implementation of the home view response.
///
/// See also: [`HomeViewV2`] and [`HomeViewV3`]
///
/// And the web version: [`WebHomeView`]
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct HomeView {
    /// The collection of top banner in the home page.
    #[prost(message, repeated, tag = "1")]
    top_banners: ::prost::alloc::vec::Vec<Banner>,
    /// The list of featured/updated titles.
    #[prost(message, repeated, tag = "2")]
    titles: ::prost::alloc::vec::Vec<super::titles::UpdatedTitleGroup>,
    /// Popup banner information.
    #[prost(message, optional, tag = "9")]
    popup_banner: ::core::option::Option<PopupMessage>,
    /// Should the popup banner be shown or not.
    #[prost(bool, tag = "10")]
    show_popup: bool,
}

/// Home view response
///
/// The following is `v2` implementation of the home view response.
///
/// See also: [`HomeView`] and [`HomeViewV3`]
///
/// And the web version: [`WebHomeViewV2`]
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct HomeViewV2 {
    /// The collection of top banner in the home page.
    #[prost(message, repeated, tag = "1")]
    top_banners: ::prost::alloc::vec::Vec<Banner>,
    /// The updated titles.
    #[prost(message, repeated, tag = "2")]
    updated_titles: ::prost::alloc::vec::Vec<super::titles::UpdatedTitleGroupOriginal>,
    /// History of recently read titles.
    #[prost(message, repeated, tag = "3")]
    history: ::prost::alloc::vec::Vec<super::titles::SubscribedTitle>,
    /// Upcoming chapter of the titles.
    #[prost(message, repeated, tag = "4")]
    upcoming: ::prost::alloc::vec::Vec<super::titles::UpcomingChapterTitle>,
    /// A title that is highlighted.
    #[prost(message, optional, tag = "5")]
    highlighted: ::core::option::Option<super::titles::HighlightedTitle>,
    /// Popup banner information.
    #[prost(message, optional, tag = "9")]
    popup_banner: ::core::option::Option<PopupMessage>,
    /// Should the popup banner be shown or not.
    #[prost(bool, tag = "10")]
    show_popup: bool,
}

/// Home view response
///
/// The following is `v3` implementation of the home view response.
///
/// See also: [`HomeView`] and [`HomeViewV2`]
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct HomeViewV3 {
    /// The collection of top banner in the home page.
    #[prost(message, repeated, tag = "1")]
    top_banners: ::prost::alloc::vec::Vec<Banner>,
    /// The updated title groups
    #[prost(message, repeated, tag = "2")]
    groups: ::prost::alloc::vec::Vec<super::titles::UpdatedTitleGroupV2>,
    /// Popup banner information.
    #[prost(message, optional, tag = "9")]
    popup_banner: ::core::option::Option<PopupMessage>,
    /// Should the popup banner be shown or not.
    #[prost(bool, tag = "10")]
    show_popup: bool,
    /// The user subscription information
    #[prost(message, optional, tag = "11")]
    subscription: ::core::option::Option<super::accounts::UserSubscription>,
    /// Service announcements
    #[prost(message, repeated, tag = "12")]
    announcement: ::prost::alloc::vec::Vec<super::others::ServiceAnnouncement>,
}

/// Web version of home view
///
/// The following is `v1` implementation of the web home view response.
///
/// See also: [`WebHomeViewV2`], [`WebHomeViewV3`], and [`WebHomeViewV4`]
///
/// And the app version: [`HomeView`]
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct WebHomeView {
    /// The collection of top banner in the home page.
    #[prost(message, repeated, tag = "1")]
    top_banners: ::prost::alloc::vec::Vec<Banner>,
    /// The list of featured/updated titles.
    #[prost(message, repeated, tag = "2")]
    titles: ::prost::alloc::vec::Vec<super::titles::UpdatedTitleGroup>,
    /// The current titles ranking.
    #[prost(message, repeated, tag = "3")]
    rankings: ::prost::alloc::vec::Vec<super::titles::Title>,
    /// Popup banner information.
    #[prost(message, optional, tag = "4")]
    popup_banner: ::core::option::Option<PopupMessage>,
    /// Featured titles.
    #[prost(message, repeated, tag = "5")]
    featured_titles: ::prost::alloc::vec::Vec<super::titles::TitleList>,
    /// Should the popup banner be shown or not.
    #[prost(bool, tag = "10")]
    show_popup: bool,
}

/// Web version of home view
///
/// The following is `v2` implementation of the web home view response.
///
/// See also: [`WebHomeView`], [`WebHomeViewV3`], and [`WebHomeViewV4`]
///
/// And the app version: [`HomeViewV2`]
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct WebHomeViewV2 {
    /// The collection of top banner in the home page.
    #[prost(message, repeated, tag = "1")]
    top_banners: ::prost::alloc::vec::Vec<Banner>,
    /// The updated titles.
    #[prost(message, repeated, tag = "2")]
    updated_titles: ::prost::alloc::vec::Vec<super::titles::UpdatedTitleGroupOriginal>,
    /// History of recently read titles.
    #[prost(message, repeated, tag = "3")]
    history: ::prost::alloc::vec::Vec<super::titles::SubscribedTitle>,
    /// Upcoming chapter of the titles.
    #[prost(message, repeated, tag = "4")]
    upcoming: ::prost::alloc::vec::Vec<super::titles::UpcomingChapterTitle>,
    /// A title that is highlighted.
    #[prost(message, optional, tag = "5")]
    highlighted: ::core::option::Option<super::titles::HighlightedTitle>,
    /// The current titles ranking.
    #[prost(message, repeated, tag = "6")]
    rankings: ::prost::alloc::vec::Vec<super::titles::Title>,
    /// Popup banner information.
    #[prost(message, optional, tag = "7")]
    popup_banner: ::core::option::Option<PopupMessage>,
    /// Featured titles.
    #[prost(message, repeated, tag = "8")]
    featured_titles: ::prost::alloc::vec::Vec<super::titles::TitleList>,
}

/// Web version of home view
///
/// The following is `v3` implementation of the web home view response.
///
/// See also: [`WebHomeView`], [`WebHomeViewV2`], and [`WebHomeViewV4`]
///
/// And the app version: [`HomeViewV3`]
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct WebHomeViewV3 {
    /// The collection of top banner in the home page.
    #[prost(message, repeated, tag = "1")]
    top_banners: ::prost::alloc::vec::Vec<Banner>,
    /// The updated title groups
    #[prost(message, repeated, tag = "2")]
    groups: ::prost::alloc::vec::Vec<super::titles::UpdatedTitleGroupV2>,
    /// The current titles ranking.
    #[prost(message, repeated, tag = "3")]
    rankings: ::prost::alloc::vec::Vec<super::titles::Title>,
    /// Popup banner information.
    #[prost(message, optional, tag = "4")]
    popup_banner: ::core::option::Option<PopupMessage>,
    /// Featured titles.
    #[prost(message, repeated, tag = "5")]
    featured_titles: ::prost::alloc::vec::Vec<super::titles::TitleList>,
}

/// Web version of home view
///
/// The following is `v4` implementation of the web home view response.
///
/// See also: [`WebHomeView`], [`WebHomeViewV2`], and [`WebHomeViewV3`]
///
/// There is no equivalent app version for this.
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct WebHomeViewV4 {
    /// The collection of top banner in the home page.
    #[prost(message, repeated, tag = "1")]
    top_banners: ::prost::alloc::vec::Vec<Banner>,
    /// The updated title groups
    #[prost(message, repeated, tag = "2")]
    groups: ::prost::alloc::vec::Vec<super::titles::UpdatedTitleGroupV2>,
    /// The current titles ranking.
    #[prost(message, repeated, tag = "3")]
    rankings: ::prost::alloc::vec::Vec<super::titles::TitleRankingGroup>,
    /// Popup banner information.
    #[prost(message, optional, tag = "4")]
    popup_banner: ::core::option::Option<PopupMessage>,
    /// Featured titles.
    #[prost(message, repeated, tag = "5")]
    featured_titles: ::prost::alloc::vec::Vec<super::titles::TitleList>,
    /// Service announcements
    #[prost(message, repeated, tag = "6")]
    announcements: ::prost::alloc::vec::Vec<super::others::ServiceAnnouncement>,
}
