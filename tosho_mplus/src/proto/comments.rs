//! A module containing information related to comments and feedback.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

#![allow(clippy::derive_partial_eq_without_eq)]

use tosho_macros::AutoGetter;

/// A comment response
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct Comment {
    /// The comment ID
    #[prost(uint64, tag = "1")]
    id: u64,
    /// The comment index
    #[prost(uint64, tag = "2")]
    index: u64,
    /// The commentor user name
    #[prost(string, tag = "3")]
    user_name: ::prost::alloc::string::String,
    /// The commentor user avatar URL
    #[prost(string, tag = "4")]
    user_avatar: ::prost::alloc::string::String,
    /// Is the commentor ourselves?
    #[prost(bool, tag = "6")]
    is_self: bool,
    /// Is the comment liked by us?
    #[prost(bool, tag = "7")]
    liked: bool,
    /// The number of likes
    #[prost(uint64, tag = "9")]
    likes: u64,
    /// The comment content/body
    #[prost(string, tag = "10")]
    content: ::prost::alloc::string::String,
    /// The UNIX timestamp of the comment creation
    #[prost(int64, tag = "11")]
    timestamp: i64,
}

/// A comment icon data
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct CommentIcon {
    /// The comment icon ID
    #[prost(uint64, tag = "1")]
    id: u64,
    /// The comment icon URL
    #[prost(string, tag = "2")]
    url: ::prost::alloc::string::String,
}

/// A comment list response
#[derive(Clone, AutoGetter, PartialEq, ::prost::Message)]
pub struct CommentList {
    /// The list of comments
    #[prost(message, repeated, tag = "1")]
    comments: ::prost::alloc::vec::Vec<Comment>,
    #[prost(bool, tag = "2")]
    #[allow(clippy::missing_docs_in_private_items)]
    set_user_name: bool,
}
