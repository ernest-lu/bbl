/// A block of C++ code
///
pub enum Line {
    Block(Block),
    Statement(String),
}

pub struct Program {
    pub solve_block: Block,
}

impl Program {
    pub fn new() -> Self {
        Self {
            solve_block: Block::new_with_pre_block("void solve() ".to_string(), 0),
        }
    }

    pub fn to_string(&self) -> String {
        let mut header = r#"
#include <bits/stdc++.h>
using namespace std;
using ll = long long;
"#;

        let solve_fn = self.solve_block.to_string();

        let main_fn = r#"
int main() {
    cin.tie(0)->sync_with_stdio(false);
    solve();
    return 0;
}"#;

        header.to_string() + &solve_fn + &main_fn
    }
}

pub struct Block {
    pre_block: Option<String>,
    statements: Vec<Line>,
    pub indent_level: usize,
}

impl Block {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
            pre_block: None,
            indent_level: 0,
        }
    }
    pub fn new_with_pre_block(pre_block: String, indent_level: usize) -> Self {
        Self {
            statements: Vec::new(),
            pre_block: Some(pre_block),
            indent_level: indent_level,
        }
    }

    pub fn add_statement(&mut self, stmt: Line) {
        self.statements.push(stmt);
    }

    pub fn add_line(&mut self, line: String) {
        self.statements.push(Line::Statement(line));
    }
    pub fn add_line_s(&mut self, line: &str) {
        self.statements.push(Line::Statement(line.to_string()));
    }

    pub fn add_block(&mut self, block: Block) {
        self.statements.push(Line::Block(block));
    }

    pub fn to_string(&self) -> String {
        let mut res = String::new();
        if let Some(ref pre_block) = self.pre_block {
            res += pre_block;
        }
        res += "{\n";
        res += &self
            .statements
            .iter()
            .map(|s| {
                let x = format!(
                    "{}{}",
                    " ".repeat(4 * (self.indent_level + 1)),
                    match s {
                        Line::Block(b) => b.to_string(),
                        Line::Statement(s) => s.to_string(),
                    }
                );
                // println!("{}", x);
                x
            })
            .collect::<Vec<_>>()
            .join("\n");
        res += &format!("\n{}}}", " ".repeat(4 * self.indent_level));
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block() {
        let mut block = Block::new();
        block.add_line_s("int x = 42;");
        block.add_line_s("std::cout << x << std::endl;");

        let expected = r#"{
    int x = 42;
    std::cout << x << std::endl;
}"#;
        assert_eq!(block.to_string(), expected);
    }
}
