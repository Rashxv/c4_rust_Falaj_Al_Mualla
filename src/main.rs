use c4_rust::parser::Parser;

fn main() {
    let source = r#"
{
    return 1 + 2 * 3 - 4;
}
"#;


    let mut parser = Parser::new(source);
    parser.parse();
}