pub mod ast;
pub mod parser;
#[cfg(test)]
mod parser_test;
pub mod typeck;

use std::env;
use std::fs;

fn main() {
    let file = env::args().nth(1).expect("No file provided");
    let mut src = fs::read_to_string(file).expect("Failed to read file");

    if !src.ends_with('\n') {
        src.push('\n');
    }

    let prog = parser::parse_program(&src).expect("Failed to parse program");

    let mut checker = typeck::TypeChecker::new();
    match checker.check_program(&prog.Program().unwrap()) {
        Ok(_) => println!("Type check passed!"),
        Err(e) => println!("Type error: {}", e.message),
    }
}
