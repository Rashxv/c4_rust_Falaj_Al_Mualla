// src/main.rs
use std::fs;
use c4_rust::parser::Parser;
use c4_rust::vm::VM;

fn main() {
    // 1. Read the original C4 source
    let source = fs::read_to_string("input/c4_original.c")
        .expect("Failed to read C4 source");

    // 2. Parse & compile it into our Instruction stream
    let mut parser = Parser::new(&source);
    parser.parse();

    // 3. Find where `main` begins
    let main_ip = parser
        .main_label
        .expect("No `main` function found in C4 source");

    // 4. Prepare the VM (push N zero-inits for locals)
    let mut vm = VM::new(parser.functions.iter().map(|(name, addr)| {
        let arity = *parser.function_arity.get(name).unwrap_or(&0);
        (*addr, arity)
    }).collect());
        for _ in 0..parser.locals.len() {
        vm.stack.push(0);
    }

    // 5. Execute (self-host!)
    let result = vm.run_from(&parser.code, main_ip)
        .expect("VM didn't return a value");

    println!("C4 self-hosted result: {}", result);
}
