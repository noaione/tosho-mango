//! A module containing information related to manga chapter.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

use std::str::FromStr;

use crate::helper::SubscriptionPlan;

/// A single chapter information
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Chapter {
    /// Title ID
    #[prost(uint64, tag = "1")]
    pub title_id: u64,
    /// Chapter ID
    #[prost(uint64, tag = "2")]
    pub chapter_id: u64,
    /// Chapter title
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
    /// Chapter subtitle
    #[prost(string, optional, tag = "4")]
    pub subtitle: ::core::option::Option<::prost::alloc::string::String>,
    /// Chapter thumbnail URL
    #[prost(string, tag = "5")]
    pub thumbnail: ::prost::alloc::string::String,
    /// Chapter published/start UNIX timestamp
    #[prost(sint64, tag = "6")]
    pub published_at: i64,
    /// Chapter end viewing period UNIX timestamp
    #[prost(sint64, optional, tag = "7")]
    pub end_at: ::core::option::Option<i64>,
    /// Is the chapter already viewed?
    #[prost(bool, tag = "8")]
    pub viewed: bool,
    /// Is the chapter can be read in vertical mode only?
    #[prost(bool, tag = "9")]
    pub vertical_only: bool,
    /// Chapter end viewing by ticket timestamp
    #[prost(sint64, optional, tag = "10")]
    pub ticket_end_at: ::core::option::Option<i64>,
    /// Is the chapter can be read for free?
    #[prost(bool, tag = "11")]
    pub free: bool,
    /// Is the chapter can be read in horizontal mode only?
    #[prost(bool, tag = "12")]
    pub horizontal_only: bool,
    /// Chapter view count
    #[prost(uint64, tag = "13")]
    pub view_count: u64,
    /// Chapter comment count
    #[prost(uint64, tag = "14")]
    pub comment_count: u64,
}

impl Chapter {
    /// Can this chapter be read for free?
    pub fn is_free(&self) -> bool {
        if self.free {
            return true;
        }

        if let Some(end_at) = self.end_at {
            let current_time = chrono::Utc::now().timestamp();
            current_time < end_at
        } else {
            false
        }
    }

    /// Can this chapter be read with ticket?
    pub fn is_ticketed(&self) -> bool {
        if let Some(ticket_end_at) = self.ticket_end_at {
            let current_time = chrono::Utc::now().timestamp();
            current_time < ticket_end_at
        } else {
            false
        }
    }

    /// Get the default viewing mode
    pub fn default_view_mode(&self) -> String {
        if self.vertical_only {
            "vertical".to_string()
        } else {
            "horizontal".to_string()
        }
    }

    /// Format the chapter title and subtitle into a single string.
    ///
    /// If the subtitle is [`None`], the title will be returned as is.
    pub fn as_chapter_title(&self) -> String {
        let base_title = self.title.clone();
        if let Some(subtitle) = self.subtitle.clone() {
            format!("{} — {}", base_title, subtitle)
        } else {
            base_title
        }
    }
}

/// A group of chapters
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChapterGroup {
    /// The chapter numbers range
    #[prost(string, tag = "1")]
    pub chapters: ::prost::alloc::string::String,
    /// The first chapters list
    #[prost(message, repeated, tag = "2")]
    pub first_chapters: ::prost::alloc::vec::Vec<Chapter>,
    /// The mid chapters list
    #[prost(message, repeated, tag = "3")]
    pub mid_chapters: ::prost::alloc::vec::Vec<Chapter>,
    /// The last chapters list
    #[prost(message, repeated, tag = "4")]
    pub last_chapters: ::prost::alloc::vec::Vec<Chapter>,
}

impl ChapterGroup {
    /// Group the chapters into a single list
    pub fn flatten(&self) -> Vec<Chapter> {
        let mut chapters = Vec::new();
        chapters.extend_from_slice(&self.first_chapters);
        chapters.extend_from_slice(&self.mid_chapters);
        chapters.extend_from_slice(&self.last_chapters);
        chapters
    }
}

/// A page of a chapter
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChapterPage {
    /// The page url
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    /// The image width
    #[prost(uint64, tag = "2")]
    pub width: u64,
    /// The image height
    #[prost(uint64, tag = "3")]
    pub height: u64,
    /// The image type/kind
    #[prost(enumeration = "super::PageType", tag = "4")]
    pub kind: i32,
    /// The image encryption key
    #[prost(string, optional, tag = "5")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
}

