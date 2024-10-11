//! A module containing information related to point acquisition and usage.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

#![allow(clippy::derive_partial_eq_without_eq)]

use tosho_macros::AutoGetter;

use super::{SubscriptionKind, SubscriptionStatus};

/// The user point information.
///
/// This will be available on almost each request.
#[derive(Clone, PartialEq, Copy, AutoGetter, ::prost::Message)]
pub struct UserPoint {
    /// Free/daily coins that you have.
    #[prost(uint64, tag = "1")]
    free: u64,
    /// Event/XP coins that you have.
    #[prost(uint64, tag = "2")]
    event: u64,
    /// Paid coins that you have.
    #[prost(uint64, tag = "3")]
    paid: u64,
}

impl UserPoint {
    /// Returns the total amount of points.
    ///
    /// # Examples
    /// ```
    /// # use tosho_musq::proto::UserPoint;
    /// #
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

    /// Subtract points from free slot
    pub fn subtract_free(&mut self, amount: u64) {
        self.free = self.free.saturating_sub(amount);
    }

    /// Subtract points from event slot
    pub fn subtract_event(&mut self, amount: u64) {
        self.event = self.event.saturating_sub(amount);
    }

    /// Subtract points from paid slot
    pub fn subtract_paid(&mut self, amount: u64) {
        self.paid = self.paid.saturating_sub(amount);
    }

    /// Add points to free slot
    pub fn add_free(&mut self, amount: u64) {
        self.free = self.free.saturating_add(amount);
    }

    /// Add points to event slot
    pub fn add_event(&mut self, amount: u64) {
        self.event = self.event.saturating_add(amount);
    }

    /// Add points to paid slot
    pub fn add_paid(&mut self, amount: u64) {
        self.paid = self.paid.saturating_add(amount);
    }

    /// Set points of free slot
    pub fn set_free(&mut self, amount: u64) {
        self.free = amount;
    }

    /// Set points of event slot
    pub fn set_event(&mut self, amount: u64) {
        self.event = amount;
    }

    /// Set points of paid slot
    pub fn set_paid(&mut self, amount: u64) {
        self.paid = amount;
    }
}

/// The user subscription information.
#[derive(Clone, PartialEq, AutoGetter, ::prost::Message)]
pub struct Subscription {
    /// The monthly subscription ID.
    #[prost(string, tag = "1")]
    monthly_id: ::prost::alloc::string::String,
    /// The yearly subscription ID.
    #[prost(string, tag = "2")]
    yearly_id: ::prost::alloc::string::String,
    /// The subscription kind of this subscription.
    #[prost(enumeration = "SubscriptionKind", tag = "3")]
    #[skip_field]
    status: i32,
    /// The unix timestamp of the end of the subscription.
    #[prost(int64, tag = "4")]
    end: i64,
    /// The event point that we will get from the subscription.
    #[prost(uint64, tag = "5")]
    event_point: u64,
    /// The subscription name.
    #[prost(string, tag = "6")]
    name: ::prost::alloc::string::String,
    /// The seasonally (tri-annual) subscription ID.
    #[prost(string, optional, tag = "7")]
    #[skip_field]
    seasonally_id: ::core::option::Option<::prost::alloc::string::String>,
    /// The half yearly subscription ID.
    #[prost(string, optional, tag = "8")]
    #[skip_field]
    half_yearly_id: ::core::option::Option<::prost::alloc::string::String>,
    /// The subscription banner URL.
    #[prost(string, optional, tag = "9")]
    #[skip_field]
    banner: ::core::option::Option<::prost::alloc::string::String>,
    /// The subscription series URL scheme.
    #[prost(string, optional, tag = "10")]
    #[skip_field]
    series_url_scheme: ::core::option::Option<::prost::alloc::string::String>,
    /// The monthly subscription descriptions.
    #[prost(string, repeated, tag = "11")]
    monthly_descriptions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
#[derive(Clone, PartialEq, AutoGetter, ::prost::Message)]
pub struct PointShopView {
    /// The user purse or point.
    #[prost(message, tag = "1")]
    #[copyable]
    user_point: ::core::option::Option<UserPoint>,
    /// The user point limit.
    #[prost(message, tag = "2")]
    #[copyable]
    point_limit: ::core::option::Option<UserPoint>,
    /// The next free point recovery time in seconds.
    #[prost(uint64, tag = "3")]
    next_recovery: u64,
    /// The subscription list.
    #[prost(message, repeated, tag = "4")]
    subscriptions: ::prost::alloc::vec::Vec<Subscription>,
    /// The billing or purchase list.
    #[prost(message, repeated, tag = "5")]
    billings: ::prost::alloc::vec::Vec<Billing>,
    /// The default selected billing index(?).
    #[prost(uint64, tag = "6")]
    default_select: u64,
    /// The user subscription status.
    #[prost(enumeration = "SubscriptionStatus", tag = "7")]
    #[skip_field]
    subscription_status: i32,
    /// The subscription terms and billing information.
    #[prost(string, optional, tag = "8")]
    #[skip_field]
    subscription_terms: ::core::option::Option<::prost::alloc::string::String>,
}

/// The node of each point purchase history.
#[derive(Clone, PartialEq, AutoGetter, ::prost::Message)]
pub struct PointHistory {
    /// The displayed/title text.
    #[prost(string, tag = "1")]
    displayed_text: ::prost::alloc::string::String,
    /// The free point that we use/get from the purchase.
    #[prost(uint64, tag = "2")]
    free_point: u64,
    /// The event point that we use/get from the purchase.
    #[prost(uint64, tag = "3")]
    event_point: u64,
    /// The paid point that we use/get from the purchase.
    #[prost(uint64, tag = "4")]
    paid_point: u64,
    /// The unix timestamp of the purchase/acquisition.
    #[prost(uint64, tag = "5")]
    created_at: u64,
}

/// Represents the point history view responses.
///
/// The ``Shop`` -> ``Acquisition History`` section in the actual app.
#[derive(Clone, PartialEq, AutoGetter, ::prost::Message)]
pub struct PointHistoryView {
    /// The user purse or point.
    #[prost(message, tag = "1")]
    #[copyable]
    pub user_point: ::core::option::Option<UserPoint>,
    /// The point history list.
    #[prost(message, repeated, tag = "2")]
    pub logs: ::prost::alloc::vec::Vec<PointHistory>,
}
