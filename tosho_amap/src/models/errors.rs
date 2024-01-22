/// The used error type for the API.
#[derive(Debug)]
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
