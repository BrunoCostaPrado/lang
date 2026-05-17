#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OpCode {
    Constant,
    Nil,
    True,
    False,
    Pop,
    GetLocal,
    SetLocal,
    GetGlobal,
    SetGlobal,
    DefineGlobal,
    Equal,
    Greater,
    Less,
    Add,
    Subtract,
    Multiply,
    Divide,
    Mod,
    Not,
    Negate,
    Print,
    Jump,
    JumpIfFalse,
    JumpIfTrue,
    Loop,
    Call,
    Return,
    ReturnValue,
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub opcode: OpCode,
    pub operands: Vec<usize>,
    pub line: usize,
}

impl Instruction {
    pub fn new(opcode: OpCode, line: usize) -> Self {
        Instruction {
            opcode,
            operands: vec![],
            line,
        }
    }

    pub fn with_operand(opcode: OpCode, operand: usize, line: usize) -> Self {
        Instruction {
            opcode,
            operands: vec![operand],
            line,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Constant {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Nil,
}

impl Constant {
    pub fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        match self {
            Constant::Nil => bytes.push(0),
            Constant::Bool(b) => {
                bytes.push(1);
                bytes.push(*b as u8);
            }
            Constant::Int(v) => {
                bytes.push(2);
                bytes.extend_from_slice(&v.to_le_bytes());
            }
            Constant::Float(v) => {
                bytes.push(3);
                bytes.extend_from_slice(&v.to_le_bytes());
            }
            Constant::String(s) => {
                bytes.push(4);
                bytes.extend_from_slice(&(s.len() as u32).to_le_bytes());
                bytes.extend_from_slice(s.as_bytes());
            }
        }
        bytes
    }
}
