#![allow(warnings)]
use crate::parser::Parser;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

mod exp;
mod express;
mod express_test;
mod lex;
mod node;
mod parser;
mod token;

fn main() -> Result<(), String> {
    let start = Instant::now();
    let mut str = String::new();
    let file_path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/jquery.js");
    print!("{file_path}");
    File::open(file_path)
        .unwrap()
        .read_to_string(&mut str)
        .expect("Failed to read file");
    // println!("{:#?}", str);

    let mut parser = Parser::new(str)?;
    let ast = parser.parse();

    // println!("{:#?}", ast);
    // println!("{:#?}", parser.loc);
    let duration = start.elapsed();
    println!("耗时: {:.2?}", duration);
    Ok(())
}
