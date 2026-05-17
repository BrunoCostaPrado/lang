use std::collections::HashMap;
use bcp_parser::ast::*;
use crate::types::Type;

pub struct TypeChecker {
    globals: HashMap<String, Type>,
    locals: Vec<HashMap<String, Type>>,
    errors: Vec<String>,
}

impl TypeChecker {
    pub fn new() -> Self {
        let mut globals = HashMap::new();
        globals.insert("print".to_string(), Type::Function {
            params: vec![Type::Generic("T")],
            ret: Box::new(Type::Void),
        });
        globals.insert("println".to_string(), Type::Function {
            params: vec![Type::Generic("T")],
            ret: Box::new(Type::Void),
        });
        TypeChecker {
            globals,
            locals: vec![HashMap::new()],
            errors: Vec::new(),
        }
    }

    pub fn check(&mut self, program: &Program) -> Result<(), Vec<String>> {
        for stmt in &program.stmts {
            self.check_stmt(stmt);
        }
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    fn check_stmt(&mut self, stmt: &Stmt) -> Option<Type> {
        match stmt {
            Stmt::VarDeclaration(decl) => {
                let inferred = decl.value.as_ref().map(|v| self.infer_expr(v));
                let declared = decl.type_annotation.as_ref().map(|a| self.type_from_annotation(a));
                let ty = declared.or(inferred).unwrap_or(Type::Unknown);
                self.locals.last_mut().unwrap().insert(decl.name.clone(), ty);
                None
            }
            Stmt::FnDeclaration(fndecl) => {
                self.locals.push(HashMap::new());
                for (name, _) in &fndecl.params {
                    self.locals.last_mut().unwrap().insert(name.clone(), Type::Unknown);
                }
                match &*fndecl.body {
                    ExprOrBlock::Expr(e) => { self.infer_expr(e); }
                    ExprOrBlock::Block(stmts) => {
                        for s in stmts {
                            self.check_stmt(s);
                        }
                    }
                }
                self.locals.pop();
                None
            }
            Stmt::Expression(expr) => { self.infer_expr(expr); None }
            Stmt::Return(val) => { val.as_ref().map(|v| self.infer_expr(v)); None }
            Stmt::While(cond, body) => {
                self.infer_expr(cond);
                self.locals.push(HashMap::new());
                for s in body { self.check_stmt(s); }
                self.locals.pop();
                None
            }
            Stmt::For { iter, body, .. } => {
                self.infer_expr(iter);
                self.locals.push(HashMap::new());
                for s in body { self.check_stmt(s); }
                self.locals.pop();
                None
            }
            Stmt::Block(stmts) => {
                self.locals.push(HashMap::new());
                for s in stmts { self.check_stmt(s); }
                self.locals.pop();
                None
            }
        }
    }

    fn infer_expr(&self, expr: &Expr) -> Type {
        match expr {
            Expr::Int(_) => Type::I64,
            Expr::Float(_) => Type::F64,
            Expr::Bool(_) => Type::Bool,
            Expr::String(_) => Type::String,
            Expr::Null => Type::Unknown,
            Expr::Identifier(name) => {
                for scope in self.locals.iter().rev() {
                    if let Some(ty) = scope.get(name) {
                        return ty.clone();
                    }
                }
                self.globals.get(name).cloned().unwrap_or(Type::Unknown)
            }
            Expr::Binary { left, operator, right } => {
                let _l = self.infer_expr(left);
                let _r = self.infer_expr(right);
                match operator {
                    BinaryOp::Equal | BinaryOp::NotEqual
                    | BinaryOp::Less | BinaryOp::Greater
                    | BinaryOp::LessEqual | BinaryOp::GreaterEqual
                    | BinaryOp::And | BinaryOp::Or => Type::Bool,
                    _ => Type::I64,
                }
            }
            Expr::Unary { right, .. } => self.infer_expr(right),
            Expr::Assignment { value, .. } => self.infer_expr(value),
            Expr::Call { callee, .. } => {
                match self.infer_expr(callee) {
                    Type::Function { ret, .. } => *ret,
                    _ => Type::Unknown,
                }
            }
            Expr::If { condition, then_branch, else_branch } => {
                self.infer_expr(condition);
                let t = self.infer_expr(then_branch);
                else_branch.as_ref().map(|e| self.infer_expr(e));
                t
            }
            Expr::FieldAccess { object, .. } => { self.infer_expr(object); Type::Unknown }
            Expr::StructLiteral { .. } => Type::Unknown,
            Expr::Match { value, .. } => { self.infer_expr(value); Type::Unknown }
            Expr::Array(_) => Type::Unknown,
            Expr::Tuple(_) => Type::Unknown,
        }
    }

    fn type_from_annotation(&self, ann: &TypeAnnotation) -> Type {
        match ann.name.as_str() {
            "i32" => Type::I32,
            "i64" => Type::I64,
            "u32" => Type::U32,
            "u64" => Type::U64,
            "f32" => Type::F32,
            "f64" => Type::F64,
            "bool" => Type::Bool,
            "string" => Type::String,
            _ => Type::Unknown,
        }
    }
}
