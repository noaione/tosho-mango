//! A module containing all the models used in the library.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

#![warn(clippy::missing_docs_in_private_items)]

pub mod accounts;
pub mod comic;
pub mod common;
pub mod enums;
pub mod errors;

pub use accounts::*;
pub use comic::*;
pub use common::*;
pub use enums::*;
pub use errors::*;
