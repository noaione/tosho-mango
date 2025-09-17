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
