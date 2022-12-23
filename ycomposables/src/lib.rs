use proc_macro::TokenStream;
use syn::parse::Parser;
use syn::{parse, parse_macro_input, ItemStruct};
use quote::quote;

mod size;

use size::Size;

#[proc_macro_attribute]
pub fn size_props(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let _ = parse_macro_input!(args as parse::Nothing);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub size: Size })
                .unwrap(),
        );
    }

    return quote! {
        #item_struct
    }.into();
}