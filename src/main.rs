use std::fs;
use c4_rust::parser::Parser;
use c4_rust::vm::VM;

fn main() {
    let source = fs::read_to_string("input/c4_original.c").expect("Failed to read C4 source");

    let mut parser = Parser::new(&source);
    parser.parse();

    if let Some(main_ip) = parser.main_label {
        let mut vm = VM::new();

        for _ in 0..parser.locals.len() {
            vm.stack.push(0);
        }

        let result = vm.run_from(&parser.code, main_ip);
        println!("C4 self-hosted result: {:?}", result);
    } else {
        println!("No main() function found.");
    }
}
