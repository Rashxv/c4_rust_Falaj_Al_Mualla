use c4_rust::parser::Parser;
use c4_rust::vm::VM;

fn run_and_return(source: &str) -> i64 {
    let mut parser = Parser::new(source);
    parser.parse();
    let mut vm = VM::new();

    // Simulate a function frame with N local variables
    let num_locals = parser.locals.len();
    for _ in 0..num_locals {
        vm.stack.push(0);
    }
    
    vm.run(&parser.code).expect("VM did not return a value")
    
}

#[test]
fn test_if_false_returns_second() {
    let source = r#"
    {
        if (0) return 123;
        return 456;
    }
    "#;
    assert_eq!(run_and_return(source), 456);
}

#[test]
fn test_if_true_returns_first() {
    let source = r#"
    {
        if (1) return 42;
        return 99;
    }
    "#;
    assert_eq!(run_and_return(source), 42);
}

#[test]
fn test_while_never_runs() {
    let source = r#"
    {
        while (0) return 1;
        return 2;
    }
    "#;
    assert_eq!(run_and_return(source), 2);
}

#[test]
fn test_simple_expression() {
    let source = r#"
    {
        return 1 + 2 * 3 - 4;
    }
    "#;
    assert_eq!(run_and_return(source), 3);
}

#[test]
fn test_variable_declaration_and_use() {
    let source = r#"
        {
            int x;
            x = 3 * 4;
            return x + 2;
        }
    "#;

    let mut parser = Parser::new(source);
    parser.parse();

    let mut vm = VM::new();

    // FIX: Simulate function frame by pushing local variable space
    let num_locals = parser.locals.len();
    for _ in 0..num_locals {
        vm.stack.push(0);
    }

    let result = vm.run(&parser.code);
    assert_eq!(result, Some(14)); // x = 12; return x + 2;
}

#[test]
fn test_function_call_with_args() {
    let source = r#"
    int add() {
        return 2 + 3;
    }

    int main() {
        return add();
    }
    "#;

    let mut parser = Parser::new(source);
    parser.parse();
    let mut vm = VM::new();
    let result = vm.run_from(&parser.code, parser.main_label.unwrap());
    assert_eq!(result, Some(5));
}

#[test]
fn test_function_call() {
    let source = r#"
        int add() {
            return 2 + 3;
        }

        int main() {
            return add();
        }
    "#;

    let mut parser = c4_rust::parser::Parser::new(source);
    parser.parse();

    let mut vm = c4_rust::vm::VM::new();

    // Simulate locals (main doesn't use any, but parser sets this up)
    for _ in 0..parser.locals.len() {
        vm.stack.push(0);
    }

    let main_ip = parser.main_label.expect("No main() label");
    let result = vm.run_from(&parser.code, main_ip);
    assert_eq!(result, Some(5));
}

#[test]
fn test_function_with_arguments() {
    let source = r#"
        int add(int a, int b) {
            return a + b;
        }

        int main() {
            return add(7, 5);
        }
    "#;

    let mut parser = c4_rust::parser::Parser::new(source);
    parser.parse();

    let mut vm = c4_rust::vm::VM::new();

    // Push locals for main()
    for _ in 0..parser.locals.len() {
        vm.stack.push(0);
    }

    let main_ip = parser.main_label.expect("No main label found");
    let result = vm.run_from(&parser.code, main_ip);
    assert_eq!(result, Some(12));
}
