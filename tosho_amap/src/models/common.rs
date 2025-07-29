//! A module containing common models used in the library.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

use serde::{Deserialize, Serialize};
use tosho_common::FailableResponse;
use tosho_macros::AutoGetter;

use super::AMAPIError;

/// The header of each request result.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize, PartialEq)]
pub struct ResultHeader {
    /// The result of the request.
    result: bool,
    /// Error message.
    message: Option<String>,
}

/// The body which contains error message.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ErrorBody {
    /// The error response code
    #[serde(rename = "error_code")]
    code: i32,
    /// The list of error messages
    #[serde(rename = "error_message_list")]
    messages: Vec<String>,
}

/// Wrapper for [`ResultHeader`]
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct StatusResult {
    /// The result of the request.
    header: ResultHeader,
    /// The content of the response, this is just a stub
    #[serde(default)]
    body: Option<serde_json::Value>,
}

impl FailableResponse for StatusResult {
    /// Try to unwrap the body into [`ErrorBody`] and return the error message.
    fn format_error(&self) -> String {
        // try to unwrap the body into ErrorBody
        if let Some(body) = &self.body
            && let Ok(error_body) = serde_json::from_value::<ErrorBody>(body.clone())
        {
            error_body.messages.join(", ")
        } else {
            "Unknown error occured".to_string()
        }
    }

    /// Raise/return an error if the response code is not 0.
    fn raise_for_status(&self) -> Result<(), tosho_common::ToshoError> {
        if !self.header.result {
            let message = self
                .header
                .message
                .clone()
                .unwrap_or_else(|| self.format_error());

            Err(AMAPIError { message }.into())
        } else {
            Ok(())
        }
    }
}

/// The result of the request.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct AMResult<R> {
    /// The result of the request.
    header: ResultHeader,
    /// The content of the request.
    #[serde(bound(
        deserialize = "R: Deserialize<'de>, R: Clone",
        serialize = "R: Serialize, R: Clone"
    ))]
    body: Option<R>,
}

/// The result of the request.
///
/// Wrapper for [`AMResult`]
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct APIResult<R> {
    /// The content of the request.
    #[serde(bound(
        deserialize = "R: Deserialize<'de>, R: Clone",
        serialize = "R: Serialize, R: Clone"
    ))]
    result: AMResult<R>,
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

        assert!(data.result.header.result);
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
        assert_eq!(
            raise_error.unwrap_err().to_string(),
            "An error occurred: Unable to authenticate"
        );
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

        assert!(data.result.header.result);
        assert_eq!(data.result.header.message, None);

        let content_unwrap = data.result.body.unwrap();
        assert_eq!(content_unwrap.magic, 123);
        assert_eq!(content_unwrap.value, "magic value");
    }
}
