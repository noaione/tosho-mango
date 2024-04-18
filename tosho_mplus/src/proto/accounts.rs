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

/// User account ticket information
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserTickets {
    /// Current ticket amount
    #[prost(uint64, tag = "1")]
    pub ticket: u64,
    /// Next ticket refresh in UNIX timestamp
    #[prost(sint64, tag = "2")]
    pub next_refresh: i64,
}

/// User account subscription information
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserSubscription {
    /// Subscription plan type
    ///
    /// TODO: Move into enum when all plans are known
    #[prost(string, tag = "1")]
    pub plan: ::prost::alloc::string::String,
    /// Next payment in UNIX timestamp
    #[prost(sint64, tag = "2")]
    pub next_payment: i64,
    /// Is the current subscription a trial?
    #[prost(bool, tag = "3")]
    pub trial: bool,
    /// Is the subscription is currently on the process being downgraded?
    #[prost(bool, tag = "4")]
    pub downgrading: bool,
}
