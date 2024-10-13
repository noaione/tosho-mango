//! An implementation collection for struct

//! A custom derive collection macro for ClickHouse model data.

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{punctuated::Punctuated, Attribute, Expr, Lit, Meta, Token};

static KNOWN_COPYABLE_FIELD: &[&str; 16] = &[
    "u8", "u16", "u32", "u64", "u128", "i8", "i16", "i32", "i64", "i128", "f32", "f64", "f128",
    "bool", "char", "usize",
];

#[derive(Default, Clone, Copy)]
struct AutoGetterAttr {
    unref: bool,
    cloned: bool,
}

fn get_autogetter_attr(attrs: Vec<Attribute>) -> Result<AutoGetterAttr, syn::Error> {
    let mut unref = false;
    let mut cloned = false;

    for attr in &attrs {
        if attr.path().is_ident("auto_getters") {
            let nested = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

            for meta in nested {
                if let Meta::NameValue(nameval) = meta {
                    if nameval.path.is_ident("unref") {
                        // Is a boolean
                        if let Expr::Lit(lit) = nameval.value {
                            if let Lit::Bool(val) = lit.lit {
                                unref = val.value;
                            } else {
                                return Err(syn::Error::new_spanned(
                                    lit,
                                    "Expected a boolean value for `unref`",
                                ));
                            }
                        } else {
                            return Err(syn::Error::new_spanned(
                                nameval.value,
                                "Expected a boolean value for `unref`",
                            ));
                        }
                    } else if nameval.path.is_ident("cloned") {
                        cloned = true;
                    } else {
                        return Err(syn::Error::new_spanned(
                            nameval.path,
                            "Unknown attribute for `auto_getters`",
                        ));
                    }
                }
            }
        }
    }

    Ok(AutoGetterAttr { unref, cloned })
}

/// The main function to expand the `AutoGetter` derive macro
///
/// # Examples
/// ```rust
/// # use tosho_macros::AutoGetter;
/// #[derive(AutoGetter)]
/// pub struct Model {
///     id: String,
///     username: String,
/// }
/// ```
///
/// Will generate
///
/// ```rust
/// # struct Model {
/// #     id: String,
/// #     username: String,
/// # }
/// impl Model {
///     pub fn id(&self) -> &str {
///        &self.id
///     }
///
///     pub fn username(&self) -> &str {
///        &self.username
///     }
/// }
/// ```
///
/// When the field use `Option` it will generate a getter that returns `Option<&T>`
/// If setting, it will set the value to `Some(value)` with param of the `T`
pub(crate) fn impl_autogetter(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let (fields, attrs_config) = match &ast.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(fields) => match get_autogetter_attr(ast.attrs.clone()) {
                Ok(attrs) => (fields, attrs),
                Err(err) => return TokenStream::from(err.to_compile_error()),
            },
            _ => {
                return TokenStream::from(
                    syn::Error::new_spanned(
                        ast,
                        "Expected a struct with named fields for the `AutoGetter` derive macro",
                    )
                    .to_compile_error(),
                );
            }
        },
        _ => {
            return TokenStream::from(
                syn::Error::new_spanned(ast, "Expected a struct for the `AutoGetter` derive macro")
                    .to_compile_error(),
            );
        }
    };

    let mut getters: Vec<proc_macro2::TokenStream> = Vec::new();

    for field in fields.named.iter() {
        let field_name = field.ident.as_ref().unwrap();
        let field_ty = &field.ty;

        if field_has_ident(field, "skip_field") {
            continue;
        }

        let field = if let Some(inner_ty) = get_inner_type_of_option(field_ty) {
            expand_option_field(field, field_name, inner_ty, attrs_config)
        } else {
            expand_regular_field(field, field_name, attrs_config)
        };

        getters.push(field);
    }

    let generics = &ast.generics;
    let (impl_gen, ty_gen, where_cl) = generics.split_for_impl();

    let expanded = quote::quote! {
        impl #impl_gen #name #ty_gen #where_cl {
            #(#getters)*
        }
    };

    expanded.into()
}

fn expand_option_field(
    field: &syn::Field,
    field_name: &syn::Ident,
    inner_type: &syn::Type,
    attrs_config: AutoGetterAttr,
) -> proc_macro2::TokenStream {
    let doc_get = make_field_comment(field, true);
    let is_cloned = field_has_ident(field, "deref_clone") || attrs_config.cloned;

    if is_cloned {
        return quote::quote! {
            #[doc = #doc_get]
            pub fn #field_name(self) -> Option<#inner_type> {
                self.#field_name
            }
        };
    }

    // If string, we can use as_deref
    if is_string_field(inner_type) {
        let has_vec = has_inner_type_with_x(inner_type, "Vec");

        if has_vec {
            quote::quote! {
                #[doc = #doc_get]
                pub fn #field_name(&self) -> Option<&[String]> {
                    self.#field_name.as_deref()
                }
            }
        } else {
            quote::quote! {
                #[doc = #doc_get]
                pub fn #field_name(&self) -> Option<&str> {
                    self.#field_name.as_deref()
                }
            }
        }
    } else {
        // Modify the field type to be a reference
        let is_copyable = field_has_ident(field, "copyable") || is_copy_able_field(inner_type);

        let get_field = if is_copyable || attrs_config.unref {
            quote::quote! {
                #[doc = #doc_get]
                pub fn #field_name(&self) -> Option<#inner_type> {
                    self.#field_name
                }
            }
        } else if let Some(inner_ty) = get_inner_type_of_vec(inner_type) {
            quote::quote! {
                #[doc = #doc_get]
                pub fn #field_name(&self) -> Option<&[#inner_ty]> {
                    self.#field_name.as_deref()
                }
            }
        } else {
            quote::quote! {
                #[doc = #doc_get]
                pub fn #field_name(&self) -> Option<&#inner_type> {
                    self.#field_name.as_ref()
                }
            }
        };

        // And the set ident to be just the field type without the Option
        let getter = quote::quote! {
            #get_field
        };

        getter
    }
}

