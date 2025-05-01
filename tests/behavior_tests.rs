use c4_rust::parser::Parser;
use c4_rust::vm::VM;

fn run_and_return(source: &str) -> i64 {
    let mut parser = Parser::new(source);
    parser.parse();
    let mut vm = VM::new();
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