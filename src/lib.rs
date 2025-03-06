extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, DataEnum};
use convert_case::{Case, Casing};

#[proc_macro_derive(Textures)]
pub fn textures(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident; // The name of the enum
    let data_enum = if let Data::Enum(data_enum) = input.data {
        data_enum
    } else {
        // Not an enum, so panic or handle as necessary
        panic!("EnumTextureBytes only works with enums!");
    };

    let variants = data_enum.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let file_name = format!("/assets/textures/{}.png", variant_name.to_string().to_case(Case::Snake));
        quote! {
            #name::#variant_name => include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), #file_name)).to_vec(),
        }
    });

    let expanded = quote! {
        impl Into<u32> for #name {
            fn into(self) -> u32 {
                self as u32
            }
        }

        impl Textures for #name {
            fn bytes(&self) -> Vec<u8> {
                match self {
                    #( #variants )*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
