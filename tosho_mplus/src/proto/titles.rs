//! A module containing information related to manga titles.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

use super::{Banner, Language, TitleUpdateStatus};

/// A single title information
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Title {
    /// The title ID
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// The title name
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// The title author
    #[prost(string, tag = "3")]
    pub author: ::prost::alloc::string::String,
    /// The portrait image URL
    #[prost(string, tag = "4")]
    pub portrait: ::prost::alloc::string::String,
    /// The landscape image URL
    #[prost(string, tag = "5")]
    pub landscape: ::prost::alloc::string::String,
    /// The view count of the title
    #[prost(uint64, tag = "6")]
    pub view_count: u64,
    /// The language of the title
    #[prost(enumeration = "Language", tag = "7")]
    pub language: i32,
    /// The title status
    #[prost(enumeration = "TitleUpdateStatus", tag = "8")]
    pub status: i32,
}

/// A list of titles contained into a group
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TitleList {
    /// The group name
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The list of titles
    #[prost(message, repeated, tag = "2")]
    pub titles: ::prost::alloc::vec::Vec<Title>,
}

/// A list of titles with no grouping information
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TitleListOnly {
    /// The list of titles
    #[prost(message, repeated, tag = "1")]
    pub titles: ::prost::alloc::vec::Vec<Title>,
}

/// An updated title information
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatedTitle {
    /// The title itself that got updated
    #[prost(message, tag = "1")]
    pub title: ::core::option::Option<Title>,
    /// Chapter ID that got updated
    #[prost(uint64, tag = "2")]
    pub chapter_id: u64,
    /// The chapter title
    #[prost(string, optional, tag = "3")]
    pub chapter_title: ::core::option::Option<::prost::alloc::string::String>,
    /// The chapter subtitle
    #[prost(string, optional, tag = "4")]
    pub chapter_subtitle: ::core::option::Option<::prost::alloc::string::String>,
    /// Is chapter is latest chapter update or not.
    #[prost(bool, tag = "5")]
    pub latest: bool,
    /// Does the chapter can only be read in long-strip mode only.
    #[prost(bool, tag = "6")]
    pub long_strip_only: bool,
    /// Does the chapter can only be read in horizontal mode only.
    #[prost(bool, tag = "7")]
    pub horizontal_only: bool,
}

/// A list of updated titles grouped by something
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatedTitleGroup {
    /// The group name
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The list of updated titles
    #[prost(message, repeated, tag = "2")]
    pub titles: ::prost::alloc::vec::Vec<UpdatedTitle>,
}

/// The detailed contents of the featured titles
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeaturedTitleContents {
    /// The featured banner contents
    #[prost(message, optional, tag = "1")]
    pub banner: ::core::option::Option<Banner>,
    /// A list of featured titles
    #[prost(message, optional, tag = "2")]
    pub titles: ::core::option::Option<TitleList>,
}

/// A list of featured titles
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeaturedTitles {
    /// The main featured title
    #[prost(message, optional, tag = "1")]
    pub main: ::core::option::Option<Banner>,
    /// The first sub featured title
    #[prost(message, optional, tag = "2")]
    pub sub1: ::core::option::Option<Banner>,
    /// The second sub featured title
    #[prost(message, optional, tag = "3")]
    pub sub2: ::core::option::Option<Banner>,
    /// The featured title contents
    #[prost(message, repeated, tag = "4")]
    pub contents: ::prost::alloc::vec::Vec<FeaturedTitleContents>,
}
