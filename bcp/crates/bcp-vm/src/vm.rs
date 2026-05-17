use std::collections::HashMap;
use bcp_bytecode::{Instruction, OpCode};
use crate::chunk::Chunk;
use crate::value::Value;

pub struct VM {
    chunks: Vec<Chunk>,
    stack: Vec<Value>,
    globals: HashMap<String, Value>,
    ip: usize,
    chunk_idx: usize,
}

impl VM {
    pub fn new() -> Self {
        VM {
            chunks: vec![],
            stack: vec![],
            globals: HashMap::new(),
            ip: 0,
            chunk_idx: 0,
        }
    }

    pub fn add_chunk(&mut self, chunk: Chunk) -> usize {
        let idx = self.chunks.len();
        self.chunks.push(chunk);
        idx
    }

    pub fn interpret(&mut self, entry_idx: usize) -> Result<Value, String> {
        self.chunk_idx = entry_idx;
        self.ip = 0;
        self.run()
    }

    fn current_chunk(&self) -> &Chunk {
        &self.chunks[self.chunk_idx]
    }

    fn read_instruction(&self) -> &Instruction {
        &self.current_chunk().instructions[self.ip]
    }

    fn read_constant(&self, idx: usize) -> Value {
        self.current_chunk().constants[idx].clone()
    }

