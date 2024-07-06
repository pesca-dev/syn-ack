use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    Attribute, DataStruct, DeriveInput, Expr, ExprLit, Field, Fields, Lit, Meta, MetaNameValue,
    Path, PathSegment, TypePath,
};

#[proc_macro_derive(Ics, attributes(key, skip))]
pub fn derive_ics(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, data, attrs, ..
    } = syn::parse(input).unwrap();

    let mut begin = quote! {};
    let mut end = quote! {};

    if let Some(attr) = find_attribute_with_key(&attrs, "key") {
        if let Some(key) = get_key_literal_from_namevalue(attr) {
            let begin_string = format!("BEGIN:{key}");
            let end_string = format!("END:{key}");

            begin = quote! {
                f.write_fmt(format_args!("{}\n", #begin_string))?;
            };

            end = quote! {
                f.write_fmt(format_args!("{}\n", #end_string))?;
            };
        };
    }

    let syn::Data::Struct(DataStruct { fields, .. }) = data else {
        todo!()
    };

    let Fields::Named(fields) = fields else {
        unreachable!()
    };

    let to_strings = fields.named.iter().map(|f| {
        let Field {
            ident, attrs, ty, ..
        } = f;

        let Some(ident) = ident else { todo!() };

        let mut display = ident.to_string().to_uppercase();

        if find_attribute_with_key(attrs, "skip").is_some() {
            return quote! {};
        }

        if let Some(attr) = find_attribute_with_key(attrs, "key") {
            if let Some(literal) = get_key_literal_from_namevalue(attr) {
                display = literal;
            };
        }

        let format_string = format!("{display}:{{}}\n");

        match ty {
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
                                f.write_fmt(format_args!(#format_string, inner.to_string()))?;
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
            _ => todo!(),
        }
    });

    let gen = quote! {
        impl std::fmt::Display for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                #begin
                #(#to_strings)*
                #end
                Ok(())
            }
        }
    };

    gen.into()
}

fn get_key_literal_from_namevalue(attr: &Attribute) -> Option<String> {
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

fn find_attribute_with_key<'a>(attrs: &'a [Attribute], key: &str) -> Option<&'a Attribute> {
    attrs
        .iter()
        .find(|attr| attr.path().get_ident().map(|attr_ident| *attr_ident == key) == Some(true))
}
