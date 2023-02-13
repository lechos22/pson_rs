use std::{collections::HashMap, error::Error, hash::Hash};

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
    pub fn as_null(&self) -> Option<()> {
        match self {
            Expr::Null() => Some(()),
            _ => None,
        }
    }
    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            Expr::Boolean(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_integer(&self) -> Option<i128> {
        match self {
            Expr::Integer(n) => Some(*n),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            Expr::Float(n) => Some(*n),
            _ => None,
        }
    }
    pub fn as_string(&self) -> Option<String> {
        match self {
            Expr::String(s) => Some(s.to_string()),
            _ => None,
        }
    }
    pub fn as_array(&self) -> Option<Vec<Expr>> {
        match self {
            Expr::Array(a) => Some(a.to_vec()),
            _ => None,
        }
    }
    pub fn as_map(&self) -> Option<HashMap<String, Expr>> {
        match self {
            Expr::Map(m) => Some(m.to_owned()),
            _ => None,
        }
    }
}

impl Hash for Expr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Expr::Null() => state.write_u8(0),
            Expr::Boolean(b) => {
                state.write_u8(1);
                state.write_u8(*b as u8);
            }
            Expr::Integer(n) => {
                state.write_u8(2);
                state.write_i128(*n);
            }
            Expr::Float(n) => {
                state.write_u8(3);
                state.write_u64(n.to_bits());
            }
            Expr::String(s) => {
                state.write_u8(4);
                state.write(s.as_bytes());
            }
            Expr::Array(a) => {
                state.write_u8(5);
                for e in a {
                    e.hash(state);
                }
            }
            Expr::Map(m) => {
                state.write_u8(6);
                for (k, v) in m {
                    k.hash(state);
                    v.hash(state);
                }
            }
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

impl ToString for Expr {
    fn to_string(&self) -> String {
        match self {
            Expr::Null() => "N".to_string(),
            Expr::Boolean(b) => match b {
                true => "T".to_string(),
                false => "F".to_string(),
            },
            Expr::Integer(n) => n.to_string(),
            Expr::Float(n) => n.to_string(),
            Expr::String(s) => s.clone(),
            Expr::Array(a) => format!(
                "[{}]",
                a.iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join(" "),
            ),
            Expr::Map(m) => format!(
                "({})",
                m.iter()
                .map(|(k, v)| format!("{} {}", k, v.to_string()))
                .collect::<Vec<String>>()
                .join(" ")
            ),
        }
    }
}
