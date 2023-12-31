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
