use std::env::args;

mod arg_parser;
mod config;
mod repl;

fn main() {
    let arg_config = arg_parser::parse_args(&mut args().skip(1)).unwrap();
    println!("{:?}", arg_config);
    let interactive = arg_config.interactive || arg_config.file.is_none();
    if interactive {
        repl::repl();
    }
    else {
        println!("Running file: {}", arg_config.file.unwrap());
    }
}
