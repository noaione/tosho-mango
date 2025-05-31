//! A implementation collection of Enum related derive and expansion

use crate::common::get_field_comment;
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

pub(crate) fn impl_auto_doc_fiels(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let variants = match &ast.data {
        syn::Data::Enum(v) => &v.variants,
        _ => panic!("`AutoDocFields` can only be derived for enums"),
    };

    // Get each variants doc data
    let mut variant_docs = vec![];
    for variant in variants {
        let variant_name = &variant.ident;
        let comments = get_field_comment(&variant.attrs);

        // Create match arms
        let quote_arms = if let Some(comments) = comments {
            quote::quote! {
                #name::#variant_name => Some(#comments),
            }
        } else {
            quote::quote! {
                #name::#variant_name => None,
            }
        };

        variant_docs.push(quote_arms);
    }

    let tokens = quote::quote! {
        impl #name {
            /// Get the documentation or docstring of the current enums.
            pub fn get_doc(&self) -> Option<&'static str> {
                match self {
                    #(#variant_docs)*
                }
            }
        }
    };

    tokens.into()
}

pub(crate) fn impl_prost_enum_unrecognized(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let name = &ast.ident;

    let data = match &ast.data {
        syn::Data::Enum(data) => data,
        _ => {
            return TokenStream::from(
                syn::Error::new_spanned(ast, "Expected a struct for the `AutoGetter` derive macro")
                    .to_compile_error(),
            );
        }
    };

    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut invalid_variant = None;
    let mut regular_variants = Vec::new();

    // Validate each variant
    for variant in &data.variants {
        // Check if this variant has the invalid_enum attribute
        let has_invalid_attr = variant
            .attrs
            .iter()
            .any(|attr| attr.path().is_ident("invalid_enum"));

        // Every variant must have an explicit discriminant
        match &variant.discriminant {
            Some(discriminant) => {
                if has_invalid_attr {
                    if invalid_variant.is_some() {
                        return TokenStream::from(
                            syn::Error::new_spanned(
                                variant,
                                "Only one variant can have the #[invalid_enum] attribute",
                            )
                            .to_compile_error(),
                        );
                    }
                    match expand_expr_enum_unrecognized_i32(&discriminant.1) {
                        Ok(value) => {
                            invalid_variant =
                                Some((variant.ident.clone(), discriminant.1.clone(), value));
                        }
                        Err(e) => return TokenStream::from(e.to_compile_error()),
                    }
                } else {
                    match expand_expr_enum_unrecognized_i32(&discriminant.1) {
                        Ok(value) => {
                            let expr = discriminant.1.clone();
                            regular_variants.push((variant.ident.clone(), expr, value));
                        }
                        Err(e) => return TokenStream::from(e.to_compile_error()),
                    }
                }
            }
            None => {
                return TokenStream::from(
                    syn::Error::new_spanned(
                        variant,
                        "Expected a variant with an explicit value (e.g., '= 0' or '= 1')",
                    )
                    .to_compile_error(),
                );
            }
        }
    }

    let (invalid_ident, invalid_value) = match invalid_variant {
        Some((ident, value, num)) => {
            if num >= 0 {
                return TokenStream::from(
                    syn::Error::new_spanned(
                        value,
                        "The value of invalid variant must be negative (e.g., -1, -2, etc.)",
                    )
                    .to_compile_error(),
                );
            }
            (ident, value)
        }
        None => {
            return TokenStream::from(
                syn::Error::new_spanned(
                    ast,
                    "Expected exactly one variant with the #[invalid_enum] attribute",
                )
                .to_compile_error(),
            );
        }
    };

    // Validate all regular variants
    if regular_variants.is_empty() {
        return TokenStream::from(
            syn::Error::new_spanned(
                ast,
                "Expected at least one variant without the #[invalid_enum] attribute",
            )
            .to_compile_error(),
        );
    }

    for (ident, _, value) in &regular_variants {
        if *value < 0 {
            return TokenStream::from(
                syn::Error::new_spanned(
                    ident,
                    "The value of regular variants must be non-negative (e.g., 0, 1, 2, etc.)",
                )
                .to_compile_error(),
            );
        }
    }

    let valid_doc = format!("Returns `true` if `value` is a variant of `{}`", name,);
    let from_i32_doc = format!(
        "Converts an `i32` value to the corresponding `{}` enum variant, returning the `{}::{}` variant if it doesn't match any variant.",
        name, name, invalid_ident,
    );

    let default = regular_variants[0].0.clone();

    let is_valid = regular_variants
        .iter()
        .map(|(_, value, _)| quote::quote! {#value => true});
    let from_i32 = regular_variants.iter().map(
        |(ident, value, _)| quote::quote! {#value => ::core::option::Option::Some(#name::#ident)},
    );
    let try_from = regular_variants.iter().map(
        |(ident, value, _)| quote::quote! {#value => ::core::result::Result::Ok(#name::#ident)},
    );

    // Generate the implementation
    let expanded = quote::quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #[doc=#valid_doc]
            pub fn is_valid(value: i32) -> bool {
                match value {
                    #(#is_valid,)*
                    #invalid_value => false,
                    _ => false,
                }
            }

            #[deprecated = "User the TryFrom<i32> implementation instead"]
            #[doc=#from_i32_doc]
            pub fn from_i32(value: i32) -> ::core::option::Option<#name> {
                match value {
                    #(#from_i32,)*
                    #invalid_value => ::core::option::Option::Some(#name::#invalid_ident),
                    _ => ::core::option::Option::Some(#name::#invalid_ident),
                }
            }
        }

        impl #impl_generics ::core::default::Default for #name #ty_generics #where_clause {
            fn default() -> Self {
                #name::#default
            }
        }

        impl #impl_generics ::core::convert::From::<#name> for i32 #ty_generics #where_clause {
            fn from(value: #name) -> i32 {
                value as i32
            }
        }

        impl #impl_generics ::core::convert::TryFrom::<i32> for #name #ty_generics #where_clause {
            type Error = ::prost::UnknownEnumValue;

            fn try_from(value: i32) -> ::core::result::Result<#name, ::prost::UnknownEnumValue> {
                match value {
                    #(#try_from,)*
                    #invalid_value => ::core::result::Result::Ok(#name::#invalid_ident),
                    _ => ::core::result::Result::Ok(#name::#invalid_ident),
                }
            }
        }
    };

    expanded.into()
}

