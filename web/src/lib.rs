use std::cell::RefCell;
use wasm_bindgen::prelude::*;

thread_local! {
    static STDIN_BUFFER: RefCell<String> = RefCell::new(String::new());
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn compile_and_run(input: &str, stdin: &str) -> String {
    // Store stdin for later use
    STDIN_BUFFER.with(|buffer| {
        *buffer.borrow_mut() = stdin.to_string();
    });

    // Parse BBL code
    let ast = match bbl_frontend::parser::parse_program(input) {
        Ok(ast) => *ast, // Dereference the Box<AstNode>
        Err(e) => {
            return format!("Parse error: {}", e);
        }
    };

    // Extract Program from AstNode
    let program = match ast {
        bbl_frontend::ast::AstNode::Program(prog) => prog,
        _ => {
            return "Expected Program AST node".to_string();
        }
    };

    // Generate C++ code
    bbl_backend::codegen::generate(&program)
}

#[wasm_bindgen]
pub fn cin_next() -> String {
    let mut input = String::new();
    STDIN_BUFFER.with(|buffer| {
        let mut buf = buffer.borrow_mut();
        if let Some(pos) = buf.find(char::is_whitespace) {
            input = buf[..pos].to_string();
            *buf = buf[pos + 1..].to_string();
        } else {
            input = buf.clone();
            buf.clear();
        }
    });
    input
}

#[wasm_bindgen]
pub fn cout_print(text: &str) {
    log(text);
}
