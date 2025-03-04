mod codegen;

use bbl_frontend::ast::{Expr, IntegerLiteral, PrintExpr, Program};
use bbl_frontend::parser::parse_program;
use codegen::generate;
use std::{env, fs};

fn main() {
    // Create a simple test program that prints a number
    let file = env::args().nth(1).expect("No file provided");
    let src = fs::read_to_string(file).expect("Failed to read file");

    let src = if src.ends_with('\n') { src } else { src + "\n" };

    let prog = parse_program(&src)
        .expect("Failed to parse program")
        .Program()
        .unwrap();

    // Create and run the processor
    let program = generate(&prog);

    println!("{}", program);
}
