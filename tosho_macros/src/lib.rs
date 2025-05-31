#![warn(missing_docs, clippy::empty_docs, rustdoc::broken_intra_doc_links)]
#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;

mod common;
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
/// # Attributes
/// The following is an attribute that can be used on each field of the struct.
/// - `#[copyable]` will make the field return a copy instead of a reference (only for `Copy` types or primitives).
/// - `#[skip_field]` will make the field not have a getter or be skipped.
/// - `#[deref_clone]` will not return the reference and will return an "Owned" type (no actual cloning happens).
///
/// And, an attribute can be used on the struct as well.
/// - `#[auto_getters(unref = true)]` similar to `#[copyable]` but for all the fields.
/// - `#[auto_getters(cloned = true)]` similar to `#[deref_clone]` but for all the fields.
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
///
/// Another one with `cloned` or `deref_clone`
/// ```rust,no_run
/// # use tosho_macros::AutoGetter;
/// #
/// #[derive(AutoGetter)]
/// pub struct Data {
///     #[copyable]
///     id: i64,
///     #[deref_clone]
///     username: String,
/// }
///
/// # fn main() {
/// let data = Data { id: 1, username: "test".to_string() };
/// let owned_username = data.username(); // Data is now "moved" or "owned"
///
/// assert_eq!(owned_username, "test");
/// # }
/// ```
#[proc_macro_derive(
    AutoGetter,
    attributes(auto_getters, copyable, skip_field, deref_clone)
)]
pub fn autogetter_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    // Generate the implementation of the trait
    structs::impl_autogetter(&input)
}

/// Create a function that is derived from the enum and returns the
/// documentation of the variant as a static string that can be used.
///
/// # Example
/// ```rust
/// # use tosho_macros::AutoDocFields;
/// #
/// #[derive(AutoDocFields)]
/// enum TestEnum {
///     /// Create a new item
///     Create,
///     /// Read an item
///     Read,
///     NoComment,
/// }
///
/// # fn main() {
/// assert_eq!(TestEnum::Create.get_doc(), Some("Create a new item"));
/// assert_eq!(TestEnum::Read.get_doc(), Some("Read an item"));
/// assert_eq!(TestEnum::NoComment.get_doc(), None);
/// # }
/// ```
#[proc_macro_derive(AutoDocFields)]
pub fn autodocfields_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    enums::impl_auto_doc_fiels(&ast)
}

/// A custom derive macro that follows similar pattern to `prost::Enumeration`
///
/// This version allows us to define an enum with an unrecognized variant
/// instead of falling back to a default value.
///
/// # Example
/// ```rust
/// # use tosho_macros::ProstEnumUnrecognized;
/// #
/// #[derive(ProstEnumUnrecognized, Debug, PartialEq, Eq, PartialOrd, Ord)]
/// enum TestEnum {
///     Any = 0,
///     Paid = 1,
///     // #[invalid_enum] is always required to be present
///     #[invalid_enum]
///     Unrecognized = -1,
/// }
///
/// # fn main() {
/// assert_eq!(TestEnum::try_from(2).unwrap(), TestEnum::Unrecognized);
/// assert_eq!(TestEnum::try_from(0).unwrap(), TestEnum::Any);
/// # }
/// ```
#[proc_macro_derive(ProstEnumUnrecognized, attributes(invalid_enum))]
pub fn prost_enum_unrecognized_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    enums::impl_prost_enum_unrecognized(&ast)
}
