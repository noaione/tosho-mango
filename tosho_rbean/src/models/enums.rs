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

impl ToString for SortOption {
    /// Convert the enum to a string used by the API.
    fn to_string(&self) -> String {
        match self {
            SortOption::Alphabetical => "alphabetical".to_string(),
            SortOption::Recent => "recent_series".to_string(),
            SortOption::Popular => "popular".to_string(),
        }
    }
}
