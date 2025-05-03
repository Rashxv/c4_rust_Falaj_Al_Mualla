/// The `Token` and `TokenKind` enums define the lexical units recognized by the lexer during tokenization
/// of the C-like source code. Each token consists of a kind (such as identifiers, keywords, literals, or symbols)
/// and its corresponding span within the input text. `TokenKind` enumerates all the valid categories of tokens,
/// including control keywords, operators, punctuation, literals, and type names. These tokens form the input
/// stream consumed by the parser to construct the program’s abstract syntax and semantics.
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