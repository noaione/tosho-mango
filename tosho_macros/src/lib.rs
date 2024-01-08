use proc_macro::TokenStream;

/// Derives [`serde::Serialize`] for an enum using [`ToString`]
#[proc_macro_derive(SerializeEnum)]
pub fn serializenum_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_serenum_derive(&ast)
}

/// Derives [`serde::Deserialize`] for an enum using [`std::str::FromStr`]
#[proc_macro_derive(DeserializeEnum)]
pub fn deserializeenum_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_deserenum_derive(&ast)
}

/// Derives [`serde::Serialize`] for an enum in i32 mode.
#[proc_macro_derive(SerializeEnum32)]
pub fn serializenum32_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_serenum32_derive(&ast)
}

/// Derives [`serde::Deserialize`] for an enum in i32 mode.
#[proc_macro_derive(DeserializeEnum32)]
pub fn deserializeenum32_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_deserenum32_derive(&ast, false)
}

/// Derives [`serde::Deserialize`] for an enum in i32 mode with fallback to [`std::default::Default`].
#[proc_macro_derive(DeserializeEnum32Fallback)]
pub fn deserializeenum32fallback_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_deserenum32_derive(&ast, true)
}

/// Derives an enum that would implement `.to_name()`
#[proc_macro_derive(EnumName)]
pub fn enumname_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_enumname_derive(&ast)
}

/// Derives an enum that would implement `::count()` to return the number of variants
#[proc_macro_derive(EnumCount)]
pub fn enumcount_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_enumcount_derive(&ast)
}

/// Derives an enum that would implement From<u32>.
#[proc_macro_derive(EnumU32)]
pub fn enumu32_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_enumu32_derive(&ast, false)
}

/// Derives an enum that would implement From<u32> with fallback.
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

#[proc_macro]
pub fn enum_error(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as EnumErrorMacroInput);
    // enum_error
    let name = &input.name;
    let tokens = quote::quote! {
        #[derive(Debug)]
        pub struct #name {
            original: String,
        }

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let s: &str = stringify!(#name);
                // remove FromStrError
                let s = s.strip_suffix("FromStrError").unwrap_or(s);
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