    fn run(&mut self) -> Result<Value, String> {
        loop {
            if self.ip >= self.current_chunk().instructions.len() {
                return Err("Program counter超出范围".to_string());
            }

            let inst = self.read_instruction().clone();
            self.ip += 1;

            match inst.opcode {
                OpCode::Constant => {
                    let val = self.read_constant(inst.operands[0]);
                    self.stack.push(val);
                }
                OpCode::Nil => self.stack.push(Value::Nil),
                OpCode::True => self.stack.push(Value::Bool(true)),
                OpCode::False => self.stack.push(Value::Bool(false)),
                OpCode::Pop => {
                    self.stack.pop();
                }
                OpCode::GetLocal => {
                    let idx = inst.operands[0];
                    if idx < self.stack.len() {
                        let val = self.stack[idx].clone();
                        self.stack.push(val);
                    } else {
                        self.stack.push(Value::Nil);
                    }
                }
                OpCode::SetLocal => {
                    let idx = inst.operands[0];
                    let val = self.stack.pop().unwrap_or(Value::Nil);
                    if idx < self.stack.len() {
                        self.stack[idx] = val;
                    }
                }
                OpCode::DefineGlobal => {
                    let val = self.stack.pop().unwrap_or(Value::Nil);
                    let name = format!("var_{}", inst.operands[0]);
                    self.globals.insert(name, val);
                }
                OpCode::GetGlobal => {
                    let name = format!("var_{}", inst.operands[0]);
                    let val = self.globals.get(&name).cloned().unwrap_or(Value::Nil);
                    self.stack.push(val);
                }
                OpCode::SetGlobal => {
                    let val = self.stack.pop().unwrap_or(Value::Nil);
                    let name = format!("var_{}", inst.operands[0]);
                    self.globals.insert(name, val);
                }
                OpCode::Add => {
                    let b = self.stack.pop().unwrap_or(Value::Nil);
                    let a = self.stack.pop().unwrap_or(Value::Nil);
                    let result = match (&a, &b) {
                        (Value::Int(a), Value::Int(b)) => Value::Int(a + b),
                        (Value::Float(a), Value::Float(b)) => Value::Float(a + b),
                        (Value::String(a), Value::String(b)) => {
                            Value::String(format!("{}{}", a, b))
                        }
                        _ => {
                            return Err(format!(
                                "Cannot add {:?} and {:?}", a, b
                            ));
                        }
                    };
                    self.stack.push(result);
                }
                OpCode::Subtract => {
                    let b = self.stack.pop().unwrap_or(Value::Nil);
                    let a = self.stack.pop().unwrap_or(Value::Nil);
                    match (&a, &b) {
                        (Value::Int(a), Value::Int(b)) => self.stack.push(Value::Int(a - b)),
                        (Value::Float(a), Value::Float(b)) => self.stack.push(Value::Float(a - b)),
                        _ => return Err("Cannot subtract".to_string()),
                    }
                }
                OpCode::Multiply => {
                    let b = self.stack.pop().unwrap_or(Value::Nil);
                    let a = self.stack.pop().unwrap_or(Value::Nil);
                    match (&a, &b) {
                        (Value::Int(a), Value::Int(b)) => self.stack.push(Value::Int(a * b)),
                        (Value::Float(a), Value::Float(b)) => self.stack.push(Value::Float(a * b)),
                        _ => return Err("Cannot multiply".to_string()),
                    }
                }
                OpCode::Divide => {
                    let b = self.stack.pop().unwrap_or(Value::Nil);
                    let a = self.stack.pop().unwrap_or(Value::Nil);
                    match (&a, &b) {
                        (Value::Int(a), Value::Int(b)) => {
                            if *b == 0 {
                                return Err("Division by zero".to_string());
                            }
                            self.stack.push(Value::Int(a / b));
                        }
                        (Value::Float(a), Value::Float(b)) => {
                            self.stack.push(Value::Float(a / b));
                        }
                        _ => return Err("Cannot divide".to_string()),
                    }
                }
                OpCode::Mod => {
                    let b = self.stack.pop().unwrap_or(Value::Nil);
                    let a = self.stack.pop().unwrap_or(Value::Nil);
                    match (&a, &b) {
                        (Value::Int(a), Value::Int(b)) => self.stack.push(Value::Int(a % b)),
                        _ => return Err("Cannot mod".to_string()),
                    }
                }
                OpCode::Negate => {
                    let val = self.stack.pop().unwrap_or(Value::Nil);
                    match val {
                        Value::Int(v) => self.stack.push(Value::Int(-v)),
                        Value::Float(v) => self.stack.push(Value::Float(-v)),
                        _ => return Err("Cannot negate".to_string()),
                    }
                }
                OpCode::Not => {
                    let val = self.stack.pop().unwrap_or(Value::Nil);
                    self.stack.push(Value::Bool(!val.is_truthy()));
                }
                OpCode::Equal => {
                    let b = self.stack.pop().unwrap_or(Value::Nil);
                    let a = self.stack.pop().unwrap_or(Value::Nil);
                    self.stack.push(Value::Bool(a == b));
                }
                OpCode::Greater => {
                    let b = self.stack.pop().unwrap_or(Value::Nil);
                    let a = self.stack.pop().unwrap_or(Value::Nil);
                    let result = match (&a, &b) {
                        (Value::Int(a), Value::Int(b)) => Value::Bool(a > b),
                        (Value::Float(a), Value::Float(b)) => Value::Bool(a > b),
                        _ => Value::Bool(false),
                    };
                    self.stack.push(result);
                }
                OpCode::Less => {
                    let b = self.stack.pop().unwrap_or(Value::Nil);
                    let a = self.stack.pop().unwrap_or(Value::Nil);
                    let result = match (&a, &b) {
                        (Value::Int(a), Value::Int(b)) => Value::Bool(a < b),
                        (Value::Float(a), Value::Float(b)) => Value::Bool(a < b),
                        _ => Value::Bool(false),
                    };
                    self.stack.push(result);
                }
                OpCode::Jump => {
                    if inst.operands.is_empty() {
                        return Err("Jump with no target".to_string());
                    }
                    self.ip = inst.operands[0];
                }
                OpCode::JumpIfFalse => {
                    let val = self.stack.last().cloned().unwrap_or(Value::Nil);
                    if !val.is_truthy() && !inst.operands.is_empty() {
                        self.ip = inst.operands[0];
                    }
                }
                OpCode::JumpIfTrue => {
                    let val = self.stack.last().cloned().unwrap_or(Value::Nil);
                    if val.is_truthy() && !inst.operands.is_empty() {
                        self.ip = inst.operands[0];
                    }
                }
                OpCode::Loop => {
                    if !inst.operands.is_empty() {
                        self.ip = inst.operands[0];
                    }
                }
                OpCode::Call => {
                    let _arity = inst.operands.first().copied().unwrap_or(0);
                    // post-MVP: function calls
                    // For now, treat as no-op (pop args)
                    for _ in 0.._arity {
                        self.stack.pop();
                    }
                }
                OpCode::Print => {
                    if let Some(val) = self.stack.last() {
                        println!("{}", val);
                    }
                }
                OpCode::Return => {
                    return Ok(Value::Nil);
                }
                OpCode::ReturnValue => {
                    let val = self.stack.pop().unwrap_or(Value::Nil);
                    return Ok(val);
                }
            }
        }
    }
}
