use std::collections::HashMap;

use proc_macro::{TokenStream, TokenTree};

struct Pairs<T>{
    iter: Box<dyn Iterator<Item = T>>
}

impl Iterator<> for Pairs<TokenTree>{
    type Item = (TokenTree, TokenTree);
    fn next(&mut self) -> Option<Self::Item> {
        match (self.iter.next(), self.iter.next()) {
            (Some(a), Some(b)) => Some((a, b)),
            (Some(_), None) => panic!(),
            _ => None,
        }
    }
}

struct PsonDef{
    name: String,
    body: Option<String>,
}

impl PsonDef{
    fn primitive(name: String) -> PsonDef {
        PsonDef{
            name,
            body: None
        }
    }
}

#[proc_macro]
pub fn pson_schema(input: TokenStream) -> TokenStream {
    let iter = Pairs{iter: Box::new(input.into_iter())}; 
    let mut lookup: HashMap<String, String> = HashMap::new(); // this lookup table is going to be used to map the name of the struct to its body
    let schema: HashMap<String, PsonDef> = iter.map(|(key, value)|{
        if let TokenTree::Ident(name) = key {
            (name, value)
        } else {
            panic!("Key must be an identifier");
        }
    }).map(|(name, value)|{
        match value {
            TokenTree::Group(_pson_object) => {
                todo!("Implement object parsing")
            },
            TokenTree::Ident(pson_primitive) => {
                match pson_primitive.to_string().as_str() {
                    "string" => {
                        (name.to_string(), PsonDef::primitive("String".to_string()))
                    }
                    "unsgn" => {
                        (name.to_string(), PsonDef::primitive("u64".to_string()))
                    }
                    "int" => {
                        (name.to_string(), PsonDef::primitive("i64".to_string()))
                    }
                    "float" => {
                        (name.to_string(), PsonDef::primitive("f64".to_string()))
                    }
                    "bool" => {
                        (name.to_string(), PsonDef::primitive("bool".to_string()))
                    }
                    _ => {
                        panic!("Unknown primitive type");
                    }
                }
            },
            _ => panic!("Unrecognized token"),
        }
    }).collect();
    return schema
        .iter()
        .map(|(name, object)|{
            format!("type {} = {};", name, object.name)
        })
        .collect::<String>()
        .parse()
        .unwrap();
}