/// A chapter page of a banners
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChapterPageBanner {
    /// Banner title
    #[prost(string, optional, tag = "1")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    /// Banner list
    #[prost(message, repeated, tag = "2")]
    pub banners: ::prost::alloc::vec::Vec<super::common::Banner>,
}

/// A chapter last page response
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChapterPageLastPage {
    /// Current chapter
    #[prost(message, optional, tag = "1")]
    pub chapter: ::core::option::Option<Chapter>,
    /// Next chapter
    #[prost(message, optional, tag = "2")]
    pub next_chapter: ::core::option::Option<Chapter>,
    /// Top comments of this chapter
    #[prost(message, repeated, tag = "3")]
    pub top_comments: ::prost::alloc::vec::Vec<super::comments::Comment>,
    /// Is the user subscribed
    #[prost(bool, tag = "4")]
    pub subscribed: bool,
    /// The next chapter timestamp
    #[prost(sint64, optional, tag = "5")]
    pub next_chapter_at: ::core::option::Option<i64>,
    /// The chapter type
    #[prost(enumeration = "super::ChapterType", tag = "6")]
    pub chapter_type: i32,
    /// Movie reward of the chapter
    // #[prost(message, optional, tag = "8")]
    // pub movie_reward: ::core::option::Option<super::common::PopupMessage>,
    /// Banner list
    #[prost(message, optional, tag = "9")]
    pub banner: ::core::option::Option<super::common::Banner>,
    /// Title ticket list
    #[prost(message, repeated, tag = "10")]
    pub title_tickets: ::prost::alloc::vec::Vec<super::titles::Title>,
    /// Publisher banner
    #[prost(message, optional, tag = "11")]
    pub publisher_banner: ::core::option::Option<super::common::Banner>,
    /// User tickets
    #[prost(message, optional, tag = "12")]
    pub user_tickets: ::core::option::Option<super::accounts::UserTickets>,
    /// Is next chapter can be read by ticket?
    #[prost(bool, tag = "13")]
    pub next_chapter_ticket: bool,
    /// Is next chapter can be read for free one time only?
    #[prost(bool, tag = "14")]
    pub next_chapter_free: bool,
    /// Is next chapter can be read only with subscription?
    #[prost(bool, tag = "16")]
    pub next_chapter_subscription: bool,
}

/// A chapter page response
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChapterPageResponse {
    /// A response to a chapter page (a.k.a the manga page)
    #[prost(message, optional, tag = "1")]
    pub page: ::core::option::Option<ChapterPage>,
    /// A response to a banner page
    #[prost(message, optional, tag = "2")]
    pub banner: ::core::option::Option<ChapterPageBanner>,
    /// A response to a last page
    #[prost(message, optional, tag = "3")]
    pub last_page: ::core::option::Option<ChapterPageLastPage>,
    /// A response to an insert banner
    #[prost(message, optional, tag = "5")]
    pub insert_banner: ::core::option::Option<ChapterPageBanner>,
}

/// A chapter viewer response
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChapterViewer {
    /// Chapter pages
    #[prost(message, repeated, tag = "1")]
    pub pages: ::prost::alloc::vec::Vec<ChapterPageResponse>,
    /// Chapter ID
    #[prost(uint64, tag = "2")]
    pub chapter_id: u64,
    /// All available chapters
    #[prost(message, repeated, tag = "3")]
    pub chapters: ::prost::alloc::vec::Vec<Chapter>,
    // SNS: 4
    /// Manga title
    #[prost(string, tag = "5")]
    pub title: ::prost::alloc::string::String,
    /// Chapter title
    #[prost(string, tag = "6")]
    pub chapter_title: ::prost::alloc::string::String,
    /// Number of comments
    #[prost(uint64, tag = "7")]
    pub comment_count: u64,
    /// Is vertical only?
    #[prost(bool, tag = "8")]
    pub vertical_only: bool,
    /// Title ID
    #[prost(uint64, tag = "9")]
    pub title_id: u64,
    /// Is the first page on the right side (first page is odd number)
    #[prost(bool, tag = "10")]
    pub first_page_right: bool,
    /// Region code of the title
    #[prost(string, tag = "11")]
    pub region_code: ::prost::alloc::string::String,
    /// Is horizontal only?
    #[prost(bool, tag = "12")]
    pub horizontal_only: bool,
    /// User subscription info
    #[prost(message, optional, tag = "13")]
    pub user_subscription: ::core::option::Option<super::accounts::UserSubscription>,
    /// User plan type
    #[prost(string, tag = "14")]
    pub plan_type: ::prost::alloc::string::String,
}

impl ChapterViewer {
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
