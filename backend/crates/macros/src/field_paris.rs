use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields};

pub fn derive_fieldpairs(item: TokenStream) -> TokenStream {
    let input  = parse_macro_input!(item as DeriveInput);
    let ident  = &input.ident;

    let fields = match input.data {
        syn::Data::Struct(s) => match s.fields {
            Fields::Named(n) => n.named,
            _ => return quote!(compile_error!("FieldKeyValuePairs: named struct only");).into(),
        },
        _ => return quote!(compile_error!("FieldKeyValuePairs: struct only");).into(),
    };

    let inserts = fields.iter().map(|f|{
        let id = &f.ident;
        let ty = &f.ty;
        quote! {
            map.insert(
                <#ty as ::domain::valueobject::ApiColumn>::NAME.into(),
                ::serde_json::json!( self.#id.to_string() )
            );
        }
    });

    quote! {
        impl FieldPairs for #ident {
            fn put_into(&self, map:&mut ::serde_json::Map<String, ::serde_json::Value>) {
                #(#inserts)*
            }
        }
    }.into()
}
