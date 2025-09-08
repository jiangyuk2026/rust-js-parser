#![allow(warnings)]

use crate::parser::Parser;
use std::fs;
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
    for path in vec!["b.js", "jquery.js", "react.development.js", "cloudflare.js"] {
        let start = Instant::now();
        let mut str = String::new();
        let file_path = format!("{}/{}/{}", env!("CARGO_MANIFEST_DIR"), "src", path);
        print!("{file_path}");
        File::open(file_path)
            .unwrap()
            .read_to_string(&mut str)
            .expect("Failed to read file");
        // println!("{:#?}", str);

        let mut parser = Parser::new(str)?;
        let ast = parser.parse();

        // println!("{:#?}", ast);
        if ast.is_err() {
            println!("{:#?}", parser.loc);
            println!("{:#?}", ast);
        }
        let duration = start.elapsed();
        if ast.is_ok() {
            println!("耗时: {:.2?}", duration);
            let out_path = format!("{}/{}/{}", env!("CARGO_MANIFEST_DIR"), "out", path);
            let mut result_txt = "".to_string();
            for node in ast?.iter() {
                result_txt += &node.print_node();
            }
            println!("{:?}", result_txt);
            fs::write(out_path, result_txt).expect("Failed to write to file");
            println!();
        }
    }
    Ok(())
}
