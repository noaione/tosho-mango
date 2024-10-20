use syn::Attribute;

pub(crate) fn get_field_comment(attrs: &[Attribute]) -> Option<String> {
    // Check if field has doc-comment
    let field_comment: Vec<String> = attrs
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("doc") {
                if let syn::Meta::NameValue(name_val) = &attr.meta {
                    if let syn::Expr::Lit(doc_lit) = &name_val.value {
                        if let syn::Lit::Str(doc_str) = &doc_lit.lit {
                            let doc_val = doc_str.value();

                            let doc_val_fix = if doc_val.trim() == "" {
                                "\n".to_string()
                            } else {
                                format!("{}\n", doc_val.trim())
                            };

                            return Some(doc_val_fix);
                        }
                    }
                }
            }
            None
        })
        .collect();

    let joined_cmt = field_comment.join("").trim().to_string();

    if joined_cmt.is_empty() {
        None
    } else {
        Some(joined_cmt)
    }
}
