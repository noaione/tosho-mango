//! A module containing all the errors used in the library.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

use tosho_common::{ToshoError, make_error};

/// The used error type for the API.
#[derive(Debug)]
pub struct KMAPIError {
    /// The error code from the API.
    pub error_code: i32,
    /// The error message from the API.
    pub message: String,
}

impl std::fmt::Display for KMAPIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "An error occurred with status {}: {}",
            self.error_code, self.message
        )
    }
}

impl std::error::Error for KMAPIError {}

/// An error when you don't have enough point to buy a titles chapters.
#[derive(Debug)]
pub struct KMAPINotEnoughPointsError {
    /// The error message
    pub message: String,
    /// The amount of points you need to buy the chapters.
    pub points_needed: u64,
    /// The amount of points you have.
    pub points_have: u64,
}

impl std::fmt::Display for KMAPINotEnoughPointsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({} points needed, {} points have)",
            self.message, self.points_needed, self.points_have
        )
    }
}

impl std::error::Error for KMAPINotEnoughPointsError {}

impl From<KMAPIError> for ToshoError {
    fn from(value: KMAPIError) -> Self {
        make_error!("{}", value)
    }
}

impl From<KMAPINotEnoughPointsError> for ToshoError {
    fn from(value: KMAPINotEnoughPointsError) -> Self {
        make_error!("{}", value)
    }
}
