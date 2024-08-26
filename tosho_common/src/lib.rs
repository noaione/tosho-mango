#![doc = include_str!("../README.md")]
pub mod errors;
#[cfg(feature = "id-gen")]
pub mod generator;
pub mod parser;

pub use errors::*;
#[cfg(feature = "id-gen")]
pub use generator::*;
pub use parser::*;

/// Create a [`ToshoError`] with the formatted string and return it.
///
/// This will use [`ToshoError::CommonError`] type to create the error.
#[macro_export]
macro_rules! make_error {
    // Accept string that can be formatted, then also accept a list of arguments
    ($($arg:tt)*) => {
        // Return a ToshoError::Error with the formatted string
        $crate::ToshoError::new(format!($($arg)*))
    };
}

/// Create a [`ToshoError`] with the formatted string and return it.
///
/// This will use [`ToshoError::CommonError`] type to create the error.
///
/// The main difference with [`make_error!`] is that this macro will automatically
/// return the error essentially "bailling" the function.
#[macro_export]
macro_rules! bail_on_error {
    // Accept string that can be formatted, then also accept a list of arguments
    ($($arg:tt)*) => {
        // Return a ToshoError::Error with the formatted string
        return Err($crate::ToshoError::new(format!($($arg)*)))
    };
}
