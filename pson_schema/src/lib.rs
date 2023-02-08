use std::{hash::{Hash, Hasher}, collections::HashMap, error::Error, sync::Mutex};

use proc_macro::{TokenStream, TokenTree};

#[proc_macro]
pub fn pson_schema(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();
    let mut schema: HashMap<String, Box<dyn ToString>> = HashMap::new();
    let mut lookup: HashMap<String, String> = HashMap::new(); // this lookup table is going to be used to map the name of the struct to its body
    while let (Some(key), value) = (iter.next(), iter.next()) {
        if let TokenTree::Ident(name) = key {
            if schema.contains_key(&name.to_string()) {
                panic!("Duplicate key");
            }
            match value {
                Some(TokenTree::Group(pson_object)) => {
                    todo!("Implement object parsing")
                },
                Some(TokenTree::Ident(pson_primitive)) => {
                    match pson_primitive.to_string().as_str() {
                        "string" => {
                            schema.insert(name.to_string(), Box::new("String"));
                        }
                        "unsgn" => {
                            schema.insert(name.to_string(), Box::new("u64"));
                        }
                        "int" => {
                            schema.insert(name.to_string(), Box::new("i64"));
                        }
                        "float" => {
                            schema.insert(name.to_string(), Box::new("f64"));
                        }
                        "bool" => {
                            schema.insert(name.to_string(), Box::new("bool"));
                        }
                        _ => {
                            panic!("Unknown primitive type");
                        }
                    }
                },
                None => panic!("Did not find a type for key {}", name.to_string()),
                _ => panic!("Unrecognized token"),
            }
        } else {
            panic!("Key must be an identifier");
        }
    }
    return schema
        .iter()
        .map(|(name, object)|{
            format!("type {} = {};", name, object.to_string())
        })
        .collect::<String>()
        .parse()
        .unwrap();
}
