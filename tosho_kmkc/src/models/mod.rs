//! A module containing all the models used in the library.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

pub mod account;
pub mod common;
mod datetime;
pub mod enums;
pub mod episode;
pub mod errors;
pub mod other;
pub mod titles;

pub use account::*;
pub use common::*;
pub use enums::*;
pub use episode::*;
pub use errors::*;
pub use other::*;
pub use titles::*;
