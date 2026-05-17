use bcp_vm::{Chunk, VM, Value};
use bcp_bytecode::{Instruction, OpCode, Constant};

#[test]
fn test_simple_addition() {
    let mut vm = VM::new();

    let mut bc = bcp_bytecode::Chunk::new("test");
    let c1 = bc.add_constant(Constant::Int(1));
    bc.instructions.push(Instruction::with_operand(OpCode::Constant, c1, 1));
    let c2 = bc.add_constant(Constant::Int(2));
    bc.instructions.push(Instruction::with_operand(OpCode::Constant, c2, 1));
    bc.instructions.push(Instruction::new(OpCode::Add, 1));
    bc.instructions.push(Instruction::new(OpCode::ReturnValue, 1));

    let chunk = Chunk::from_bytecode_chunk(bc);
    let idx = vm.add_chunk(chunk);
    let result = vm.interpret(idx);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Int(3));
}

#[test]
fn test_comparison() {
    let mut vm = VM::new();
    let mut bc = bcp_bytecode::Chunk::new("test");
    let c5 = bc.add_constant(Constant::Int(5));
    let c3 = bc.add_constant(Constant::Int(3));
    bc.instructions.push(Instruction::with_operand(OpCode::Constant, c5, 1));
    bc.instructions.push(Instruction::with_operand(OpCode::Constant, c3, 1));
    bc.instructions.push(Instruction::new(OpCode::Greater, 1));
    bc.instructions.push(Instruction::new(OpCode::ReturnValue, 1));

    let chunk = Chunk::from_bytecode_chunk(bc);
    let idx = vm.add_chunk(chunk);
    let result = vm.interpret(idx);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Bool(true));
}
