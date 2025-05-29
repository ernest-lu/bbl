use crate::ast::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct TypeError {
    pub message: String,
}

pub type TypeResult<T> = Result<T, TypeError>;

pub struct TypeChecker {
    pub symbol_table: HashMap<String, Type>,
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {
            symbol_table: HashMap::new(),
        }
    }

    pub fn check_program(&mut self, program: &Program) -> TypeResult<()> {
        for expr in &program.expressions {
            self.check_expr(expr)?;
        }
        Ok(())
    }

    pub fn check_expr(&mut self, expr: &Expr) -> TypeResult<Type> {
        match expr {
            Expr::Integer(_) => Ok(Type::Int),
            Expr::Float(_) => Ok(Type::Float),
            Expr::String(_) => Ok(Type::String),
            Expr::Boolean(_) => Ok(Type::Bool),
            Expr::Identifier(id) => self.symbol_table.get(&id.value).cloned().ok_or(TypeError {
                message: format!("Undefined variable '{}'", id.value),
            }),
            Expr::AssignmentExpr(assign) => {
                let rhs_type = self.check_expr(&assign.value)?;
                let lhs_type = assign.target.associated_type.clone();
                if rhs_type != lhs_type {
                    return Err(TypeError {
                        message: format!(
                            "Type mismatch in assignment to '{}': expected {:?}, got {:?}",
                            assign.target.value.value, lhs_type, rhs_type
                        ),
                    });
                }
                self.symbol_table
                    .insert(assign.target.value.value.clone(), lhs_type.clone());
                Ok(lhs_type)
            }
            Expr::ReassignmentExpr(reassign) => {
                let rhs_type = self.check_expr(&reassign.value)?;
                let var_type = self
                    .symbol_table
                    .get(&reassign.target.value)
                    .ok_or(TypeError {
                        message: format!("Undefined variable '{}'", reassign.target.value),
                    })?;
                if *var_type != rhs_type {
                    return Err(TypeError {
                        message: format!(
                            "Type mismatch in reassignment to '{}': expected {:?}, got {:?}",
                            reassign.target.value, var_type, rhs_type
                        ),
                    });
                }
                Ok(var_type.clone())
            }
            Expr::BinOp(binop) => {
                let left_type = self.check_expr(&binop.left)?;
                let right_type = self.check_expr(&binop.right)?;
                if left_type != right_type {
                    return Err(TypeError {
                        message: format!(
                            "Type mismatch in binary operation '{}': left is {:?}, right is {:?}",
                            binop.op, left_type, right_type
                        ),
                    });
                }
                // For now, just return the type if it's int/float/string/bool
                match binop.op.as_str() {
                    "+" | "-" | "*" | "/" => {
                        if left_type == Type::Int || left_type == Type::Float {
                            Ok(left_type)
                        } else {
                            Err(TypeError {
                                message: format!(
                                    "Operator '{}' not supported for type {:?}",
                                    binop.op, left_type
                                ),
                            })
                        }
                    }
                    "==" | "!=" | "<" | ">" | "<=" | ">=" => Ok(Type::Bool),
                    _ => Err(TypeError {
                        message: format!("Unknown operator '{}'", binop.op),
                    }),
                }
            }
            Expr::ListExpr(list) => {
                // Check all elements have the same type
                let mut elem_type: Option<Type> = None;
                for elem in &list.elems {
                    let t = self.check_expr(elem)?;
                    if let Some(ref et) = elem_type {
                        if *et != t {
                            return Err(TypeError {
                                message: format!(
                                    "List elements have mismatched types: {:?} vs {:?}",
                                    et, t
                                ),
                            });
                        }
                    } else {
                        elem_type = Some(t);
                    }
                }
                Ok(Type::List(Box::new(elem_type.unwrap_or(Type::None))))
            }
            Expr::PrintExpr(print) => {
                self.check_expr(&print.arg)?;
                Ok(Type::None)
            }
            Expr::IfExpr(ifexpr) => {
                let cond_type = self.check_expr(&ifexpr.condition)?;
                if cond_type != Type::Bool {
                    return Err(TypeError {
                        message: "Condition in if expression must be boolean".to_string(),
                    });
                }
                for expr in &ifexpr.then_block {
                    self.check_expr(expr)?;
                }
                if let Some(else_block) = &ifexpr.else_block {
                    for expr in else_block {
                        self.check_expr(expr)?;
                    }
                }
                Ok(Type::None)
            }
            Expr::RepExpr(repexpr) => {
                let count_type = self.check_expr(&repexpr.num_iterations)?;
                if count_type != Type::Int {
                    return Err(TypeError {
                        message: "rep count must be int".to_string(),
                    });
                }
                for expr in &repexpr.body {
                    self.check_expr(expr)?;
                }
                Ok(Type::None)
            }
            Expr::FunctionDef(func) => {
                // Save current symbol table
                let old_table = self.symbol_table.clone();
                // Add arguments to symbol table
                for arg in &func.args {
                    self.symbol_table
                        .insert(arg.value.value.clone(), arg.associated_type.clone());
                }
                for expr in &func.body {
                    self.check_expr(expr)?;
                }
                // Restore symbol table
                self.symbol_table = old_table;
                Ok(Type::FunctionType(
                    func.args
                        .iter()
                        .map(|a| a.associated_type.clone())
                        .collect(),
                    Box::new(None),
                ))
            }
            Expr::ReturnExpr(ret) => self.check_expr(&ret.value),
            Expr::UnOp(unop) => {
                let arg_type = self.check_expr(&unop.arg)?;
                match unop.op.as_str() {
                    "-" => {
                        if arg_type == Type::Int || arg_type == Type::Float {
                            Ok(arg_type)
                        } else {
                            Err(TypeError {
                                message: format!("Unary '-' not supported for type {:?}", arg_type),
                            })
                        }
                    }
                    "!" => {
                        if arg_type == Type::Bool {
                            Ok(Type::Bool)
                        } else {
                            Err(TypeError {
                                message: format!("Unary '!' not supported for type {:?}", arg_type),
                            })
                        }
                    }
                    _ => Err(TypeError {
                        message: format!("Unknown unary operator '{}'", unop.op),
                    }),
                }
            }
            Expr::NoneExpr(_) => Ok(Type::None),
            Expr::MethodCallExpr(_) => Err(TypeError {
                message: "Method calls not supported in type checker yet".to_string(),
            }),
        }
    }
}

fn main() {
    let prog = parser::parse_program(&src).expect("Failed to parse program");
    let prog = prog.Program().unwrap();

    let mut checker = typeck::TypeChecker::new();
    match checker.check_program(&prog) {
        Ok(_) => println!("Type check passed!"),
        Err(e) => println!("Type error: {}", e.message),
    }
}
