use std::{collections::HashMap, error::Error};

#[derive(Debug)]
pub enum Expr {
    Null(),
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Expr>),
    Map(HashMap<String, Expr>),
}

impl Expr {
    pub fn from(s: &String) -> Result<Self, Box<dyn Error>> {
        if s == "N" {
            Ok(Expr::Null())
        } else if s == "T" {
            Ok(Expr::Boolean(true))
        } else if s == "F" {
            Ok(Expr::Boolean(false))
        } else if let Ok(n) = s.parse::<f64>() {
            Ok(Expr::Number(n))
        } else {
            Ok(Expr::String(s.to_string()))
        }
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Expr::Null(), Expr::Null()) => true,
            (Expr::Boolean(a), Expr::Boolean(b)) => a == b,
            (Expr::Number(a), Expr::Number(b)) => a == b,
            (Expr::String(a), Expr::String(b)) => a == b,
            (Expr::Array(a), Expr::Array(b)) => a == b,
            (Expr::Map(a), Expr::Map(b)) => a == b,
            _ => false,
        }
    }
}
