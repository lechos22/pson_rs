use std::{collections::{HashMap, hash_map::DefaultHasher}, hash::{Hasher, Hash}};

use proc_macro::{TokenStream, TokenTree, Delimiter};

struct Pairs<T>{
    iter: Box<dyn Iterator<Item = T>>
}

impl Iterator<> for Pairs<TokenTree>{
    type Item = (TokenTree, TokenTree);
    fn next(&mut self) -> Option<Self::Item> {
        match (self.iter.next(), self.iter.next()) {
            (Some(a), Some(b)) => Some((a, b)),
            (Some(_), None) => panic!("Unable to pair an odd number of tokens"),
            _ => None,
        }
    }
}

struct PsonDef{
    name: String,
    body: Option<String>,
    children: Option<Vec<PsonDef>>,
}

impl PsonDef{
    fn primitive(name: String) -> PsonDef {
        PsonDef{
            name,
            body: None,
            children: None
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
    PsonDef::primitive(match name{
        "string" => "String".to_string(),
        "unsgn" => "u64".to_string(),
        "int" => "i64".to_string(),
        "float" => "f64".to_string(),
        "bool" => "bool".to_string(),
        "null" => "()".to_string(),
        tag if(tag.starts_with("_")) => name[1..].to_string(),
        _ => panic!("Unrecognized primitive type: {}", name)
    })
}

fn pson_struct_tuple(token_trees: Vec<TokenTree>) -> PsonDef {
    if token_trees.len() != 2 {
        panic!("Struct definition tuple must have exactly two elements")
    }
    let kind = match &token_trees[0] {
        TokenTree::Ident(kind) => kind.to_string(),
        _ => panic!("Struct type must be an identifier")
    };
    let kind_str = kind.as_str();
    fn single_typed(kind: &str, token_tree: TokenTree) -> PsonDef {
        let body = parse_pson_schema(token_tree);
        let mut hasher = DefaultHasher::new();
        kind.hash(&mut hasher);
        body.name.hash(&mut hasher);
        let name = format!("Pson{}", hasher.finish());
        PsonDef{
            name: name.clone(),
            body: Some(format!("type {} = Vec<{}>;", name.clone(), body.name)),
            children: Some(vec![body])
        }
    }
    fn map(kind: &str, pairs: Pairs<TokenTree>) -> PsonDef {
        let mut hasher = DefaultHasher::new();
        "map".hash(&mut hasher);
        kind.hash(&mut hasher);
        let mut children_names: Vec<String> = vec![];
        let mut children: Vec<PsonDef> = vec![];
        // todo: change to reduce that builds the body string
        pairs.for_each(|(key, value)|{
            key.to_string().hash(&mut hasher);
            let value = parse_pson_schema(value);
            value.name.hash(&mut hasher);
            children_names.push(key.to_string());
            children.push(value);
        });
        let name = format!("Pson{}", hasher.finish());
        let body = format!("struct {} {{{}}}", name, children_names.into_iter().zip((&children).into_iter())
            .map(|(name, child)|format!("{}: {}", name.to_string(), child.name))
            .collect::<Vec<_>>()
            .join(",")
        );
        PsonDef{
            name: name.clone(),
            body: Some(body),
            children: Some(children)
        }
    }
    match kind_str {
        "map" => {
            let pairs = match &token_trees[1] {
                TokenTree::Group(group) => {
                    if group.delimiter() != Delimiter::Parenthesis {
                        panic!("Map body must be a parenthesized list of key-value pairs")
                    }
                    Pairs{iter: Box::from(group.stream().into_iter())}
                },
                _ => panic!("Map body must be a parenthesized list of key-value pairs")
            };
            map(kind_str, pairs)
        },
        "array" | "option" => single_typed(kind_str, token_trees[1].clone()),
        "tuple" => todo!(),
        _ => panic!("Struct type must be one of map, array, option or tuple")
    }
}

fn parse_pson_schema(input: TokenTree) -> PsonDef {
    match input {
        TokenTree::Group(pson_object) => {
            match pson_object.delimiter() {
                Delimiter::Bracket => pson_struct_tuple(
                    pson_object
                        .stream()
                        .into_iter()
                        .collect::<Vec<_>>()
                ),
                _ => panic!("Unrecognized delimiter: {:?}", pson_object.delimiter())
            }
        },
        TokenTree::Ident(name) => pson_primitive(name.to_string().as_str()),
        _ => panic!("Unrecognized token: {}", input),
    }
}

#[proc_macro]
pub fn pson_schemas(input: TokenStream) -> TokenStream {
    let iter = Pairs{iter: Box::new(input.into_iter())};

    let schema: HashMap<String, PsonDef> = iter
        .map(|(key, value)|{
            match key {
                TokenTree::Ident(name) => (name.to_string(), value),
                _ => panic!("Key must be an identifier")
            }
        })
        .map(|(name, value)|{
            (name, parse_pson_schema(value))
        })
        .collect();

    let iter_base = schema
        .iter()
        .map(|(name, object)|{
            format!("type {} = {};", name, object.name)
        });
    let iter_bodies = schema
        .iter()
        .flat_map(|(_, object)|{
            object.to_flat_iter()
        });
    iter_base
        .chain(iter_bodies)
        .map(|s| {println!("{}", s); s}) // TODO: Remove this
        .collect::<String>()
        .parse()
        .unwrap()
}
