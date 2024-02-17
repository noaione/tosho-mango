//! A module containing information related to chapter.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

#![allow(clippy::derive_partial_eq_without_eq)]

use super::enums::{Badge, ConsumptionType, Status};

/// Represents a single chapter.
///
/// The following is ``v1`` implementation of the chapter that used by the API.
///
/// See also: [``ChapterV2``]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Chapter {
    /// The chapter ID.
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// The chapter title.
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// The chapter subtitle, usually the actual chapter title.
    #[prost(string, optional, tag = "3")]
    pub subtitle: ::core::option::Option<::prost::alloc::string::String>,
    /// The chapter thumbnail URL.
    #[prost(string, tag = "4")]
    pub thumbnail_url: ::prost::alloc::string::String,
    /// The chapter consumption type.
    #[prost(enumeration = "ConsumptionType", tag = "5")]
    pub consumption: i32,
    /// The chapter price in coins, check with [``Self::consumption``] to see which type of coins
    /// can be used to read this chapter.
    #[prost(uint64, tag = "6")]
    pub price: u64,
    /// How much chapter rental period left in seconds.
    ///
    /// If the value is ``0``, the chapter rental period has ended.
    /// If the value is ``None``, the chapter is not yet rented.
    #[prost(uint64, optional, tag = "7")]
    pub end_of_rental_period: ::core::option::Option<u64>,
    /// How many comments this chapter has.
    #[prost(uint64, optional, tag = "8")]
    pub comments: ::core::option::Option<u64>,
    /// When this chapter was published.
    #[prost(string, optional, tag = "9")]
    pub published_at: ::core::option::Option<::prost::alloc::string::String>,
    /// The chapter badge.
    #[prost(enumeration = "Badge", tag = "10")]
    pub badge: i32,
    /// The first page URL of this chapter.
    #[prost(string, tag = "11")]
    pub first_page_url: ::prost::alloc::string::String,
}

impl Chapter {
    /// Whether or not this chapter is free.
    ///
    /// # Examples
    /// ```
    /// use tosho_musq::proto::Chapter;
    ///
    /// let mut chapter = Chapter {
    ///     id: 1,
    ///     title: "Test".to_string(),
    ///     subtitle: Some("Subtitle".to_string()),
    ///     thumbnail_url: "https://example.com".to_string(),
    ///     consumption: 1,
    ///     price: 0,
    ///     end_of_rental_period: Some(0),
    ///     comments: Some(0),
    ///     published_at: Some("2021-01-01T00:00:00Z".to_string()),
    ///     badge: 0,
    ///     first_page_url: "https://example.com".to_string(),
    /// };
    ///
    /// assert!(chapter.is_free());
    ///
    /// chapter.price = 10;
    ///
    /// assert!(!chapter.is_free());
    /// ```
    pub fn is_free(&self) -> bool {
        self.price == 0
    }

    /// Format the chapter title and subtitle into a single string.
    ///
    /// If the subtitle is [`None`], the title will be returned as is.
    ///
    /// # Examples
    /// ```
    /// use tosho_musq::proto::Chapter;
    ///
    /// let mut chapter = Chapter {
    ///     id: 1,
    ///     title: "Test".to_string(),
    ///     subtitle: Some("Subtitle".to_string()),
    ///     thumbnail_url: "https://example.com".to_string(),
    ///     consumption: 1,
    ///     price: 0,
    ///     end_of_rental_period: Some(0),
    ///     comments: Some(0),
    ///     published_at: Some("2021-01-01T00:00:00Z".to_string()),
    ///     badge: 0,
    ///     first_page_url: "https://example.com".to_string(),
    /// };
    ///
    /// assert_eq!(chapter.as_chapter_title(), "Test — Subtitle");
    ///
    /// chapter.subtitle = None;
    ///
    /// assert_eq!(chapter.as_chapter_title(), "Test");
    /// ```
    pub fn as_chapter_title(&self) -> String {
        let base_title = self.title.clone();
        if let Some(subtitle) = self.subtitle.clone() {
            format!("{} — {}", base_title, subtitle)
        } else {
            base_title
        }
    }
}

