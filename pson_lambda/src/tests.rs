use std::{fs::File, io::Read};

#[test]
pub(crate) fn factorial(){
    let file_name = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/factorial.pson");
    println!("file_name: {}", file_name);
    let file = File::open(file_name).unwrap();
    let content = file
        .bytes()
        .map(|b| b.unwrap())
        .collect::<Vec<u8>>();
    let content_str = String::from_utf8_lossy(content.as_slice());
    crate::compiler::compile_module(content_str.as_ref()).unwrap();
}
