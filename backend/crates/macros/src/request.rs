use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, DeriveInput,
};


pub fn derive_req_spec(item: TokenStream) -> TokenStream {
    let input  = parse_macro_input!(item as DeriveInput);
    let ident  = &input.ident;

    let expanded = quote! {
        impl ::kernel::core::ReqSpec for #ident {}
    };
    expanded.into()
}

pub fn derive_res_spec(item: TokenStream) -> TokenStream {
    let input  = parse_macro_input!(item as DeriveInput);
    let ident  = &input.ident;

    let expanded = quote! {
        impl ::kernel::core::ResSpec for #ident {}
    };
    expanded.into()
}