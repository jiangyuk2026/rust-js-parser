#![allow(warnings)]

use std::fs::File;
use std::io::Read;

mod express;
mod lex;
mod exp;
mod parser;
mod express_test;
mod node;

fn main() {
    let mut str = String::new();
    let file_path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/a.js");
    print!("{file_path}");
    File::open(file_path)
        .unwrap()
        .read_to_string(&mut str)
        .expect("Failed to read file");
    println!("{:#?}", str);
}