use pest_derive::Parser;

use crate::ast;
use crate::ast::AstNode;
use pest::Parser;

#[derive(Parser)]
#[grammar = "src/parser.pest"]
pub struct BdlParser;

// The Rule enum is automatically generated by pest_derive

use crate::ast::Expr;
use crate::ast::IntegerLiteral;
use crate::ast::PrintExpr;
use crate::ast::Program;
use pest::iterators::Pair;

fn build_ast_from_expr(pair: Pair<Rule>) -> Option<AstNode> {
    match pair.as_rule() {
        Rule::program => {
            let nodes = pair
                .into_inner()
                .filter_map(|pair| build_ast_from_expr(pair)?.Expr())
                .collect::<Vec<Expr>>();
            Some(AstNode::Program(Program { expressions: nodes }))
        }
        Rule::expression => build_ast_from_expr(pair.into_inner().next()?),
        Rule::typed_identifier => {
            let nodes = pair.into_inner().collect::<Vec<Pair<Rule>>>();
            assert!(nodes.len() == 2);
        }
        Rule::identifier => None,
        Rule::assignment => {
            let mut inner_rules = pair.into_inner().collect::<Vec<Pair<Rule>>>();
            // typed identifier and expression
            assert!(inner_rules.len() == 2);

            let expr = build_ast_from_expr(inner_rules.pop()?)?.Expr();
            let identifier = build_ast_from_expr(inner_rules.pop()?)?.TypedIdentifier();

            Some(AstNode::Expr(Expr::AssignmentExpr(ast::AssignmentExpr {
                target: identifier?,
                value: Box::new(expr?),
            })))
        }
        Rule::print_expr => {
            let expr = build_ast_from_expr(pair.into_inner().next()?)?.Expr();
            let print_expr = PrintExpr::new(expr?);
            Some(AstNode::Expr(Expr::PrintExpr(print_expr)))
        }
        Rule::integer => {
            let int_value = pair.as_str().parse::<i128>().unwrap();
            Some(AstNode::Expr(Expr::Integer(IntegerLiteral::new(int_value))))
        }
        _ => None,
    }
}

pub fn parse_program(input: &str) -> Result<Box<AstNode>, String> {
    match BdlParser::parse(Rule::program, input) {
        Ok(parsed) => {
            for pair in parsed {
                let node = match build_ast_from_expr(pair) {
                    Some(n) => n,
                    None => return Err("Failed to build AST from expression".to_string()),
                };
                return Ok(Box::new(node));
            }
        }
        Err(e) => {
            return Err(e.to_string());
        }
    }
    return Err("Failed to parse program".to_string());
}
