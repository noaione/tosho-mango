//! A module containing information related to reader.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

use serde::{Deserialize, Serialize};
use tosho_macros::AutoGetter;

/// A single page information
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ReaderPage {
    /// The page number (1-based)
    #[serde(rename = "index")]
    page_number: u64,
    /// The page UUID
    uuid: String,
    /// The position of the page in the reader (left, right, center)
    position: String,
    /// The image URLs for the page
    image: super::common::ImageReaderUrl,
    /// Aspect ratio of the page (width / height)
    aspect_ratio: f64,
    // TODO: figure out `frames` array field
}

/// Response containing the pages information
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ReaderPages {
    /// The issue ID
    id: u32,
    /// The issue UUID
    uuid: String,
    /// Reading direction
    ///
    /// TODO: Change to enum
    reading_direction: String,
    /// The full title of the issue
    #[serde(rename = "full_title")]
    title: String,
    /// The list of pages
    pages: Vec<ReaderPage>,
    /// The reader version
    version: u32,
}

/// Wrapper for the reader pages response
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ReaderPagesWithMeta {
    /// The issue ID
    id: u32,
    /// The issue UUID
    uuid: String,
    /// The issue slug
    slug: String,
    /// Total pages in the issue
    total_pages: u64,
    /// The pages information
    #[serde(rename = "frameflow")]
    pages: ReaderPages,
}

/// Full response from the reader pages endpoint
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub(crate) struct ReaderPagesResponse {
    /// The actual issue information
    #[serde(rename = "issue")]
    #[deref_clone]
    data: ReaderPagesWithMeta,
}
