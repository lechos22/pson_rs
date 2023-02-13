use std::{error::Error};

use pson::{PsonParser, Expr};

pub(crate) fn compile_module(code: &str) -> Result<(), Box<dyn Error>> {
    let mut parser = PsonParser::new(code.chars());
    parser.parse()?;
    let ast = parser.get()?;
    println!("{}", ast.to_string());
    Ok(())
}
