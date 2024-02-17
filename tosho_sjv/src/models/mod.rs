//! A module containing all the models used in the library.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

use serde::{Deserialize, Serialize};

pub mod account;
pub(crate) mod datetime;
pub mod enums;
pub mod manga;

pub use account::*;
pub use enums::*;
pub use manga::*;

/// A simple response to check if request successful or not
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleResponse {
    pub ok: IntBool,
    pub error: Option<String>,
}

impl SimpleResponse {
    /// Check if response is OK
    pub fn is_ok(&self) -> bool {
        self.ok == IntBool::True
    }

    /// Check if response is not OK
    pub fn is_err(&self) -> bool {
        self.ok == IntBool::False || self.error.is_some()
    }
}
