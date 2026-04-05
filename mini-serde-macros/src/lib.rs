use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Serialize)]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let serialize_impl = match &input.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(fields) => {
                    let field_names = fields.named.iter().map(|f| &f.ident);
                    let field_names_str = field_names.clone().map(|f| f.as_ref().unwrap().to_string());
                    quote! {
                        impl mini_serde::ToJSONString for #name {
                            fn to_json_value(&self) -> mini_serde::Json {
                                let mut map = std::collections::HashMap::new();
                                #(map.insert(#field_names_str.to_string(), self.#field_names.to_json_value());)*
                                mini_serde::Json::Object(map)
                            }
                        }
                    }
                }
                _ => panic!("Only named fields are supported"),
            }
        }
        _ => panic!("Only structs are supported"),
    };

    serialize_impl.into()
}

#[proc_macro_derive(Deserialize)]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let deserialize_impl = match &input.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(fields) => {
                    let field_names = fields.named.iter().map(|f| &f.ident);
                    let field_names_str = field_names.clone().map(|f| f.as_ref().unwrap().to_string());
                    let field_types = fields.named.iter().map(|f| &f.ty);
                    quote! {
                        impl mini_serde::FromString for #name {
                            fn from_json_value(v: mini_serde::Json) -> Result<Self, String> {
                                if let mini_serde::Json::Object(map) = v {
                                    Ok(#name {
                                        #(#field_names: {
                                            let key = #field_names_str;
                                            let value = map.get(key).ok_or_else(|| format!("missing field: {}", key))?.clone();
                                            <#field_types as mini_serde::FromString>::from_json_value(value)?
                                        },)*
                                    })
                                } else {
                                    Err("expected object".to_string())
                                }
                            }
                        }
                    }
                }
                _ => panic!("Only named fields are supported"),
            }
        }
        _ => panic!("Only structs are supported"),
    };

    deserialize_impl.into()
}
