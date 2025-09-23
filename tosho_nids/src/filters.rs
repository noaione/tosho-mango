//! A list of filters that could be used in various endpoints.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

use std::collections::HashMap;

use tosho_macros::EnumName;

/// The order to sort by.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumName)]
pub enum SortOrder {
    /// Ascending order
    ASC,
    /// Descending order
    DESC,
}

/// The field to sort by.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SortBy {
    /// Sort by ID
    Id,
    /// Sort by title
    Title,
    /// Sort by full title
    FullTitle,
    /// Sort by name
    Name,
    /// Sort by display name
    ///
    /// Alternative to `Name` (used sometimes in creators)
    DisplayName,
    /// Sort by issue number
    IssueNumber,
    /// Sort by book index (similar to issue number)
    BookIndex,
    /// Sort by release date
    ReleaseDate,
    /// Sort by publication date
    PublicationDate,
    /// Any other field
    Any(String),
}

impl SortBy {
    /// Get the string representation of the sort by field.
    pub fn as_str(&self) -> &str {
        match self {
            SortBy::Id => "id",
            SortBy::Title => "title",
            SortBy::Name => "name",
            SortBy::DisplayName => "display_name",
            SortBy::FullTitle => "full_title",
            SortBy::IssueNumber => "issue_number",
            SortBy::BookIndex => "book_index",
            SortBy::ReleaseDate => "release_date",
            SortBy::PublicationDate => "original_publication_date",
            SortBy::Any(field) => field.as_ref(),
        }
    }

    /// From string to [`SortBy`] enum
    pub fn from_string(s: impl AsRef<str>) -> Self {
        let s = s.as_ref();
        match s {
            "id" => SortBy::Id,
            "title" => SortBy::Title,
            "name" => SortBy::Name,
            "display_name" => SortBy::DisplayName,
            "full_title" => SortBy::FullTitle,
            "issue_number" => SortBy::IssueNumber,
            "book_index" => SortBy::BookIndex,
            "release_date" => SortBy::ReleaseDate,
            "publication_date" => SortBy::PublicationDate,
            "original_publication_date" => SortBy::PublicationDate,
            other => SortBy::Any(other.to_string()),
        }
    }
}

/// Some common filter types used in various endpoints.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilterType {
    /// Filter by ID
    Id,
    /// Filter by UUID
    Uuid,
    /// Filter by format
    ///
    /// Example: issue,ashcan
    Format,
    /// Filter by series run ID
    SeriesRunId,
    /// Filter by release date start (ISO 8601 format)
    ReleaseDateStart,
    /// Filter by release date end (ISO 8601 format)
    ReleaseDateEnd,
    /// Filter by genre ID
    GenreId,
    /// Filter by imprint ID
    ImprintId,
    /// Filter by publisher ID
    PublisherId,
    /// Filter by publisher slug
    PublisherSlug,
    /// Filter by any arbitrary string key-value pair
    Any(String),
}

impl FilterType {
    /// Get the string representation of the filter type.
    pub fn as_str(&self) -> &str {
        match self {
            FilterType::Id => "id",
            FilterType::Uuid => "uuid",
            FilterType::Format => "format",
            FilterType::SeriesRunId => "series_run_id",
            FilterType::ReleaseDateStart => "release_date_start",
            FilterType::ReleaseDateEnd => "release_date_end",
            FilterType::GenreId => "genre_id",
            FilterType::ImprintId => "publisher_imprint_id",
            FilterType::PublisherId => "publisher_id",
            FilterType::PublisherSlug => "publisher_slug",
            FilterType::Any(key) => key.as_ref(),
        }
    }

    /// From string to [`FilterType`] enum
    pub fn from_string(s: impl AsRef<str>) -> Self {
        let s = s.as_ref();
        match s {
            "id" => FilterType::Id,
            "uuid" => FilterType::Uuid,
            "format" => FilterType::Format,
            "series_run_id" => FilterType::SeriesRunId,
            "release_date_start" => FilterType::ReleaseDateStart,
            "release_date_end" => FilterType::ReleaseDateEnd,
            "genre_id" => FilterType::GenreId,
            "publisher_imprint_id" => FilterType::ImprintId,
            "publisher_id" => FilterType::PublisherId,
            "publisher_slug" => FilterType::PublisherSlug,
            other => FilterType::Any(other.to_string()),
        }
    }
}

