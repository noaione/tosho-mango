//! A module containing information related to manga chapter.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

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
