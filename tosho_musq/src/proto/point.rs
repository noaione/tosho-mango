//! A module containing information related to point acquisition and usage.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

#![allow(clippy::derive_partial_eq_without_eq)]

use super::SubscriptionKind;

/// The user point information.
///
/// This will be available on almost each request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserPoint {
    /// Free/daily coins that you have.
    #[prost(uint64, tag = "1")]
    pub free: u64,
    /// Event/XP coins that you have.
    #[prost(uint64, tag = "2")]
    pub event: u64,
    /// Paid coins that you have.
    #[prost(uint64, tag = "3")]
    pub paid: u64,
}

impl UserPoint {
    /// Returns the total amount of points.
    ///
    /// # Examples
    /// ```
    /// use tosho_musq::proto::UserPoint;
    ///
    /// let points = UserPoint {
    ///    free: 100,
    ///    event: 200,
    ///    paid: 300,
    /// };
    ///
    /// assert_eq!(points.sum(), 600);
    /// ```
    pub fn sum(&self) -> u64 {
        self.free + self.event + self.paid
    }
}

/// The user subscription information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Subscription {
    /// The monthly subscription ID.
    #[prost(string, tag = "1")]
    pub monthly_id: ::prost::alloc::string::String,
    /// The yearly subscription ID.
    #[prost(string, tag = "2")]
    pub yearly_id: ::prost::alloc::string::String,
    /// The subscription kind of this subscription.
    #[prost(enumeration = "SubscriptionKind", tag = "3")]
    pub status: i32,
    /// The unix timestamp of the end of the subscription.
    #[prost(int64, tag = "4")]
    pub end: i64,
    /// The event point that we will get from the subscription.
    #[prost(uint64, tag = "5")]
    pub event_point: u64,
    /// The subscription name.
    #[prost(string, tag = "6")]
    pub name: ::prost::alloc::string::String,
    /// The seasonally (tri-annual) subscription ID.
    #[prost(string, optional, tag = "7")]
    pub seasonally_id: ::core::option::Option<::prost::alloc::string::String>,
    /// The half yearly subscription ID.
    #[prost(string, optional, tag = "8")]
    pub half_yearly_id: ::core::option::Option<::prost::alloc::string::String>,
    /// The subscription banner URL.
    #[prost(string, optional, tag = "9")]
    pub banner: ::core::option::Option<::prost::alloc::string::String>,
    /// The subscription series URL scheme.
    #[prost(string, optional, tag = "10")]
    pub series_url_scheme: ::core::option::Option<::prost::alloc::string::String>,
    /// The monthly subscription descriptions.
    #[prost(string, repeated, tag = "11")]
    pub monthly_descriptions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}

/// The billing or the coin purchase information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Billing {
    /// The billing ID.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The event point that we will get from the purchase.
    #[prost(uint64, tag = "2")]
    pub event_point: u64,
    /// The paid point that we will get from the purchase.
    #[prost(uint64, tag = "3")]
    pub paid_point: u64,
    /// The purchase/billing details.
    #[prost(string, tag = "4")]
    pub details: ::prost::alloc::string::String,
}

impl Billing {
    /// The total point that we will get from the purchases.
    ///
    /// # Example
    /// ```
    /// use tosho_musq::proto::Billing;
    ///
    /// let billing = Billing {
    ///    id: "id".to_string(),
    ///    event_point: 100,
    ///    paid_point: 100,
    ///    details: "details".to_string(),
    /// };
    ///
    /// assert_eq!(billing.total_point(), 200);
    /// ```
    pub fn total_point(&self) -> u64 {
        self.event_point + self.paid_point
    }
}

/// Represents the point shop view responses.
///
/// The ``Shop`` section in the actual app.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PointShopView {
    /// The user purse or point.
    #[prost(message, tag = "1")]
    pub user_point: ::core::option::Option<UserPoint>,
    /// The user point limit.
    #[prost(message, tag = "2")]
    pub point_limit: ::core::option::Option<UserPoint>,
    /// The next free point recovery time in seconds.
    #[prost(uint64, tag = "3")]
    pub next_recovery: u64,
    /// The subscription list.
    #[prost(message, repeated, tag = "4")]
    pub subscriptions: ::prost::alloc::vec::Vec<Subscription>,
    /// The billing or purchase list.
    #[prost(message, repeated, tag = "5")]
    pub billings: ::prost::alloc::vec::Vec<Billing>,
    /// The default selected billing index(?).
    #[prost(uint64, tag = "6")]
    pub default_select: u64,
}

/// The node of each point purchase history.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PointHistory {
    /// The displayed/title text.
    #[prost(string, tag = "1")]
    pub displayed_text: ::prost::alloc::string::String,
    /// The free point that we use/get from the purchase.
    #[prost(uint64, tag = "2")]
    pub free_point: u64,
    /// The event point that we use/get from the purchase.
    #[prost(uint64, tag = "3")]
    pub event_point: u64,
    /// The paid point that we use/get from the purchase.
    #[prost(uint64, tag = "4")]
    pub paid_point: u64,
    /// The unix timestamp of the purchase/acquisition.
    #[prost(uint64, tag = "5")]
    pub created_at: u64,
}

/// Represents the point history view responses.
///
/// The ``Shop`` -> ``Acquisition History`` section in the actual app.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PointHistoryView {
    /// The user purse or point.
    #[prost(message, tag = "1")]
    pub user_point: ::core::option::Option<super::UserPoint>,
    /// The point history list.
    #[prost(message, repeated, tag = "2")]
    pub logs: ::prost::alloc::vec::Vec<PointHistory>,
}
