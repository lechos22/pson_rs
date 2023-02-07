use std::{str::Chars, error::Error, collections::HashMap};

trait CharContainer {
    fn chars_iter(&self) -> Chars<'_>;
}

impl CharContainer for &str {
    fn chars_iter(&self) -> Chars<'_> {
        self.chars()
    }
}

impl CharContainer for String {
    fn chars_iter(&self) -> Chars<'_> {
        self.chars()
    }
}

#[allow(unused)]
#[derive(Debug)]
enum Expr {
    Number(f64),
    String(String),
    Array(Vec<Expr>),
    Map(HashMap<String, Expr>),
}

#[derive(Debug)]
enum FrameKind {
    Array,
    Map,
}

#[derive(Debug)]
struct Frame {
    exprs: Vec<Expr>,
    kind: FrameKind
}

impl Frame {
    fn new(kind: FrameKind) -> Self {
        Self{
            exprs: Vec::new(),
            kind
        }
    }
    fn push(self: &mut Frame, expr: Expr) {
        self.exprs.push(expr);
    }
    fn to_array(self: Frame) -> Result<Expr, Box<dyn Error>> {
        Ok(Expr::Array(self.exprs))
    }
    fn to_map(self: &mut Frame) -> Result<Expr, Box<dyn Error>> {
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

struct Scanner<'a> {
    frame_stack: Vec<Frame>,
    buf: String,
    it: Chars<'a>,
}

impl Scanner<'_> {
    fn new<'a>(text: &'a impl CharContainer) -> Scanner<'a> {
        Scanner {
            frame_stack: vec![Frame::new(FrameKind::Array)],
            buf: String::new(),
            it: text.chars_iter()
        }
    }
    fn process_buffer(&mut self) -> Result<(), Box<dyn Error>>{
        if !self.buf.is_empty() {
            let top = self.frame_stack.last_mut().ok_or("invalid pson")?;
            top.push(Expr::String(self.buf.clone()));
            self.buf.clear();
        }
        Ok(())
    }
    fn scan_quoted_string(&mut self) -> Result<(), Box<dyn Error>>{
        while let Some(c) = self.it.next() {
            if c == '"' {
                break;
            }
            self.buf.push(c);
        }
        self.process_buffer()?;
        Ok(())
    }
    fn close_frame(&mut self) -> Result<(), Box<dyn Error>> {
        let mut frame = self.frame_stack.pop().ok_or("invalid pson")?;
        let top = self.frame_stack.last_mut().ok_or("invalid pson")?;
        match frame.kind {
            FrameKind::Array => top.push(frame.to_array()?),
            FrameKind::Map => top.push(frame.to_map()?),
        }
        Ok(())
    }
    fn scan(&mut self) -> Result<(), Box<dyn Error>> {
        while let Some(c) = self.it.next() {
            match c {
                '[' => self.frame_stack.push(Frame::new(FrameKind::Array)),
                '(' => self.frame_stack.push(Frame::new(FrameKind::Map)),
                '<' => self.close_frame()?,
                ' ' | '\t' | '\n' | '\r' => self.process_buffer()?,
                '"' => self.scan_quoted_string()?,
                _ => self.buf.push(c)
            }
        };
        self.process_buffer()?;
        Ok(())
    }
    fn get(&mut self) -> Result<Expr, Box<dyn Error>> {
        if self.frame_stack.len() != 1 {
            Err("invalid pson")?;
        }
        let top = self.frame_stack.pop().ok_or("invalid pson")?;
        let expr = top.to_array()?;
        Ok(expr)
    }
}


fn main() {
    let text = r#"(a (b (c "789"<<<"#;
    let mut scanner = Scanner::new(&text);
    scanner.scan().unwrap();
    println!("{:?}", scanner.get().unwrap());
}
