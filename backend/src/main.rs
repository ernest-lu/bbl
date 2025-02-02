mod codegen;

use bdl_frontend::ast::{Expr, IntegerLiteral, PrintExpr, Program};
use bdl_frontend::parser::parse_program;
use codegen::generate;
use std::{env, fs};

fn main() {
    // Create a simple test program that prints a number
    let file = env::args().nth(1).expect("No file provided");
    let src = fs::read_to_string(file).expect("Failed to read file");
    let prog = parse_program(&src)
        .expect("Failed to parse program")
        .Program()
        .unwrap();

    // Create and run the processor
    let program = generate(&prog);

    println!("{}", program);
}
