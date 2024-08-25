pub mod errors;
pub mod parser;

pub use errors::*;
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

/// Same as [`make_error!`] but will immediately return the error.
#[macro_export]
macro_rules! bail_on_error {
    // Accept string that can be formatted, then also accept a list of arguments
    ($($arg:tt)*) => {
        // Return a ToshoError::Error with the formatted string
        return Err($crate::ToshoError::new(format!($($arg)*)))
    };
}
