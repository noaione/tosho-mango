//! A implementation collection of Enum related derive and expansion

use proc_macro::TokenStream;

pub(crate) fn impl_enumname_derive(ast: &syn::DeriveInput) -> TokenStream {
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

pub(crate) fn impl_enumcount_derive(ast: &syn::DeriveInput) -> TokenStream {
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

pub(crate) fn impl_enumu32_derive(ast: &syn::DeriveInput, with_default: bool) -> TokenStream {
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

pub(crate) struct EnumErrorMacroInput {
    name: syn::Ident,
}

impl syn::parse::Parse for EnumErrorMacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
        })
    }
}

pub(crate) fn impl_enum_error(ast: &EnumErrorMacroInput) -> TokenStream {
    // enum_error
    let name = &ast.name;
    let stripped_name = name.to_string();
    let stripped_name = stripped_name.strip_suffix("FromStrError").unwrap();

    let mut error_str = String::new();
    error_str.push_str("\"{}\" is not a valid ");
    error_str.push_str(stripped_name);
    error_str.push_str(" type");

    let tokens = quote::quote! {
        #[doc = "Error struct when parsing [`"]
        #[doc = #stripped_name]
        #[doc = "`] from string fails"]
        #[derive(Debug)]
        pub struct #name {
            original: String,
        }

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, #error_str, self.original)
            }
        }
    };

    tokens.into()
}
