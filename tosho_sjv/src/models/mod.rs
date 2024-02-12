use serde::{Deserialize, Serialize};

pub mod account;
pub mod datetime;
pub mod enums;
pub(super) mod manga;

pub use account::*;
pub use enums::*;
pub use manga::*;

/// A simple response to check if request successful or not
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleResponse {
    pub ok: IntBool,
}

impl SimpleResponse {
    /// Check if response is OK
    pub fn is_ok(&self) -> bool {
        self.ok == IntBool::True
    }
}
