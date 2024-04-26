//! A module containing information related to banners.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

#![allow(clippy::derive_partial_eq_without_eq)]

use super::{Banner, PopupMessage};

/// Home view response
///
/// The following is `v1` implementation of the home view response.
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

/// Web version of home view
///
/// The following is `v1` implementation of the web home view response.
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
