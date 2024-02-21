#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SortOption {
    Alphabetical,
    Recent,
    Popular,
}

impl ToString for SortOption {
    fn to_string(&self) -> String {
        match self {
            SortOption::Alphabetical => "alphabetical".to_string(),
            SortOption::Recent => "recent_series".to_string(),
            SortOption::Popular => "popular".to_string(),
        }
    }
}
