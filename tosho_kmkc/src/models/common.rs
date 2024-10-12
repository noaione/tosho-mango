//! A module containing common models used in the library.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

use serde::{Deserialize, Serialize};
use tosho_common::FailableResponse;
use tosho_macros::AutoGetter;

use super::KMAPIError;

/// Simple ID object model.
#[derive(Debug, Clone, Copy, AutoGetter, Serialize, Deserialize)]
pub struct SimpleId {
    /// The ID itself.
    id: i32,
}

/// The base response for all API calls.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct StatusResponse {
    /// The status of the response, usually "success" or "fail".
    status: String,
    /// The response code of the response, usually 0 for success.
    response_code: i32,
    /// The error message of the response, usually empty for success.
    error_message: String,
}

impl FailableResponse for StatusResponse {
    /// Raise/return an error if the response code is not 0.
    fn raise_for_status(&self) -> tosho_common::ToshoResult<()> {
        if self.response_code != 0 {
            let api_error = KMAPIError {
                error_code: self.response_code,
                message: self.error_message.clone(),
            };
            Err(api_error.into())
        } else {
            Ok(())
        }
    }

    /// Format the error message.
    fn format_error(&self) -> String {
        KMAPIError {
            error_code: self.response_code,
            message: self.error_message.clone(),
        }
        .to_string()
    }
}
