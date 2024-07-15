use quote::{format_ident, quote};
use syn::{DataStruct, DeriveInput, Field};

#[proc_macro_derive(Repository)]
pub fn derive_repository(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput { ident, data, .. } = syn::parse(input).unwrap();

    let syn::Data::Struct(DataStruct { fields, .. }) = data else {
        panic!("#[derive(Repository)] is only supported for structs")
    };

    let mut field_names = vec![];

    let mut field_declarations = vec![];

    fields.iter().for_each(|Field { ident, ty, .. }| {
        if let Some(ident) = ident {
            if ident.to_string().as_str() == "id" {
                return;
            }
        }

        field_names.push(quote! {
            #ident,
        });

        field_declarations.push(quote! {
            pub #ident: #ty,
        });
    });

    let create_payload_ident = format_ident!("Create{ident}Payload");

    let gen = quote! {
        #[derive(Debug, Default, Clone, Hash, PartialEq, Serialize, Deserialize)]
        pub struct #create_payload_ident {
            #(#field_declarations)*
        }
    };

    gen.into()
}
