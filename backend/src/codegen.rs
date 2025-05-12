use bbl_frontend::ast::{
    AssignmentExpr, BinOpExpr, Expr, FunctionDef, Identifier, IfExpr, ListExpr, MethodCallExpr,
    PrintExpr, ReassignmentExpr, RepExpr, Type, UnOpExpr,
};

use cpp_codegen::{Block, Line, Program};

pub fn generate(ast: &bbl_frontend::ast::Program) -> String {
    // Create a new program with solve function
    let mut program = Program::new();
    let solve_block = &mut program.solve_block;

    // Generate code for each expression
    for expr in &ast.expressions {
        process_expression(solve_block, expr);
    }
    program.to_string()
}

// Helper function to generate a unique variable name
fn generate_variable_name() -> String {
    static mut COUNTER: u32 = 0;
    unsafe {
        COUNTER += 1;
        format!("var_{}", COUNTER)
    }
}

// Process an expression and optionally return a string or add it to the block
fn process_expression(context: &mut Block, expr: &Expr) -> Option<String> {
    match expr {
        Expr::Integer(i) => Some(format!("{}LL", i.value)),
        Expr::Float(f) => Some(format!("{}LL", f.value)),
        Expr::String(s) => Some(format!("\"{}\"", s.value)),
        Expr::AssignmentExpr(assign) => {
            generate_assignment(context, assign);
            None
        }
        Expr::ReassignmentExpr(reassign) => {
            generate_reassignment(context, reassign);
            None
        }
        Expr::MethodCallExpr(method) => {
            todo!()
        }
        Expr::PrintExpr(print) => {
            generate_print(context, print);
            None
        }
        Expr::IfExpr(if_expr) => {
            generate_if(context, if_expr);
            None
        }
        Expr::RepExpr(rep) => {
            generate_rep(context, rep);
            None
        }
        Expr::Identifier(id) => Some(id.value.clone()),
        Expr::ListExpr(list) => generate_list_expr(context, list),
        Expr::BinOp(binop) => generate_binop(context, binop),
        Expr::UnOp(unop) => generate_unop(context, unop),
        Expr::FunctionDef(func) => {
            generate_function_def(context, func);
            None
        }
        Expr::ReturnExpr(ret) => {
            todo!()
        }
        Expr::NoneExpr(_) => {
            // No-op
            todo!()
        }
        Expr::Boolean(b) => {
            // No-op
            match b.value {
                true => Some("true".to_string()),
                false => Some("false".to_string()),
            }
        }
    }
}

fn get_type_string(inp_type: &Type) -> String {
    match inp_type {
        Type::Int => "int".to_string(),
        Type::Float => "float".to_string(),
        Type::String => "string".to_string(),
        Type::Bool => "bool".to_string(),
        Type::None => "none".to_string(),
        Type::List(c) => format!("vector<{}>", get_type_string(c)),
        Type::Tuple(_) => todo!(),
        Type::FunctionType(_, _) => "auto".to_string(),
    }
}

fn generate_function_def(context: &mut Block, func: &FunctionDef) -> Option<String> {
    let fn_pre_header = format!(
        "auto {} = [&]({}) -> {} ",
        func.name.value,
        func.args
            .iter()
            .map(|arg| format!(
                "{} {}",
                get_type_string(&arg.associated_type),
                arg.value.value
            ))
            .collect::<Vec<String>>()
            .join(", "),
        "auto"
    );

    let mut new_block = Block::new_with_pre_block(fn_pre_header, context.indent_level + 1);
    for expr in &func.body {
        process_expression(&mut new_block, expr);
    }
    context.add_block(new_block);

    None
}

fn generate_assignment(context: &mut Block, assign: &AssignmentExpr) -> Option<String> {
    let val_result = process_expression(context, &assign.value)?;
    let var_type = get_type_string(&assign.target.associated_type);
    let var_name = &assign.target.value.value;
    let const_header = if assign.const_var { "const " } else { "" };
    context.add_line(format!(
        "{}{} {} = {};",
        const_header, var_type, var_name, val_result
    ));
    None
}

fn generate_reassignment(context: &mut Block, assign: &ReassignmentExpr) -> Option<String> {
    let val_result = process_expression(context, &assign.value)?;
    let var_name = &assign.target.value;
    context.add_line(format!("{} = {};", var_name, val_result));
    None
}

fn generate_print(context: &mut Block, print: &PrintExpr) -> Option<String> {
    let val_result = process_expression(context, &print.arg)?;
    context.add_line(format!("cout << {} << '\\n';", val_result));
    None
}

fn generate_if(context: &mut Block, if_expr: &IfExpr) -> Option<String> {
    let condition = process_expression(context, &if_expr.condition)?;

    let mut new_block = Block::new_with_pre_block(
        "if (".to_string() + &condition + ") ",
        context.indent_level + 1,
    );

    for expr in &if_expr.then_block {
        process_expression(&mut new_block, expr);
    }

    context.add_block(new_block);
    if let Some(else_block) = &if_expr.else_block {
        let mut new_block =
            Block::new_with_pre_block("else ".to_string(), context.indent_level + 1);
        for expr in else_block {
            process_expression(&mut new_block, expr);
        }
        context.add_block(new_block);
    }
    None
}

fn generate_rep(context: &mut Block, rep: &RepExpr) -> Option<String> {
    let count = process_expression(context, &rep.num_iterations)?;
    let new_var_name = generate_variable_name();
    let mut new_block = Block::new_with_pre_block(
        format!(
            "for (int {} = 0; {} < {}; {}++) ",
            new_var_name, new_var_name, count, new_var_name
        ),
        context.indent_level + 1,
    );
    for expr in &rep.body {
        process_expression(&mut new_block, expr);
    }
    context.add_block(new_block);
    None
}

fn generate_list_expr(context: &mut Block, list: &ListExpr) -> Option<String> {
    let joined_string = "vector {".to_owned()
        + &list
            .elems
            .iter()
            .map(|e| process_expression(context, e))
            .collect::<Option<Vec<_>>>()?
            .join(", ")
        + "}";
    Some(joined_string.to_string())
}

fn generate_binop(context: &mut Block, binop: &BinOpExpr) -> Option<String> {
    let left_result = process_expression(context, &binop.left)?;
    let right_result = process_expression(context, &binop.right)?;
    Some(format!("{} {} {}", left_result, binop.op, right_result))
}

fn generate_unop(context: &mut Block, unop: &UnOpExpr) -> Option<String> {
    let result = process_expression(context, &unop.arg)?;
    Some(format!("{}({})", unop.op, result))
}