/// Represents a single chapter.
///
/// The following is ``v2`` implementation of the chapter that used by the API.
///
/// See also: [``Chapter``]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChapterV2 {
    /// The chapter ID.
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// The chapter title.
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// The chapter subtitle, usually the actual chapter title.
    #[prost(string, optional, tag = "3")]
    pub subtitle: ::core::option::Option<::prost::alloc::string::String>,
    /// The chapter thumbnail URL.
    #[prost(string, tag = "4")]
    pub thumbnail_url: ::prost::alloc::string::String,
    /// The chapter consumption type.
    #[prost(enumeration = "ConsumptionType", tag = "5")]
    pub consumption: i32,
    /// The chapter price in coins, check with [``Self::consumption``] to see which type of coins
    /// can be used to read this chapter.
    #[prost(uint64, tag = "6")]
    pub price: u64,
    /// How much chapter rental period left in seconds.
    ///
    /// If the value is ``0``, the chapter rental period has ended.
    /// If the value is ``None``, the chapter is not yet rented.
    #[prost(uint64, optional, tag = "7")]
    pub end_of_rental_period: ::core::option::Option<u64>,
    /// How many comments this chapter has.
    #[prost(uint64, optional, tag = "8")]
    pub comments: ::core::option::Option<u64>,
    /// When this chapter was published.
    #[prost(string, optional, tag = "9")]
    pub published_at: ::core::option::Option<::prost::alloc::string::String>,
    /// The chapter badge.
    #[prost(enumeration = "Badge", tag = "10")]
    pub badge: i32,
    /// The first page URL of this chapter.
    #[prost(string, tag = "11")]
    pub first_page_url: ::prost::alloc::string::String,
    /// Whether this is the final chapter or not.
    #[prost(bool, tag = "12")]
    pub final_chapter: bool,
    /// How many pages this chapter has.
    #[prost(uint64, tag = "13")]
    pub page_count: u64,
    /// How many times this chapter has been read.
    #[prost(uint64, tag = "14")]
    pub read_count: u64,
}

impl ChapterV2 {
    /// Whether or not this chapter is free.
    ///
    /// # Examples
    /// ```
    /// use tosho_musq::proto::ChapterV2;
    ///
    /// let mut chapter = ChapterV2 {
    ///     id: 1,
    ///     title: "Test".to_string(),
    ///     subtitle: Some("Subtitle".to_string()),
    ///     thumbnail_url: "https://example.com".to_string(),
    ///     consumption: 1,
    ///     price: 0,
    ///     end_of_rental_period: Some(0),
    ///     comments: Some(0),
    ///     published_at: Some("2021-01-01T00:00:00Z".to_string()),
    ///     badge: 0,
    ///     first_page_url: "https://example.com".to_string(),
    ///     final_chapter: false,
    ///     page_count: 2,
    ///     read_count: 0,
    /// };
    ///
    /// assert!(chapter.is_free());
    ///
    /// chapter.price = 10;
    ///
    /// assert!(!chapter.is_free());
    /// ```
    pub fn is_free(&self) -> bool {
        self.price == 0
    }

    /// Format the chapter title and subtitle into a single string.
    ///
    /// If the subtitle is [`None`], the title will be returned as is.
    ///
    /// # Examples
    /// ```
    /// use tosho_musq::proto::ChapterV2;
    ///
    /// let mut chapter = ChapterV2 {
    ///     id: 1,
    ///     title: "Test".to_string(),
    ///     subtitle: Some("Subtitle".to_string()),
    ///     thumbnail_url: "https://example.com".to_string(),
    ///     consumption: 1,
    ///     price: 0,
    ///     end_of_rental_period: Some(0),
    ///     comments: Some(0),
    ///     published_at: Some("2021-01-01T00:00:00Z".to_string()),
    ///     badge: 0,
    ///     first_page_url: "https://example.com".to_string(),
    ///     final_chapter: false,
    ///     page_count: 2,
    ///     read_count: 0,
    /// };
    ///
    /// assert_eq!(chapter.as_chapter_title(), "Test — Subtitle");
    ///
    /// chapter.subtitle = None;
    ///
    /// assert_eq!(chapter.as_chapter_title(), "Test");
    /// ```
    pub fn as_chapter_title(&self) -> String {
        let base_title = self.title.clone();
        if let Some(subtitle) = self.subtitle.clone() {
            format!("{} — {}", base_title, subtitle)
        } else {
            base_title
        }
    }
}

impl From<ChapterV2> for Chapter {
    fn from(value: ChapterV2) -> Self {
        Self {
            id: value.id,
            title: value.title,
            subtitle: value.subtitle,
            thumbnail_url: value.thumbnail_url,
            consumption: value.consumption,
            price: value.price,
            end_of_rental_period: value.end_of_rental_period,
            comments: value.comments,
            published_at: value.published_at,
            badge: value.badge,
            first_page_url: value.first_page_url,
        }
    }
}

/// Represents a chapter page.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChapterPage {
    /// The page URL.
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    /// The video HLS URL.
    #[prost(string, optional, tag = "2")]
    pub video_url: ::core::option::Option<::prost::alloc::string::String>,
    /// The chapter page URL intents.
    #[prost(string, optional, tag = "3")]
    pub intent_url: ::core::option::Option<::prost::alloc::string::String>,
    /// The extra ID, if any.
    #[prost(uint64, optional, tag = "4")]
    pub extra_id: ::core::option::Option<u64>,
}

