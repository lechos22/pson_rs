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
    Null(),
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Expr>),
    Map(HashMap<String, Expr>),
}

impl Expr {
    fn from(s: &String) -> Result<Self, Box<dyn Error>> {
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

struct PsonScanner<'a> {
    frame_stack: Vec<Frame>,
    buf: String,
    it: Chars<'a>,
}

impl PsonScanner<'_> {
    fn new<'a>(text: &'a impl CharContainer) -> PsonScanner<'a> {
        PsonScanner {
            frame_stack: vec![Frame::new(FrameKind::Array)],
            buf: String::new(),
            it: text.chars_iter()
        }
    }
    fn process_buffer(&mut self) -> Result<(), Box<dyn Error>>{
        if !self.buf.is_empty() {
            let top = self.frame_stack.last_mut().ok_or("invalid pson")?;
            top.push(Expr::from(&self.buf)?);
            self.buf.clear();
        }
        Ok(())
    }
    fn read_hex_escape(&mut self, target: &mut String) -> Result<(), Box<dyn Error>> {
        let mut buf = String::with_capacity(2);
        for _ in 0..2 {
            if let Some(c) = self.it.next() {
                buf.push(c);
            } else {
                Err("invalid pson")?;
            }
        }
        let n = u8::from_str_radix(&buf, 16).map_err(|_| "invalid pson")?;
        target.push(n as char);
        Ok(())
    }
    fn scan_quoted_string(&mut self) -> Result<(), Box<dyn Error>>{
        self.process_buffer()?;
        let mut buf = String::new();
        while let Some(c) = self.it.next() {
            match c {
                '"' => break,
                '\\' => {
                    if let Some(c) = self.it.next() {
                        match c {
                            'n' => buf.push('\n'),
                            't' => buf.push('\t'),
                            'r' => buf.push('\r'),
                            '"' => buf.push('"'),
                            '\\' => buf.push('\\'),
                            'x' => self.read_hex_escape(&mut buf)?,
                             _  => buf.push(c)
                        }
                    } else {
                        Err("invalid pson")?;
                    }
                }
                _ => buf.push(c)
            }
        }
        let top = self.frame_stack.last_mut().ok_or("invalid pson")?;
        top.push(Expr::String(buf));
        Ok(())
    }
    fn close_frame(&mut self) -> Result<(), Box<dyn Error>> {
        self.process_buffer()?;
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
    let mut input_buffer = String::new();
    std::io::stdin().read_line(&mut input_buffer).unwrap();
    let mut scanner = PsonScanner::new(&input_buffer);
    scanner.scan().unwrap();
    println!("{:?}", scanner.get().unwrap());
}
