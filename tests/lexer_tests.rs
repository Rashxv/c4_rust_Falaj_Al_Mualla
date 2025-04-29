use c4_rust::lexer::Lexer;
use c4_rust::token::TokenKind;

fn tokenize_kinds(input: &str) -> Vec<TokenKind> {
    let mut lexer = Lexer::new(input);
    let mut kinds = Vec::new();
    loop {
        let tok = lexer.next_token();
        kinds.push(tok.kind.clone());
        if matches!(tok.kind, TokenKind::Eof) {
            break;
        }
    }
    kinds
}

#[test]
fn test_operators() {
    let input = "== != <= >= && || ++ -- = + - * / %";
    let expected = vec![
        TokenKind::Eq,
        TokenKind::Ne,
        TokenKind::Le,
        TokenKind::Ge,
        TokenKind::Lan,
        TokenKind::Lor,
        TokenKind::Inc,
        TokenKind::Dec,
        TokenKind::Assign,
        TokenKind::Add,
        TokenKind::Sub,
        TokenKind::Mul,
        TokenKind::Div,
        TokenKind::Mod,
        TokenKind::Eof,
    ];
    assert_eq!(tokenize_kinds(input), expected);
}

#[test]
fn test_string_literal() {
    let input = r#" "hello\nworld" "#;
    let expected = vec![
        TokenKind::String("hello\nworld".to_string()),
        TokenKind::Eof,
    ];
    assert_eq!(tokenize_kinds(input), expected);
}

#[test]
fn test_char_literal() {
    let input = r#" 'a' '\n' "#;
    let expected = vec![
        TokenKind::CharLiteral('a'),
        TokenKind::CharLiteral('\n'),
        TokenKind::Eof,
    ];
    assert_eq!(tokenize_kinds(input), expected);
}

#[test]
fn test_skips_comments() {
    let input = r#"
        int main() { // this is a comment
            return 42; // another comment
        }
    "#;

    let expected = vec![
        TokenKind::Int,
        TokenKind::Id("main".to_string()),
        TokenKind::LParen,
        TokenKind::RParen,
        TokenKind::LBrace,
        TokenKind::Return,
        TokenKind::Num(42),
        TokenKind::Semicolon,
        TokenKind::RBrace,
        TokenKind::Eof,
    ];

    assert_eq!(tokenize_kinds(input), expected);
}
