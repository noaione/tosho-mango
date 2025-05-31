//! A module containing all the errors used in the library.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

use tosho_common::{ToshoError, make_error};

/// The used error type for the API.
#[derive(Debug, Clone)]
pub struct AMAPIError {
    /// The error message from the API.
    pub message: String,
}

impl std::fmt::Display for AMAPIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "An error occurred: {}", self.message)
    }
}

impl std::error::Error for AMAPIError {}

impl From<AMAPIError> for ToshoError {
    fn from(e: AMAPIError) -> Self {
        make_error!("{}", e)
    }
}
