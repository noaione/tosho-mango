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
use tosho_common::FailableResponse;

/// A simple response to check if request successful or not
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleResponse {
    pub ok: IntBool,
    pub error: Option<String>,
}

impl FailableResponse for SimpleResponse {
    fn format_error(&self) -> String {
        self.error
            .clone()
            .unwrap_or_else(|| "Unknown error".to_string())
    }

    fn raise_for_status(&self) -> tosho_common::ToshoResult<()> {
        if let Some(error) = &self.error {
            if self.ok != IntBool::True {
                return Err(tosho_common::make_error!("{}", error));
            }
        }
        Ok(())
    }
}
