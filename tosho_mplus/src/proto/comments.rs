//! A module containing information related to comments and feedback.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

#![allow(clippy::derive_partial_eq_without_eq)]

/// A comment response
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Comment {
    /// The comment ID
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// The comment index
    #[prost(uint64, tag = "2")]
    pub index: u64,
    /// The commentor user name
    #[prost(string, tag = "3")]
    pub user_name: ::prost::alloc::string::String,
    /// The commentor user avatar URL
    #[prost(string, tag = "4")]
    pub user_avatar: ::prost::alloc::string::String,
    /// Is the commentor ourselves?
    #[prost(bool, tag = "6")]
    pub is_self: bool,
    /// Is the comment liked by us?
    #[prost(bool, tag = "7")]
    pub liked: bool,
    /// The number of likes
    #[prost(uint64, tag = "9")]
    pub likes: u64,
    /// The comment content/body
    #[prost(string, tag = "10")]
    pub content: ::prost::alloc::string::String,
    /// The UNIX timestamp of the comment creation
    #[prost(uint64, tag = "11")]
    pub timestamp: u64,
}

/// A comment icon data
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommentIcon {
    /// The comment icon ID
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// The comment icon URL
    #[prost(string, tag = "2")]
    pub url: ::prost::alloc::string::String,
}

/// A comment list response
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommentList {
    /// The list of comments
    #[prost(message, repeated, tag = "1")]
    pub comments: ::prost::alloc::vec::Vec<Comment>,
    #[prost(bool, tag = "2")]
    pub set_user_name: bool,
}
