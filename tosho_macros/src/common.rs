use base64::Engine as _;
use syn::Attribute;

pub(crate) fn get_field_comment(attrs: &[Attribute]) -> Option<String> {
    // Check if field has doc-comment
    let field_comment: Vec<String> = attrs
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("doc") {
                if let syn::Meta::NameValue(name_val) = &attr.meta
                    && let syn::Expr::Lit(doc_lit) = &name_val.value
                    && let syn::Lit::Str(doc_str) = &doc_lit.lit
                {
                    let doc_val = doc_str.value();

                    let doc_val_fix = if doc_val.trim() == "" {
                        "\n".to_string()
                    } else {
                        format!("{}\n", doc_val.trim())
                    };

                    Some(doc_val_fix)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    let joined_cmt = field_comment.join("").trim().to_string();

    if joined_cmt.is_empty() {
        None
    } else {
        Some(joined_cmt)
    }
}

pub(crate) struct CompTimeBase64Input {
    // This should only have a string literal
    base64_str: syn::LitStr,
}

impl syn::parse::Parse for CompTimeBase64Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let base64_str: syn::LitStr = input.parse()?;

        Ok(CompTimeBase64Input { base64_str })
    }
}

pub(crate) fn impl_base64_decode(input: &CompTimeBase64Input) -> proc_macro::TokenStream {
    let base64_str = &input.base64_str;
    let base64_as_str = base64_str.value();
    let decoded_bytes = match base64::engine::general_purpose::STANDARD.decode(base64_as_str) {
        Ok(bytes) => bytes,
        Err(_) => {
            return proc_macro::TokenStream::from(
                syn::Error::new_spanned(base64_str, "Invalid base64 provided").to_compile_error(),
            );
        }
    };

    let decoded_string = match String::from_utf8(decoded_bytes) {
        Ok(string) => string,
        Err(_) => {
            return proc_macro::TokenStream::from(
                syn::Error::new_spanned(base64_str, "Base64 is not valid UTF-8").to_compile_error(),
            );
        }
    };

    let generate = quote::quote! {
        #decoded_string
    };
    generate.into()
}
