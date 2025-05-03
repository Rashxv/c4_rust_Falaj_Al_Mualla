/// Enum representing different kinds of tokens.
#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    // Literals
    Num(i64),
    Id(String),

    // Keywords
    If,
    Else,
    While,
    Return,
    Int,
    Char,
    Sizeof,
    Enum,
    Float(f64), 

    // Operators and punctuation
    Assign,  // =
    Add,     // +
    Sub,     // -
    Mul,     // *
    Div,     // /
    Mod,     // %
    Eq,      // ==
    Ne,      // !=
    Lt,      // <
    Gt,      // >
    Le,      // <=
    Ge,      // >=
    And,     // &
    Or,      // |
    Xor,     // ^
    Not,     // !
    Inc,     // ++
    Dec,     // --
    Cond,    // ?
    Lor,     // ||
    Lan,     // &&
    Shl,     // <<
    Shr,     // >>

    // Delimiters
    LParen,  // (
    RParen,  // )
    LBrace,  // {
    RBrace,  // }
    LBracket,// [
    RBracket,// ]
    Semicolon,
    Comma,
    Colon,

    // Special
    String(String),
    CharLiteral(char),
    Eof,
}

/// Struct representing a token with line and position info.
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
}