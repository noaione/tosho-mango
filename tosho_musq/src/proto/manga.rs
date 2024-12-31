//! A module containing information related to manga.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

use tosho_macros::AutoGetter;

use super::{
    AccountSubscriptionStatus, BadgeManga, LabelBadgeManga, Status, SubscriptionBadge, UserPoint,
};

/// The tag or genre information.
#[derive(Clone, PartialEq, AutoGetter, ::prost::Message)]
pub struct Tag {
    /// The tag ID.
    #[prost(uint64, tag = "1")]
    id: u64,
    /// The tag name.
    #[prost(string, tag = "2")]
    name: ::prost::alloc::string::String,
    /// The tag image URL.
    #[prost(string, optional, tag = "3")]
    #[skip_field]
    image_url: ::core::option::Option<::prost::alloc::string::String>,
}

/// The button that will be shown in the manga detail page
/// that the user can interact with to view a chapter.
///
/// This is made for the [`Chapter`](struct.Chapter.html) struct.
#[derive(Clone, PartialEq, AutoGetter, ::prost::Message)]
pub struct ViewButton {
    /// The chapter that will be accessed if user click this button.
    #[prost(message, tag = "1")]
    chapter: ::core::option::Option<super::Chapter>,
    /// The button text.
    #[prost(string, tag = "2")]
    text: ::prost::alloc::string::String,
}

/// The button that will be shown in the manga detail page
/// that the user can interact with to view a chapter.
///
/// This is made for the [`ChapterV2`](struct.ChapterV2.html) struct.
#[derive(Clone, PartialEq, AutoGetter, ::prost::Message)]
pub struct ViewButtonV2 {
    /// The chapter that will be accessed if user click this button.
    #[prost(message, tag = "1")]
    chapter: ::core::option::Option<super::ChapterV2>,
    /// The button text.
    #[prost(string, tag = "2")]
    text: ::prost::alloc::string::String,
}

/// A hidden chapters range.
///
/// Made only for the [``MangaDetailV2``].
#[derive(Clone, Copy, PartialEq, AutoGetter, ::prost::Message)]
pub struct ChaptersRange {
    /// The start chapter ID.
    #[prost(uint64, tag = "1")]
    start: u64,
    /// The end chapter ID.
    #[prost(uint64, tag = "2")]
    end: u64,
}

/// Manga detail information responses.
///
/// This is the ``v1`` version of the manga detail response.
///
/// See also: [``MangaDetailV2``]
#[derive(Clone, PartialEq, AutoGetter, ::prost::Message)]
pub struct MangaDetail {
    /// The status of the requests.
    #[prost(enumeration = "Status", tag = "1")]
    #[skip_field]
    status: i32,
    /// The user point information.
    #[prost(message, tag = "2")]
    #[copyable]
    user_point: ::core::option::Option<UserPoint>,
    /// The manga title.
    #[prost(string, tag = "3")]
    title: ::prost::alloc::string::String,
    /// The manga authors, separated by comma.
    #[prost(string, tag = "4")]
    authors: ::prost::alloc::string::String,
    /// The manga copyright information.
    #[prost(string, tag = "5")]
    copyright: ::prost::alloc::string::String,
    /// The next chapter update time in datetime format.
    #[prost(string, optional, tag = "6")]
    #[skip_field]
    next_update: ::core::option::Option<::prost::alloc::string::String>,
    /// The manga warning/sensitive information.
    #[prost(string, optional, tag = "7")]
    #[skip_field]
    warning: ::core::option::Option<::prost::alloc::string::String>,
    /// The manga description.
    #[prost(string, tag = "8")]
    description: ::prost::alloc::string::String,
    /// Whether the description is displayed or not.
    #[prost(bool, tag = "9")]
    display_description: bool,
    /// The manga tags/genres.
    #[prost(message, repeated, tag = "10")]
    tags: ::prost::alloc::vec::Vec<Tag>,
    /// The manga thumbnail URL.
    #[prost(string, tag = "11")]
    thumbnail_url: ::prost::alloc::string::String,
    /// The manga video thumbnail URL.
    #[prost(string, optional, tag = "12")]
    #[skip_field]
    video_url: ::core::option::Option<::prost::alloc::string::String>,
    /// The manga chapters.
    #[prost(message, repeated, tag = "13")]
    chapters: ::prost::alloc::vec::Vec<super::Chapter>,
    /// Whether the manga is favorited or not.
    #[prost(bool, tag = "14")]
    is_favorite: bool,
    /// The view button, if any.
    #[prost(message, optional, tag = "15")]
    view_button: ::core::option::Option<ViewButton>,
    /// Whether the manga comments is enabled or not.
    #[prost(bool, tag = "16")]
    is_comment_enabled: bool,
    /// Related manga list.
    #[prost(message, repeated, tag = "17")]
    related_manga: ::prost::alloc::vec::Vec<MangaResultNode>,
}

