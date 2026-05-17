use bcp_lexer::tokenize;
use bcp_parser::Parser;
use bcp_typeck::TypeChecker;
use bcp_bytecode::Codegen;
use bcp_vm::{Chunk, VM};

fn run_source(source: &str) -> Result<String, String> {
    let tokens = tokenize(source).map_err(|e| format!("Lex: {}", e))?;
    let mut parser = Parser::new(tokens);
    let program = parser.parse().map_err(|e| format!("Parse: {}", e))?;
    let mut checker = TypeChecker::new();
    checker.check(&program).map_err(|errors| format!("Type: {:?}", errors))?;
    let mut codegen = Codegen::new();
    let chunks = codegen.generate(&program).map_err(|e| format!("Codegen: {}", e))?;
    let mut vm = VM::new();
    let mut results = Vec::new();
    for bc in chunks {
        let vmc = Chunk::from_bytecode_chunk(bc);
        let idx = vm.add_chunk(vmc);
        let val = vm.interpret(idx).map_err(|e| format!("Runtime: {}", e))?;
        results.push(format!("{}", val));
    }
    Ok(results.join("\n"))
}

#[test]
fn test_var_decl() {
    let _result = run_source("let x = 42").unwrap();
    // just verify it doesn't crash — result is nil since codegen pops expression
}

#[test]
fn test_simple_addition() {
    // Expressions that leave a value on the stack before implicit return
    let source = "1 + 2";
    let tokens = tokenize(source).unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut checker = TypeChecker::new();
    assert!(checker.check(&program).is_ok());
}

#[test]
fn test_typeck_accepts_valid() {
    let source = "let x i32 = 10";
    let tokens = tokenize(source).unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut checker = TypeChecker::new();
    assert!(checker.check(&program).is_ok());
}

#[test]
fn test_codegen_produces_instructions() {
    let source = "let x = 42";
    let tokens = tokenize(source).unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut codegen = Codegen::new();
    let chunks = codegen.generate(&program).unwrap();
    assert!(!chunks.is_empty());
    assert!(!chunks[0].instructions.is_empty());
}

#[test]
fn test_full_pipeline_no_crash() {
    let source = "
let x = 10
let y = 20
let z = x + y
";
    let result = run_source(source);
    assert!(result.is_ok());
}

#[test]
fn test_if_expression() {
    let source = "if true { 1 } else { 2 }";
    let tokens = tokenize(source).unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut checker = TypeChecker::new();
    assert!(checker.check(&program).is_ok());
}

#[test]
fn test_while_loop() {
    let source = "
let x = 0
while x < 5 {
    let x = x + 1
}
";
    let tokens = tokenize(source).unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut checker = TypeChecker::new();
    assert!(checker.check(&program).is_ok());
}
