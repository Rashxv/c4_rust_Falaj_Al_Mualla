use c4_rust::lexer::Lexer;
use c4_rust::token::TokenKind;

fn main() {
    let source = r#"
        int main() {
            return 42;
        }
    "#;

    let mut lexer = Lexer::new(source);
    loop {
        let tok = lexer.next_token();
        println!("{:?}", tok);
        if matches!(tok.kind, TokenKind::Eof) {
            break;
        }
    }
}
