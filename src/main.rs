use c4_rust::parser::Parser;
use c4_rust::vm::VM;

fn main() {
    let source = r#"
    {
        return 1 + 2 * 3 - 4;
    }
    "#;

    let mut parser = Parser::new(source);
    parser.parse();

    println!("\n=== Emitted Instructions ===");
    for (i, instr) in parser.code.iter().enumerate() {
        println!("{:04}: {:?}", i, instr);
    }

    println!("\n=== Executing ===");
    let mut vm = VM::new();
    if let Some(result) = vm.run(&parser.code) {
        println!("Program returned: {}", result);
    } else {
        println!("No result returned.");
    }
}
