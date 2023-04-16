use proc_macro::TokenStream;
use quote::quote;
use syn::{Field, FieldsUnnamed, ItemEnum, Variant};

#[proc_macro_derive(EnumWrap)]
pub fn enum_wrap(input: TokenStream) -> TokenStream {
    let enum_item = syn::parse_macro_input!(input as ItemEnum);

    let enum_ident = enum_item.ident;

    let mut impls = Vec::new();

    for Variant { ident, fields, .. } in enum_item.variants.into_iter() {
        let syn::Fields::Unnamed(FieldsUnnamed { unnamed, .. }) = fields else {
            panic!("support only unnamed variants");
        };

        let Field { ty, .. } = unnamed.first().unwrap().clone();

        impls.push(quote! {
            impl From<#ty> for #enum_ident {
                fn from(value: #ty) -> Self {
                    Self::#ident(value)
                }
            }
        });
    }

    TokenStream::from(quote! { #(#impls)* })
}

#[proc_macro_attribute]
pub fn display(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_clone = proc_macro2::TokenStream::from(input.clone());

    let attr = proc_macro2::TokenStream::from(attr);

    let struct_item = syn::parse_macro_input!(input as syn::ItemStruct);
    let struct_name = struct_item.ident;

    if let syn::Fields::Named(fields) = &struct_item.fields {
        let field_names = fields
            .named
            .iter()
            .map(|field| field.ident.clone().unwrap())
            .collect::<Vec<_>>();

        let result = quote! {
            #input_clone

            impl std::fmt::Display for #struct_name {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    let Self {
                        #(#field_names),*
                    } = self;

                    write!(f, #attr)
                }
            }
        };

        TokenStream::from(result)
    } else {
        panic!("display only supports Struct variants with named fields")
    }
}
