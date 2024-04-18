//! A module containing information related to manga titles.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

use super::{
    Banner, Chapter, ChapterGroup, Label, Language, PublisherItem, Tag, TitleUpdateStatus,
    UserSubscription, UserTickets,
};

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

/// A detailed title information
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TitleDetail {
    /// The title information
    #[prost(message, tag = "1")]
    pub title: ::core::option::Option<Title>,
    /// The title image URL
    #[prost(string, tag = "2")]
    pub image: ::prost::alloc::string::String,
    /// The title overview/description
    #[prost(string, tag = "3")]
    pub overview: ::prost::alloc::string::String,
    /// The background image URL
    #[prost(string, tag = "4")]
    pub background_image: ::prost::alloc::string::String,
    /// Next update UNIX timestamp
    #[prost(sfixed64, optional, tag = "5")]
    pub next_update: ::core::option::Option<i64>,
    /// Update frequency in seconds
    #[prost(uint64, optional, tag = "6")]
    pub update_frequency: ::core::option::Option<u64>,
    /// Viewing period description
    #[prost(string, optional, tag = "7")]
    pub viewing_period: ::core::option::Option<::prost::alloc::string::String>,
    /// Non-appereance message
    #[prost(string, optional, tag = "8")]
    pub non_appearance_message: ::core::option::Option<::prost::alloc::string::String>,
    /// A list of first chapters that can be read
    #[prost(message, repeated, tag = "9")]
    pub first_chapters: ::prost::alloc::vec::Vec<Chapter>,
    /// A list of last chapters that can be read
    #[prost(message, repeated, tag = "10")]
    pub last_chapters: ::prost::alloc::vec::Vec<Chapter>,
    /// A list of banners
    #[prost(message, repeated, tag = "11")]
    pub banners: ::prost::alloc::vec::Vec<Banner>,
    /// A list of recommended titles
    #[prost(message, repeated, tag = "12")]
    pub recommended_titles: ::prost::alloc::vec::Vec<Title>,
    // SNS field: 13
    /// Is the title simulpublished with the original release?
    #[prost(bool, tag = "14")]
    pub simulpublish: bool,
    /// Is the user subscribed to this title?
    #[prost(bool, tag = "15")]
    pub subscribed: bool,
    /// The title rating
    #[prost(uint64, tag = "16")]
    pub rating: u64,
    /// Is the chapters in descending order?
    #[prost(bool, tag = "17")]
    pub descending: bool,
    /// The title number of views
    #[prost(uint64, tag = "18")]
    pub view_count: u64,
    /// Publisher items
    #[prost(message, repeated, tag = "19")]
    pub publishers: ::prost::alloc::vec::Vec<PublisherItem>,
    /// Title banners
    #[prost(message, repeated, tag = "20")]
    pub title_banners: ::prost::alloc::vec::Vec<Banner>,
    /// User tikcet information
    #[prost(message, optional, tag = "21")]
    pub user_tickets: ::core::option::Option<UserTickets>,
    /// Chapter that can be claimed with ticket
    #[prost(message, repeated, tag = "22")]
    pub ticket_chapters: ::prost::alloc::vec::Vec<Chapter>,
    /// Ticket title list
    #[prost(message, repeated, tag = "23")]
    pub ticket_titles: ::prost::alloc::vec::Vec<Title>,
    /// Has a chapter in-between
    #[prost(bool, tag = "24")]
    pub has_in_between: bool,
    /// Publisher banner
    #[prost(message, optional, tag = "25")]
    pub publisher_banner: ::core::option::Option<Banner>,
    // Ads field: 26
    /// Other languages available for the title
    #[prost(message, repeated, tag = "27")]
    pub other_languages: ::prost::alloc::vec::Vec<TitleLanguages>,
    /// Grouped chapters list
    #[prost(message, repeated, tag = "28")]
    pub chapter_groups: ::prost::alloc::vec::Vec<ChapterGroup>,
    // Free dialogue view: 29
    /// Region code of the title
    #[prost(string, tag = "30")]
    pub region_code: ::prost::alloc::string::String,
    /// Tags of the title
    #[prost(message, repeated, tag = "31")]
    pub tags: ::prost::alloc::vec::Vec<Tag>,
    /// Specific label applied to the title
    #[prost(message, optional, tag = "32")]
    pub title_labels: ::core::option::Option<TitleLabels>,
    /// User subscription information
    #[prost(message, optional, tag = "33")]
    pub user_subscription: ::core::option::Option<UserSubscription>,
    /// Label applied to the title
    #[prost(message, optional, tag = "34")]
    pub label: ::core::option::Option<Label>,
    /// Is the first time reading would be free?
    #[prost(bool, tag = "35")]
    pub first_time_free: bool,
}

/// An information about a title with available languages
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TitleLanguages {
    /// Title ID
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// Title language
    #[prost(enumeration = "Language", tag = "2")]
    pub language: i32,
}

/// Title labels information
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TitleLabels {
    /// Release schedule of the title
    #[prost(uint64, tag = "1")]
    pub release_schedule: u64,
    /// Is the title simulpublished with the original release?
    #[prost(bool, tag = "2")]
    pub simulpublish: bool,
    /// Plan type of the title
    #[prost(string, tag = "3")]
    pub plan_type: ::prost::alloc::string::String,
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
