// src/lexer.rs
use crate::token::{Token, TokenKind};

pub struct Lexer<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    line: usize,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer from source code string.
    pub fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars().peekable(),
            line: 1,
        }
    }

    /// Consume the next character, updating line count on `\n`.
    fn bump(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        if c == '\n' {
            self.line += 1;
        }
        Some(c)
    }

    /// Peek the upcoming character without consuming.
    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    /// Skip whitespace and `//` comments.
    // src/lexer.rs, inside impl<'a> Lexer<'a>
    fn skip_whitespace_and_comments(&mut self) {
        loop {
            // 1) Skip all whitespace
            while let Some(&c) = self.chars.peek() {
                if c.is_whitespace() {
                    self.bump();
                } else {
                    break;
                }
            }

            // 2) If we see "//", skip to end of line
            if self.peek() == Some('/') {
                // peek second character
                let mut clone = self.chars.clone();
                clone.next();
                if clone.peek() == Some(&'/') {
                    // consume "//"
                    self.bump();
                    self.bump();
                    // skip until newline or EOF
                    while let Some(c) = self.bump() {
                        if c == '\n' {
                            break;
                        }
                    }
                    // now re-loop to skip any whitespace/comments again
                    continue;
                }
            }

            // nothing more to skip
            break;
        }
    }

    /// Return the next token.
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace_and_comments();

        let line = self.line;
        let c = match self.peek() {
            Some(c) => c,
            None => {
                return Token {
                    kind: TokenKind::Eof,
                    line,
                };
            }
        };

        // Numbers
        // at the top of next_token():
        if c.is_ascii_digit() {
            let mut s = String::new();
            let mut has_dot = false;
            // consume all digits, allowing one dot
            while let Some(&ch) = self.chars.peek() {
                match ch {
                    '0'..='9' => {
                        s.push(ch);
                        self.bump();
                    }
                    '.' if !has_dot => {
                        has_dot = true;
                        s.push('.');
                        self.bump();
                    }
                    _ => break
                }
            }
            let line = self.line;
            if has_dot {
                let f = s.parse::<f64>()
                    .expect("lexer: invalid float literal");
                return Token { kind: TokenKind::Float(f), line };
            } else {
                let i = s.parse::<i64>()
                    .expect("lexer: invalid integer literal");
                return Token { kind: TokenKind::Num(i), line };
            }
        }

        // Identifiers or keywords
        if c.is_ascii_alphabetic() || c == '_' {
            return self.lex_identifier_or_keyword();
        }
        // String literal
        if c == '"' {
            return self.lex_string_literal();
        }
        // Char literal
        if c == '\'' {
            return self.lex_char_literal();
        }

        // Multi-char operators & single-char tokens
        let tok = match c {
            '=' => {
                self.bump();
                if self.peek() == Some('=') {
                    self.bump();
                    TokenKind::Eq
                } else {
                    TokenKind::Assign
                }
            }
            '!' => {
                self.bump();
                if self.peek() == Some('=') {
                    self.bump();
                    TokenKind::Ne
                } else {
                    TokenKind::Not
                }
            }
            '<' => {
                self.bump();
                if self.peek() == Some('=') {
                    self.bump();
                    TokenKind::Le
                } else if self.peek() == Some('<') {
                    self.bump();
                    TokenKind::Shl
                } else {
                    TokenKind::Lt
                }
            }
            '>' => {
                self.bump();
                if self.peek() == Some('=') {
                    self.bump();
                    TokenKind::Ge
                } else if self.peek() == Some('>') {
                    self.bump();
                    TokenKind::Shr
                } else {
                    TokenKind::Gt
                }
            }
            '&' => {
                self.bump();
                if self.peek() == Some('&') {
                    self.bump();
                    TokenKind::Lan
                } else {
                    TokenKind::And
                }
            }

            '|' => {
                self.bump();
                if self.peek() == Some('|') {
                    self.bump();
                    TokenKind::Lor
                } else {
                    TokenKind::Or
                }
            }
            '^' => {
                self.bump();
                TokenKind::Xor
            }

            '+' => {
                self.bump();
                if self.peek() == Some('+') {
                    self.bump();
                    TokenKind::Inc
                } else {
                    TokenKind::Add
                }
            }
            '-' => {
                self.bump();
                if self.peek() == Some('-') {
                    self.bump();
                    TokenKind::Dec
                } else {
                    TokenKind::Sub
                }
            }
            '*' => {
                self.bump();
                TokenKind::Mul
            }
            '/' => {
                self.bump();
                TokenKind::Div
            }
            '%' => {
                self.bump();
                TokenKind::Mod
            }
            '?' => {
                self.bump();
                TokenKind::Cond
            }
            ':' => {
                self.bump();
                TokenKind::Colon
            }
            ';' => {
                self.bump();
                TokenKind::Semicolon
            }
            ',' => {
                self.bump();
                TokenKind::Comma
            }
            '(' => {
                self.bump();
                TokenKind::LParen
            }
            ')' => {
                self.bump();
                TokenKind::RParen
            }
            '{' => {
                self.bump();
                TokenKind::LBrace
            }
            '}' => {
                self.bump();
                TokenKind::RBrace
            }
            '[' => {
                self.bump();
                TokenKind::LBracket
            }
            ']' => {
                self.bump();
                TokenKind::RBracket
            }
            _ => {
                // Unknown character: skip and get the next token
                self.bump();
                return self.next_token();
            }
        };

        Token { kind: tok, line }
    }

    /// Lex an integer literal.
    fn lex_number(&mut self) -> Token {
        let line = self.line;
        let mut val = 0i64;
        while let Some(c) = self.peek() {
            if let Some(d) = c.to_digit(10) {
                val = val * 10 + d as i64;
                self.bump();
            } else {
                break;
            }
        }
        Token {
            kind: TokenKind::Num(val),
            line,
        }
    }

    /// Lex identifiers and keywords.
    fn lex_identifier_or_keyword(&mut self) -> Token {
        let line = self.line;
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || c == '_' {
                s.push(c);
                self.bump();
            } else {
                break;
            }
        }
        let kind = match s.as_str() {
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "while" => TokenKind::While,
            "return" => TokenKind::Return,
            "int" => TokenKind::Int,
            "char" => TokenKind::Char,
            "sizeof" => TokenKind::Sizeof,
            "enum" => TokenKind::Enum,
            _ => TokenKind::Id(s),
        };
        Token { kind, line }
    }

    /// Lex a string literal, handling escape sequences.
    fn lex_string_literal(&mut self) -> Token {
        let line = self.line;
        self.bump(); // consume opening `"`
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if c == '"' {
                self.bump();
                break;
            }
            if c == '\\' {
                self.bump();
                if let Some(esc) = self.bump() {
                    let real = match esc {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '\\' => '\\',
                        '"' => '"',
                        '\'' => '\'',
                        other => other,
                    };
                    s.push(real);
                    continue;
                }
            } else {
                s.push(c);
                self.bump();
            }
        }
        Token {
            kind: TokenKind::String(s),
            line,
        }
    }

    /// Lex a character literal, handling escape sequences.
    fn lex_char_literal(&mut self) -> Token {
        let line = self.line;
        self.bump(); // consume opening `'`
        let mut ch = '\0';
        if let Some(c) = self.bump() {
            if c == '\\' {
                if let Some(esc) = self.bump() {
                    ch = match esc {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '\\' => '\\',
                        '\'' => '\'',
                        other => other,
                    };
                }
            } else {
                ch = c;
            }
        }
        // consume closing `'`
        if self.peek() == Some('\'') {
            self.bump();
        }
        Token {
            kind: TokenKind::CharLiteral(ch),
            line,
        }
    }
}
