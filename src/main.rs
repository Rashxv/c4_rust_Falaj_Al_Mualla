// src/main.rs
use std::env;
use std::fs;
use c4_rust::parser::Parser;
use c4_rust::vm::VM;

fn main() {
    // Allow passing the file as a CLI argument
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <source_file.c>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let source = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Failed to read file: {}", filename));

    // Compile
    let mut parser = Parser::new(&source);
    parser.parse();

    let main_ip = parser
        .main_label
        .expect("No `main` function found in source");

    let mut vm = VM::new(parser.functions.iter().map(|(name, addr)| {
        let arity = *parser.function_arity.get(name).unwrap_or(&0);
        (*addr, arity)
    }).collect());

    // Push zeroes for top-level locals (outside functions)
    for _ in 0..parser.locals.len() {
        vm.stack.push(0);
    }

    // Execute
    let result = vm.run_from(&parser.code, main_ip)
        .expect("Execution failed");

    println!("\nProgram result: {}", result);
}
