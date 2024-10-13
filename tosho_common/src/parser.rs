//! Helper functions to parse responses from the API.

use crate::ToshoResult;

/// Trait for all responses that can be parsed.
pub trait FailableResponse {
    /// Raise an error if the response is not successful.
    fn raise_for_status(&self) -> ToshoResult<()>;
    /// Format the error message.
    fn format_error(&self) -> String;
}

/// Parse a JSON response.
#[cfg(feature = "serde")]
pub async fn parse_json_response<T>(response: reqwest::Response) -> ToshoResult<T>
where
    T: serde::de::DeserializeOwned,
{
    let stat_code = response.status();
    let headers = response.headers().clone();
    let url = response.url().clone();
    let raw_text = response.text().await?;
    let json: T = ::serde_json::from_str(&raw_text)
        .map_err(|e| crate::ToshoDetailedParseError::new(e, stat_code, &headers, &url, raw_text))?;

    Ok(json)
}

/// Parse a JSON response with two possible response types.
///
/// This function is useful when the API returns some kind of error response
#[cfg(feature = "serde")]
pub async fn parse_json_response_failable<T, E>(response: reqwest::Response) -> ToshoResult<T>
where
    T: serde::de::DeserializeOwned,
    E: serde::de::DeserializeOwned + std::fmt::Debug + FailableResponse,
{
    let stat_code = response.status();
    let headers = response.headers().clone();
    let url = response.url().clone();
    let raw_text = response.text().await?;
    let status_resp: E = ::serde_json::from_str(&raw_text).map_err(|e| {
        crate::ToshoDetailedParseError::new(e, stat_code, &headers, &url, &raw_text)
    })?;

    match status_resp.raise_for_status() {
        Ok(_) => {
            let json: T = ::serde_json::from_str(&raw_text).map_err(|e| {
                crate::ToshoDetailedParseError::new(e, stat_code, &headers, &url, raw_text)
            })?;
            Ok(json)
        }
        // If the status response is an error, return the error
        Err(e) => {
            let error_message = status_resp.format_error();
            let fail_error = crate::ToshoDetailedFailableError::new(error_message, e);
            Err(crate::ToshoError::ParseError(
                crate::ToshoParseError::SerdeFailableError(fail_error),
            ))
        }
    }
}

/// Parse a Protobuf response.
#[cfg(feature = "protobuf")]
pub async fn parse_protobuf_response<T>(response: reqwest::Response) -> ToshoResult<T>
where
    T: ::prost::Message + Clone + Default,
{
    if response.status().is_success() {
        let bytes_data = response.bytes().await?;
        let decoded = T::decode(&bytes_data[..])?;

        Ok(decoded)
    } else {
        let status_code = response.status();

        Err(crate::ToshoError::ParseError(
            crate::ToshoParseError::InvalidStatusCode(status_code),
        ))
    }
}
