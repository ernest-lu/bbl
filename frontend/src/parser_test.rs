#[cfg(test)]
mod tests {
    use crate::parser::{BblParser, Rule};
    use pest::Parser;

    fn parse(input: &str, rule: Rule) -> bool {
        BblParser::parse(rule, input).is_ok()
    }

    #[test]
    fn test_basic_values() {
        // Integers
        assert!(parse("42", Rule::integer));
        assert!(parse("-42", Rule::integer));

        // Floats
        assert!(parse("3.14", Rule::float));
        assert!(parse("-3.14", Rule::float));

        // Identifiers
        assert!(parse("x", Rule::identifier));
        assert!(parse("variable_name", Rule::identifier));
        assert!(parse("camelCase", Rule::identifier));
        assert!(!parse("1variable", Rule::identifier)); // Should fail
    }

    #[test]
    fn test_typed_identifiers() {
        assert!(parse("x: int", Rule::typed_identifier));
        assert!(parse("nums: list<int>", Rule::typed_identifier));
        assert!(parse("point: tuple<float>", Rule::typed_identifier));
    }

    #[test]
    fn test_assignments() {
        assert!(parse("x: int = 42", Rule::assignment));
        assert!(parse("y: float = 3.14", Rule::assignment));
        assert!(parse("nums: list<int> = method()", Rule::assignment));
    }

    #[test]
    fn test_function_definitions() {
        assert!(parse(
            r#"def add(x: int, y: int) { 
                x + y
            }"#,
            Rule::function_def
        ));
    }

    #[test]
    fn test_method_calls() {
        assert!(parse("print(42)", Rule::method_call));
        assert!(parse("add(x, y)", Rule::method_call));
        assert!(parse("complex(3.14, other())", Rule::method_call));
    }

    #[test]
    fn test_if_expressions() {
        assert!(parse("if x { print(42) print(42) }", Rule::if_expr));
        assert!(parse("if x { print(42) } else { print(0) }", Rule::if_expr));
    }

    #[test]
    fn test_rep_expressions() {
        assert!(parse("rep 5 { print(42) }", Rule::rep_expr));
    }

    #[test]
    fn test_binary_operations() {
        // Arithmetic
        assert!(parse("5 + 3", Rule::bin_op));
        assert!(parse("x - y", Rule::bin_op));
        assert!(parse("10 * 20", Rule::bin_op));
        assert!(parse("a / b", Rule::bin_op));

        // Comparisons
        assert!(parse("x == y", Rule::bin_op));
        assert!(parse("x != y", Rule::bin_op));
        assert!(parse("x < y", Rule::bin_op));
        assert!(parse("x > y", Rule::bin_op));
        assert!(parse("x <= y", Rule::bin_op));
        assert!(parse("x >= y", Rule::bin_op));

        // Logical
        assert!(parse("x && y", Rule::bin_op));
        assert!(parse("x || y", Rule::bin_op));
    }

    #[test]
    fn test_unary_operations() {
        assert!(parse("!true", Rule::un_op));
        assert!(parse("-42", Rule::un_op));
    }

    #[test]
    fn test_block_expr() {
        let program = "{ print(x) print(z) print(x) print(y) }";
        assert!(parse(program, Rule::block));
    }

    #[test]
    fn test_if_expr() {
        let program = r#"
            if x > 0 {
                print(x)
                print(y)
            } else {
                print(-x)
            }
        "#;
        println!("Program: {}", parse(program, Rule::if_expr));
        assert!(parse(program, Rule::if_expr));
    }

    #[test]
    fn test_complete_programs() {
        let program = r#"
            x: int = 42
            if x > 0 {
                print(x)
            } else {
                print(-x)
            }
            
            rep 3 { 
                print(x)
            }"#;
        println!("Program: {}", parse(program, Rule::program));
        assert!(parse(program, Rule::program));

        let program_with_multiple_functions = r#"
            def add(x: int, y: int) {
                x + y
            }

            result: int = add(5, 3)
            print(result)
        "#;
        // assert!(parse(program_with_multiple_functions, Rule::program));
    }
}
