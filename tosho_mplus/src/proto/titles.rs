//! A module containing information related to manga titles.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

use std::str::FromStr;

use tosho_macros::AutoGetter;

use crate::helper::SubscriptionPlan;

use super::{
    Banner, Chapter, ChapterGroup, Label, Language, PublisherItem, Tag, TitleUpdateStatus,
    UserSubscription, UserTickets,
};

/// A single title information
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct Title {
    /// The title ID
    #[prost(uint64, tag = "1")]
    id: u64,
    /// The title name
    #[prost(string, tag = "2")]
    title: ::prost::alloc::string::String,
    /// The title author
    #[prost(string, tag = "3")]
    author: ::prost::alloc::string::String,
    /// The portrait image URL
    #[prost(string, tag = "4")]
    portrait: ::prost::alloc::string::String,
    /// The landscape image URL
    #[prost(string, tag = "5")]
    landscape: ::prost::alloc::string::String,
    /// The view count of the title
    #[prost(uint64, tag = "6")]
    view_count: u64,
    /// The language of the title
    #[prost(enumeration = "Language", tag = "7")]
    #[skip_field]
    language: i32,
    /// The title status
    #[prost(enumeration = "TitleUpdateStatus", tag = "8")]
    #[skip_field]
    status: i32,
}

/// A detailed title information
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct TitleDetail {
    /// The title information
    #[prost(message, tag = "1")]
    title: ::core::option::Option<Title>,
    /// The title image URL
    #[prost(string, tag = "2")]
    image: ::prost::alloc::string::String,
    /// The title overview/description
    #[prost(string, tag = "3")]
    overview: ::prost::alloc::string::String,
    /// The background image URL
    #[prost(string, tag = "4")]
    background_image: ::prost::alloc::string::String,
    /// Next update UNIX timestamp
    #[prost(int64, optional, tag = "5")]
    #[skip_field]
    next_update: ::core::option::Option<i64>,
    /// Update frequency in seconds
    #[prost(uint64, optional, tag = "6")]
    #[skip_field]
    update_frequency: ::core::option::Option<u64>,
    /// Viewing period description
    #[prost(string, optional, tag = "7")]
    #[skip_field]
    viewing_period: ::core::option::Option<::prost::alloc::string::String>,
    /// Non-appereance message
    #[prost(string, optional, tag = "8")]
    #[skip_field]
    non_appearance_message: ::core::option::Option<::prost::alloc::string::String>,
    /// A list of first chapters that can be read
    #[prost(message, repeated, tag = "9")]
    first_chapters: ::prost::alloc::vec::Vec<Chapter>,
    /// A list of last chapters that can be read
    #[prost(message, repeated, tag = "10")]
    last_chapters: ::prost::alloc::vec::Vec<Chapter>,
    /// A list of banners
    #[prost(message, repeated, tag = "11")]
    banners: ::prost::alloc::vec::Vec<Banner>,
    /// A list of recommended titles
    #[prost(message, repeated, tag = "12")]
    recommended_titles: ::prost::alloc::vec::Vec<Title>,
    // SNS field: 13
    /// Is the title simulpublished with the original release?
    #[prost(bool, tag = "14")]
    simulpublish: bool,
    /// Is the user subscribed to this title?
    #[prost(bool, tag = "15")]
    subscribed: bool,
    /// The title rating
    #[prost(enumeration = "super::TitleRating", tag = "16")]
    #[skip_field]
    rating: i32,
    /// Is the chapters in descending order?
    #[prost(bool, tag = "17")]
    descending: bool,
    /// The title number of views
    #[prost(uint64, tag = "18")]
    view_count: u64,
    /// Publisher items
    #[prost(message, repeated, tag = "19")]
    publishers: ::prost::alloc::vec::Vec<PublisherItem>,
    /// Title banners
    #[prost(message, repeated, tag = "20")]
    title_banners: ::prost::alloc::vec::Vec<Banner>,
    /// User tikcet information
    #[prost(message, optional, tag = "21")]
    #[copyable]
    user_tickets: ::core::option::Option<UserTickets>,
    /// Chapter that can be claimed with ticket
    #[prost(message, repeated, tag = "22")]
    ticket_chapters: ::prost::alloc::vec::Vec<Chapter>,
    /// Ticket title list
    #[prost(message, repeated, tag = "23")]
    ticket_titles: ::prost::alloc::vec::Vec<Title>,
    /// Has a chapter in-between
    #[prost(bool, tag = "24")]
    has_in_between: bool,
    /// Publisher banner
    #[prost(message, optional, tag = "25")]
    publisher_banner: ::core::option::Option<Banner>,
    // Ads field: 26
    /// Other languages available for the title
    #[prost(message, repeated, tag = "27")]
    other_languages: ::prost::alloc::vec::Vec<TitleLanguages>,
    /// Grouped chapters list
    #[prost(message, repeated, tag = "28")]
    chapter_groups: ::prost::alloc::vec::Vec<ChapterGroup>,
    // Free dialogue view: 29
    /// Region code of the title
    #[prost(string, tag = "30")]
    region_code: ::prost::alloc::string::String,
    /// Tags of the title
    #[prost(message, repeated, tag = "31")]
    tags: ::prost::alloc::vec::Vec<Tag>,
    /// Specific label applied to the title
    #[prost(message, tag = "32")]
    title_labels: ::core::option::Option<TitleLabels>,
    /// User subscription information
    #[prost(message, optional, tag = "33")]
    user_subscription: ::core::option::Option<UserSubscription>,
    /// Label applied to the title
    #[prost(message, optional, tag = "34")]
    label: ::core::option::Option<Label>,
    /// Is the first time reading would be free?
    #[prost(bool, tag = "35")]
    first_time_free: bool,
}

