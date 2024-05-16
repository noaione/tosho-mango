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

impl std::fmt::Display for ImageQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageQuality::Low => write!(f, "low"),
            ImageQuality::Normal => write!(f, "high"),
            ImageQuality::High => write!(f, "super_high"),
        }
    }
}

/// The subscriptions plan tier.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    tosho_macros::SerializeEnum,
    tosho_macros::DeserializeEnum,
    tosho_macros::EnumName,
)]
pub enum SubscriptionPlan {
    /// Basic or user has no subscription
    Basic,
    /// The standard tier, which includes all the currently releasing chapters
    Standard,
    /// Deluxe tier, which is standard tier with extra perks that allows reading finished series
    Deluxe,
}

tosho_macros::enum_error!(SubscriptionPlanFromStrError);

impl FromStr for SubscriptionPlan {
    type Err = SubscriptionPlanFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "basic" => Ok(SubscriptionPlan::Basic),
            "standard" => Ok(SubscriptionPlan::Standard),
            "deluxe" => Ok(SubscriptionPlan::Deluxe),
            _ => Err(SubscriptionPlanFromStrError {
                original: s.to_string(),
            }),
        }
    }
}

impl std::fmt::Display for SubscriptionPlan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubscriptionPlan::Basic => write!(f, "basic"),
            SubscriptionPlan::Standard => write!(f, "standard"),
            SubscriptionPlan::Deluxe => write!(f, "deluxe"),
        }
    }
}

/// The title ranking type.
#[derive(
    Debug, Clone, Copy, PartialEq, tosho_macros::SerializeEnum, tosho_macros::DeserializeEnum,
)]
pub enum RankingType {
    /// The current hottest title ranking
    Hottest,
    /// The currently trending title ranking
    Trending,
    /// Completed title ranking
    Completed,
}

tosho_macros::enum_error!(RankingTypeFromStrError);

impl FromStr for RankingType {
    type Err = RankingTypeFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "hottest" | "hot" => Ok(RankingType::Hottest),
            "trending" => Ok(RankingType::Trending),
            "completed" | "complete" => Ok(RankingType::Completed),
            _ => Err(RankingTypeFromStrError {
                original: s.to_string(),
            }),
        }
    }
}

impl std::fmt::Display for RankingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RankingType::Hottest => write!(f, "hottest"),
            RankingType::Trending => write!(f, "trending"),
            RankingType::Completed => write!(f, "completed"),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_plan_type_ord() {
        use super::SubscriptionPlan;

        assert!(SubscriptionPlan::Basic < SubscriptionPlan::Standard);
        assert!(SubscriptionPlan::Standard < SubscriptionPlan::Deluxe);
        assert!(SubscriptionPlan::Basic < SubscriptionPlan::Deluxe);
        assert!(SubscriptionPlan::Deluxe >= SubscriptionPlan::Standard);
        assert!(SubscriptionPlan::Standard >= SubscriptionPlan::Standard);
    }
}
