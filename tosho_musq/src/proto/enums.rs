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
    /// An error has occurred.
    Unrecognized = -1,
}

/// Subscription status of the user.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
pub enum SubscriptionStatus {
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
