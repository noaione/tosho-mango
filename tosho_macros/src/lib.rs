//! # tosho-macros
//!
//! A collection of macros used by [`tosho`](https://github.com/noaione/tosho-mango) and the other sources crates.
//!
//! ## License
//!
//! This project is licensed with MIT License ([LICENSE](https://github.com/noaione/tosho-mango/blob/master/LICENSE) or <http://opensource.org/licenses/MIT>)

use proc_macro::TokenStream;

/// Derives [`serde::Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) for an enum using [`ToString`]
///
/// # Example
/// ```
/// use serde::Serialize;
/// use tosho_macros::SerializeEnum;
///
/// #[derive(SerializeEnum)]
/// enum TestEnum {
///     Create,
///     Read,
/// }
///
/// impl ToString for TestEnum {
///     fn to_string(&self) -> String {
///         match self {
///             TestEnum::Create => "create".to_string(),
///             TestEnum::Read => "read".to_string(),
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
    impl_serenum_derive(&ast)
}

/// Derives [`serde::Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) for an enum using [`std::str::FromStr`]
///
/// # Example
/// ```
/// use serde::Deserialize;
/// use tosho_macros::DeserializeEnum;
///
/// #[derive(DeserializeEnum)]
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
    impl_deserenum_derive(&ast)
}

/// Derives [`serde::Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) for an enum in i32 mode.
///
/// # Example
/// ```
/// use serde::Serialize;
/// use tosho_macros::SerializeEnum32;
///
/// #[derive(SerializeEnum32)]
/// enum TestEnum {
///     Create = 0,
///     Read = 1,
/// }
/// ```
#[proc_macro_derive(SerializeEnum32)]
pub fn serializenum32_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_serenum32_derive(&ast)
}

/// Derives [`serde::Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) for an enum in i32 mode.
///
/// # Example
/// ```
/// use serde::Deserialize;
/// use tosho_macros::DeserializeEnum32;
///
/// #[derive(DeserializeEnum32)]
/// enum TestEnum {
///     Create = 0,
///     Read = 1,
/// }
/// ```
#[proc_macro_derive(DeserializeEnum32)]
pub fn deserializeenum32_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_deserenum32_derive(&ast, false)
}

/// Derives [`serde::Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) for an enum in i32 mode with fallback to [`std::default::Default`].
///
/// # Example
/// ```
/// use serde::Deserialize;
/// use tosho_macros::DeserializeEnum32Fallback;
///
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
    impl_deserenum32_derive(&ast, true)
}

/// Derives an enum that would implement `.to_name()`
///
/// # Example
/// ```
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
    impl_enumname_derive(&ast)
}

/// Derives an enum that would implement `::count()` to return the number of variants
///
/// # Example
/// ```
/// #[derive(EnumCount, Clone, Debug)]
/// enum TestEnum {
///     Create,
///     Read,
/// }
///
/// let test_enum = TestEnum::Create;
/// assert_eq!(TestEnum::count(), 2);
/// ```
#[proc_macro_derive(EnumCount)]
pub fn enumcount_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_enumcount_derive(&ast)
}

/// Derives an enum that would implement [`From<u32>`].
#[proc_macro_derive(EnumU32)]
pub fn enumu32_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_enumu32_derive(&ast, false)
}

/// Derives an enum that would implement [`From<u32>`] with fallback.
#[proc_macro_derive(EnumU32Fallback)]
pub fn enumu32fallback_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_enumu32_derive(&ast, true)
}

struct EnumErrorMacroInput {
    name: syn::Ident,
}

impl syn::parse::Parse for EnumErrorMacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
        })
    }
}

/// Create an error struct for an enum that implements [`std::fmt::Display`] that can be used
/// when using other macros to derive [`std::str::FromStr`] for an enum.
///
/// # Example
/// ```
/// use tosho_macros::enum_error;
///
/// enum_error!(TestEnumFromStrError);
/// ```
#[proc_macro]
pub fn enum_error(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as EnumErrorMacroInput);
    // enum_error
    let name = &input.name;
    let stripped_name = name.to_string();
    let stripped_name = stripped_name.strip_suffix("FromStrError").unwrap();

    let tokens = quote::quote! {
        #[doc = "Error struct when parsing `"]
        #[doc = #stripped_name]
        #[doc = "` from string fails"]
        #[derive(Debug)]
        pub struct #name {
            original: String,
        }

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let s: &str = stringify!(#stripped_name);
                write!(f, "\"{}\" is not a valid {} type", self.original, s)
            }
        }
    };

    tokens.into()
}

fn impl_serenum_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    match &ast.data {
        syn::Data::Enum(_) => {}
        _ => panic!("`SerializeEnum` can only be derived for enums"),
    };

    let tokens = quote::quote! {
        impl Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(&self.to_string())
            }
        }
    };
    tokens.into()
}

