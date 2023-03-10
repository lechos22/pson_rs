use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

use proc_macro::{Delimiter, TokenStream, TokenTree};

struct Pairs<T> {
    iter: Box<dyn Iterator<Item = T>>,
}

impl Iterator for Pairs<TokenTree> {
    type Item = (TokenTree, TokenTree);
    fn next(&mut self) -> Option<Self::Item> {
        match (self.iter.next(), self.iter.next()) {
            (Some(a), Some(b)) => Some((a, b)),
            (Some(_), None) => panic!("Unable to pair an odd number of tokens"),
            _ => None,
        }
    }
}

struct PsonDef {
    name: String,
    body: Option<String>,
    children: Option<Vec<PsonDef>>,
}

impl PsonDef {
    fn primitive(name: String) -> PsonDef {
        PsonDef {
            name,
            body: None,
            children: None,
        }
    }
    fn to_flat_iter(&self) -> Vec<String> {
        let mut iter = vec![];
        if let Some(body) = &self.body {
            iter.push(body.clone());
        }
        if let Some(children) = &self.children {
            for child in children {
                iter.extend(child.to_flat_iter());
            }
        }
        iter
    }
}

fn pson_primitive(name: &str) -> PsonDef {
    PsonDef::primitive(match name {
        "string" => "String".to_string(),
        "unsgn" => "u64".to_string(),
        "int" => "i64".to_string(),
        "float" => "f64".to_string(),
        "bool" => "bool".to_string(),
        "null" => "()".to_string(),
        tag if (tag.starts_with("_")) => name[1..].to_string(),
        _ => panic!("Unrecognized primitive type: {}", name),
    })
}

fn pson_struct_tuple(token_trees: Vec<TokenTree>) -> PsonDef {
    if token_trees.len() != 2 {
        panic!("Struct definition tuple must have exactly two elements")
    }
    let kind = match &token_trees[0] {
        TokenTree::Ident(kind) => kind.to_string(),
        _ => panic!("Struct type must be an identifier"),
    };
    let kind_str = kind.as_str();
    fn single_typed(kind: &str, token_tree: TokenTree) -> PsonDef {
        let body = parse_pson_schema(token_tree);
        let mut hasher = DefaultHasher::new();
        kind.hash(&mut hasher);
        body.name.hash(&mut hasher);
        let name = format!("Pson{}", hasher.finish());
        let base_type = match kind {
            "array" => "Vec",
            "option" => "Option",
            _ => panic!()
        };
        PsonDef {
            name: name.clone(),
            body: Some(format!("type {}={}<{}>;", name, base_type, body.name)),
            children: Some(vec![body]),
        }
    }
    fn map(pairs: Pairs<TokenTree>) -> PsonDef {
        let mut hasher = DefaultHasher::new();
        "map".hash(&mut hasher);
        let mut children = Vec::<PsonDef>::new();
        let body_inner = pairs
            .map(|(key, value)| {
                key.to_string().hash(&mut hasher);
                let value = parse_pson_schema(value);
                value.name.hash(&mut hasher);
                let code = format!("{}:{}", key.to_string(), value.name);
                (value, code)
            })
            .fold(
                String::new(),
                |body_builder, (child, code)| {
                    children.push(child);
                    body_builder + &code + ","
                }, // this is still bad, I should find another way to do this
            );
        let name = format!("Pson{}", hasher.finish());
        let body = format!("struct {}{{{}}}", name.clone(), body_inner);
        PsonDef {
            name: name.clone(),
            body: Some(body),
            children: Some(children),
        }
    }
    fn tuple(iter: impl Iterator<Item = TokenTree>) -> PsonDef {
        let mut hasher = DefaultHasher::new();
        "tuple".hash(&mut hasher);
        let mut children = Vec::<PsonDef>::new();
        let body_inner = iter
            .map(|value| {
                let value = parse_pson_schema(value);
                value.name.hash(&mut hasher);
                let name = value.name.clone();
                (value, name)
            })
            .fold(
                String::new(),
                |body_builder, (child, code)| {
                    children.push(child);
                    body_builder + &code + ","
                }, // this is still bad, I should find another way to do this
            );
        let name = format!("Pson{}", hasher.finish());
        let body = format!("type {}=({});", name.clone(), body_inner);
        PsonDef {
            name: name.clone(),
            body: Some(body),
            children: Some(children),
        }
    }
    match kind_str {
        "map" => {
            let pairs = match &token_trees[1] {
                TokenTree::Group(group) => {
                    if group.delimiter() != Delimiter::Parenthesis {
                        panic!("Map body must be a parenthesized list of key-value pairs")
                    }
                    Pairs {
                        iter: Box::from(group.stream().into_iter()),
                    }
                }
                _ => panic!("Map body must be a parenthesized list of key-value pairs"),
            };
            map(pairs)
        }
        "array" | "option" => single_typed(kind_str, token_trees[1].clone()),
        "tuple" => {
            let iter = match &token_trees[1] {
                TokenTree::Group(group) => {
                    if group.delimiter() != Delimiter::Bracket {
                        panic!("Tuple body must be a parenthesized list of types in tuple")
                    }
                    Box::from(group.stream().into_iter())
                }
                _ => panic!("Map body must be a parenthesized list of types in tuple"),
            };
            tuple(iter)
        },
        _ => panic!("Struct type must be one of map, array, option or tuple"),
    }
}

fn parse_pson_schema(input: TokenTree) -> PsonDef {
    match input {
        TokenTree::Group(pson_object) => match pson_object.delimiter() {
            Delimiter::Bracket => {
                pson_struct_tuple(pson_object.stream().into_iter().collect::<Vec<_>>())
            }
            _ => panic!("Unrecognized delimiter: {:?}", pson_object.delimiter()),
        },
        TokenTree::Ident(name) => pson_primitive(name.to_string().as_str()),
        _ => panic!("Unrecognized token: {}", input),
    }
}

#[proc_macro]
pub fn pson_schemas(input: TokenStream) -> TokenStream {
    let iter = Pairs {
        iter: Box::new(input.into_iter()),
    };

    let schema: HashMap<String, PsonDef> = iter
        .map(|(key, value)| match key {
            TokenTree::Ident(name) => (name.to_string(), value),
            _ => panic!("Key must be an identifier"),
        })
        .map(|(name, value)| (name, parse_pson_schema(value)))
        .collect();

    let iter_base = schema
        .iter()
        .map(|(name, object)| format!("type {}={};", name, object.name));
    let iter_bodies = schema.iter().flat_map(|(_, object)| object.to_flat_iter());
    iter_base
        .chain(iter_bodies)
        .map(|s| {
            println!("{}", s);
            s
        }) // TODO: Remove this
        .collect::<String>()
        .parse()
        .unwrap()
}