fn expand_regular_field(
    field: &syn::Field,
    field_name: &syn::Ident,
    attrs_config: AutoGetterAttr,
) -> proc_macro2::TokenStream {
    let field_ty = &field.ty;
    let doc_get = make_field_comment(field, false);
    let is_cloned = field_has_ident(field, "deref_clone") || attrs_config.cloned;

    if is_cloned {
        return quote::quote! {
            #[doc = #doc_get]
            pub fn #field_name(self) -> #field_ty {
                self.#field_name
            }
        };
    }

    // If string, we can use as_deref
    if is_string_field(field_ty) {
        let has_vec = has_inner_type_with_x(field_ty, "Vec");

        if has_vec {
            quote::quote! {
                #[doc = #doc_get]
                pub fn #field_name(&self) -> &[String] {
                    &self.#field_name
                }
            }
        } else {
            quote::quote! {
                #[doc = #doc_get]
                pub fn #field_name(&self) -> &str {
                    &self.#field_name
                }
            }
        }
    } else {
        let is_copyable = field_has_ident(field, "copyable") || is_copy_able_field(field_ty);

        let get_field = if is_copyable || attrs_config.unref {
            quote::quote! {
                #[doc = #doc_get]
                pub fn #field_name(&self) -> #field_ty {
                    self.#field_name
                }
            }
        } else if let Some(inner_ty) = get_inner_type_of_vec(field_ty) {
            quote::quote! {
                #[doc = #doc_get]
                pub fn #field_name(&self) -> &[#inner_ty] {
                    &self.#field_name
                }
            }
        } else {
            quote::quote! {
                #[doc = #doc_get]
                pub fn #field_name(&self) -> &#field_ty {
                    &self.#field_name
                }
            }
        };

        let getter = quote::quote! {
            #get_field
        };

        getter
    }
}

fn get_inner_type_of_x<'a>(ty: &'a syn::Type, x: &'a str) -> Option<&'a syn::Type> {
    if let syn::Type::Path(type_path) = ty {
        // Check if it's a path type, and the first segment of the path is "x"
        for segment in &type_path.path.segments {
            // If we found "x", ensure that the argument is "AngleBracketed"
            if segment.ident == x {
                if let syn::PathArguments::AngleBracketed(angle_bracketed) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_type)) =
                        angle_bracketed.args.first()
                    {
                        return Some(inner_type);
                    }
                }
            }
        }
    }
    None
}

fn get_inner_type_of_option(ty: &syn::Type) -> Option<&syn::Type> {
    get_inner_type_of_x(ty, "Option")
}

fn get_inner_type_of_vec(ty: &syn::Type) -> Option<&syn::Type> {
    get_inner_type_of_x(ty, "Vec")
}

fn has_inner_type_with_x(ty: &syn::Type, x: &str) -> bool {
    if let syn::Type::Path(type_path) = ty {
        // Check if it's a path type, and the first segment of the path is "x"
        for segment in &type_path.path.segments {
            // If we found "x", ensure that the argument is "AngleBracketed"
            if segment.ident == x {
                if let syn::PathArguments::AngleBracketed(angle_bracketed) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(_)) = angle_bracketed.args.first() {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn field_has_ident(field: &syn::Field, ident: &str) -> bool {
    field.attrs.iter().any(|attr| attr.path().is_ident(ident))
}

fn is_copy_able_field(ty: &syn::Type) -> bool {
    let ty_str = ty.clone().into_token_stream().to_string();
    KNOWN_COPYABLE_FIELD.contains(&ty_str.as_str())
}

fn is_string_field(ty: &syn::Type) -> bool {
    // If Path, get the last segment and check if the Ident is "String"
    if let syn::Type::Path(type_path) = ty {
        if let Some(last_segment) = type_path.path.segments.last() {
            last_segment.ident == "String"
        } else {
            false
        }
    } else {
        false
    }
}

/// Generate or get field comment
///
/// If `option_mode` use the "if it exists" comment
fn make_field_comment(field: &syn::Field, option_mode: bool) -> String {
    // Check if field has doc-comment
    let field_comment: Vec<String> = field
        .attrs
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("doc") {
                if let syn::Meta::NameValue(name_val) = &attr.meta {
                    if let syn::Expr::Lit(doc_lit) = &name_val.value {
                        if let syn::Lit::Str(doc_str) = &doc_lit.lit {
                            let doc_val = doc_str.value();

                            let doc_val_fix = if doc_val.trim() == "" {
                                "\n\n".to_string()
                            } else {
                                doc_val
                            };

                            return Some(doc_val_fix);
                        }
                    }
                }
            }
            None
        })
        .collect();

    let ident = field.ident.as_ref().unwrap();
    let joined_cmt = field_comment.join("").trim().to_string();

    if joined_cmt.is_empty() {
        let if_it_exists = if option_mode { " if it exists" } else { "" };

        format!("Get the value of `{}`{}", ident, if_it_exists)
    } else {
        joined_cmt
    }
}
