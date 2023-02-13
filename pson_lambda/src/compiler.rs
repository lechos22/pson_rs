use std::{error::Error, collections::HashSet};

use pson::{PsonParser, Expr};

fn scan_for_constants(ast: Vec<Expr>) -> HashSet<String>{
    ast
        .iter()
        .filter_map(Expr::as_array)
        .filter(|arr| arr.len() == 3)
        .filter_map(|arr|
            match &arr[0].as_string().map(|n| n == "$"){
                Some(true) => Some(arr),
                _ => None,
            }
        )
        .filter_map(|arr|
            match &arr[1].as_string(){
                Some(name) => Some(name.clone()),
                None => None,
            }
        )
        .collect()
}

pub(crate) fn compile_module(code: &str) -> Result<(), Box<dyn Error>> {
    let mut parser = PsonParser::new(code.chars());
    parser.parse()?;
    let ast = parser.get()?;
    println!("{}", ast.to_string());
    // search for global constants(functions included) defined as [$ name value]
    let constants = scan_for_constants((&ast).as_array().unwrap());
    println!("module constants: {:?}", constants);
    Ok(())
}
