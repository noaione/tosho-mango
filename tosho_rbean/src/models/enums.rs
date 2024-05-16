//! A module containing information related to enums used in the library.

/// Sorting options for searching.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SortOption {
    /// Sort by alphabetical order.
    Alphabetical,
    /// Sort by recent series.
    Recent,
    /// Sort by popular series.
    Popular,
}

impl std::fmt::Display for SortOption {
    /// Convert the enum to a string used by the API.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortOption::Alphabetical => write!(f, "alphabetical"),
            SortOption::Recent => write!(f, "recent_series"),
            SortOption::Popular => write!(f, "popular"),
        }
    }
}
