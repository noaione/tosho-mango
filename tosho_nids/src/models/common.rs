//! A common module used across multiple models.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

use serde::{Deserialize, Serialize};
use tosho_macros::AutoGetter;

/// A collection of image URLs in different sizes.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ImageUrl {
    /// Original sized URL
    original_url: String,
    /// Mobile sized URL
    #[serde(default)]
    #[skip_field]
    mobile_url: Option<String>,
    /// Thumbnail sized URL
    #[serde(default)]
    #[skip_field]
    thumbnail_url: Option<String>,
    /// Medium sized URL
    #[serde(default)]
    #[skip_field]
    medium_url: Option<String>,
}

impl ImageUrl {
    /// Get the thumbnail URL, would fallback to either mobile or medium
    pub fn thumbnail_url(&self) -> Option<&str> {
        if let Some(url) = &self.thumbnail_url {
            Some(url)
        } else if let Some(url) = &self.mobile_url {
            Some(url)
        } else if let Some(url) = &self.medium_url {
            Some(url)
        } else {
            None
        }
    }

    /// Get the mobile URL, would fallback to medium
    pub fn mobile_url(&self) -> Option<&str> {
        if let Some(url) = &self.mobile_url {
            Some(url)
        } else if let Some(url) = &self.medium_url {
            Some(url)
        } else {
            None
        }
    }

    /// Get the medium URL, would fallback to mobile
    pub fn medium_url(&self) -> Option<&str> {
        if let Some(url) = &self.medium_url {
            Some(url)
        } else if let Some(url) = &self.mobile_url {
            Some(url)
        } else {
            None
        }
    }
}

/// A collection of URL for the reader.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ImageReaderUrl {
    /// The original image URL
    url: String,
    /// Thumbnail sized image URL
    thumbnail_url: String,
    /// Mobile sized image URL
    mobile_url: String,
}

/// Creator information of the book/volume.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct Creator {
    /// Creator ID
    id: u32,
    /// Create display name
    #[serde(rename = "display_name")]
    name: String,
    /// Creator URL slug
    slug: String,
    /// Creator UUID
    uuid: String,
    // TODO: image which is null right now
    /// The user available roles in all the issues exist in the site
    #[serde(default)]
    roles: Option<Vec<String>>,
}

/// Genre/tag information of the book/volume.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct Genre {
    /// Genre ID
    id: u32,
    /// Genre UUID
    uuid: String,
    /// Genre name
    name: String,
    /// The "full-er" name of the genre
    ///
    /// Although right now it's just adding `COMICS & GRAPHICS NOVELS` prefix
    #[serde(default, rename = "bisac_name")]
    full_name: Option<String>,
}

/// A collection of publisher icon URLs in different sizes.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct PublisherIcon {
    /// Original sized URL
    original_url: String,
    /// Mobile sized URL
    mobile_url: String,
    /// Thumbnail sized URL
    thumbnail_url: String,
}

/// The publisher information
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct Publisher {
    /// Publisher ID
    id: u32,
    /// Publisher UUID
    uuid: String,
    /// Publisher name
    name: String,
    /// Publisher URL slug
    slug: String,
    /// Publisher image icon
    #[serde(default)]
    image: Option<PublisherIcon>,
    /// The total issues count for this publisher
    #[serde(default)]
    issues_count: Option<u64>,
    /// The total collectibles count for this publisher
    #[serde(default)]
    collected_editions_count: Option<u64>,
    /// The total series count for this publisher
    #[serde(default)]
    series_count: Option<u64>,
}

/// The imprint information
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct Imprint {
    /// Imprint ID
    id: u32,
    /// Imprint UUID
    uuid: String,
    /// Imprint name
    name: String,
    /// Imprint URL slug
    slug: String,
    /// The type of imprint (e.g. "primary", "secondary", etc.)
    #[serde(default)]
    imprint_type: Option<String>,
    /// Imprint image icon
    image: Option<ImageUrl>,
}

/// The refreshed auth token
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct RefreshedToken {
    /// The new access token
    access_token: String,
    /// The new refresh token
    refresh_token: String,
    /// When the access token expires (in seconds)
    expires_in: u64,
    /// When the access token expires at
    expires_at: i64,
}
