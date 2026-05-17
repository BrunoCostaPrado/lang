use bcp_parser::ast::*;
use crate::instruction::{Constant, Instruction, OpCode};
use std::collections::HashMap;

pub struct Chunk {
    pub instructions: Vec<Instruction>,
    pub constants: Vec<Constant>,
    pub name: String,
}

impl Chunk {
    pub fn new(name: &str) -> Self {
        Chunk {
            instructions: vec![],
            constants: vec![],
            name: name.to_string(),
        }
    }

    pub fn add_constant(&mut self, val: Constant) -> usize {
        let idx = self.constants.len();
        self.constants.push(val);
        idx
    }

    fn emit(&mut self, inst: Instruction) {
        self.instructions.push(inst);
    }
}

pub struct Codegen {
    locals: Vec<HashMap<String, usize>>,
    #[allow(dead_code)]
    chunks: Vec<Chunk>,
    #[allow(dead_code)]
    current_chunk: usize,
}

impl Codegen {
    pub fn new() -> Self {
        Codegen {
            locals: vec![HashMap::new()],
            chunks: vec![],
            current_chunk: 0,
        }
    }

    pub fn generate(&mut self, program: &Program) -> Result<Vec<Chunk>, String> {
        let mut main_chunk = Chunk::new("main");
        self.compile_program(program, &mut main_chunk)?;
        Ok(vec![main_chunk])
    }

    fn compile_program(&mut self, program: &Program, chunk: &mut Chunk) -> Result<(), String> {
        for stmt in &program.stmts {
            self.compile_stmt(stmt, chunk)?;
        }
        chunk.emit(Instruction::new(OpCode::Nil, 0));
        chunk.emit(Instruction::new(OpCode::Return, 0));
        Ok(())
    }

    fn compile_stmt(&mut self, stmt: &Stmt, chunk: &mut Chunk) -> Result<(), String> {
        match stmt {
            Stmt::Expression(expr) => {
                self.compile_expr(expr, chunk)?;
                chunk.emit(Instruction::new(OpCode::Pop, 0));
                Ok(())
            }
            Stmt::VarDeclaration(decl) => {
                if let Some(val) = &decl.value {
                    self.compile_expr(val, chunk)?;
                } else {
                    chunk.emit(Instruction::new(OpCode::Nil, 0));
                }
                let idx = self.locals.last().unwrap().len();
                self.locals.last_mut().unwrap().insert(decl.name.clone(), idx);
                chunk.emit(Instruction::with_operand(OpCode::DefineGlobal, idx, 0));
                Ok(())
            }
            Stmt::Block(stmts) => {
                self.locals.push(HashMap::new());
                for s in stmts {
                    self.compile_stmt(s, chunk)?;
                }
                self.locals.pop();
                Ok(())
            }
            Stmt::Return(val) => {
                if let Some(expr) = val {
                    self.compile_expr(expr, chunk)?;
                    chunk.emit(Instruction::new(OpCode::ReturnValue, 0));
                } else {
                    chunk.emit(Instruction::new(OpCode::Return, 0));
                }
                Ok(())
            }
            Stmt::While(condition, body) => {
                let loop_start = chunk.instructions.len();
                self.compile_expr(condition, chunk)?;
                let exit_jump = chunk.instructions.len();
                chunk.emit(Instruction::new(OpCode::JumpIfFalse, 0));
                chunk.emit(Instruction::new(OpCode::Pop, 0));
                for s in body {
                    self.compile_stmt(s, chunk)?;
                }
                chunk.emit(Instruction::with_operand(OpCode::Loop, loop_start, 0));
                let exit_pos = chunk.instructions.len();
                if let Some(inst) = chunk.instructions.get_mut(exit_jump) {
                    inst.operands = vec![exit_pos + 1];
                }
                chunk.emit(Instruction::new(OpCode::Pop, 0));
                Ok(())
            }
            Stmt::For { var: _, iter, body } => {
                self.compile_expr(iter, chunk)?;
                chunk.emit(Instruction::new(OpCode::Pop, 0));
                self.locals.push(HashMap::new());
                for s in body {
                    self.compile_stmt(s, chunk)?;
                }
                self.locals.pop();
                Ok(())
            }
            Stmt::FnDeclaration(_fndecl) => {
                Ok(())
            }
        }
    }

