//! A module containing all the models/proto mapping used in the library.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

#![allow(clippy::derive_partial_eq_without_eq)]

pub mod common;
pub mod enums;

pub use common::*;
pub use enums::*;

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuccessResponse {
    #[prost(uint64, tag = "1")]
    pub stub: u64,
}

#[derive(Clone, PartialEq, ::prost::Oneof)]
pub enum SuccessOrError {
    #[prost(message, tag = "1")]
    Success(SuccessResponse),
    #[prost(message, tag = "2")]
    Error(ErrorResponse),
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(oneof = "SuccessOrError", tags = "1, 2")]
    pub response: ::core::option::Option<SuccessOrError>,
}