impl ChapterPage {
    /// The file name of the image.
    ///
    /// # Examples
    /// ```
    /// use tosho_musq::proto::ChapterPage;
    ///
    /// let page = ChapterPage {
    ///     url: "/path/to/image.avif".to_string(),
    ///     video_url: None,
    ///     intent_url: None,
    ///     extra_id: None,
    /// };
    ///
    /// assert_eq!(page.file_name(), "image.avif");
    /// ```
    pub fn file_name(&self) -> String {
        let url = self.url.clone();
        // split at the last slash
        let split: Vec<&str> = url.rsplitn(2, '/').collect();
        // Remove extra URL parameters
        let file_name: Vec<&str> = split[0].split('?').collect();
        file_name[0].to_string()
    }

    /// The file extension of the image.
    ///
    /// # Examples
    /// ```
    /// use tosho_musq::proto::ChapterPage;
    ///
    /// let page = ChapterPage {
    ///     url: "/path/to/image.avif".to_string(),
    ///     video_url: None,
    ///     intent_url: None,
    ///     extra_id: None,
    /// };
    ///
    /// assert_eq!(page.extension(), "avif");
    /// ```
    pub fn extension(&self) -> String {
        let file_name = self.file_name();
        // split at the last dot
        let split: Vec<&str> = file_name.rsplitn(2, '.').collect();

        if split.len() == 2 {
            split[0].to_string()
        } else {
            "".to_string()
        }
    }

    /// The file stem of the image.
    ///
    /// # Examples
    /// ```
    /// use tosho_musq::proto::ChapterPage;
    ///
    /// let page = ChapterPage {
    ///     url: "/path/to/image.avif".to_string(),
    ///     video_url: None,
    ///     intent_url: None,
    ///     extra_id: None,
    /// };
    ///
    /// assert_eq!(page.file_stem(), "image");
    /// ```
    pub fn file_stem(&self) -> String {
        let file_name = self.file_name();
        // split at the last dot
        let split: Vec<&str> = file_name.rsplitn(2, '.').collect();

        if split.len() == 2 {
            split[1].to_string()
        } else {
            file_name
        }
    }
}

/// Represents a chapter viewer response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChapterViewer {
    /// The status of the request.
    #[prost(enumeration = "Status", tag = "1")]
    pub status: i32,
    /// The user purse or point.
    #[prost(message, tag = "2")]
    pub user_point: ::core::option::Option<super::UserPoint>,
    /// The chapter images list.
    #[prost(message, repeated, tag = "3")]
    pub images: ::prost::alloc::vec::Vec<ChapterPage>,
    /// The next chapter, if any.
    #[prost(message, optional, tag = "4")]
    pub next_chapter: ::core::option::Option<Chapter>,
    /// The previous chapter, if any.
    #[prost(message, optional, tag = "5")]
    pub previous_chapter: ::core::option::Option<Chapter>,
    /// The chapter page start.
    #[prost(uint64, tag = "6")]
    pub page_start: u64,
    /// Whether the chapter comment is enabled or not.
    #[prost(bool, tag = "8")]
    pub is_comment_enabled: bool,
}

/// Represents an SNS/Social Media sharing info.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SNSInfo {
    /// The text body.
    #[prost(string, tag = "1")]
    pub body: ::prost::alloc::string::String,
    /// The URL/intent url.
    #[prost(string, tag = "2")]
    pub url: ::prost::alloc::string::String,
}

/// Represents a single page? block
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PageBlock {
    /// The chapter ID.
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// The chapter title.
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// The images list for the current block.
    #[prost(message, repeated, tag = "3")]
    pub images: ::prost::alloc::vec::Vec<ChapterPage>,
    /// Whether this is the last page or not.
    #[prost(bool, tag = "4")]
    pub last_page: bool,
    /// The chapter page start.
    #[prost(uint64, tag = "5")]
    pub start_page: u64,
    /// The chapter SNS.
    #[prost(message, tag = "6")]
    pub sns: ::core::option::Option<SNSInfo>,
    /// The chapter page start.
    #[prost(uint64, tag = "7")]
    pub page_start: u64,
    /// The chapter page end.
    #[prost(uint64, tag = "8")]
    pub page_end: u64,
}

/// Represents a chapter viewer response.
///
/// The following is ``v2`` implementation of the chapter viewer response that used by the API.
///
/// See also: [``ChapterViewer``]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChapterViewerV2 {
    /// The status of the request.
    #[prost(enumeration = "Status", tag = "1")]
    pub status: i32,
    /// The user purse or point.
    #[prost(message, tag = "2")]
    pub user_point: ::core::option::Option<super::UserPoint>,
    /// The chapter images list.
    #[prost(message, repeated, tag = "3")]
    pub blocks: ::prost::alloc::vec::Vec<PageBlock>,
    /// The next chapter, if any.
    #[prost(message, optional, tag = "4")]
    pub next_chapter: ::core::option::Option<ChapterV2>,
    /// Whether the chapter comment is enabled or not.
    #[prost(bool, tag = "5")]
    pub is_comment_enabled: bool,
    /// Whether the chapter view guide is enabled or not.
    #[prost(bool, tag = "6")]
    pub enable_guide: bool,
}
