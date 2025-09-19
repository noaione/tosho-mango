//! A module containing information related to book/volume issues.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

use serde::{Deserialize, Serialize};
use tosho_macros::AutoGetter;

/// A simple information about the series run in an issue
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct SeriesRunIssue {
    /// Series run ID
    id: u32,
    /// Series run UUID
    uuid: String,
    /// Series run title
    title: String,
    /// Series run URL slug
    slug: String,
    /// Is the series book can be resold?
    #[serde(rename = "books_are_resellable")]
    is_resellable: bool,
    /// Is the series book can be remarqued?
    #[serde(rename = "books_are_remarquable")]
    is_remarquable: bool,
    /// Is the series book can be downloaded?
    #[serde(rename = "books_are_downloadable")]
    is_downloadable: bool,
    /// The start date of the series in simple YYYY-MM-DD format
    #[serde(
        serialize_with = "super::datetime::serialize_opt_yyyymmdd",
        deserialize_with = "super::datetime::deserialize_opt_yyyymmdd"
    )]
    start_date: Option<chrono::DateTime<chrono::FixedOffset>>,
    /// The end date of the series in simple YYYY-MM-DD format
    #[serde(
        serialize_with = "super::datetime::serialize_opt_yyyymmdd",
        deserialize_with = "super::datetime::deserialize_opt_yyyymmdd"
    )]
    end_date: Option<chrono::DateTime<chrono::FixedOffset>>,
}

/// A detailed information about the series run in an issue
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct SeriesRunDetailed {
    /// Series run ID
    id: u32,
    /// Series run UUID
    uuid: String,
    /// Series run title
    title: String,
    /// Series run URL slug
    slug: String,
    /// The total issues in this series run
    issues_count: u32,
    /// The series publisher
    publisher: super::common::Publisher,
    /// The series cover
    cover: Option<super::common::ImageUrl>,
    /// The start date of the series in simple YYYY-MM-DD format
    #[serde(
        serialize_with = "super::datetime::serialize_opt_yyyymmdd",
        deserialize_with = "super::datetime::deserialize_opt_yyyymmdd"
    )]
    start_date: Option<chrono::DateTime<chrono::FixedOffset>>,
    /// The end date of the series in simple YYYY-MM-DD format
    #[serde(
        serialize_with = "super::datetime::serialize_opt_yyyymmdd",
        deserialize_with = "super::datetime::deserialize_opt_yyyymmdd"
    )]
    end_date: Option<chrono::DateTime<chrono::FixedOffset>>,
}

/// A range of issues in a series run edition
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct SeriesRunEditionIssueRange {
    /// The starting issue number in the range
    min: String,
    /// The ending issue number in the range
    max: String,
}

/// A edition information in a series run
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct SeriesRunEdition {
    /// Edition/issue ID
    id: u32,
    /// Edition/issue UUID
    uuid: String,
    /// Edition/issue name
    title: String,
    /// Edition/issue volume number
    #[serde(rename = "volume_number")]
    volume: Option<String>,
    /// The price of the issue in USD
    price_usd: u64,
    /// Cover image URLs
    cover: super::common::ImageUrl,
    /// Issue range in this edition
    issue_range: SeriesRunEditionIssueRange,
}

/// Banner image for the series run
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct SeriesRunBanner {
    /// URL of the banner image
    #[serde(rename = "series_run_banner_url")]
    url: String,
}

/// Information about the series run including its editions
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct SeriesRunWithEditions {
    /// Series run ID
    id: u32,
    /// Series run UUID
    uuid: String,
    /// Series run title
    title: String,
    /// Series run URL slug
    slug: String,
    /// Banner image for the series run
    banner: Option<SeriesRunBanner>,
    /// The description of the series run
    description: Option<String>,
    /// The total issues in this series run
    issues_count: u32,
    /// The total editions in this series run
    #[serde(rename = "collected_editions_count")]
    editions_count: u32,
    /// The list of editions in this series run
    #[serde(rename = "collected_editions")]
    editions: Vec<SeriesRunEdition>,
    /// The publisher of this series run
    publisher: super::common::Publisher,
    /// The imprint of this series run
    #[serde(rename = "publisher_imprint")]
    imprint: Option<super::common::Imprint>,
    /// The start date of the series in simple YYYY-MM-DD format
    #[serde(
        serialize_with = "super::datetime::serialize_opt_yyyymmdd",
        deserialize_with = "super::datetime::deserialize_opt_yyyymmdd"
    )]
    start_date: Option<chrono::DateTime<chrono::FixedOffset>>,
    /// The end date of the series in simple YYYY-MM-DD format
    #[serde(
        serialize_with = "super::datetime::serialize_opt_yyyymmdd",
        deserialize_with = "super::datetime::deserialize_opt_yyyymmdd"
    )]
    end_date: Option<chrono::DateTime<chrono::FixedOffset>>,
}

/// Response for series run list
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct SeriesRunList {
    /// Total pages available using the current page size
    #[serde(rename = "pages_count")]
    pages: u32,
    /// Total series runs available
    #[serde(rename = "total_count")]
    count: u64,
    /// List of series runs
    #[serde(rename = "series_run")]
    data: Vec<SeriesRunDetailed>,
}

/// Response for series run with editions
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct SeriesRunWithEditionsResponse {
    /// The actual series run information
    #[serde(rename = "series_run")]
    #[deref_clone]
    data: SeriesRunWithEditions,
}
