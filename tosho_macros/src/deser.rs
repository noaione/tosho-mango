//! A implementation collection of some ser/de stuff

use proc_macro::TokenStream;

pub(crate) fn impl_serenum_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    match &ast.data {
        syn::Data::Enum(_) => {}
        _ => panic!("`SerializeEnum` can only be derived for enums"),
    };

    let tokens = quote::quote! {
        impl serde::Serialize for #name {
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

pub(crate) fn impl_deserenum_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    match &ast.data {
        syn::Data::Enum(_) => {}
        _ => panic!("`DeserializeEnum` can only be derived for enums"),
    };

    let tokens = quote::quote! {
        impl<'de> serde::Deserialize<'de> for #name {
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

pub(crate) fn impl_serenum32_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    // We want to get the values of the enum variants to create our match arms
    let variants = match &ast.data {
        syn::Data::Enum(v) => &v.variants,
        _ => panic!("`SerializeEnum32` can only be derived for enums"),
    };

    let mut match_arms = vec![];
    for variant in variants {
        let variant_name = &variant.ident;
        let variant_name_str = variant_name.to_string();
        let value = if let Some((_, expr)) = &variant.discriminant {
            quote::quote! { #expr }
        } else {
            quote::quote! { #variant_name_str.parse().unwrap() }
        };

        match_arms.push(quote::quote! {
            #name::#variant_name => serializer.serialize_i32(#value),
        });
    }

    let tokens = quote::quote! {
        impl serde::Serialize for #name {
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

pub(crate) fn impl_deserenum32_derive(ast: &syn::DeriveInput, with_default: bool) -> TokenStream {
    let name = &ast.ident;

    // We want to get the values of the enum variants to create our match arms
    let variants = match &ast.data {
        syn::Data::Enum(v) => &v.variants,
        _ => panic!("`DeserializeEnum32` can only be derived for enums"),
    };

    let mut match_arms = vec![];
    for variant in variants {
        let variant_name = &variant.ident;
        let variant_name_str = variant_name.to_string();
        let value = if let Some((_, expr)) = &variant.discriminant {
            quote::quote! { #expr }
        } else {
            quote::quote! { #variant_name_str.parse().unwrap() }
        };

        match_arms.push(quote::quote! {
            #value => Ok(#name::#variant_name),
        });
    }

    let name_str = name.to_string();
    match with_default {
        true => {
            match_arms.push(quote::quote! {
                _ => Ok(#name::default()),
            });
        }
        false => match_arms.push(quote::quote! {
            _ => Err(serde::de::Error::custom(format!("Invalid {} value: {}", #name_str, s))),
        }),
    }

    let tokens = quote::quote! {
        impl<'de> serde::Deserialize<'de> for #name {
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
