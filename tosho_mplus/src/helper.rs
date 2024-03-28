//! Provides a collection of helper Structs that can be used.
//!
//! ```rust
//! use tosho_mplus::ImageQuality;
//!
//! let hq_img = ImageQuality::High;
//! ```

use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// The image quality to be downloaded.
#[derive(
    Debug, Clone, Copy, PartialEq, tosho_macros::SerializeEnum, tosho_macros::DeserializeEnum,
)]
pub enum ImageQuality {
    /// Low quality images
    Low,
    /// Normal quality images
    Normal,
    /// High quality images
    High,
}

tosho_macros::enum_error!(ImageQualityFromStrError);

impl FromStr for ImageQuality {
    type Err = ImageQualityFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "low" => Ok(ImageQuality::Low),
            "high" | "normal" | "middle" | "standard" => Ok(ImageQuality::Normal),
            "super_high" | "high_quality" => Ok(ImageQuality::High),
            _ => Err(ImageQualityFromStrError {
                original: s.to_string(),
            }),
        }
    }
}

impl ToString for ImageQuality {
    fn to_string(&self) -> String {
        match self {
            ImageQuality::Low => "low".to_string(),
            ImageQuality::Normal => "high".to_string(),
            ImageQuality::High => "super_high".to_string(),
        }
    }
}
