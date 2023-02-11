use std::{error::Error, io::{self, Write}};

use pson::Expr;

struct Context {
}

impl Context {
    fn new() -> Context {
        Context {}
    }
}

impl Clone for Context {
    fn clone(&self) -> Context {
        Context {}
    }
}

struct InterpretLocks {
    string_lock: bool,
    brace_lock_count: usize,
}

impl InterpretLocks {
    fn new() -> InterpretLocks {
        InterpretLocks {
            string_lock: false,
            brace_lock_count: 0,
        }
    }
    fn is_locked(&self) -> bool {
        self.string_lock || self.brace_lock_count > 0
    }
    fn update(&self, line: &String) -> InterpretLocks {
        line
            .chars()
            .fold(&mut self.clone(), |locks, c| {
                match c {
                    '"' => locks.string_lock = !locks.string_lock,
                    '(' | '[' if !locks.string_lock => locks.brace_lock_count += 1,
                    ')' | ']' if !locks.string_lock => locks.brace_lock_count -= 1,
                    _ => (),
                }
                locks
            })
            .to_owned()
    }
}

impl Clone for InterpretLocks {
    fn clone(&self) -> InterpretLocks {
        InterpretLocks {
            string_lock: self.string_lock,
            brace_lock_count: self.brace_lock_count,
        }
    }
}

struct Repl {
    context: Context,
    input_buffer: String,
    interpret_locks: InterpretLocks,
}

impl Repl {
    fn new() -> Repl {
        Repl {
            context: Context::new(),
            input_buffer: String::new(),
            interpret_locks: InterpretLocks::new(),
        }
    }
}

impl Clone for Repl {
    fn clone(&self) -> Repl {
        Repl {
            context: self.context.clone(),
            input_buffer: self.input_buffer.clone(),
            interpret_locks: self.interpret_locks.clone(),
        }
    }
}

fn eval(old_repl: &Repl) -> Result<Repl, Box<dyn Error>> {
    use pson::PsonParser;
    let repl = old_repl.clone();
    let mut parser = PsonParser::new(repl.input_buffer.chars());
    parser.parse()?;
    let exprs =
        if let Expr::Array(t) = parser.get()? {t}
        else {unreachable!("parser should only return array")};
    if exprs.len() == 0 {
        Ok(repl)
    } else {
        todo!("evalute exprs");
    }
}

pub fn repl(){
    println!("Welcome to PSON lambda version {}!", env!("CARGO_PKG_VERSION"));
    println!("Type '\\help' for help.");
    println!("Type '\\exit' to exit.");
    print!("> ");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .lines()
        .map(|line| line.ok())
        .take_while(
            |line| line
                .as_ref()
                .map(|line| line != "\\exit")
                .unwrap_or(true)
        )
        .filter_map(|line| line)
        .fold(
            Repl::new(),
            |old_repl, line| {
                let mut repl = old_repl.clone();
                repl.interpret_locks = repl.interpret_locks.update(&line);
                if repl.interpret_locks.is_locked() {
                    repl.input_buffer.push_str(&line);
                }
                if !repl.interpret_locks.is_locked() {
                    if line == "\\exit" {
                        println!("Goodbye!");
                        std::process::exit(0)
                    }
                    else if line == "\\help" {
                        println!("Type '\\exit' to exit.");
                        print!("> ");
                        std::io::stdout().flush().unwrap();
                        repl
                    }
                    else {
                        let new_repl = eval(&repl).unwrap(); // TODO: Handle errors
                        print!("> ");
                        std::io::stdout().flush().unwrap();
                        new_repl
                    }
                }
                else {
                    repl
                }
            }
        );
}
