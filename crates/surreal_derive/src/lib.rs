use core::panic;

use quote::{format_ident, quote};
use syn::{DataStruct, DeriveInput};

#[proc_macro_attribute]
pub fn repository(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let DeriveInput { ident, data, .. } = syn::parse(input.clone()).expect("not a derive input");

    let syn::Data::Struct(DataStruct { fields, .. }) = data else {
        panic!("#[derive(Repository)] is only supported for structs")
    };

    let mut field_declarations = vec![];

    fields.iter().for_each(|field| {
        field_declarations.push(quote! {
            #field,
        });
    });

    let create_payload_ident = format_ident!("Create{ident}Payload");

    let gen = quote! {
        #[derive(Default, Clone, Debug, Hash, Serialize, Deserialize)]
        pub struct #ident {
            pub id: Option<surrealdb::sql::Thing>,
            #(#field_declarations)*
        }

        #[derive(Debug, Default, Clone, Hash, PartialEq, Serialize, Deserialize)]
        pub struct #create_payload_ident {
            #(#field_declarations)*
        }
    };

    gen.into()
}