impl TitleDetail {
    /// Flatten the chapters group more into a single [`Vec`] list
    pub fn flat_chapters_group(&self) -> Vec<Chapter> {
        self.chapter_groups
            .iter()
            .flat_map(|group| group.flatten())
            .collect()
    }

    /// A mutable reference to chapters group
    pub fn chapter_groups_mut(&mut self) -> &mut Vec<ChapterGroup> {
        &mut self.chapter_groups
    }
}

/// An information about a title with available languages
#[derive(Clone, Copy, AutoGetter, PartialEq, ::prost::Message)]
pub struct TitleLanguages {
    /// Title ID
    #[prost(uint64, tag = "1")]
    id: u64,
    /// Title language
    #[prost(enumeration = "Language", tag = "2")]
    #[skip_field]
    language: i32,
}

/// Title labels information
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct TitleLabels {
    /// Release schedule of the title
    #[prost(enumeration = "super::TitleReleaseSchedule", tag = "1")]
    #[skip_field]
    release_schedule: i32,
    /// Is the title simulpublished with the original release?
    #[prost(bool, tag = "2")]
    simulpublish: bool,
    /// Plan type of the title
    #[prost(string, tag = "3")]
    #[skip_field]
    plan_type: ::prost::alloc::string::String,
}

impl TitleLabels {
    /// Get the actual subscriptions plan type
    ///
    /// This will return the actual [`SubscriptionPlan`] type
    /// and fallback to [`SubscriptionPlan::Basic`] if the plan is not recognized.
    pub fn plan_type(&self) -> SubscriptionPlan {
        match SubscriptionPlan::from_str(&self.plan_type) {
            Ok(plan) => plan,
            Err(_) => SubscriptionPlan::Basic,
        }
    }
}

/// A list of titles contained into a group
///
/// This is a `v1` implementation of the title list.
///
/// See also: [`TitleListV2`]
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct TitleList {
    /// The group name
    #[prost(string, tag = "1")]
    name: ::prost::alloc::string::String,
    /// The list of titles
    #[prost(message, repeated, tag = "2")]
    titles: ::prost::alloc::vec::Vec<Title>,
}

/// A list of titles contained into a group
///
/// This is a `v2` implementation of the title list.
///
/// See also: [`TitleList`]
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct TitleListV2 {
    /// The group name
    #[prost(string, tag = "1")]
    name: ::prost::alloc::string::String,
    /// The list of titles
    #[prost(message, repeated, tag = "2")]
    titles: ::prost::alloc::vec::Vec<Title>,
    /// The group tags
    #[prost(message, repeated, tag = "3")]
    tags: ::prost::alloc::vec::Vec<Tag>,
    /// The group label
    #[prost(message, optional, tag = "4")]
    label: ::core::option::Option<Label>,
    /// The next chapter start timestamp
    #[prost(int64, optional, tag = "5")]
    #[skip_field]
    next_start_at: ::core::option::Option<i64>,
}

