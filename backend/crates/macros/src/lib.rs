use field_paris::derive_fieldpairs;
use generate_req::derive_generate_req;
use request::{derive_req_spec, derive_res_spec};
use proc_macro::TokenStream;

mod field_paris;
mod generate_req;
mod request;

#[proc_macro_derive(GenerateReq, attributes(inner, endpoint, method, fixed))]
pub fn generate_req(input: TokenStream) -> TokenStream {
    derive_generate_req(input)
}

#[proc_macro_derive(FieldKeyValuePairs)]
pub fn field_paris(input: TokenStream) -> TokenStream {
    derive_fieldpairs(input)
}

#[proc_macro_derive(RequestSpec)]
pub fn req_spec(input: TokenStream) -> TokenStream {
    derive_req_spec(input)
}

#[proc_macro_derive(ResponseSpec)]
pub fn res_spec(input: TokenStream) -> TokenStream {
    derive_res_spec(input)
}