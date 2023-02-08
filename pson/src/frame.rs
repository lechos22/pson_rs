use std::{error::Error, collections::HashMap};

use crate::expr::Expr;

#[derive(Debug)]
pub(crate) enum FrameKind {
    Array,
    Map,
}

#[derive(Debug)]
pub(crate) struct Frame {
    exprs: Vec<Expr>,
    pub(crate) kind: FrameKind
}

impl Frame {
    pub(crate) fn new(kind: FrameKind) -> Self {
        Self{
            exprs: Vec::new(),
            kind
        }
    }
    pub(crate) fn push(self: &mut Frame, expr: Expr) {
        self.exprs.push(expr);
    }
    pub(crate) fn to_array(self: Frame) -> Result<Expr, Box<dyn Error>> {
        Ok(Expr::Array(self.exprs))
    }
    pub(crate) fn to_map(self: &mut Frame) -> Result<Expr, Box<dyn Error>> {
        let mut map: HashMap<String, Expr> = HashMap::new();
        while let Some(value) = self.exprs.pop() {
            if let Some(key) = self.exprs.pop() {
                let key_str = match key {
                    Expr::String(s) => s,
                    _ => Err("invalid map")?,
                };
                map.insert(key_str, value);
            } else {
                Err("invalid map")?;
            }
        }
        Ok(Expr::Map(map))
    }
}
