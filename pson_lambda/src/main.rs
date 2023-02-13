use std::env::args;
use std::fs::File;
use std::io::Read;

mod arg_parser;
mod config;
mod repl;
mod context;
mod compiler;
#[cfg(test)]
mod tests;

fn main() {
    let arg_config = arg_parser::parse_args(&mut args().skip(1)).unwrap();
    println!("{:?}", arg_config);
    let interactive = arg_config.interactive || arg_config.file.is_none();
    if interactive {
        repl::repl();
    }
    else {
        let file_name = arg_config.file.unwrap();
        let file = File::open(file_name).unwrap();
        let content = file
            .bytes()
            .map(|b| b.unwrap())
            .collect::<Vec<u8>>();
        let content_str = String::from_utf8_lossy(content.as_slice());
        compiler::compile_module(content_str.as_ref()).unwrap();
    }
}
