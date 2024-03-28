//! A module containing all the models/proto mapping used in the library.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

#![allow(clippy::derive_partial_eq_without_eq)]

pub mod common;
pub mod enums;

pub use common::*;
pub use enums::*;

/// Indicate a success response
///
/// Depending on the request type, not all field will be available.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuccessResponse {
    #[prost(uint64, tag = "1")]
    pub stub: u64,
}

/// A success or error response enum
///
/// This is used like `oneOf` in Protobuf so only one of them
/// will be available.
///
/// TODO:
/// - Remove [`Box`] when all message type are implemented in [`SuccessResponse`]
#[derive(Clone, PartialEq, ::prost::Oneof)]
pub enum SuccessOrError {
    #[prost(message, tag = "1")]
    Success(Box<SuccessResponse>),
    #[prost(message, tag = "2")]
    Error(Box<ErrorResponse>),
}

/// Proto response from the API, wrap a simple `oneOf` of [`SuccessOrError`]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(oneof = "SuccessOrError", tags = "1, 2")]
    pub response: ::core::option::Option<SuccessOrError>,
}
