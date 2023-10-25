use proc_macro::TokenStream;
use quote::quote;
use syn::Data;

#[proc_macro_derive(Flag)]
pub fn flag_trait_derive(input: TokenStream) -> TokenStream {
    let parsed_input = syn::parse(input).unwrap();
    impl_flag(&parsed_input)
}

fn impl_flag(p_input: &syn::DeriveInput) -> TokenStream {
    let name = &p_input.ident;
    let d_enum = match &p_input.data {
        Data::Enum(d_enum) => d_enum,
        _ => panic!("Enum only derivative macro"),
    };
    let variants = d_enum.variants.iter().map(|v| v.ident.clone());
    let variants_len = variants.len();

    let gen = quote! {
        impl Flag for #name {
            const SIZE: usize = {#variants_len / 8 + 1};
            fn into_usize(&self) -> usize {
                *self as usize
            }
        }

        impl #name {
            pub fn debug_flags(flags: &Flags<{#variants_len / 8 + 1}>) {
                print!("{}:", stringify!(#name));
                #(
                    if flags.has_flag(Self::#variants) {
                        print!(" {}", stringify!(#variants));
                    }
                )*;
                print!("\n");
            }
        }
    };
    gen.into()
}
