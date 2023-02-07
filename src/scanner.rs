use std::{str::Chars, error::Error};

use crate::expr::Expr;
use crate::frame::{Frame, FrameKind};

pub struct PsonScanner<'a> {
    frame_stack: Vec<Frame>,
    buffer: String,
    it: Chars<'a>,
}

impl PsonScanner<'_> {
    pub fn new<'a>(text: Chars<'a>) -> PsonScanner<'a> {
        PsonScanner {
            frame_stack: vec![Frame::new(FrameKind::Array)],
            buffer: String::new(),
            it: text
        }
    }
    pub fn with_buffer_capacity<'a>(text: Chars<'a>, capacity: usize) -> PsonScanner<'a> {
        PsonScanner {
            frame_stack: vec![Frame::new(FrameKind::Array)],
            buffer: String::with_capacity(capacity),
            it: text
        }
    }
    pub fn process_buffer(&mut self) -> Result<(), Box<dyn Error>>{
        if !self.buffer.is_empty() {
            let top = self.frame_stack.last_mut().ok_or("invalid pson")?;
            top.push(Expr::from(&self.buffer)?);
            self.buffer.clear();
        }
        Ok(())
    }
    pub fn read_hex_escape(&mut self) -> Result<(), Box<dyn Error>> {
        let mut buf = String::with_capacity(2);
        for _ in 0..2 {
            if let Some(c) = self.it.next() {
                buf.push(c);
            } else {
                Err("invalid pson")?;
            }
        }
        let n = u8::from_str_radix(&buf, 16).map_err(|_| "invalid pson")?;
        self.buffer.push(n as char);
        Ok(())
    }
    pub fn scan_quoted_string(&mut self) -> Result<(), Box<dyn Error>>{
        self.process_buffer()?;
        while let Some(c) = self.it.next() {
            match c {
                '"' => break,
                '\\' => {
                    if let Some(c) = self.it.next() {
                        match c {
                            'n' => self.buffer.push('\n'),
                            't' => self.buffer.push('\t'),
                            'r' => self.buffer.push('\r'),
                            '"' => self.buffer.push('"'),
                            '\\' => self.buffer.push('\\'),
                            'x' => self.read_hex_escape()?,
                             _  => self.buffer.push(c)
                        }
                    } else {
                        Err("invalid pson")?;
                    }
                }
                _ => self.buffer.push(c)
            }
        }
        let top = self.frame_stack.last_mut().ok_or("invalid pson")?;
        top.push(Expr::String(self.buffer.clone()));
        self.buffer.clear();
        Ok(())
    }
    pub fn close_frame(&mut self) -> Result<(), Box<dyn Error>> {
        self.process_buffer()?;
        let mut frame = self.frame_stack.pop().ok_or("invalid pson")?;
        let top = self.frame_stack.last_mut().ok_or("invalid pson")?;
        match frame.kind {
            FrameKind::Array => top.push(frame.to_array()?),
            FrameKind::Map => top.push(frame.to_map()?),
        }
        Ok(())
    }
    pub fn scan(&mut self) -> Result<(), Box<dyn Error>> {
        while let Some(c) = self.it.next() {
            match c {
                '[' => self.frame_stack.push(Frame::new(FrameKind::Array)),
                '(' => self.frame_stack.push(Frame::new(FrameKind::Map)),
                '<' => self.close_frame()?,
                ' ' | '\t' | '\n' | '\r' => self.process_buffer()?,
                '"' => self.scan_quoted_string()?,
                _ => self.buffer.push(c)
            }
        };
        self.process_buffer()?;
        Ok(())
    }
    pub fn get(&mut self) -> Result<Expr, Box<dyn Error>> {
        if self.frame_stack.len() != 1 {
            Err("invalid pson")?;
        }
        let top = self.frame_stack.pop().ok_or("invalid pson")?;
        let expr = top.to_array()?;
        Ok(expr)
    }
}