/// The scope of the filter, used in issue endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterScope {
    /// New releases collection
    Frontlist,
    /// Backlog collection
    Backlist,
    /// On sale collection (both frontlist and backlist)
    OnSale,
    /// Best selling collection
    BestSelling,
    /// New releases collection (although unused right now)
    NewReleases,
}

impl FilterScope {
    /// Get the string representation of the filter scope.
    pub fn as_str(&self) -> &'static str {
        match self {
            FilterScope::Frontlist => "frontlist",
            FilterScope::Backlist => "backlist",
            FilterScope::OnSale => "on_sale",
            FilterScope::BestSelling => "best_selling",
            FilterScope::NewReleases => "new_releases",
        }
    }
}

/// Filter handler
#[derive(Clone)]
pub struct Filter {
    filters: Vec<(FilterType, String)>,
    direction: Option<SortOrder>,
    order_by: Option<SortBy>,
    page: Option<u32>,
    per_page: Option<u32>,
    scope: Option<FilterScope>,
}

impl Filter {
    /// Create a new filter
    pub fn new() -> Self {
        Self {
            filters: Vec::new(),
            direction: None,
            order_by: None,
            page: Some(1),      // Sane default
            per_page: Some(20), // Sane default
            scope: None,
        }
    }

    /// Add a filter
    pub fn add_filter(mut self, filter_type: FilterType, value: impl ToString) -> Self {
        self.filters.push((filter_type, value.to_string()));
        self
    }

    /// Add a filter in place
    pub fn add_filter_mut(&mut self, filter_type: FilterType, value: impl ToString) -> &mut Self {
        self.filters.push((filter_type, value.to_string()));
        self
    }

    /// Set the sort order
    pub fn with_order(mut self, order_by: SortBy, direction: SortOrder) -> Self {
        self.order_by = Some(order_by);
        self.direction = Some(direction);
        self
    }

    /// Set the sort order in place
    pub fn set_order(&mut self, order_by: SortBy, direction: SortOrder) -> &mut Self {
        self.order_by = Some(order_by);
        self.direction = Some(direction);
        self
    }

    /// Set the page number
    pub fn with_page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// Set the page number in place
    pub fn set_page(&mut self, page: u32) -> &mut Self {
        self.page = Some(page);
        self
    }

    /// Set the number of items per page
    pub fn with_per_page(mut self, per_page: u32) -> Self {
        self.per_page = Some(per_page);
        self
    }

    /// Set the number of items per page in place
    pub fn set_per_page(&mut self, per_page: u32) -> &mut Self {
        self.per_page = Some(per_page);
        self
    }

    /// Clear all filters
    pub fn clear_filters(mut self) -> Self {
        self.filters.clear();
        self.per_page = None;
        self.page = None;
        self.direction = None;
        self.order_by = None;
        self.scope = None;
        self
    }

    /// Clear all filters in place
    pub fn clear_filters_mut(&mut self) -> &mut Self {
        self.filters.clear();
        self.per_page = None;
        self.page = None;
        self.direction = None;
        self.order_by = None;
        self.scope = None;
        self
    }

    /// Set the scope of the filter (only for issues endpoint)
    pub fn with_scope(mut self, scope: FilterScope) -> Self {
        self.scope = Some(scope);
        self
    }

    /// Set the scope of the filter in place (only for issues endpoint)
    pub fn set_scope(&mut self, scope: FilterScope) -> &mut Self {
        self.scope = Some(scope);
        self
    }

    /// Convert the filter to a query string for use in requests
    pub(crate) fn to_params(&self) -> HashMap<String, String> {
        let mut query = HashMap::new();

        for (filter_type, value) in &self.filters {
            let filter_key = format!("filter[{}]", filter_type.as_str());
            query.insert(filter_key, value.clone());
        }

        if let Some(direction) = self.direction {
            query.insert("direction".to_string(), direction.to_name().to_string());
        }

        if let Some(order_by) = &self.order_by {
            query.insert("order_by".to_string(), order_by.as_str().to_string());
        }

        if let Some(page) = self.page {
            query.insert("page".to_string(), page.to_string());
        }

        if let Some(per_page) = self.per_page {
            query.insert("per_page".to_string(), per_page.to_string());
        }

        if let Some(scope) = self.scope {
            query.insert("scope".to_string(), scope.as_str().to_string());
        }

        query
    }
}

impl Default for Filter {
    fn default() -> Self {
        Self::new()
    }
}
