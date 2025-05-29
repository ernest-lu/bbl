#[derive(Debug, Clone)]
pub struct Program {
    pub expressions: Vec<Expr>,
}

// ast next!

#[derive(Debug, Clone)]
pub struct FunctionDef {
    pub name: Identifier,
    pub args: Vec<TypedIdentifier>,
    pub body: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    String,
    Bool,
    List(Box<Type>),
    Tuple(Box<Type>),
    // type of parameters, return type
    FunctionType(Vec<Type>, Box<Option<Type>>),
    None,
}

trait ExprNode {
    fn get_type(&self) -> Type;
}

#[derive(Debug, Clone)]
pub struct TypedIdentifier {
    pub value: Identifier,
    pub associated_type: Type,
}

impl TypedIdentifier {
    pub fn new(value: Identifier, associated_type: Type) -> TypedIdentifier {
        TypedIdentifier {
            value,
            associated_type,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IntegerLiteral {
    pub value: i128,
}

impl IntegerLiteral {
    pub fn new(value: i128) -> IntegerLiteral {
        IntegerLiteral { value }
    }
}

#[derive(Debug, Clone)]
pub struct StringLiteral {
    pub value: String,
}

impl StringLiteral {
    pub fn new(value: String) -> StringLiteral {
        StringLiteral { value }
    }
}

#[derive(Debug, Clone)]
pub struct FloatLiteral {
    pub value: f64,
}

impl FloatLiteral {
    pub fn new(value: f64) -> FloatLiteral {
        FloatLiteral { value }
    }
}

#[derive(Debug, Clone)]
pub struct BooleanLiteral {
    pub value: bool,
}

impl BooleanLiteral {
    pub fn new(value: bool) -> BooleanLiteral {
        BooleanLiteral { value }
    }
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub value: String,
}

impl Identifier {
    pub fn new(value: String) -> Identifier {
        Identifier { value }
    }
}

#[derive(Debug, Clone)]
pub struct AssignmentExpr {
    pub target: TypedIdentifier,
    pub value: Box<Expr>,
    pub const_var: bool,
}

impl AssignmentExpr {
    pub fn new(target: TypedIdentifier, value: Expr, const_var: bool) -> AssignmentExpr {
        AssignmentExpr {
            target,
            value: Box::new(value),
            const_var,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReassignmentExpr {
    pub target: Identifier,
    pub value: Box<Expr>,
}

impl ReassignmentExpr {
    pub fn new(target: Identifier, value: Expr) -> ReassignmentExpr {
        ReassignmentExpr {
            target,
            value: Box::new(value),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MethodCallExpr {
    pub method_name: Identifier,
    pub args: Vec<Expr>,
}

impl MethodCallExpr {
    pub fn new(method_name: Identifier, args: Vec<Expr>) -> MethodCallExpr {
        MethodCallExpr { method_name, args }
    }
}

#[derive(Debug, Clone)]
pub struct PrintExpr {
    pub arg: Box<Expr>,
}

impl PrintExpr {
    pub fn new(arg: Expr) -> PrintExpr {
        PrintExpr { arg: Box::new(arg) }
    }
}

#[derive(Debug, Clone)]
pub struct NoneExpr;

impl NoneExpr {
    pub fn new() -> NoneExpr {
        NoneExpr
    }
}

#[derive(Debug, Clone)]
pub struct IfExpr {
    pub condition: Box<Expr>,
    pub then_block: Vec<Expr>,
    pub else_block: Option<Vec<Expr>>,
}

impl IfExpr {
    pub fn new(condition: Expr, then_block: Vec<Expr>, else_block: Option<Vec<Expr>>) -> IfExpr {
        IfExpr {
            condition: Box::new(condition),
            then_block,
            else_block,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RepExpr {
    pub num_iterations: Box<Expr>,
    pub body: Vec<Expr>,
}

impl RepExpr {
    pub fn new(num_iterations: Expr) -> RepExpr {
        RepExpr {
            num_iterations: Box::new(num_iterations),
            body: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReturnExpr {
    pub value: Box<Expr>,
}

impl ReturnExpr {
    pub fn new(value: Expr) -> ReturnExpr {
        ReturnExpr {
            value: Box::new(value),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ListExpr {
    pub elems: Vec<Expr>,
}

impl ListExpr {
    pub fn new(elems: Vec<Expr>) -> ListExpr {
        ListExpr { elems }
    }
}

#[derive(Debug, Clone)]
pub struct BinOpExpr {
    pub left: Box<Expr>,
    pub op: String,
    pub right: Box<Expr>,
}

impl BinOpExpr {
    pub fn new(left: Expr, op: String, right: Expr) -> BinOpExpr {
        BinOpExpr {
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnOpExpr {
    pub op: String,
    pub arg: Box<Expr>,
}

impl UnOpExpr {
    pub fn new(op: String, arg: Expr) -> UnOpExpr {
        UnOpExpr {
            op,
            arg: Box::new(arg),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Integer(IntegerLiteral),
    Float(FloatLiteral),
    Boolean(BooleanLiteral),
    String(StringLiteral),
    Identifier(Identifier),
    AssignmentExpr(AssignmentExpr),
    ReassignmentExpr(ReassignmentExpr),
    MethodCallExpr(MethodCallExpr),
    PrintExpr(PrintExpr),
    IfExpr(IfExpr),
    RepExpr(RepExpr),
    ListExpr(ListExpr),
    BinOp(BinOpExpr),
    UnOp(UnOpExpr),
    FunctionDef(FunctionDef),
    NoneExpr(NoneExpr),
    ReturnExpr(ReturnExpr),
}

impl Expr {
    pub fn Integer(self) -> Option<IntegerLiteral> {
        if let Expr::Integer(i) = self {
            Some(i)
        } else {
            None
        }
    }

    pub fn Boolean(self) -> Option<BooleanLiteral> {
        if let Expr::Boolean(b) = self {
            Some(b)
        } else {
            None
        }
    }

    pub fn Float(self) -> Option<FloatLiteral> {
        if let Expr::Float(f) = self {
            Some(f)
        } else {
            None
        }
    }

    pub fn String(self) -> Option<StringLiteral> {
        if let Expr::String(s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn Identifier(self) -> Option<Identifier> {
        if let Expr::Identifier(i) = self {
            Some(i)
        } else {
            None
        }
    }

    pub fn AssignmentExpr(self) -> Option<AssignmentExpr> {
        if let Expr::AssignmentExpr(a) = self {
            Some(a)
        } else {
            None
        }
    }

    pub fn ReassignmentExpr(self) -> Option<ReassignmentExpr> {
        if let Expr::ReassignmentExpr(r) = self {
            Some(r)
        } else {
            None
        }
    }

    pub fn MethodCallExpr(self) -> Option<MethodCallExpr> {
        if let Expr::MethodCallExpr(m) = self {
            Some(m)
        } else {
            None
        }
    }

    pub fn PrintExpr(self) -> Option<PrintExpr> {
        if let Expr::PrintExpr(p) = self {
            Some(p)
        } else {
            None
        }
    }

    pub fn IfExpr(self) -> Option<IfExpr> {
        if let Expr::IfExpr(i) = self {
            Some(i)
        } else {
            None
        }
    }

    pub fn RepExpr(self) -> Option<RepExpr> {
        if let Expr::RepExpr(r) = self {
            Some(r)
        } else {
            None
        }
    }

    pub fn BinOp(self) -> Option<BinOpExpr> {
        if let Expr::BinOp(b) = self {
            Some(b)
        } else {
            None
        }
    }

    pub fn UnOp(self) -> Option<UnOpExpr> {
        if let Expr::UnOp(u) = self {
            Some(u)
        } else {
            None
        }
    }

    pub fn FunctionDef(self) -> Option<FunctionDef> {
        if let Expr::FunctionDef(f) = self {
            Some(f)
        } else {
            None
        }
    }

    pub fn NoneExpr(self) -> Option<NoneExpr> {
        if let Expr::NoneExpr(n) = self {
            Some(n)
        } else {
            None
        }
    }
}

impl Expr {
    pub fn get_type(&self) -> Type {
        match self {
            Expr::Integer(_) => Type::Int,
            Expr::Float(_) => Type::Float,
            Expr::String(_) => Type::String,
            Expr::Boolean(_) => Type::Bool,
            Expr::Identifier(_) => todo!(),
            Expr::ReturnExpr(_) => todo!(),
            Expr::AssignmentExpr(_) => todo!(),
            Expr::ReassignmentExpr(_) => todo!(),
            Expr::MethodCallExpr(_) => todo!(),
            Expr::PrintExpr(_) => todo!(),
            Expr::IfExpr(_) => todo!(),
            Expr::RepExpr(_) => todo!(),
            Expr::ListExpr(_) => todo!(),
            Expr::BinOp(_) => todo!(),
            Expr::UnOp(expr) => expr.arg.get_type(),
            Expr::FunctionDef(fd) => todo!(),
            Expr::NoneExpr(_) => Type::None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AstNode {
    Program(Program),
    Expr(Expr),
    TypedIdentifier(TypedIdentifier),
    Type(Type),
    VecExpr(Vec<Expr>),
}

impl AstNode {
    pub fn get_type(&self) -> Type {
        match self {
            AstNode::Program(program) => program
                .expressions
                .last()
                .unwrap_or(&Expr::NoneExpr(NoneExpr))
                .get_type(),
            AstNode::Expr(expr) => expr.get_type(),
            AstNode::TypedIdentifier(identifier) => identifier.associated_type.clone(),
            AstNode::Type(t) => t.clone(),
            AstNode::VecExpr(exprs) => todo!(),
        }
    }

    pub fn Expr(self) -> Option<Expr> {
        if let AstNode::Expr(e) = self {
            Some(e)
        } else {
            None
        }
    }

    pub fn Program(self) -> Option<Program> {
        if let AstNode::Program(p) = self {
            Some(p)
        } else {
            None
        }
    }

    pub fn TypedIdentifier(self) -> Option<TypedIdentifier> {
        if let AstNode::TypedIdentifier(i) = self {
            Some(i)
        } else {
            None
        }
    }

    pub fn Type(self) -> Option<Type> {
        if let AstNode::Type(t) = self {
            Some(t)
        } else {
            None
        }
    }

    pub fn VecExpr(self) -> Option<Vec<Expr>> {
        if let AstNode::VecExpr(e) = self {
            Some(e)
        } else {
            None
        }
    }
}
