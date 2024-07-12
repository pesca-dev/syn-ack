mod ics;
mod utils;

use quote::quote;
use syn::DeriveInput;

use ics::{
    derive_ics_for_enum, derive_ics_for_struct, find_attribute_with_key,
    get_key_literal_from_namevalue,
};
use utils::derive_utils_for_struct;

#[proc_macro_derive(Utils, attributes(skip))]
pub fn derive_utils(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput { ident, data, .. } = syn::parse(input).unwrap();
    let utils = match data {
        syn::Data::Struct(data) => derive_utils_for_struct(data),
        syn::Data::Enum(_) => todo!("enums are currently not supported"),
        syn::Data::Union(_) => todo!("unions are currently not supported"),
    };

    let gen = quote! {
        impl #ident {
            #utils
        }
    };

    gen.into()
}

#[proc_macro_derive(Ics, attributes(key, skip))]
pub fn derive_ics(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
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

    let to_strings = match data {
        syn::Data::Struct(data) => derive_ics_for_struct(data),
        syn::Data::Enum(data) => derive_ics_for_enum(data),
        syn::Data::Union(_) => todo!("unions are currently not supported"),
    };

    let gen = quote! {
        impl std::fmt::Display for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                #begin
                #to_strings
                #end
                Ok(())
            }
        }
    };

    gen.into()
}
