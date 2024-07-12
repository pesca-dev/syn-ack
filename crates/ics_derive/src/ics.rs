use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use syn::{
    Attribute, DataEnum, DataStruct, Expr, ExprLit, Field, Fields, Lit, Meta, MetaNameValue, Path,
    PathSegment, TypePath, TypeTuple, Variant,
};

pub fn derive_ics_for_struct(DataStruct { fields, .. }: DataStruct) -> TokenStream {
    let Fields::Named(fields) = fields else {
        unreachable!()
    };
    let to_strings = fields.named.into_iter().map(|f| {
        let Field {
            ident, attrs, ty, ..
        } = f;
        let Some(ident) = ident else {
            todo!("currently unnamed structs are not supported")
        };
        let mut display = ident.to_string().to_uppercase();
        if find_attribute_with_key(&attrs, "skip").is_some() {
            return quote! {};
        }

        if let Some(attr) = find_attribute_with_key(&attrs, "key") {
            if let Some(literal) = get_key_literal_from_namevalue(attr) {
                display = literal;
            };
        }

        let format_string = format!("{display}:{{}}\n");

        match &ty {
            syn::Type::Path(TypePath {
                path: Path { segments, .. },
                ..
            }) => {
                let PathSegment {
                    ident: type_name, ..
                } = segments.last().unwrap();

                let ident_name = type_name.to_string();

                match ident_name.as_str() {
                    "Vec" => {
                        quote! {
                            for inner in &self.#ident {
                                f.write_fmt(format_args!("{}\n", inner.to_string()))?;
                            }
                        }
                    }
                    "Option" => {
                        quote! {
                            if let Some(inner) = &self.#ident {
                                f.write_fmt(format_args!(#format_string, inner.to_string()))?;
                            }
                        }
                    }
                    _ => {
                        quote! {
                            f.write_fmt(format_args!(#format_string, self.#ident.to_string()))?;
                        }
                    }
                }
            }
            syn::Type::Tuple(TypeTuple { .. }) => {
                quote! {}
            }
            _ => {
                todo!("this type is currently not supported")
            }
        }
    });

    quote! { #(#to_strings)* }
}

pub fn derive_ics_for_enum(DataEnum { variants, .. }: DataEnum) -> TokenStream {
    let to_strings = variants.into_iter().map(|Variant { ident, fields, .. }| {
        if !fields.is_empty() {
            quote! {
                Self::#ident(inner) => f.write_fmt(format_args!("{}", inner.to_string()))?,
            }
        } else {
            quote! {
                Self::#ident => {},
            }
        }
    });

    quote! {
        match self {
            #(#to_strings)*
        }
    }
}

pub fn get_key_literal_from_namevalue(attr: &Attribute) -> Option<String> {
    if let Meta::NameValue(MetaNameValue {
        value: Expr::Lit(ExprLit {
            lit: Lit::Str(literal),
            ..
        }),
        ..
    }) = &attr.meta
    {
        return Some(
            literal
                .to_token_stream()
                .to_string()
                .trim_matches('"')
                .to_string(),
        );
    }
    None
}

pub fn find_attribute_with_key<'a>(attrs: &'a [Attribute], key: &str) -> Option<&'a Attribute> {
    attrs
        .iter()
        .find(|attr| attr.path().get_ident().map(|attr_ident| *attr_ident == key) == Some(true))
}
