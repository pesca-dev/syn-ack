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

    let repository_ident = format_ident!("{ident}Repository");

    let gen = quote! {
        #[derive(Debug, Default, Clone, Hash, PartialEq)]
        struct #repository_ident {
            id: Option<surrealdb::sql::Thing>,
            #(#field_declarations)*
        }

        impl #repository_ident {
            pub fn id(&self) -> Option<String> {
                self.id.as_ref().map(|id| format!("{}:{}", id.tb, id.id))
            }
        }

        impl From<#repository_ident> for #ident{
            fn from(value: #repository_ident) -> #ident {
                let id = value.id().unwrap_or_default();
                let #repository_ident { #(#field_names)* .. } = value;

                #ident {
                    id,
                    #(#field_names)*
                }
            }
        }
    };

    gen.into()
}
