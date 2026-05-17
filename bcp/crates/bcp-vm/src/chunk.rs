use bcp_bytecode::Chunk as BytecodeChunk;
use crate::value::Value;

pub struct Chunk {
    pub instructions: Vec<bcp_bytecode::Instruction>,
    pub constants: Vec<Value>,
    pub name: String,
}

impl Chunk {
    pub fn from_bytecode_chunk(bc: BytecodeChunk) -> Self {
        let constants: Vec<Value> = bc.constants.into_iter().map(Value::from).collect();
        Chunk {
            instructions: bc.instructions,
            constants,
            name: bc.name,
        }
    }
}
