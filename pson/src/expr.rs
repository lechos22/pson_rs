use std::{collections::HashMap, error::Error};

#[derive(Debug)]
pub enum Expr {
    Null(),
    Boolean(bool),
    Integer(i128),
    Float(f64),
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
        } else if let Ok(n) = s.parse::<i128>() {
            Ok(Expr::Integer(n))
        } else if let Ok(n) = s.parse::<f64>() {
            Ok(Expr::Float(n))
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
            (Expr::Integer(a), Expr::Integer(b)) => a == b,
            (Expr::Float(a), Expr::Float(b)) => a == b,
            (Expr::Integer(a), Expr::Float(b)) => *a as f64 == *b,
            (Expr::Float(a), Expr::Integer(b)) => *a == *b as f64,
            (Expr::String(a), Expr::String(b)) => a == b,
            (Expr::Array(a), Expr::Array(b)) => a == b,
            (Expr::Map(a), Expr::Map(b)) => a == b,
            _ => false,
        }
    }
}

impl Clone for Expr {
    fn clone(&self) -> Self {
        match self {
            Expr::Null() => Expr::Null(),
            Expr::Boolean(b) => Expr::Boolean(*b),
            Expr::Integer(n) => Expr::Integer(*n),
            Expr::Float(n) => Expr::Float(*n),
            Expr::String(s) => Expr::String(s.to_string()),
            Expr::Array(a) => Expr::Array(a.clone()),
            Expr::Map(m) => Expr::Map(m.clone()),
        }
    }
}
