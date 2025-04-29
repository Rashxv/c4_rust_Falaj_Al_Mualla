use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Token,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut lexer = Lexer::new(source);
        let current = lexer.next_token();
        Self { lexer, current }
    }

    fn next_token(&mut self) {
        self.current = self.lexer.next_token();
    }

    pub fn parse(&mut self) {
        while self.current.kind != TokenKind::Eof {
            self.stmt();
        }
    }

    fn stmt(&mut self) {
        match &self.current.kind {
            TokenKind::If => {
                println!("Parsing 'if' statement");
                self.next_token(); // consume 'if'
    
                if self.current.kind != TokenKind::LParen {
                    panic!("Expected '(' after 'if'");
                }
                self.next_token(); // consume '('
    
                self.expr(); // parse condition
    
                if self.current.kind != TokenKind::RParen {
                    panic!("Expected ')' after 'if' condition");
                }
                self.next_token(); // consume ')'
    
                self.stmt(); // parse the body of if
            }
            TokenKind::While => {
                println!("Parsing 'while' loop");
                self.next_token();
                if self.current.kind != TokenKind::LParen {
                    panic!("Expected '(' after 'while'");
                }
                self.next_token();
                self.expr();
                if self.current.kind != TokenKind::RParen {
                    panic!("Expected ')' after 'while' condition");
                }
                self.next_token();
                self.stmt();
            }
            TokenKind::Return => {
                println!("Parsing 'return' statement");
                self.next_token();
                self.expr();
                if self.current.kind == TokenKind::Semicolon {
                    self.next_token();
                } else {
                    panic!("Expected semicolon after return expression");
                }
            }
            TokenKind::LBrace => {
                println!("Parsing block");
                self.next_token();
                while self.current.kind != TokenKind::RBrace {
                    self.stmt();
                }
                self.next_token(); // consume '}'
            }
            _ => {
                println!("Parsing expression statement");
                self.expr();
                if self.current.kind == TokenKind::Semicolon {
                    self.next_token();
                } else {
                    panic!("Expected semicolon after expression");
                }
            }
        }
    }
    

    fn expr(&mut self) {
        self.expr_bp(0); // starting at lowest precedence
    }

    fn expr_bp(&mut self, min_bp: u8) {
        println!("Parsing expression starting with {:?}", self.current.kind);

        // Parse atomic values first
        match &self.current.kind {
            TokenKind::Num(n) => {
                println!("Found number: {}", n);
                self.next_token();
            }
            TokenKind::Id(name) => {
                println!("Found identifier: {}", name);
                self.next_token();
            }
            TokenKind::LParen => {
                self.next_token(); // consume '('
                self.expr_bp(0);   // parse inner expression
                if self.current.kind != TokenKind::RParen {
                    panic!("Expected ')'");
                }
                self.next_token(); // consume ')'
            }
            _ => {
                panic!("Unexpected token in expression: {:?}", self.current.kind);
            }
        }

        loop {
            let op_bp = self.get_precedence();
            if op_bp == 0 || op_bp < min_bp {
                break;
            }
            

            let op = self.current.kind.clone();
            self.next_token();
            println!("Parsing operator: {:?}", op);

            self.expr_bp(op_bp + 1); // parse right-hand side with higher precedence
        }
    }

    // Simple precedence rules based on C4
    fn get_precedence(&self) -> u8 {
        match self.current.kind {
            TokenKind::Assign => 1,
            TokenKind::Lor    => 2,
            TokenKind::Lan    => 3,
            TokenKind::Or     => 4,
            TokenKind::Xor    => 5,
            TokenKind::And    => 6,
            TokenKind::Eq | TokenKind::Ne => 7,
            TokenKind::Lt | TokenKind::Gt | TokenKind::Le | TokenKind::Ge => 8,
            TokenKind::Shl | TokenKind::Shr => 9,
            TokenKind::Add | TokenKind::Sub => 10,
            TokenKind::Mul | TokenKind::Div | TokenKind::Mod => 11,
            _ => 0,
        }
    }
}