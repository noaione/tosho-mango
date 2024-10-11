//! # tosho-macros
//!
//! A collection of macros used by [`tosho`](https://github.com/noaione/tosho-mango) and the other sources crates.
//!
//! ## License
//!
//! This project is licensed with MIT License ([LICENSE](https://github.com/noaione/tosho-mango/blob/master/LICENSE) or <http://opensource.org/licenses/MIT>)

use proc_macro::TokenStream;

mod deser;
mod enums;
mod structs;

/// Derives [`serde::Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) for an enum using [`std::fmt::Display`]
///
/// # Example
/// ```rust
/// # use tosho_macros::SerializeEnum;
/// #
/// #[derive(SerializeEnum)]
/// enum TestEnum {
///     Create,
///     Read,
/// }
///
/// impl std::fmt::Display for TestEnum {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         match self {
///             TestEnum::Create => write!(f, "create"),
///             TestEnum::Read => write!(f, "read"),
///         }
///     }
/// }
///
/// let test_enum = TestEnum::Create;
/// assert_eq!(test_enum.to_string(), "create");
/// ```
#[proc_macro_derive(SerializeEnum)]
pub fn serializenum_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    deser::impl_serenum_derive(&ast)
}

/// Derives [`serde::Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) for an enum using [`std::str::FromStr`]
///
/// # Example
/// ```rust
/// # use tosho_macros::DeserializeEnum;
/// #
/// #[derive(DeserializeEnum, PartialEq, Eq, Debug)]
/// enum TestEnum {
///     Create,
///     Read,
/// }
///
/// tosho_macros::enum_error!(TestEnumFromStrError);
///
/// impl std::str::FromStr for TestEnum {
///     type Err = TestEnumFromStrError;
///     
///     fn from_str(s: &str) -> Result<Self, Self::Err> {
///          match s {
///             "create" => Ok(TestEnum::Create),
///             "read" => Ok(TestEnum::Read),
///             _ => Err(TestEnumFromStrError {
///                 original: s.to_string(),
///             }),
///         }
///     }
/// }
///
/// let test_enum: TestEnum = "create".parse().unwrap();
/// assert_eq!(test_enum, TestEnum::Create);
/// ```
#[proc_macro_derive(DeserializeEnum)]
pub fn deserializeenum_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    deser::impl_deserenum_derive(&ast)
}

/// Derives [`serde::Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) for an enum in i32 mode.
///
/// # Example
/// ```rust
/// # use tosho_macros::SerializeEnum32;
/// #
/// #[derive(SerializeEnum32)]
/// enum TestEnum {
///     Create = 0,
///     Read = 1,
/// }
/// ```
#[proc_macro_derive(SerializeEnum32)]
pub fn serializenum32_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    deser::impl_serenum32_derive(&ast)
}

/// Derives [`serde::Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) for an enum in i32 mode.
///
/// # Example
/// ```rust
/// # use tosho_macros::DeserializeEnum32;
/// #
/// #[derive(DeserializeEnum32)]
/// enum TestEnum {
///     Create = 0,
///     Read = 1,
/// }
/// ```
#[proc_macro_derive(DeserializeEnum32)]
pub fn deserializeenum32_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    deser::impl_deserenum32_derive(&ast, false)
}

/// Derives [`serde::Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) for an enum in i32 mode with fallback to [`std::default::Default`].
///
/// # Example
/// ```rust
/// # use tosho_macros::DeserializeEnum32Fallback;
/// #
/// #[derive(DeserializeEnum32Fallback, Default)]
/// enum TestEnum {
///     #[default]
///     Unknown = -1,
///     Create = 0,
///     Read = 1,
/// }
/// ```
#[proc_macro_derive(DeserializeEnum32Fallback)]
pub fn deserializeenum32fallback_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    deser::impl_deserenum32_derive(&ast, true)
}

/// Derives an enum that would implement `.to_name()`
///
/// # Example
/// ```rust
/// # use tosho_macros::EnumName;
/// #
/// #[derive(EnumName, Clone, Debug)]
/// enum TestEnum {
///     Create,
///     Read,
/// }
///
/// let test_enum = TestEnum::Create;
/// assert_eq!(test_enum.to_name(), "Create");
/// ```
#[proc_macro_derive(EnumName)]
pub fn enumname_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    enums::impl_enumname_derive(&ast)
}

/// Derives an enum that would implement `::count()` to return the number of variants
///
/// # Example
/// ```rust
/// # use tosho_macros::EnumCount;
/// #
/// #[derive(EnumCount, Clone, Debug)]
/// enum TestEnum {
///     Create,
///     Read,
/// }
///
/// assert_eq!(TestEnum::count(), 2);
/// ```
#[proc_macro_derive(EnumCount)]
pub fn enumcount_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    enums::impl_enumcount_derive(&ast)
}

/// Derives an enum that would implement [`From<u32>`].
#[proc_macro_derive(EnumU32)]
pub fn enumu32_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    enums::impl_enumu32_derive(&ast, false)
}

/// Derives an enum that would implement [`From<u32>`] with fallback.
#[proc_macro_derive(EnumU32Fallback)]
pub fn enumu32fallback_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    enums::impl_enumu32_derive(&ast, true)
}

/// Create an error struct for an enum that implements [`std::fmt::Display`] that can be used
/// when using other macros to derive [`std::str::FromStr`] for an enum.
///
/// # Example
/// ```rust
/// # use tosho_macros::enum_error;
/// #
/// enum TestEnum {
///     Foo,
///     Bar,
/// }
///
/// enum_error!(TestEnumFromStrError);
/// ```
#[proc_macro]
pub fn enum_error(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as enums::EnumErrorMacroInput);
    enums::impl_enum_error(&input)
}

/// Derive the `AutoGetter` macro for a struct
///
/// Automatically expand each field into their own getter function for all private field.
///
/// # Examples
/// ```rust
/// # use tosho_macros::AutoGetter;
/// #
/// #[derive(AutoGetter)]
/// pub struct Data {
///     #[copyable]
///     id: i64,
///     username: String,
///     #[skip_field]
///     pos: u32,
/// }
///
/// # fn main() {
/// let data = Data { id: 1, username: "test".to_string(), pos: 0 };
///
/// assert_eq!(data.id(), 1);
/// assert_eq!(data.username(), "test");
/// // "pos" field doesn't have getter
/// assert_eq!(data.pos, 0);
/// # }
/// ```
#[proc_macro_derive(AutoGetter, attributes(auto_getters, copyable, skip_field))]
pub fn autogetter_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    // Generate the implementation of the trait
    structs::impl_autogetter(&input)
}
