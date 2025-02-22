use bbl_frontend::ast::{
    AssignmentExpr, BinOpExpr, Expr, Identifier, IfExpr, ListExpr, MethodCallExpr, PrintExpr,
    ReassignmentExpr, RepExpr, Type, UnOpExpr,
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

// Process an expression and add it to the block
fn process_expression(context: &mut Block, expr: &Expr) {
    match expr {
        Expr::Integer(i) => {
            let temp = generate_variable_name();
            context.add_line(format!("int {} = {};", temp, i.value));
        }
        Expr::Float(f) => {
            let temp = generate_variable_name();
            context.add_line(format!("double {} = {};", temp, f.value));
        }
        Expr::String(s) => {
            let temp = generate_variable_name();
            context.add_line(format!("string {} = \"{}\";", temp, s.value));
        }
        Expr::AssignmentExpr(assign) => {
            generate_assignment(context, assign);
        }
        Expr::ReassignmentExpr(reassign) => {
            generate_reassignment(context, reassign);
        }
        Expr::MethodCallExpr(method) => {
            generate_method_call(context, method);
        }
        Expr::PrintExpr(print) => {
            generate_print(context, print);
        }
        Expr::IfExpr(if_expr) => {
            generate_if(context, if_expr);
        }
        Expr::RepExpr(rep) => {
            generate_rep(context, rep);
        }
        Expr::Identifier(id) => {
            generate_identifier(context, id);
        }
        Expr::ListExpr(list) => {
            generate_list_expr(context, list);
        }
        Expr::BinOp(binop) => {
            generate_binop(context, binop);
        }
        Expr::UnOp(unop) => {
            generate_unop(context, unop);
        }
        Expr::FunctionDef(_) | Expr::ReturnExpr(_) => {
            // Not implemented yet
            todo!()
        }
        Expr::NoneExpr(_) => {
            // No-op
            ()
        }
        Expr::Boolean(_) => {
            // No-op
            ()
        }
    }
}

fn get_type_string(t: &Type) -> String {
    match t {
        Type::Int => "int".to_string(),
        Type::String => "string".to_string(),
        Type::List(t) => format!("vector<{}>", get_type_string(t)),
        _ => "auto".to_string(),
    }
}

fn generate_assignment(context: &mut Block, assign: &AssignmentExpr) {
    let var_type = get_type_string(&assign.target.associated_type);
    let var_name = &assign.target.value.value;
    let temp_var = generate_variable_name();
    process_expression(context, &assign.value);
    context.add_line(format!("{} {} = {};", var_type, var_name, temp_var));
}

fn generate_reassignment(context: &mut Block, assign: &ReassignmentExpr) {
    let var_name = &assign.target.value;
    let temp_var = generate_variable_name();
    process_expression(context, &assign.value);
    context.add_line(format!("{} = {};", var_name, temp_var));
}

fn generate_method_call(context: &mut Block, call: &MethodCallExpr) {
    let mut arg_names = Vec::new();
    for arg in &call.args {
        let temp_var = generate_variable_name();
        process_expression(context, arg);
        arg_names.push(temp_var);
    }
    let args_str = arg_names.join(", ");
    context.add_line(format!(
        "{}.{}({});",
        call.method_name.value, call.method_name.value, args_str
    ));
}

fn generate_print(context: &mut Block, print: &PrintExpr) {
    let temp_var = generate_variable_name();
    process_expression(context, &print.arg);
    context.add_line(format!("cout << {} << endl;", temp_var));
}

fn generate_if(context: &mut Block, if_expr: &IfExpr) {
    let cond_var = generate_variable_name();
    process_expression(context, &if_expr.condition);

    let mut then_block = Block::new();
    for expr in &if_expr.then_block {
        process_expression(&mut then_block, expr);
    }

    context.add_line(format!("if ({}) {{", cond_var));
    context.add_block(then_block);

    if let Some(else_vec) = &if_expr.else_block {
        let mut else_block = Block::new();
        for expr in else_vec {
            process_expression(&mut else_block, expr);
        }
        context.add_line_s("} else {");
        context.add_block(else_block);
    }
    context.add_line_s("}");
}

fn generate_rep(context: &mut Block, rep: &RepExpr) {
    let iter_var = generate_variable_name();
    let count_var = generate_variable_name();
    process_expression(context, &rep.num_iterations);

    context.add_line(format!("int {} = 0;", iter_var));
    context.add_line(format!("int {} = {};", count_var, iter_var));

    let mut loop_block = Block::new();
    for expr in &rep.body {
        process_expression(&mut loop_block, expr);
    }

    context.add_line(format!(
        "for(int {} = 0; {} < {}; {}++) {{",
        iter_var, iter_var, count_var, iter_var
    ));
    context.add_block(loop_block);
    context.add_line_s("}");
}

fn generate_binop(context: &mut Block, binop: &BinOpExpr) {
    let left_var = generate_variable_name();
    let right_var = generate_variable_name();
    process_expression(context, &binop.left);
    process_expression(context, &binop.right);
    let result_var = generate_variable_name();
    context.add_line(format!(
        "auto {} = {} {} {};",
        result_var,
        left_var,
        binop.op.as_str(),
        right_var
    ));
}

fn generate_unop(context: &mut Block, unop: &UnOpExpr) {
    let expr_var = generate_variable_name();
    process_expression(context, &unop.arg);
    let result_var = generate_variable_name();
    context.add_line(format!(
        "auto {} = {}{};",
        result_var,
        unop.op.as_str(),
        expr_var
    ));
}

fn generate_list_expr(context: &mut Block, list: &ListExpr) {
    let mut element_vars = Vec::new();
    for elem in &list.elems {
        let temp_var = generate_variable_name();
        process_expression(context, elem);
        element_vars.push(temp_var);
    }
    let result_var = generate_variable_name();
    context.add_line(format!(
        "vector<auto> {} = {{{}}};",
        result_var,
        element_vars.join(", ")
    ));
}

fn generate_identifier(context: &mut Block, id: &Identifier) {
    let result_var = generate_variable_name();
    context.add_line(format!("auto {} = {};", result_var, id.value));
}
