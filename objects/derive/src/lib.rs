use proc_macro::{TokenStream, Span};
use quote::quote;
use syn::{DeriveInput, Ident};

#[proc_macro_derive(Flag)]
pub fn derive_flag(item: TokenStream) -> TokenStream {
    let parsed_item: DeriveInput = syn::parse(item).unwrap();
    let enum_name = parsed_item.ident;
    let enum_name_const = Ident::new(&(enum_name.to_string() + "CONST"), enum_name.span());
    let attr_count = parsed_item.attrs.len();
    let gen = quote! {
        impl Flag for #enum_name {
            fn into_usize(&self) -> usize {
                *self as usize
            }
        }

        const #enum_name_const: usize = 0;
    };
    gen.into()
}
