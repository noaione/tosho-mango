//! A module containing information related to book/volume issues.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

use serde::{Deserialize, Serialize};
use tosho_macros::AutoGetter;

use crate::models::SeriesRunIssue;

/// A detailed information about an issues.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct IssueDetail {
    /// Issue ID
    id: u32,
    /// Issue UUID
    uuid: String,
    /// The series title
    title: String,
    /// The series + issue full title
    full_title: String,
    /// The issue URL slug
    slug: String,
    /// The issue description
    description: Option<String>,
    /// The list of creators involved in this issue
    creators: Vec<IssueCreator>,
    /// The list of genres/tags associated with this issue
    genres: Vec<super::common::Genre>,
    /// The issue variant identifier (e.g. "A" for Cover A, "Special Ed.", etc.)
    ///
    /// Can be an empty string if there's no variant.
    variant_identifier: String,
    /// The publisher information
    publisher: super::common::Publisher,
    /// The imprint information
    #[serde(rename = "publisher_imprint")]
    imprint: Option<super::common::Imprint>,
    /// The cover page image URLs
    cover: super::common::ImageUrl,
    /// The price of the issue in USD
    ///
    /// This is normalized following Stripe currency convention (i.e. 199 = $1.99)
    price_usd: u64,
    /// The release date of the issue in ISO 8601 format
    #[serde(with = "super::datetime")]
    release_date: chrono::DateTime<chrono::FixedOffset>,
    /// The original publication date of the issue in ISO 8601 format
    #[serde(with = "super::datetime")]
    original_publication_date: chrono::DateTime<chrono::FixedOffset>,
    /// The age rating of the issue
    age_rating: String,
    /// The total number of pages in the issue
    total_pages: u32,
    /// The language of the issue (e.g. "eng", "jpn", etc.)
    language: String,
    /// The series run information of this issue
    series_run: SeriesRunIssue,
    /// The list of available variants for this issue
    variants: Vec<IssueVariant>,
    /// The total variants of this issue
    variants_count: u32,
    /// Is this issue remarquable?
    is_remarquable: bool,
    /// Can this issue be resold?
    is_resellable: bool,
    /// Can this issue be downloaded?
    is_downloadable: bool,
    /// The status of the issue (e.g. "for-sale")
    ///
    /// Although right now, this is either "for-sale", or "post-sale" (marketplace only)
    status: String,
    /// Is this issue "active" or "available"
    active: bool,
    /// If this issue is resellable, when can this be resold again
    #[serde(
        serialize_with = "super::datetime::serialize_opt",
        deserialize_with = "super::datetime::deserialize_opt"
    )]
    marketplace_enabled_date: Option<chrono::DateTime<chrono::FixedOffset>>,
    /// The sale end date of the issue in ISO 8601 format
    #[serde(
        serialize_with = "super::datetime::serialize_opt",
        deserialize_with = "super::datetime::deserialize_opt"
    )]
    end_date: Option<chrono::DateTime<chrono::FixedOffset>>,
}

/// Information about creator on the issues
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct IssueCreator {
    /// Creator information
    creator: super::common::Creator,
    /// The role of the creator in this issue (e.g. writer, artist)
    #[serde(rename = "creator_role")]
    role: String,
}

/// A issue variant information
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct IssueVariant {
    /// Issue ID
    id: u32,
    /// Issue UUID
    uuid: String,
    /// The series title
    title: String,
    /// The series + issue full title
    full_title: String,
    /// The issue URL slug
    slug: String,
    /// The list of creators involved in this issue
    creators: Vec<IssueCreator>,
    /// The issue variant identifier (e.g. "A" for Cover A, "Special Ed.", etc.)
    ///
    /// Can be an empty string if there's no variant.
    variant_identifier: String,
    /// The cover page image URLs
    cover: super::common::ImageUrl,
    /// The price of the issue in USD
    ///
    /// This is normalized following Stripe currency convention (i.e. 199 = $1.99)
    price_usd: u64,
    /// The release date of the issue in ISO 8601 format
    #[serde(with = "super::datetime")]
    release_date: chrono::DateTime<chrono::FixedOffset>,
    /// The original publication date of the issue in ISO 8601 format
    #[serde(with = "super::datetime")]
    original_publication_date: chrono::DateTime<chrono::FixedOffset>,
    /// The status of the issue (e.g. "for-sale")
    ///
    /// Although right now, this is either "for-sale", or "post-sale" (marketplace only)
    status: String,
    /// Is this issue "active" or "available"
    active: bool,
    /// The sale end date of the issue in ISO 8601 format
    #[serde(
        serialize_with = "super::datetime::serialize_opt",
        deserialize_with = "super::datetime::deserialize_opt"
    )]
    end_date: Option<chrono::DateTime<chrono::FixedOffset>>,
}

/// A simple issue information, usually used in lists.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct IssueSummary {
    /// Issue ID
    id: u32,
    /// Issue UUID
    uuid: String,
    /// The series title
    title: String,
    /// The series + issue full title
    full_title: String,
    /// The issue URL slug
    slug: String,
    /// The issue publisher
    publisher: super::common::Publisher,
    /// The issue imprint
    #[serde(rename = "publisher_imprint")]
    imprint: Option<super::common::Imprint>,
    /// The issue cover image URLs
    cover: super::common::ImageUrl,
    /// The price of the issue in USD
    ///
    /// This is normalized following Stripe currency convention (i.e. 199 = $1.99)
    price_usd: u64,
    /// The release date of the issue in ISO 8601 format
    #[serde(with = "super::datetime")]
    release_date: chrono::DateTime<chrono::FixedOffset>,
    /// The original publication date of the issue in ISO 8601 format
    #[serde(with = "super::datetime")]
    original_publication_date: chrono::DateTime<chrono::FixedOffset>,
    /// The total variants of this issue
    variants_count: u32,
}
