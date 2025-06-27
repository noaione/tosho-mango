//! Error types for the library
//!
//! This module contains all the error types used in the library.

#[cfg(feature = "serde")]
use std::collections::HashMap;

/// The result type used in the library
pub type ToshoResult<T> = Result<T, ToshoError>;

/// The main error type used in the library
///
/// This is what being used in [`ToshoResult`] as the error type.
#[derive(Debug)]
pub enum ToshoError {
    /// Error type that happens when making a request
    RequestError(reqwest::Error),
    /// Error type that happens when authenticating
    AuthError(ToshoAuthError),
    /// Error type that happens when parsing the response from the API
    ParseError(ToshoParseError),
    /// Error type that happens when processing images
    ImageError(ToshoImageError),
    /// Error type that happens when doing any kind of IO operation (e.g. writing a file)
    IOError(std::io::Error),
    /// Error type that happens when creating new Client instance
    ClientError(ToshoClientError),
    /// Other errors that doesn't fit the other categories
    CommonError(String),
}

impl ToshoError {
    /// Create a new instance of the error
    ///
    /// This will wrap the message into a [`ToshoError::CommonError`].
    /// For other error types, use [`From`] implementations.
    pub fn new(message: impl Into<String>) -> Self {
        ToshoError::CommonError(message.into())
    }
}

/// Error type that happens when parsing the response from the API
///
/// This is specifically for [`serde`] errors.
///
/// When formatted as a string, it will show the error message, status code, headers, and a JSON excerpt.
#[cfg(feature = "serde")]
pub struct ToshoDetailedParseError {
    inner: serde_json::Error,
    status_code: reqwest::StatusCode,
    headers: HashMap<String, String>,
    url: String,
    raw_text: String,
}

#[cfg(feature = "serde")]
impl ToshoDetailedParseError {
    /// Create a new instance of the error
    pub(crate) fn new(
        inner: serde_json::Error,
        status_code: reqwest::StatusCode,
        headers: &reqwest::header::HeaderMap,
        url: &reqwest::Url,
        raw_text: impl Into<String>,
    ) -> Self {
        let mapped_headers = headers
            .iter()
            .map(|(k, v)| (k.as_str().to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();
        Self {
            inner,
            status_code,
            headers: mapped_headers,
            url: url.to_string(),
            raw_text: raw_text.into(),
        }
    }

    /// Get the JSON excerpt from the raw text
    ///
    /// This will return a string that contains where the deserialization error happened.
    ///
    /// It will take 25 characters before and after the error position.
    pub fn get_json_excerpt(&self) -> String {
        let row_line = self.inner.line() - 1;
        let split_lines = self.raw_text.split('\n').collect::<Vec<&str>>();

        let position = self.inner.column();
        let start_idx = position.saturating_sub(25);
        let end_idx = position.saturating_add(25);

        // Bound the start and end index
        let start_idx = start_idx.max(0);
        let end_idx = end_idx.min(split_lines[row_line].len());

        split_lines[row_line][start_idx..end_idx].to_string()
    }
}

#[cfg(feature = "serde")]
impl std::fmt::Display for ToshoDetailedParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "with request from {} with status code {}: {}\n\nHeaders: {:#?}\nExcerpt: {}",
            self.url,
            self.status_code,
            self.inner,
            self.headers,
            self.get_json_excerpt()
        )
    }
}

#[cfg(feature = "serde")]
impl std::fmt::Debug for ToshoDetailedParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // make struct, do not include raw_text
        f.debug_struct("ToshoDetailedParseError")
            .field("inner", &self.inner)
            .field("status_code", &self.status_code)
            .field("headers", &self.headers)
            .field("url", &self.url)
            .finish()
    }
}

/// Error type that happens when parsing the response from the API
///
/// This is specifically for [`serde`] errors that are failable.
/// and usually are called in [`crate::parse_json_response_failable`].
///
/// `inner` are usually a wrap for [`ToshoParseError::SerdeDetailedError`]
#[derive(Debug)]
pub struct ToshoDetailedFailableError {
    message: String,
    inner: Box<ToshoError>,
}

impl ToshoDetailedFailableError {
    #[cfg(feature = "serde")]
    pub(crate) fn new(message: impl Into<String>, inner: ToshoError) -> Self {
        Self {
            message: message.into(),
            inner: Box::new(inner),
        }
    }
}

impl std::fmt::Display for ToshoDetailedFailableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.message, self.inner)
    }
}