/// A list of titles with no grouping information
///
/// This is a `v1` implementation of the title list.
///
/// See also: [`TitleListOnlyV2`]
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct TitleListOnly {
    /// The list of titles
    #[prost(message, repeated, tag = "1")]
    titles: ::prost::alloc::vec::Vec<Title>,
}

/// A list of titles with no grouping information
///
/// This is a `v2` implementation of the title list.
///
/// See also: [`TitleListOnly`]
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct TitleListOnlyV2 {
    /// The list of titles
    #[prost(message, repeated, tag = "1")]
    titles: ::prost::alloc::vec::Vec<TitleListV2>,
}

/// An updated title information
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct UpdatedTitle {
    /// The title itself that got updated
    #[prost(message, tag = "1")]
    title: ::core::option::Option<Title>,
    /// Chapter ID that got updated
    #[prost(uint64, tag = "2")]
    chapter_id: u64,
    /// The chapter title
    #[prost(string, optional, tag = "3")]
    #[skip_field]
    chapter_title: ::core::option::Option<::prost::alloc::string::String>,
    /// The chapter subtitle
    #[prost(string, optional, tag = "4")]
    #[skip_field]
    chapter_subtitle: ::core::option::Option<::prost::alloc::string::String>,
    /// Is chapter is latest chapter update or not.
    #[prost(bool, tag = "5")]
    latest: bool,
    /// Does the chapter can only be read in long-strip mode only.
    #[prost(bool, tag = "6")]
    long_strip_only: bool,
    /// Does the chapter can only be read in horizontal mode only.
    #[prost(bool, tag = "7")]
    horizontal_only: bool,
}

/// A list of updated titles grouped by something
///
/// The following is `v1` implementation of the updated title group.
///
/// See also: [`UpdatedTitleGroupV2`]
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct UpdatedTitleGroup {
    /// The group name
    #[prost(string, tag = "1")]
    name: ::prost::alloc::string::String,
    /// The list of updated titles
    #[prost(message, repeated, tag = "2")]
    titles: ::prost::alloc::vec::Vec<UpdatedTitle>,
}

/// A list of grouped updated titles
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct UpdatedTitleList {
    /// The list of updated titles
    #[prost(message, repeated, tag = "1")]
    updates: ::prost::alloc::vec::Vec<UpdatedTitleGroup>,
}

/// An original implementation for updated title group
///
/// See also: [`UpdatedTitleGroupV2`]
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct UpdatedTitleGroupOriginal {
    /// The title group name
    #[prost(string, tag = "1")]
    name: ::prost::alloc::string::String,
    /// The chapter number of it
    #[prost(string, tag = "2")]
    chapter_number: ::prost::alloc::string::String,
    /// The list of updated titles
    #[prost(message, repeated, tag = "3")]
    titles: ::prost::alloc::vec::Vec<UpdatedTitle>,
    /// View count of the title
    #[prost(uint64, tag = "4")]
    view_count: u64,
    /// Title update status
    #[prost(enumeration = "TitleUpdateStatus", tag = "5")]
    #[skip_field]
    status: i32,
    /// Chapter start timestamp
    #[prost(int64, tag = "6")]
    start_at: i64,
}

/// A list of updated titles grouped by something
///
/// The following is `v2` implementation of the updated title group.
///
/// See also: [`UpdatedTitleGroup`]
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct UpdatedTitleGroupV2 {
    /// The group name
    #[prost(string, tag = "1")]
    name: ::prost::alloc::string::String,
    /// The titles in the group
    #[prost(message, repeated, tag = "2")]
    titles: ::prost::alloc::vec::Vec<UpdatedTitleGroupOriginal>,
    /// The group name days
    #[prost(uint64, tag = "3")]
    days: u64,
}

/// The detailed contents of the featured titles
///
/// This is `v1` implementation of the featured title contents.
///
/// See also: [`FeaturedTitleContentsV2`]
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct FeaturedTitleContents {
    /// The featured banner contents
    #[prost(message, optional, tag = "1")]
    banner: ::core::option::Option<Banner>,
    /// A list of featured titles
    #[prost(message, optional, tag = "2")]
    titles: ::core::option::Option<TitleList>,
}

