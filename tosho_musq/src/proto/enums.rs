//! A module containing information related to enums used in the library.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

/// The status of each request
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
pub enum Status {
    /// An error has occurred.
    Unrecognized = -1,
    /// The request was successful.
    Success = 0,
    /// The content was not found.
    ContentNotFound = 1,
}

/// The attached badge of the chapter.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
pub enum Badge {
    /// An error has occurred.
    Unrecognized = -1,
    /// No badge marking for this chapter.
    None = 0,
    /// The chapter is marked as a new chapter.
    Update = 1,
    /// The chapter is marked as an advance/early chapter.
    Advance = 2,
}

/// The attached badge of the manga.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
pub enum BadgeManga {
    /// An error has occurred.
    Unrecognized = -1,
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
}

/// The attached label badge of the manga
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
pub enum LabelBadgeManga {
    /// An error has occurred.
    Unrecognized = -1,
    /// No label badge marking for this manga.
    None = 0,
    /// MU! Original manga.
    Original = 1,
}

/// The type of coin used to read the chapter.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
pub enum ConsumptionType {
    /// An error has occurred.
    Unrecognized = -1,
    /// Any coint type can be used to read this chapter
    Any = 0,
    /// Only event or paid coins can be used to read this chapter
    EventOrPaid = 1,
    /// Only paid coins can be used to read this chapter
    Paid = 2,
}

/// Subscription status of the user.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
pub enum SubscriptionKind {
    /// An error has occurred.
    Unrecognized = -1,
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
}
