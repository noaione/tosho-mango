//! A module containing all the models/proto mapping used in the library.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

pub mod account;
pub mod chapter;
pub mod enums;
pub mod manga;
pub mod point;

pub use account::*;
pub use chapter::*;
pub use enums::*;
pub use manga::*;
pub use point::*;