    fn compile_expr(&mut self, expr: &Expr, chunk: &mut Chunk) -> Result<(), String> {
        match expr {
            Expr::Int(v) => {
                let idx = chunk.add_constant(Constant::Int(*v));
                chunk.emit(Instruction::with_operand(OpCode::Constant, idx, 0));
                Ok(())
            }
            Expr::Float(v) => {
                let idx = chunk.add_constant(Constant::Float(*v));
                chunk.emit(Instruction::with_operand(OpCode::Constant, idx, 0));
                Ok(())
            }
            Expr::Bool(v) => {
                if *v {
                    chunk.emit(Instruction::new(OpCode::True, 0));
                } else {
                    chunk.emit(Instruction::new(OpCode::False, 0));
                }
                Ok(())
            }
            Expr::String(s) => {
                let idx = chunk.add_constant(Constant::String(s.clone()));
                chunk.emit(Instruction::with_operand(OpCode::Constant, idx, 0));
                Ok(())
            }
            Expr::Null => {
                chunk.emit(Instruction::new(OpCode::Nil, 0));
                Ok(())
            }
            Expr::Identifier(name) => {
                if let Some(idx) = self.locals.last().unwrap().get(name) {
                    chunk.emit(Instruction::with_operand(OpCode::GetLocal, *idx, 0));
                } else {
                    chunk.emit(Instruction::new(OpCode::Nil, 0));
                }
                Ok(())
            }
            Expr::Binary { left, operator, right } => {
                self.compile_expr(left, chunk)?;
                self.compile_expr(right, chunk)?;
                match operator {
                    BinaryOp::Add => chunk.emit(Instruction::new(OpCode::Add, 0)),
                    BinaryOp::Sub => chunk.emit(Instruction::new(OpCode::Subtract, 0)),
                    BinaryOp::Mul => chunk.emit(Instruction::new(OpCode::Multiply, 0)),
                    BinaryOp::Div => chunk.emit(Instruction::new(OpCode::Divide, 0)),
                    BinaryOp::Mod => chunk.emit(Instruction::new(OpCode::Mod, 0)),
                    BinaryOp::Equal => chunk.emit(Instruction::new(OpCode::Equal, 0)),
                    BinaryOp::NotEqual => {
                        chunk.emit(Instruction::new(OpCode::Equal, 0));
                        chunk.emit(Instruction::new(OpCode::Not, 0));
                    }
                    BinaryOp::Less => chunk.emit(Instruction::new(OpCode::Less, 0)),
                    BinaryOp::Greater => chunk.emit(Instruction::new(OpCode::Greater, 0)),
                    BinaryOp::LessEqual => {
                        chunk.emit(Instruction::new(OpCode::Greater, 0));
                        chunk.emit(Instruction::new(OpCode::Not, 0));
                    }
                    BinaryOp::GreaterEqual => {
                        chunk.emit(Instruction::new(OpCode::Less, 0));
                        chunk.emit(Instruction::new(OpCode::Not, 0));
                    }
                    BinaryOp::And | BinaryOp::Or => {
                        chunk.emit(Instruction::new(OpCode::Pop, 0));
                    }
                }
                Ok(())
            }
            Expr::Unary { operator, right } => {
                self.compile_expr(right, chunk)?;
                match operator {
                    UnaryOp::Negate => chunk.emit(Instruction::new(OpCode::Negate, 0)),
                    UnaryOp::Not => chunk.emit(Instruction::new(OpCode::Not, 0)),
                }
                Ok(())
            }
            Expr::Assignment { target, value } => {
                self.compile_expr(value, chunk)?;
                if let Expr::Identifier(name) = target.as_ref() {
                    if let Some(idx) = self.locals.last().unwrap().get(name) {
                        chunk.emit(Instruction::with_operand(OpCode::SetLocal, *idx, 0));
                    }
                }
                Ok(())
            }
            Expr::Call { callee, args } => {
                self.compile_expr(callee, chunk)?;
                for arg in args {
                    self.compile_expr(arg, chunk)?;
                }
                chunk.emit(Instruction::with_operand(OpCode::Call, args.len(), 0));
                Ok(())
            }
            Expr::If { condition, then_branch, else_branch } => {
                self.compile_expr(condition, chunk)?;
                let else_jump = chunk.instructions.len();
                chunk.emit(Instruction::new(OpCode::JumpIfFalse, 0));
                chunk.emit(Instruction::new(OpCode::Pop, 0));
                self.compile_expr(then_branch, chunk)?;
                let end_jump = chunk.instructions.len();
                chunk.emit(Instruction::new(OpCode::Jump, 0));
                let else_pos = chunk.instructions.len();
                if let Some(inst) = chunk.instructions.get_mut(else_jump) {
                    inst.operands = vec![else_pos];
                }
                chunk.emit(Instruction::new(OpCode::Pop, 0));
                if let Some(else_expr) = else_branch {
                    self.compile_expr(else_expr, chunk)?;
                } else {
                    chunk.emit(Instruction::new(OpCode::Nil, 0));
                }
                let end_pos = chunk.instructions.len();
                if let Some(inst) = chunk.instructions.get_mut(end_jump) {
                    inst.operands = vec![end_pos];
                }
                Ok(())
            }
            Expr::FieldAccess { object, field: _ } => {
                self.compile_expr(object, chunk)?;
                Ok(())
            }
            Expr::StructLiteral { name: _, fields } => {
                for (_, val) in fields {
                    self.compile_expr(val, chunk)?;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