/// A list of featured titles
///
/// This is `v1` implementation of the featured titles.
///
/// See also: [`FeaturedTitlesV2`]
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct FeaturedTitles {
    /// The main featured title
    #[prost(message, optional, tag = "1")]
    main: ::core::option::Option<Banner>,
    /// The first sub featured title
    #[prost(message, optional, tag = "2")]
    sub1: ::core::option::Option<Banner>,
    /// The second sub featured title
    #[prost(message, optional, tag = "3")]
    sub2: ::core::option::Option<Banner>,
    /// The featured title contents
    #[prost(message, repeated, tag = "4")]
    contents: ::prost::alloc::vec::Vec<FeaturedTitleContents>,
}

/// The detailed contents of the featured titles
///
/// This is `v2` implementation of the featured title contents.
///
/// See also: [`FeaturedTitleContents`]
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct FeaturedTitleContentsV2 {
    /// The featured banner contents
    #[prost(message, optional, tag = "1")]
    banner: ::core::option::Option<Banner>,
    /// A list of featured titles
    #[prost(message, optional, tag = "2")]
    titles: ::core::option::Option<TitleList>,
    /// The ranked titles
    #[prost(message, repeated, tag = "3")]
    ranked_titles: ::prost::alloc::vec::Vec<TitleRankingGroup>,
}

/// A list of featured titles
///
/// This is `v2` implementation of the featured titles.
///
/// See also: [`FeaturedTitles`]
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct FeaturedTitlesV2 {
    /// The list of top search banners
    #[prost(message, repeated, tag = "1")]
    banners: ::prost::alloc::vec::Vec<Banner>,
    /// The featured title contents
    #[prost(message, repeated, tag = "4")]
    contents: ::prost::alloc::vec::Vec<FeaturedTitleContentsV2>,
}

/// A subscribed or favorited title information
///
/// This also used as a history of recently read titles.
/// If the title is not subscribed, you *can* assume it's a history.
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct SubscribedTitle {
    /// The title itself
    #[prost(message, tag = "1")]
    title: ::core::option::Option<Title>,
    /// Is there any latest chapter available?
    #[prost(bool, tag = "2")]
    latest: bool,
    /// Is this title subscribed?
    #[prost(bool, tag = "3")]
    subscribed: bool,
}

/// An upcoming chapter of a title
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct UpcomingChapterTitle {
    /// The title itself
    #[prost(message, tag = "1")]
    title: ::core::option::Option<Title>,
    /// The next chapter name
    #[prost(string, tag = "2")]
    chapter_name: ::prost::alloc::string::String,
    /// The next chapter release timestamp
    #[prost(int64, tag = "3")]
    release_at: i64,
}

/// A single title update information
///
/// This is a `v1` implementation of the title update information.
///
/// See also: [`TitleUpdatedV2`]
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct TitleUpdated {
    /// The title itself
    #[prost(message, tag = "1")]
    title: ::core::option::Option<Title>,
    /// The update timestamp
    #[prost(string, tag = "2")]
    updated_at: ::prost::alloc::string::String,
}

/// A single title update information
///
/// This is a `v2` implementation of the title update information.
///
/// See also: [`TitleUpdated`]
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct TitleUpdatedV2 {
    /// The titles
    #[prost(message, repeated, tag = "1")]
    titles: ::prost::alloc::vec::Vec<TitleListV2>,
    /// The update timestamp
    #[prost(string, tag = "2")]
    updated_at: ::prost::alloc::string::String,
}

/// A list of title updates
///
/// This is a `v1` implementation of the title updates.
///
/// See also: [`TitleUpdatesV2`]
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct TitleUpdates {
    /// The list of title updates
    #[prost(message, repeated, tag = "1")]
    updates: ::prost::alloc::vec::Vec<TitleUpdated>,
}

/// A list of title updates
///
/// This is a `v2` implementation of the title updates.
///
/// See also: [`TitleUpdates`]
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct TitleUpdatesV2 {
    /// The list of title updates
    #[prost(message, repeated, tag = "1")]
    updates: ::prost::alloc::vec::Vec<TitleUpdatedV2>,
}

