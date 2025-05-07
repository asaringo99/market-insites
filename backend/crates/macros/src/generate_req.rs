use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Attribute, DeriveInput, LitStr,
};

fn lit(attrs:&[Attribute], name:&str, default:Option<&str>) -> String {
    attrs.iter()
        .find(|a| a.path().is_ident(name))
        .and_then(|a| a.parse_args::<LitStr>().ok())
        .map(|l| l.value())
        .or_else(|| default.map(|d| d.to_owned()))
        .expect(&format!("#[{name}(\"…\")] が必要"))
}

fn fixed(attrs:&[Attribute]) -> Vec<(String,String)> {
    let mut out = Vec::new();
    for a in attrs.iter().filter(|a| a.path().is_ident("fixed")) {
        a.parse_nested_meta(|meta| {
            let key = meta
                .path
                .get_ident()
                .ok_or_else(|| meta.error("key ident"))?
                .to_string();

            let lit: LitStr = meta.value()?.parse()?;
            out.push((key, lit.value()));
            Ok(())
        }).unwrap();
    }
    out
}


pub fn derive_generate_req(item: TokenStream) -> TokenStream {
    let input   = parse_macro_input!(item as DeriveInput);
    let wrapper = &input.ident;

    // 属性
    let endpoint  = lit(&input.attrs, "endpoint", None);
    let method    = lit(&input.attrs, "method", Some("get"));
    let is_get    = method.eq_ignore_ascii_case("get");
    let fixed_kv  = fixed(&input.attrs);
    let inner_ty: syn::Type = match &input.data {
        syn::Data::Struct(s) => match &s.fields {
            // ← tuple 構造体でフィールドが 1 個ならそれを inner とみなす
            syn::Fields::Unnamed(u) if u.unnamed.len() == 1 =>
                u.unnamed[0].ty.clone(),
            _ => {
                return quote!(
                    compile_error!("GenerateReq: struct must be `pub struct X(pub Inner);`");
                )
                .into()
            }
        },
        _ => {
            return quote!(
                compile_error!("GenerateReq can only be derived for tuple structs");
            )
            .into()
        }
    };

    // body(query)部分の取得
    let fixed_ins = fixed_kv.iter().map(|(k,v)| {
        quote!( map.insert(#k.into(), ::serde_json::json!(#v)); )
    });

    let call = if is_get {
        quote!(.query(&req))
    } else {
        quote!(.json(&req))
    };
    let method_ident = syn::Ident::new(&method, wrapper.span());

    let expanded = quote! {

        impl ::kernel::core::ReqSpec for #wrapper {}

        impl ::serde::Serialize for #wrapper {
            fn serialize<S>(&self, se:S)->Result<S::Ok,S::Error>
            where S: ::serde::Serializer {
                let mut map = ::serde_json::Map::new();
                #(#fixed_ins)*
                ::domain::valueobject::trade::FieldPairs::put_into(&self.0, &mut map);
                ::serde_json::Value::Object(map).serialize(se)
            }
        }

        impl From<#wrapper> for reqwest::RequestBuilder {
            fn from(req: #wrapper) -> Self {
                reqwest::Client::new()
                    .#method_ident(#endpoint)
                    #call
            }
        }
    };
    expanded.into()
}