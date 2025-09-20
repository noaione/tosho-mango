//! A collection of enums used throughout the library.
//!
//! If something is missing, please [open an issue](

use std::str::FromStr;

use tosho_macros::{DeserializeEnum, EnumName, SerializeEnum, enum_error};

/// The issue sale status in the marketplace.
///
/// ```rust
/// use tosho_nids::models::SaleStatus;
///
/// let st = SaleStatus::ForSale;
/// assert_eq!(st.to_string(), "for-sale");
/// assert_eq!(st.to_name(), "ForSale");
///
/// let parsed = "post-sale".parse::<SaleStatus>().unwrap();
/// assert_eq!(parsed, SaleStatus::PostSale);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, SerializeEnum, DeserializeEnum, EnumName)]
pub enum SaleStatus {
    /// This issue is available for sale and can be purchased.
    ForSale,
    /// This issue is no longer available for sale from the publisher.
    ///
    /// You could only get it from the marketplace.
    PostSale,
}

enum_error!(SaleStatusFromStrError);

impl FromStr for SaleStatus {
    type Err = SaleStatusFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "for-sale" => Ok(SaleStatus::ForSale),
            "post-sale" => Ok(SaleStatus::PostSale),
            _ => Err(SaleStatusFromStrError {
                original: s.to_string(),
            }),
        }
    }
}

impl std::fmt::Display for SaleStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SaleStatus::ForSale => write!(f, "for-sale"),
            SaleStatus::PostSale => write!(f, "post-sale"),
        }
    }
}
