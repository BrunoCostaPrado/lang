pub type NodeId = usize;

#[derive(Debug, Clone)]
pub enum Stmt {
    VarDeclaration(VarDecl),
    FnDeclaration(FnDecl),
    Expression(Expr),
    Return(Option<Expr>),
    While(Expr, Vec<Stmt>),
    For {
        var: String,
        iter: Box<Expr>,
        body: Vec<Stmt>,
    },
    Block(Vec<Stmt>),
}

#[derive(Debug, Clone)]
pub struct VarDecl {
    pub name: String,
    pub type_annotation: Option<TypeAnnotation>,
    pub value: Option<Expr>,
    pub constant: bool,
    pub mutable: bool,
}

#[derive(Debug, Clone)]
pub struct FnDecl {
    pub name: String,
    pub params: Vec<(String, TypeAnnotation)>,
    pub return_type: Option<TypeAnnotation>,
    pub body: Box<ExprOrBlock>,
}

#[derive(Debug, Clone)]
pub enum ExprOrBlock {
    Expr(Expr),
    Block(Vec<Stmt>),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Null,
    Identifier(String),
    Binary {
        left: Box<Expr>,
        operator: BinaryOp,
        right: Box<Expr>,
    },
    Unary {
        operator: UnaryOp,
        right: Box<Expr>,
    },
    Assignment {
        target: Box<Expr>,
        value: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    If {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Option<Box<Expr>>,
    },
    Match {
        value: Box<Expr>,
        arms: Vec<MatchArm>,
    },
    StructLiteral {
        name: String,
        fields: Vec<(String, Expr)>,
    },
    FieldAccess {
        object: Box<Expr>,
        field: String,
    },
    Array(Vec<Expr>),
    Tuple(Vec<Expr>),
}

#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub body: Box<Expr>,
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Identifier(String),
    EnumVariant(String, Vec<String>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Negate,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeAnnotation {
    pub name: String,
    pub params: Vec<TypeAnnotation>,
}

impl TypeAnnotation {
    pub fn simple(name: &str) -> Self {
        TypeAnnotation {
            name: name.to_string(),
            params: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    pub stmts: Vec<Stmt>,
}