/// Error type that happens when parsing the response from the API
#[derive(Debug)]
pub enum ToshoParseError {
    /// Failed to parse the response as JSON
    #[cfg(feature = "serde")]
    SerdeDetailedError(ToshoDetailedParseError),
    /// A passthrough error of [`serde_json::Error`]
    #[cfg(feature = "serde")]
    SerdeMinimalError(serde_json::Error),
    /// A failable error when parsing the response as JSON
    #[cfg(feature = "serde")]
    SerdeFailableError(ToshoDetailedFailableError),
    /// Failed to parse the response as Protobuf
    #[cfg(feature = "protobuf")]
    ProstError(prost::DecodeError),
    /// Response is empty
    EmptyResponse,
    /// Response has invalid expected response
    ExpectedResponse(String),
    /// Invalid status code
    InvalidStatusCode(reqwest::StatusCode),
}

impl ToshoParseError {
    /// Create a new instance of the error for [`ToshoParseError::ExpectedResponse`]
    pub fn expect(response: impl Into<String>) -> ToshoError {
        ToshoParseError::ExpectedResponse(response.into()).into()
    }

    /// Create a new instance of the error for [`ToshoParseError::EmptyResponse`]
    pub fn empty() -> ToshoError {
        ToshoParseError::EmptyResponse.into()
    }
}

/// Error type that happens when processing images
#[derive(Debug)]
pub enum ToshoImageError {
    /// Number conversion error
    ConversionError(std::num::ParseIntError),
    /// Image processing error
    #[cfg(feature = "image")]
    ImageError(ToshoDetailedImageError),
    /// Image decoding error
    ReadError(String),
    /// Image encoding error
    WriteError(String),
}

/// Error type that happens when processing images
#[derive(Debug)]
#[cfg(feature = "image")]
pub struct ToshoDetailedImageError {
    inner: image::ImageError,
    description: String,
}

#[cfg(feature = "image")]
impl ToshoDetailedImageError {
    /// Create a new instance of image errow with a more detailed response
    pub fn new(inner: image::ImageError, description: impl Into<String>) -> Self {
        Self {
            inner,
            description: description.into(),
        }
    }
}

#[cfg(feature = "image")]
impl std::fmt::Display for ToshoDetailedImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.description, self.inner)
    }
}

#[cfg(feature = "serde")]
impl From<ToshoDetailedParseError> for ToshoParseError {
    fn from(value: ToshoDetailedParseError) -> Self {
        ToshoParseError::SerdeDetailedError(value)
    }
}

#[cfg(feature = "serde")]
impl From<ToshoDetailedParseError> for ToshoError {
    fn from(value: ToshoDetailedParseError) -> Self {
        ToshoError::ParseError(ToshoParseError::SerdeDetailedError(value))
    }
}

/// Error type that happens when authenticating
#[derive(Debug)]
pub enum ToshoAuthError {
    /// Happens when the credentials are invalid
    InvalidCredentials(String),
    /// Happens when the session is invalid
    InvalidSession,
    /// Happens when tosho unable to get the session
    UnknownSession,
    /// Other errors that doesn't fit the other categories
    CommonError(String),
}

/// Error type that happens when creating new Client instance
#[derive(Debug)]
pub enum ToshoClientError {
    /// Build error, this just wraps [`reqwest::Error`]
    BuildError(reqwest::Error),
    /// Fails when parsing into HTTP headers
    HeaderParseError(String),
}

impl From<reqwest::Error> for ToshoError {
    fn from(value: reqwest::Error) -> Self {
        ToshoError::RequestError(value)
    }
}

impl From<reqwest::StatusCode> for ToshoError {
    fn from(value: reqwest::StatusCode) -> Self {
        ToshoError::ParseError(ToshoParseError::InvalidStatusCode(value))
    }
}

#[cfg(feature = "protobuf")]
impl From<prost::DecodeError> for ToshoError {
    fn from(value: prost::DecodeError) -> Self {
        ToshoError::ParseError(ToshoParseError::ProstError(value))
    }
}

impl From<std::io::Error> for ToshoError {
    fn from(value: std::io::Error) -> Self {
        ToshoError::IOError(value)
    }
}

