//! A module containing all the models used in the library.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

#![warn(clippy::missing_docs_in_private_items)]

pub mod common;
pub(crate) mod datetime;
pub mod issues;
pub mod others;
pub mod reader;
pub mod series;

pub use common::*;
pub use issues::*;
pub use others::*;
pub use reader::*;
pub use series::*;

use serde::Deserialize;
use tosho_common::FailableResponse;
use tosho_macros::AutoGetter;

/// A simple error response from the API.
#[derive(Debug, Clone, AutoGetter, Deserialize)]
pub struct ErrorResponse {
    /// The error message.
    error: Option<String>,
}

impl FailableResponse for ErrorResponse {
    fn raise_for_status(&self) -> tosho_common::ToshoResult<()> {
        if let Some(error) = &self.error {
            return Err(tosho_common::make_error!("{}", error));
        }
        Ok(())
    }

    fn format_error(&self) -> String {
        self.error
            .clone()
            .unwrap_or_else(|| "Unknown error".to_string())
    }
}

/// A simple acknowledgement response from the API.
#[derive(Debug, Clone, AutoGetter, Deserialize)]
pub struct AckResponse {
    /// The acknowledgement message.
    message: String,
}
