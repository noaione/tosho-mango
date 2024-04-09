//! A module containing information related to enums used in the library.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

/// The status of each request
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
pub enum Status {
    /// The request was successful.
    Success = 0,
    /// The content was not found.
    ContentNotFound = 1,
    /// An error has occurred.
    Unrecognized = -1,
}

/// The attached badge of the chapter.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
pub enum Badge {
    /// No badge marking for this chapter.
    None = 0,
    /// The chapter is marked as a new chapter.
    Update = 1,
    /// The chapter is marked as an advance/early chapter.
    Advance = 2,
    /// An error has occurred.
    Unrecognized = -1,
}

/// The attached badge of the manga.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
pub enum BadgeManga {
    /// No badge marking for this manga.
    None = 0,
    /// The manga is marked as a new manga.
    New = 1,
    /// The manga is marked for a new chapter/update. (Filled UP! badge)
    Update = 2,
    /// The manga is marked to have a new update this week. (Outlined UP! badge)
    UpdateWeek = 3,
    /// The manga is marked with unread chapters.
    Unread = 4,
    /// An error has occurred.
    Unrecognized = -1,
}

/// The attached label badge of the manga
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
pub enum LabelBadgeManga {
    /// No label badge marking for this manga.
    None = 0,
    /// MU! Original manga.
    Original = 1,
    /// An error has occurred.
    Unrecognized = -1,
}

/// The type of coin used to read the chapter.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
pub enum ConsumptionType {
    /// Any coint type can be used to read this chapter
    Any = 0,
    /// Only event or paid coins can be used to read this chapter
    EventOrPaid = 1,
    /// Only paid coins can be used to read this chapter
    Paid = 2,
    /// This chapter is free to read
    Free = 3,
    /// This chapter is currently on rental and can be read (Not used anymore since version 2.0.0)
    Rental = 4,
    /// This chapter is purchased and can be read anytime
    Purchased = 5,
    /// This chapter is on subscriptions and can be read if user has subscriptions
    Subscription = 6,
    /// An error has occurred.
    Unrecognized = -1,
}

/// Subscription status of the user.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
pub enum SubscriptionKind {
    /// Not subscribed.
    None = 0,
    /// Subscribed monthly.
    Monthly = 1,
    /// Subscribed yearly.
    Yearly = 2,
    /// Subscribed seasonally or tri-anually.
    Seasonally = 3,
    /// Subscribed half-yearly.
    HalfYearly = 4,
    /// An error has occurred.
    Unrecognized = -1,
}

impl SubscriptionKind {
    /// Check if the user is subscribed.
    ///
    /// ```rust
    /// use tosho_musq::proto::SubscriptionKind;
    ///
    /// let kind = SubscriptionKind::Monthly;
    /// assert!(kind.is_subscribed());
    /// ```
    pub fn is_subscribed(&self) -> bool {
        // Any subscription kind except None
        *self != SubscriptionKind::None
    }

    /// Get the proper name of the subscription kind.
    /// This will return the name of the subscription kind in English.
    pub fn as_name(&self) -> String {
        match self {
            SubscriptionKind::None => "None".to_string(),
            SubscriptionKind::Monthly => "Monthly".to_string(),
            SubscriptionKind::Yearly => "Yearly".to_string(),
            SubscriptionKind::Seasonally => "Seasonally".to_string(),
            SubscriptionKind::HalfYearly => "Half-Yearly".to_string(),
            SubscriptionKind::Unrecognized => "Unrecognized".to_string(),
        }
    }
}

/// The current subscription status of the user.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
pub enum SubscriptionStatus {
    /// The user is not subscribed.
    Unsubscribed = 0,
    /// The user is subscribed.
    Subscribed = 1,
    /// An error has occurred.
    Unrecognized = -1,
}

/// The subscription badge information of a manga.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
pub enum SubscriptionBadge {
    /// No subscription badge marking for this manga.
    None = 0,
    /// Has a subscription badge marking for this manga.
    Available = 1,
    /// Has a subscription badge marking for this manga, and the user is subscribed.
    Subscribed = 2,
    /// An error has occurred.
    Unrecognized = -1,
}

impl SubscriptionBadge {
    /// Check if manga has a subscription badge.
    ///
    /// ```rust
    /// use tosho_musq::proto::SubscriptionBadge;
    ///
    /// let badge = SubscriptionBadge::Available;
    /// assert!(badge.has_badge());
    /// ```
    pub fn has_badge(&self) -> bool {
        // Either Available or Subscribed
        *self == SubscriptionBadge::Available || *self == SubscriptionBadge::Subscribed
    }
}
