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
    /// The frames in the page for guided reading
    frames: Vec<ReaderFrame>,
}

/// Reader frames information
///
/// For guided reading mode
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ReaderFrame {
    /// The frame UUID
    uuid: String,
    /// The frame index number (0-based)
    index: u64,
    /// The starting X position (0.0 - 1.0)
    x: f64,
    /// The starting Y position (0.0 - 1.0)
    y: f64,
    /// The width of the box (0.0 - 1.0)
    width: f64,
    /// The height of the box (0.0 - 1.0)
    height: f64,
    /// The opacity of the outer box (0.0 - 1.0)
    opacity: Option<f64>,
    // TODO: `transition`, `color` whenever they figured out what they will do with this
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
    /// Cover image of the issue
    #[serde(rename = "cover_image")]
    cover: super::common::ImageReaderUrl,
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

/// Frameflow stream `error` response
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct StreamFrameflowError {
    /// The error message
    message: String,
}

/// Frameflow stream `done` response
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct StreamFrameflowDone {
    /// Total pages count
    pages_count: u64,
}

/// Frameflow stream `page` response
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct StreamFrameflowPage {
    /// The page index
    index: u64,
    /// The page information
    page: ReaderPage,
}

/// Payload for the frameflow stream issue
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct FrameflowStreamIssuePayload {
    /// The issue ID
    id: u32,
    /// The issue UUID
    uuid: String,
    /// Reading direction
    ///
    /// TODO: Change to enum
    reading_direction: String,
    /// Cover image of the issue
    #[serde(rename = "cover_image")]
    cover: super::common::ImageReaderUrl,
    /// The full title of the issue
    #[serde(rename = "full_title")]
    title: String,
    /// The reader version
    version: u32,
    /// The total pages
    total_pages: u64,
}

/// Frameflow stream issue information
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct FrameflowStreamIssue {
    /// The issue ID
    id: u32,
    /// The issue UUID
    uuid: String,
    /// The issue slug
    slug: Option<String>,
    /// The total pages
    total_pages: Option<u64>,
    /// The issue group UUID
    issue_group_uuid: Option<String>,
    /// The frameflow payload
    frameflow: FrameflowStreamIssuePayload,
}

/// Frameflow stream `header` response
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct StreamFrameflowHeader {
    /// The issue information
    issue: FrameflowStreamIssue,
}

/// Frameflow stream data event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum StreamFrameflowEvent {
    /// Frameflow stream header
    #[serde(rename = "header")]
    Header(StreamFrameflowHeader),
    /// Frameflow stream page
    #[serde(rename = "page")]
    Page(StreamFrameflowPage),
    /// Frameflow stream done
    #[serde(rename = "done")]
    Done(StreamFrameflowDone),
    /// Frameflow stream error
    #[serde(rename = "error")]
    Error(StreamFrameflowError),
}

/// Collected result from the frameflow NDJSON stream endpoint.
///
/// Contains the header metadata received in the `header` event,
/// and all page data collected from `page` events, in arrival order.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct StreamedReaderPages {
    /// The stream header, containing issue and frameflow metadata
    header: StreamFrameflowHeader,
    /// All pages collected from the stream, in arrival order
    pages: Vec<ReaderPage>,
}

impl StreamedReaderPages {
    /// Create a new instance
    pub(crate) fn new(header: StreamFrameflowHeader, pages: Vec<ReaderPage>) -> Self {
        Self { header, pages }
    }
}