fn impl_deserenum_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    match &ast.data {
        syn::Data::Enum(_) => {}
        _ => panic!("`DeserializeEnum` can only be derived for enums"),
    };

    let tokens = quote::quote! {
        impl<'de> Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                s.parse::<#name>().map_err(serde::de::Error::custom)
            }
        }
    };
    tokens.into()
}

fn impl_serenum32_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    // We want to get the values of the enum variants to create our match arms
    let variants = match &ast.data {
        syn::Data::Enum(v) => &v.variants,
        _ => panic!("`SerializeEnum32` can only be derived for enums"),
    };

    let mut match_arms = vec![];
    for variant in variants {
        let variant_name = &variant.ident;
        let value = if let Some((_, expr)) = &variant.discriminant {
            quote::quote! { #expr }
        } else {
            quote::quote! { stringify!(#variant_name).parse().unwrap() }
        };

        match_arms.push(quote::quote! {
            #name::#variant_name => serializer.serialize_i32(#value),
        });
    }

    let tokens = quote::quote! {
        impl Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                // serialize to i32
                match self {
                    #(#match_arms)*
                }
            }
        }
    };
    tokens.into()
}

fn impl_deserenum32_derive(ast: &syn::DeriveInput, with_default: bool) -> TokenStream {
    let name = &ast.ident;

    // We want to get the values of the enum variants to create our match arms
    let variants = match &ast.data {
        syn::Data::Enum(v) => &v.variants,
        _ => panic!("`DeserializeEnum32` can only be derived for enums"),
    };

    let mut match_arms = vec![];
    for variant in variants {
        let variant_name = &variant.ident;
        let value = if let Some((_, expr)) = &variant.discriminant {
            quote::quote! { #expr }
        } else {
            quote::quote! { stringify!(#variant_name).parse().unwrap() }
        };

        match_arms.push(quote::quote! {
            #value => Ok(#name::#variant_name),
        });
    }

    match with_default {
        true => {
            match_arms.push(quote::quote! {
                _ => Ok(#name::default()),
            });
        }
        false => {
            match_arms.push(quote::quote! {
                _ => Err(serde::de::Error::custom(format!("Invalid {} value: {}", stringify!(#name), s))),
            })
        }
    }

    let tokens = quote::quote! {
        impl<'de> Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let s = i32::deserialize(deserializer)?;
                match s {
                    #(#match_arms)*
                }
            }
        }
    };
    tokens.into()
}

fn impl_enumname_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    // check if ast is an enum
    let variants = match &ast.data {
        syn::Data::Enum(v) => &v.variants,
        _ => panic!("`EnumName` can only be derived for enums"),
    };

    let mut arms = Vec::new();
    for variant in variants {
        let ident = &variant.ident;
        arms.push(quote::quote! {
            Self::#ident => stringify!(#ident),
        });
    }

    let tokens = quote::quote! {
        impl #name {
            /// Returns the name of the enum variant as a string
            pub fn to_name(&self) -> &'static str {
                match self {
                    #(#arms)*
                }
            }
        }
    };
    tokens.into()
}

fn impl_enumcount_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let variants = match &ast.data {
        syn::Data::Enum(v) => &v.variants,
        _ => panic!("`EnumCount` can only be derived for enums"),
    };

    let count = variants.len();

    let tokens = quote::quote! {
        impl #name {
            /// Returns the number of variants in the enum
            pub fn count() -> usize {
                #count
            }
        }
    };
    tokens.into()
}

fn impl_enumu32_derive(ast: &syn::DeriveInput, with_default: bool) -> TokenStream {
    let name = &ast.ident;

    let variants = match &ast.data {
        syn::Data::Enum(v) => &v.variants,
        _ => panic!("`EnumU32` can only be derived for enums"),
    };

    let mut match_arms = vec![];
    for variant in variants {
        let variant_name = &variant.ident;
        // convert from u32 to enum
        let value = if let Some((_, expr)) = &variant.discriminant {
            quote::quote! { #expr }
        } else {
            quote::quote! { stringify!(#variant_name).parse().unwrap() }
        };

        match_arms.push(quote::quote! {
            #value => #name::#variant_name,
        });
    }

    match with_default {
        true => {
            match_arms.push(quote::quote! {
                _ => #name::default(),
            });
        }
        false => match_arms.push(quote::quote! {
            _ => panic!("Invalid value for {}: {}", stringify!(#name), value)
        }),
    }

    let tokens = quote::quote! {
        impl From<u32> for #name {
            fn from(value: u32) -> Self {
                match value {
                    #(#match_arms)*
                }
            }
        }
    };
    tokens.into()
}
