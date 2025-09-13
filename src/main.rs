#![allow(warnings)]

use crate::node::PrintContext;
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
    for path in vec![
        "b.js",
        // "jquery.js",
        // "react.development.js",
        // "cloudflare.js"
    ] {
        let start = Instant::now();
        let mut str = String::new();
        let file_path = format!("{}/{}/{}", env!("CARGO_MANIFEST_DIR"), "src", path);
        print!("{file_path}");
        File::open(file_path)
            .unwrap()
            .read_to_string(&mut str)
            .expect("Failed to read file");
        // println!("{:#?}", str);

        str = str.replace("\r\n", "\n");
        str = str.replace("\r", "\n");

        let mut parser = Parser::new(str)?;
        let ast = parser.parse();

        if path == "b.js" {
            println!("{:#?}", ast);
        }
        if ast.is_err() {
            println!("{:#?}", parser.loc);
            println!("{:#?}", ast);
        }
        let duration = start.elapsed();
        if ast.is_ok() {
            println!("耗时: {:.2?}", duration);
            let out_path = format!("{}/{}/{}", env!("CARGO_MANIFEST_DIR"), "out", path);
            let print_context = &mut PrintContext::new();
            for node in ast?.iter() {
                node.print_node(print_context);
            }
            // println!("{:?}", result_txt);
            fs::write(out_path, &print_context.output).expect("Failed to write to file");
            println!();
        }
    }
    Ok(())
}
