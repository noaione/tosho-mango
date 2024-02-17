//! A module containing information related to user account.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

#![allow(clippy::derive_partial_eq_without_eq)]

use super::UserPoint;

/// The device connected to the account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountDevice {
    /// The device ID.
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// The device name.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// The device installation date in unix timestamp.
    #[prost(uint64, tag = "3")]
    pub install_at: u64,
}

/// The account view response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountView {
    /// The list of devices that you have logged in.
    #[prost(message, repeated, tag = "1")]
    pub devices: ::prost::alloc::vec::Vec<AccountDevice>,
    /// Whether or not you have registered your account.
    #[prost(bool, optional, tag = "2")]
    pub registered: ::core::option::Option<bool>,
    /// The login URL to connect your account.
    #[prost(string, tag = "3")]
    pub login_url: ::prost::alloc::string::String,
}

/// The setting view response
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SettingView {
    /// The bridge tag name.
    #[prost(string, tag = "1")]
    pub tag_name: ::prost::alloc::string::String,
    /// The bridge keyword.
    #[prost(string, tag = "2")]
    pub keyword: ::prost::alloc::string::String,
}

/// Your personalized profile page view response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MyPageView {
    /// The manga list that you bookmarked/favorited.
    #[prost(message, repeated, tag = "1")]
    pub favorites: ::prost::alloc::vec::Vec<super::MangaResultNode>,
    /// The manga list that you read.
    #[prost(message, repeated, tag = "2")]
    pub history: ::prost::alloc::vec::Vec<super::MangaResultNode>,
    /// The event point that we get from the registration(?).
    #[prost(uint64, tag = "3")]
    pub register_event_point: u64,
}

/// The node of each banner on the home page.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HomeBanner {
    /// The manga ID.
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// The manga thumbnail URL.
    #[prost(string, tag = "2")]
    pub image_url: ::prost::alloc::string::String,
    /// The manga intent URL.
    #[prost(string, tag = "3")]
    pub intent_url: ::prost::alloc::string::String,
}

/// The currently featured manga on the home page.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HomeFeatured {
    /// The manga ID.
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// The manga thumbnail URL.
    #[prost(string, tag = "2")]
    pub image_url: ::prost::alloc::string::String,
    /// The manga video thumbnail URL.
    #[prost(string, optional, tag = "3")]
    pub video_url: ::core::option::Option<::prost::alloc::string::String>,
    /// The manga short description.
    #[prost(string, tag = "4")]
    pub short_description: ::prost::alloc::string::String,
    /// The manga intent URL.
    #[prost(string, tag = "5")]
    pub intent_url: ::prost::alloc::string::String,
    /// The manga title.
    #[prost(string, tag = "6")]
    pub title: ::prost::alloc::string::String,
}

/// The personalized home page view response.
///
/// The following is the ``v2`` version of the response.
///
/// There is no known ``v1`` version of the response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HomeViewV2 {
    /// The user point.
    #[prost(message, tag = "1")]
    pub user_point: ::core::option::Option<UserPoint>,
    /// The top most banner list. (Big single carousel)
    #[prost(message, repeated, tag = "3")]
    pub top_banners: ::prost::alloc::vec::Vec<HomeBanner>,
    /// The top most sub-banner list. (Smaller carousel)
    #[prost(message, repeated, tag = "4")]
    pub top_sub_banners: ::prost::alloc::vec::Vec<HomeBanner>,
    /// The tutorial banner list, if any.
    #[prost(message, optional, tag = "5")]
    pub tutorial_banner: ::core::option::Option<HomeBanner>,
    /// The updated manga section name. (e.g. "Updated for You")
    #[prost(string, tag = "6")]
    pub updated_section_name: ::prost::alloc::string::String,
    /// The updated manga list.
    #[prost(message, repeated, tag = "7")]
    pub updated_titles: ::prost::alloc::vec::Vec<super::MangaResultNode>,
    /// The tag/genre list.
    #[prost(message, repeated, tag = "8")]
    pub tags: ::prost::alloc::vec::Vec<super::Tag>,
    /// The currently featured manga.
    #[prost(message, optional, tag = "9")]
    pub featured: ::core::option::Option<HomeFeatured>,
    /// The new manga section name. (e.g. "New Series")
    #[prost(string, tag = "10")]
    pub new_section_name: ::prost::alloc::string::String,
    /// The new manga list.
    #[prost(message, repeated, tag = "11")]
    pub new_titles: ::prost::alloc::vec::Vec<super::MangaResultNode>,
    /// The popular/ranking manga section name. (e.g. "Ranking")
    #[prost(string, tag = "12")]
    pub ranking_section_name: ::prost::alloc::string::String,
    /// The popular/ranking manga list.
    #[prost(message, repeated, tag = "13")]
    pub rankings: ::prost::alloc::vec::Vec<super::MangaGroup>,
    /// The ranking description.
    #[prost(string, tag = "14")]
    pub ranking_description: ::prost::alloc::string::String,
    /// The recommended banner image URL.
    #[prost(string, tag = "15")]
    pub recommended_banner_image_url: ::prost::alloc::string::String,
}
