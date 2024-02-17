//! A module containing common models used in the library.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

use serde::{Deserialize, Serialize};

use super::KMAPIError;

/// Simple ID object model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleId {
    /// The ID itself.
    pub id: i32,
}

/// The base response for all API calls.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusResponse {
    /// The status of the response, usually "success" or "fail".
    pub status: String,
    /// The response code of the response, usually 0 for success.
    pub response_code: i32,
    /// The error message of the response, usually empty for success.
    pub error_message: String,
}

impl StatusResponse {
    /// Raise/return an error if the response code is not 0.
    ///
    /// # Examples
    /// ```
    /// use tosho_kmkc::models::StatusResponse;
    ///
    /// let response = StatusResponse {
    ///     status: "success".to_string(),
    ///     response_code: 0,
    ///     error_message: "".to_string(),
    /// };
    ///
    /// assert!(response.raise_for_status().is_ok());
    ///
    /// let response = StatusResponse {
    ///    status: "fail".to_string(),
    ///    response_code: 1,
    ///    error_message: "An error occurred".to_string(),
    /// };
    ///
    /// assert!(response.raise_for_status().is_err());
    /// ```
    pub fn raise_for_status(&self) -> Result<(), KMAPIError> {
        if self.response_code != 0 {
            Err(KMAPIError {
                error_code: self.response_code,
                message: self.error_message.clone(),
            })
        } else {
            Ok(())
        }
    }
}
