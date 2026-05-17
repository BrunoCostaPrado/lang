use std::env;
use std::fs;
use std::io::{self, Write};

use bcp_lexer::tokenize;
use bcp_parser::Parser;
use bcp_typeck::TypeChecker;
use bcp_bytecode::Codegen;
use bcp_vm::{Chunk as VmChunk, VM};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        repl();
        return;
    }

    match args[1].as_str() {
        "run" => {
            if args.len() < 3 {
                eprintln!("Usage: bcp run <file.bcp>");
                std::process::exit(1);
            }
            run_file(&args[2]);
        }
        "build" => {
            if args.len() < 3 {
                eprintln!("Usage: bcp build <file.bcp>");
                std::process::exit(1);
            }
            build_file(&args[2]);
        }
        "repl" => repl(),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            eprintln!("Usage: bcp <run|build|repl> [file]");
            std::process::exit(1);
        }
    }
}

fn run_file(path: &str) {
    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    };
    execute_source(&source);
}

fn build_file(path: &str) {
    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    };

    let tokens = match tokenize(&source) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Lex error: {}", e);
            std::process::exit(1);
        }
    };
    println!("Tokenization successful. {} tokens.", tokens.len());
    for tok in &tokens {
        println!("  {}", tok);
    }
}

fn repl() {
    println!("BCP Language v0.1.0 REPL");
    println!("Type 'exit' to quit.");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }
        let input = input.trim();
        if input.is_empty() || input == "exit" {
            break;
        }
        execute_source(input);
    }
}

fn execute_source(source: &str) {
    // Step 1: Lex
    let tokens = match tokenize(source) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Lex error: {}", e);
            return;
        }
    };

    // Step 2: Parse
    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Parse error: {}", e);
            return;
        }
    };

    // Step 3: Type check
    let mut checker = TypeChecker::new();
    if let Err(errors) = checker.check(&program) {
        for e in errors {
            eprintln!("Type error: {}", e);
        }
        return;
    }

    // Step 4: Codegen
    let mut codegen = Codegen::new();
    let chunks = match codegen.generate(&program) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Codegen error: {}", e);
            return;
        }
    };

    // Step 5: Run in VM
    let mut vm = VM::new();
    for bc in chunks {
        let vmc = VmChunk::from_bytecode_chunk(bc);
        let idx = vm.add_chunk(vmc);
        match vm.interpret(idx) {
            Ok(val) => {
                println!("{}", val);
            }
            Err(e) => {
                eprintln!("Runtime error: {}", e);
            }
        }
    }
}
