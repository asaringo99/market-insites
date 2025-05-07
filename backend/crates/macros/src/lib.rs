use field_paris::derive_fieldpairs;
use generate_req::derive_generate_req;
use proc_macro::TokenStream;

mod field_paris;
mod generate_req;

#[proc_macro_derive(GenerateReq, attributes(inner, endpoint, method, fixed))]
pub fn generate_req(input: TokenStream) -> TokenStream {
    derive_generate_req(input)
}

#[proc_macro_derive(FieldKeyValuePairs)]
pub fn field_paris(input: TokenStream) -> TokenStream {
    derive_fieldpairs(input)
}