#[cfg(feature = "image")]
impl From<image::ImageError> for ToshoError {
    fn from(value: image::ImageError) -> Self {
        // Determine error kind
        match value {
            image::ImageError::Decoding(hint) => {
                let fmt_hint = match hint.format_hint() {
                    image::error::ImageFormatHint::Exact(fmt) => fmt.to_mime_type().to_string(),
                    image::error::ImageFormatHint::Name(name) => name,
                    image::error::ImageFormatHint::PathExtension(ext) => format!("{ext:?}"),
                    image::error::ImageFormatHint::Unknown => "Unknown".to_string(),
                    _ => "Unknown Error Handled".to_string(),
                };

                ToshoError::ImageError(ToshoImageError::ReadError(fmt_hint))
            }
            image::ImageError::Encoding(hint) => {
                let fmt_hint = match hint.format_hint() {
                    image::error::ImageFormatHint::Exact(fmt) => fmt.to_mime_type().to_string(),
                    image::error::ImageFormatHint::Name(name) => name,
                    image::error::ImageFormatHint::PathExtension(ext) => format!("{ext:?}"),
                    image::error::ImageFormatHint::Unknown => "Unknown".to_string(),
                    _ => "Unknown Error Handled".to_string(),
                };

                ToshoError::ImageError(ToshoImageError::WriteError(fmt_hint))
            }
            image::ImageError::IoError(io_err) => ToshoError::IOError(io_err),
            _ => ToshoError::ImageError(ToshoImageError::ImageError(ToshoDetailedImageError {
                inner: value,
                description: "Image processing error".to_string(),
            })),
        }
    }
}

#[cfg(feature = "serde")]
impl From<serde_json::Error> for ToshoError {
    fn from(value: serde_json::Error) -> Self {
        ToshoParseError::SerdeMinimalError(value).into()
    }
}

impl From<ToshoParseError> for ToshoError {
    fn from(value: ToshoParseError) -> Self {
        ToshoError::ParseError(value)
    }
}

impl From<ToshoImageError> for ToshoError {
    fn from(value: ToshoImageError) -> Self {
        ToshoError::ImageError(value)
    }
}

impl From<ToshoAuthError> for ToshoError {
    fn from(value: ToshoAuthError) -> Self {
        ToshoError::AuthError(value)
    }
}

impl From<ToshoClientError> for ToshoError {
    fn from(value: ToshoClientError) -> Self {
        ToshoError::ClientError(value)
    }
}

impl std::fmt::Display for ToshoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToshoError::CommonError(msg) => write!(f, "{msg}"),
            ToshoError::RequestError(e) => write!(f, "Request error: {e}"),
            ToshoError::ParseError(e) => write!(f, "Failed to parse response, {e}"),
            ToshoError::ImageError(e) => write!(f, "Image error: {e}"),
            ToshoError::IOError(e) => write!(f, "IO error: {e}"),
            ToshoError::ClientError(e) => write!(f, "Client error: {e}"),
            ToshoError::AuthError(e) => write!(f, "Authentication error: {e}"),
        }
    }
}

impl std::fmt::Display for ToshoParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "serde")]
            ToshoParseError::SerdeDetailedError(e) => write!(f, "{e}"),
            #[cfg(feature = "serde")]
            ToshoParseError::SerdeFailableError(e) => write!(f, "{e}"),
            #[cfg(feature = "serde")]
            ToshoParseError::SerdeMinimalError(e) => write!(f, "failed to decode JSON data: {e}"),
            #[cfg(feature = "protobuf")]
            ToshoParseError::ProstError(e) => write!(f, "{e}"),
            ToshoParseError::EmptyResponse => write!(f, "empty response received"),
            ToshoParseError::ExpectedResponse(e) => write!(
                f,
                "invalid response: expected {e} but got empty/unknown response"
            ),
            ToshoParseError::InvalidStatusCode(code) => write!(f, "invalid status code: {code}"),
        }
    }
}

impl std::fmt::Display for ToshoImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToshoImageError::ConversionError(e) => {
                write!(f, "An error occured while tyring to convert number: {e}")
            }
            #[cfg(feature = "image")]
            ToshoImageError::ImageError(e) => write!(f, "{e}"),
            ToshoImageError::ReadError(e) => write!(f, "Failed to read image: {e}"),
            ToshoImageError::WriteError(e) => write!(f, "Failed to write image: {e}"),
        }
    }
}

impl std::fmt::Display for ToshoAuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToshoAuthError::InvalidCredentials(reason) => {
                write!(f, "Invalid credentials, reason: {reason}")
            }
            ToshoAuthError::InvalidSession => write!(f, "Invalid session"),
            ToshoAuthError::UnknownSession => write!(f, "Unknown session"),
            ToshoAuthError::CommonError(e) => write!(f, "{e}"),
        }
    }
}

impl std::fmt::Display for ToshoClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToshoClientError::BuildError(e) => write!(f, "Failed to build client: {e}"),
            ToshoClientError::HeaderParseError(e) => write!(f, "Failed to parse headers: {e}"),
        }
    }
}

impl std::error::Error for ToshoError {}