/// An information about a title with tickets available
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct TitleTicket {
    /// The title itself
    #[prost(message, tag = "1")]
    title: ::core::option::Option<Title>,
    /// First chapter that can be read with ticket
    #[prost(uint64, tag = "2")]
    first_chapter: u64,
    /// Last chapter that can be read with ticket
    #[prost(uint64, tag = "3")]
    last_chapter: u64,
}

/// A list of titles with tickets available
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct TitleTicketList {
    /// The list of titles with tickets
    #[prost(message, repeated, tag = "1")]
    titles: ::prost::alloc::vec::Vec<TitleTicket>,
}

/// A title highlighted in the home view
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct HighlightedTitle {
    /// The title itself
    #[prost(message, tag = "1")]
    title: ::core::option::Option<Title>,
    /// The associated chapter ID
    #[prost(uint64, tag = "2")]
    chapter_id: u64,
    /// The list of page URL
    #[prost(string, repeated, tag = "3")]
    pages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The page height
    #[prost(uint64, tag = "4")]
    height: u64,
    /// The page width
    #[prost(uint64, tag = "5")]
    width: u64,
    /// Is vertical only?
    #[prost(bool, tag = "6")]
    vertical_only: bool,
    /// Is horizontal only?
    #[prost(bool, tag = "7")]
    horizontal_only: bool,
}

/// A free title to be read
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct FreeTitle {
    /// The title itself
    #[prost(message, tag = "1")]
    title: ::core::option::Option<Title>,
    /// The updated timestamp of the title
    #[prost(string, tag = "2")]
    updated_at: ::prost::alloc::string::String,
}

/// A list of free titles
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct FreeTitles {
    /// The list of free titles
    #[prost(message, repeated, tag = "1")]
    titles: ::prost::alloc::vec::Vec<FreeTitle>,
}

/// A label applied to a list of titles
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct LabelledTitle {
    /// The label title
    #[prost(string, tag = "1")]
    label: ::prost::alloc::string::String,
    /// The list of titles
    #[prost(message, repeated, tag = "2")]
    titles: ::prost::alloc::vec::Vec<Title>,
}

/// A list of titles with labels
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct LabelledTitles {
    /// The label itself
    #[prost(message, tag = "1")]
    label: ::core::option::Option<Label>,
    /// The list of labelled titles
    #[prost(message, repeated, tag = "2")]
    labels: ::prost::alloc::vec::Vec<LabelledTitle>,
}

/// A title in ranking list
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct TitleRankingGroup {
    /// Original title ID
    #[prost(uint64, tag = "1")]
    original_title: u64,
    /// The titles in each available languages
    #[prost(message, repeated, tag = "2")]
    titles: ::prost::alloc::vec::Vec<Title>,
    /// The ranking score/position
    #[prost(uint64, tag = "3")]
    ranking: u64,
}

/// Title ranking list response
///
/// This is `v2` implementation of the title ranking list.
///
/// See also: [`TitleListOnly`]
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct TitleRankingList {
    /// The list of banners
    #[prost(message, repeated, tag = "1")]
    banners: ::prost::alloc::vec::Vec<Banner>,
    /// The updated timestamp of this ranking
    #[prost(int64, tag = "2")]
    updated_at: i64,
    /// The list of titles
    #[prost(message, repeated, tag = "3")]
    titles: ::prost::alloc::vec::Vec<TitleRankingGroup>,
}

/// A list of titles in subscriptions plan
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct SubscriptionTitles {
    /// The plan type
    #[prost(string, tag = "1")]
    #[skip_field]
    plan: ::prost::alloc::string::String,
    /// The list of titles
    #[prost(message, repeated, tag = "2")]
    titles: ::prost::alloc::vec::Vec<Title>,
}

impl SubscriptionTitles {
    /// Get the actual subscriptions plan type
    ///
    /// This will return the actual [`SubscriptionPlan`] type
    /// and fallback to [`SubscriptionPlan::Basic`] if the plan is not recognized.
    pub fn plan(&self) -> SubscriptionPlan {
        match SubscriptionPlan::from_str(&self.plan) {
            Ok(plan) => plan,
            Err(_) => SubscriptionPlan::Basic,
        }
    }
}
