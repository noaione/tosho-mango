//! A module containing information related to user account.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

#![allow(clippy::derive_partial_eq_without_eq)]

/// Registration data response
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegistrationData {
    /// Device secret
    #[prost(string, tag = "1")]
    pub secret: ::prost::alloc::string::String,
}
