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

/// Derives an enum that would implement `.to_name()`
#[proc_macro_derive(EnumName)]
pub fn enumname_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_enumname_derive(&ast)
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
