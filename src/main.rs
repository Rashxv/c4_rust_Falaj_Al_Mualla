//! This is the entry point for the C4 Rust compiler executable.
//!
//! It handles the following steps:
//! 1. Reads the source code from a `.c` file provided as a command-line argument.
//! 2. Uses the parser to convert the source code into bytecode instructions.
//! 3. Initializes and runs the virtual machine (VM) starting from the `main` function label.
//! 4. Prints the final return value of the executed program.
//!
//! This file ties together the compiler pipeline and serves as the user-facing interface.
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

    use c4_rust::vm::Value; 

    // Push zeroes for top-level locals (outside functions)
    for _ in 0..parser.locals.len() {
        vm.stack.push(Value::Int(0)); //  wrap it in Value::Int
    }
    

    // Execute
    let result = vm.run_from(&parser.code, main_ip)
        .expect("Execution failed");

    println!("\nProgram result: {}", result);
}
