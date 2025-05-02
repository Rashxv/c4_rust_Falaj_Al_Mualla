use c4_rust::parser::Parser;
use c4_rust::vm::VM;

fn run_and_return(source: &str) -> i64 {
    let mut parser = Parser::new(source);
    parser.parse();
    let mut vm = VM::new(parser.functions.iter().map(|(name, addr)| {
        let arity = *parser.function_arity.get(name).unwrap_or(&0);
        (*addr, arity)
    }).collect());
    
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

    let mut vm = VM::new(parser.functions.iter().map(|(name, addr)| {
        let arity = *parser.function_arity.get(name).unwrap_or(&0);
        (*addr, arity)
    }).collect());
    
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
    let mut vm = VM::new(parser.functions.iter().map(|(name, addr)| {
        let arity = *parser.function_arity.get(name).unwrap_or(&0);
        (*addr, arity)
    }).collect());
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

    let mut vm = VM::new(parser.functions.iter().map(|(name, addr)| {
        let arity = *parser.function_arity.get(name).unwrap_or(&0);
        (*addr, arity)
    }).collect());
    
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

    let mut vm = VM::new(parser.functions.iter().map(|(name, addr)| {
        let arity = *parser.function_arity.get(name).unwrap_or(&0);
        (*addr, arity)
    }).collect());
    
    // Push locals for main()
    for _ in 0..parser.locals.len() {
        vm.stack.push(0);
    }

    let main_ip = parser.main_label.expect("No main label found");
    let result = vm.run_from(&parser.code, main_ip);
    assert_eq!(result, Some(12));
}


#[test]
fn test_unary_minus() {
    let source = r#"
    {
        return -5;
    }
    "#;
    assert_eq!(run_and_return(source), -5);
}

#[test]
fn test_unary_not() {
    let source = r#"
    {
        return !0;
    }
    "#;
    assert_eq!(run_and_return(source), 1);

    let source = r#"
    {
        return !123;
    }
    "#;
    assert_eq!(run_and_return(source), 0);
}

#[test]
fn test_address_and_dereference() {
    let src = r#"
    {
        int x;
        x = 99;
        return *&x;
    }
    "#;
    assert_eq!(run_and_return(src), 99);
}

#[test]
fn test_sizeof_int_and_char() {
    // sizeof(int) should be size_of::<i64>() == 8 on your platform
    let int_sz = std::mem::size_of::<i64>() as i64;
    let char_sz = std::mem::size_of::<i8>() as i64;

    let src = r#"{ return sizeof(int); }"#;
    assert_eq!(run_and_return(src), int_sz);

    let src = r#"{ return sizeof(char); }"#;
    assert_eq!(run_and_return(src), char_sz);
}

#[test]
fn test_conditional_operator() {
    let src = r#"{ return 0 ? 123 : 456; }"#;
    assert_eq!(run_and_return(src), 456);
    let src = r#"{ return 1 ? 123 : 456; }"#;
    assert_eq!(run_and_return(src), 123);
}

#[test]
fn test_comparison_operators() {
    assert_eq!(run_and_return("{ return 3 < 4; }"), 1);
    assert_eq!(run_and_return("{ return 4 < 3; }"), 0);
    assert_eq!(run_and_return("{ return 3 == 3; }"), 1);
    assert_eq!(run_and_return("{ return 3 != 3; }"), 0);
    assert_eq!(run_and_return("{ return 5 <= 5; }"), 1);
    assert_eq!(run_and_return("{ return 6 >= 7; }"), 0);
}

#[test]
fn test_bitwise_ops() {
    assert_eq!(run_and_return("{ return 6 & 3; }"), 2);
    assert_eq!(run_and_return("{ return 5 | 2; }"), 7);
    assert_eq!(run_and_return("{ return 5 ^ 3; }"), 6);
}

#[test]
fn test_shifts() {
    assert_eq!(run_and_return("{ return 1 << 4; }"), 16);
    assert_eq!(run_and_return("{ return 16 >> 2; }"), 4);
}

#[test]
fn test_cast_is_noop() {
    assert_eq!(run_and_return("{ return (int)5; }"), 5);
}

#[test]
fn test_unary_deref_and_addr_combined() {
    let src = r#"
    {
        int a;
        a = 7;
        int *p;
        p = &a;
        // now p should hold the “address” of a, so *p == 7
        return *p;
    }
    "#;
    assert_eq!(run_and_return(src), 7);
}