fn expand_expr_enum_unrecognized_i32(expr: &syn::Expr) -> Result<i32, syn::Error> {
    match expr {
        syn::Expr::Lit(lit) => {
            match &lit.lit {
                syn::Lit::Int(lit_int) => {
                    // Parse the integer literal as i32
                    lit_int.base10_parse::<i32>().map_err(|_| {
                        syn::Error::new_spanned(
                            lit,
                            "Expected a valid i32 literal for the enum value",
                        )
                    })
                }
                _ => {
                    // If it's not an integer literal, we cannot handle it
                    Err(syn::Error::new_spanned(
                        lit,
                        "Expected an integer literal for the enum value",
                    ))
                }
            }
        }
        // Handle number like -1
        syn::Expr::Unary(unary) => {
            match &unary.op {
                syn::UnOp::Neg(_) => {
                    if let syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Int(lit_int),
                        ..
                    }) = &*unary.expr
                    {
                        // Parse the integer then negate it
                        match lit_int.base10_parse::<i32>() {
                            Ok(n) => Ok(-n),
                            Err(_) => {
                                return Err(syn::Error::new_spanned(
                                    unary,
                                    "Expected a valid i32 literal for the enum value",
                                ));
                            }
                        }
                    } else {
                        Err(syn::Error::new_spanned(
                            unary,
                            "Expected an integer literal for the enum value",
                        ))
                    }
                }
                _ => {
                    // If it's not a negation, we cannot handle it
                    Err(syn::Error::new_spanned(
                        unary,
                        "Expected a negated integer literal for the enum value",
                    ))
                }
            }
        }
        _ => {
            // If the expression is not a literal, we cannot handle it
            return Err(syn::Error::new_spanned(
                expr,
                "Expected a literal integer for the enum value",
            ));
        }
    }
}
