//! A module containing common models used in the library.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

use serde::{Deserialize, Serialize};

use super::AMAPIError;

/// The header of each request result.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResultHeader {
    /// The result of the request.
    pub result: bool,
    /// Error message.
    pub message: Option<String>,
}

/// The body which contains error message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorBody {
    #[serde(rename = "error_code")]
    pub code: i32,
    #[serde(rename = "error_message_list")]
    pub messages: Vec<String>,
}

/// Wrapper for [`ResultHeader`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusResult {
    /// The result of the request.
    pub header: ResultHeader,
    #[serde(default)]
    pub body: Option<serde_json::Value>,
}

impl StatusResult {
    /// Try to unwrap the body into [`ErrorBody`] and return the error message.
    fn unwrap_body_error(&self) -> String {
        // try to unwrap the body into ErrorBody
        if let Some(body) = &self.body {
            if let Ok(error_body) = serde_json::from_value::<ErrorBody>(body.clone()) {
                return error_body.messages.join(", ");
            }
        }
        "Unknown error occured".to_string()
    }

    /// Raise/return an error if the response code is not 0.
    ///
    /// # Examples
    /// ```
    /// use tosho_amap::models::{ResultHeader, StatusResult};
    ///
    /// let response = StatusResult {
    ///     header: ResultHeader {
    ///         result: true,
    ///         message: None,
    ///     },
    ///     body: None,
    /// };
    ///
    /// assert!(response.raise_for_status().is_ok());
    ///
    /// let response = StatusResult {
    ///     header: ResultHeader {
    ///         result: false,
    ///         message: Some("An error occurred".to_string()),
    ///     },
    ///     body: None,
    /// };
    ///
    /// assert!(response.raise_for_status().is_err());
    /// ```
    pub fn raise_for_status(&self) -> Result<(), AMAPIError> {
        if !self.header.result {
            let message = self
                .header
                .message
                .clone()
                .unwrap_or_else(|| self.unwrap_body_error());

            Err(AMAPIError { message })
        } else {
            Ok(())
        }
    }
}

/// The result of the request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AMResult<R> {
    /// The result of the request.
    pub header: ResultHeader,
    /// The content of the request.
    #[serde(bound(
        deserialize = "R: Deserialize<'de>, R: Clone",
        serialize = "R: Serialize, R: Clone"
    ))]
    pub body: Option<R>,
}

/// The result of the request.
///
/// Wrapper for [`AMResult`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIResult<R> {
    /// The content of the request.
    #[serde(bound(
        deserialize = "R: Deserialize<'de>, R: Clone",
        serialize = "R: Serialize, R: Clone"
    ))]
    pub result: AMResult<R>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Serialize, Deserialize)]
    struct TestObject {
        pub value: String,
        pub magic: u32,
    }

    #[test]
    fn test_common_result_deserialize_optional() {
        let data: APIResult<ResultHeader> = serde_json::from_str(
            r#"{
                "result": {
                    "header": {
                        "result": true,
                        "message": null
                    },
                    "body": null
                }
            }"#,
        )
        .unwrap();

        assert_eq!(data.result.header.result, true);
        assert_eq!(data.result.header.message, None);
        assert_eq!(data.result.body, None);
    }

    #[test]
    fn test_common_reader_fail_raise() {
        let data: StatusResult = serde_json::from_str(
            r#"{
                "header": {
                    "result": false,
                    "message": null
                },
                "body": {
                    "error_code": 1,
                    "error_message_list": ["Unable to authenticate"]
                }
            }"#,
        )
        .unwrap();

        let raise_error = data.raise_for_status();

        assert!(raise_error.is_err());
        assert_eq!(raise_error.unwrap_err().message, "Unable to authenticate");
    }

    #[test]
    fn test_common_result_deserialize() {
        let data: APIResult<TestObject> = serde_json::from_str(
            r#"{
                "result": {
                    "header": {
                        "result": true
                    },
                    "body": {
                        "magic": 123,
                        "value": "magic value"
                    }
                }
            }"#,
        )
        .unwrap();

        assert_eq!(data.result.header.result, true);
        assert_eq!(data.result.header.message, None);

        let content_unwrap = data.result.body.unwrap();
        assert_eq!(content_unwrap.magic, 123);
        assert_eq!(content_unwrap.value, "magic value");
    }
}
