use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use syn::{
    AngleBracketedGenericArguments, DataStruct, Field, Fields, GenericArgument, Path,
    PathArguments, PathSegment, TypePath,
};

use crate::ics::find_attribute_with_key;

pub fn derive_utils_for_struct(DataStruct { fields, .. }: DataStruct) -> TokenStream {
    let Fields::Named(fields) = fields else {
        unreachable!()
    };

    let utils = fields.named.into_iter().map(|f| {
        let Field {
            ident, ty, attrs, ..
        } = f;
        let Some(ident) = ident else {
            todo!("currently unnamed structs are not supported")
        };

        if find_attribute_with_key(&attrs, "skip").is_some() {
            return quote! {};
        }

        let with_setter = format_ident!("with_{}", ident);
        let set_setter = format_ident!("set_{}", ident);
        let getter = format_ident!("get_{}", ident);

        match &ty {
            syn::Type::Path(TypePath {
                path: Path { segments, .. },
                ..
            }) => {
                let PathSegment {
                    ident: type_name,
                    arguments,
                } = segments.last().unwrap();

                let ident_name = type_name.to_string();

                if ident_name.as_str() == "Option" {
                    let PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                        args, ..
                    }) = arguments
                    else {
                        todo!()
                    };

                    let Some(GenericArgument::Type(inner_ty)) = args.first() else {
                        todo!()
                    };

                    quote! {
                        pub fn #with_setter(mut self, arg: impl Into<#inner_ty>) -> Self {
                            self.#ident = Some(arg.into());
                            self
                        }

                        pub fn #set_setter(&mut self, arg: impl Into<#inner_ty>) {
                            self.#ident = Some(arg.into());
                        }

                        pub fn #getter(&self) -> #ty {
                            self.#ident.clone()
                        }
                    }
                } else {
                    quote! {
                        pub fn #with_setter(mut self, arg: impl Into<#ty>) -> Self {
                            self.#ident = arg.into();
                            self
                        }

                        pub fn #set_setter(&mut self, arg: impl Into<#ty>) {
                            self.#ident = arg.into();
                        }

                        pub fn #getter(&self) -> #ty {
                            self.#ident.clone()
                        }
                    }
                }
            }
            other => {
                todo!("this type is currently not supported ({other:?})")
            }
        }
    });

    quote! { #(#utils)* }
}