/// Manga detail information responses.
///
/// This is the ``v2`` version of the manga detail response.
///
/// See also: [``MangaDetail``]
#[derive(Clone, PartialEq, AutoGetter, ::prost::Message)]
pub struct MangaDetailV2 {
    /// The status of the requests.
    #[prost(enumeration = "Status", tag = "1")]
    #[skip_field]
    status: i32,
    /// The user point information.
    #[prost(message, tag = "2")]
    #[copyable]
    user_point: ::core::option::Option<UserPoint>,
    /// The manga title.
    #[prost(string, tag = "3")]
    title: ::prost::alloc::string::String,
    /// The manga authors, separated by comma.
    #[prost(string, tag = "4")]
    authors: ::prost::alloc::string::String,
    /// The manga copyright information.
    #[prost(string, tag = "5")]
    copyright: ::prost::alloc::string::String,
    /// The next chapter update time in datetime format.
    #[prost(string, optional, tag = "6")]
    #[skip_field]
    next_update: ::core::option::Option<::prost::alloc::string::String>,
    /// The manga warning/sensitive information.
    #[prost(string, optional, tag = "7")]
    #[skip_field]
    warning: ::core::option::Option<::prost::alloc::string::String>,
    /// The manga description.
    #[prost(string, tag = "8")]
    description: ::prost::alloc::string::String,
    /// Whether the description is displayed or not.
    #[prost(bool, tag = "9")]
    display_description: bool,
    /// The manga tags/genres.
    #[prost(message, repeated, tag = "10")]
    tags: ::prost::alloc::vec::Vec<Tag>,
    /// The manga thumbnail URL.
    #[prost(string, tag = "11")]
    thumbnail_url: ::prost::alloc::string::String,
    /// The manga video thumbnail URL.
    #[prost(string, optional, tag = "12")]
    #[skip_field]
    video_url: ::core::option::Option<::prost::alloc::string::String>,
    /// The manga chapters.
    #[prost(message, repeated, tag = "13")]
    chapters: ::prost::alloc::vec::Vec<super::ChapterV2>,
    /// Whether the manga is favorited or not.
    #[prost(bool, tag = "14")]
    is_favorite: bool,
    /// The view button, if any.
    #[prost(message, optional, tag = "15")]
    view_button: ::core::option::Option<ViewButton>,
    /// Whether the manga comments is enabled or not.
    #[prost(bool, tag = "16")]
    is_comment_enabled: bool,
    /// Related manga list.
    #[prost(message, repeated, tag = "17")]
    related_manga: ::prost::alloc::vec::Vec<MangaResultNode>,
    /// The hidden chapters range.
    #[prost(message, optional, tag = "18")]
    hidden_chapters: ::core::option::Option<ChaptersRange>,
    /// The subscription status of the user.
    #[prost(message, optional, tag = "19")]
    subscription_status: ::core::option::Option<AccountSubscriptionStatus>,
}

/// A simplified manga information used in the search result.
#[derive(Clone, PartialEq, AutoGetter, ::prost::Message)]
pub struct MangaResultNode {
    /// The manga ID.
    #[prost(uint64, tag = "1")]
    id: u64,
    /// The manga title.
    #[prost(string, tag = "2")]
    title: ::prost::alloc::string::String,
    /// The manga cover URL.
    #[prost(string, tag = "3")]
    image_url: ::prost::alloc::string::String,
    /// The manga video thumbnail URL.
    #[prost(string, optional, tag = "4")]
    #[skip_field]
    video_url: ::core::option::Option<::prost::alloc::string::String>,
    /// The manga short description.
    #[prost(string, tag = "5")]
    short_description: ::prost::alloc::string::String,
    /// The manga campaign information.
    #[prost(string, optional, tag = "6")]
    #[skip_field]
    campaign: ::core::option::Option<::prost::alloc::string::String>,
    /// The manga bookmark/favorites count.
    #[prost(uint64, tag = "7")]
    favorites: u64,
    /// The manga badge information.
    #[prost(enumeration = "BadgeManga", tag = "8")]
    #[skip_field]
    badge: i32,
    /// The manga last update date in datetime format.
    #[prost(string, optional, tag = "9")]
    #[skip_field]
    last_update: ::core::option::Option<::prost::alloc::string::String>,
    /// The label badge information.
    #[prost(enumeration = "LabelBadgeManga", tag = "10")]
    #[skip_field]
    label_badge: i32,
    /// The subscription badge information.
    #[prost(enumeration = "SubscriptionBadge", tag = "11")]
    #[skip_field]
    subscription_badge: i32,
}

/// The manga search result responses.
///
/// Contains the manga list that match the search query,
/// or used in the weekly updates information.
#[derive(Clone, PartialEq, AutoGetter, ::prost::Message)]
pub struct MangaResults {
    /// The manga list.
    #[prost(message, repeated, tag = "1")]
    titles: ::prost::alloc::vec::Vec<MangaResultNode>,
}

/// A grouping of manga by tag/genres.
#[derive(Clone, PartialEq, AutoGetter, ::prost::Message)]
pub struct MangaGroup {
    /// The tag/genre name.
    #[prost(string, tag = "1")]
    name: ::prost::alloc::string::String,
    /// The associated manga list.
    #[prost(message, repeated, tag = "2")]
    titles: ::prost::alloc::vec::Vec<MangaResultNode>,
    /// The tag/genre ID.
    #[prost(uint64, tag = "3")]
    tag_id: u64,
}
