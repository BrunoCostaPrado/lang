use std::fmt;

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Bool(b) => *b,
            _ => true,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Bool(b) => write!(f, "{b}"),
            Value::Int(v) => write!(f, "{v}"),
            Value::Float(v) => write!(f, "{v}"),
            Value::String(s) => write!(f, "{s}"),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Nil, Value::Nil) => true,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => (a - b).abs() < f64::EPSILON,
            (Value::String(a), Value::String(b)) => a == b,
            _ => false,
        }
    }
}

impl From<bcp_bytecode::Constant> for Value {
    fn from(c: bcp_bytecode::Constant) -> Self {
        match c {
            bcp_bytecode::Constant::Nil => Value::Nil,
            bcp_bytecode::Constant::Bool(b) => Value::Bool(b),
            bcp_bytecode::Constant::Int(v) => Value::Int(v),
            bcp_bytecode::Constant::Float(v) => Value::Float(v),
            bcp_bytecode::Constant::String(s) => Value::String(s),
        }
    }
}
