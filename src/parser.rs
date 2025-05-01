use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};
use crate::instruction::Instruction;
use std::collections::HashMap;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Token,
    pub code: Vec<Instruction>,
    label_id: usize,
    pub functions: HashMap<String, usize>, // in struct Parser
    pub main_label: Option<usize>,
    pub locals: HashMap<String, usize>,
    next_local_offset: usize,       
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut lexer = Lexer::new(source);
        let current = lexer.next_token();
        Self {
            lexer,
            current,
            code: Vec::new(),
            label_id: 0,
            locals: HashMap::new(),
            next_local_offset: 0,
            functions: HashMap::new(),
            main_label: None,
        }
            }

    fn next_token(&mut self) {
        self.current = self.lexer.next_token();
    }
    
    pub fn parse(&mut self) {
        while self.current.kind != TokenKind::Eof {
            if self.current.kind == TokenKind::Int {
                self.next_token();
            
                if let TokenKind::Id(name) = &self.current.kind {
                    let func_name = name.clone();
                     // —— SCOPING FIX — reset locals for each function ——
                     self.locals.clear();
                     self.next_local_offset = 0;
                     self.next_token();
                     if self.current.kind == TokenKind::LParen {
                        self.next_token(); // consume '('
                        
                        let mut param_names = vec![];

                        while self.current.kind == TokenKind::Int || self.current.kind == TokenKind::Char {
                            self.next_token(); // consume type

                            if let TokenKind::Id(param_name) = &self.current.kind {
                                param_names.push(param_name.clone());
                                self.next_token(); // consume identifier

                                if self.current.kind == TokenKind::Comma {
                                    self.next_token(); // consume ','
                                } else {
                                    break;
                                }
                            } else {
                                panic!("Expected identifier in parameter list");
                            }
                        }

                        // Assign stack offsets in order (leftmost argument = offset 0)
                        for (i, name) in param_names.iter().enumerate() {
                            self.locals.insert(name.clone(), i);
                            self.next_local_offset += 1;
                        }
                        


            
                        self.next_token(); // consume ')'
            
                        if self.current.kind == TokenKind::LBrace {
                            let entry = self.code.len();
                            self.functions.insert(func_name.clone(), entry);
                            if func_name == "main" {
                                self.main_label = Some(entry);
                            }
            
                            self.code.push(Instruction::Enter(0)); // Placeholder
                            let locals_before = self.locals.len();
            
                            self.stmt(); // Parse function body
            
                            let locals_after = self.locals.len();
                            let n_locals = locals_after - locals_before;
            
                            // Patch the Enter instruction with number of locals
                            if let Instruction::Enter(ref mut n) = self.code[entry] {
                                *n = n_locals;
                            }
            
                            self.code.push(Instruction::Leave);
                            continue;
                        }
                    }
                }
            }
            
    
            self.stmt(); // fallback
        }
    
        self.code.push(Instruction::Label(9999)); // Fake exit
    }
    
    

    fn new_label(&mut self) -> usize {
        let id = self.label_id;
        self.label_id += 1;
        id
    }
    
    fn stmt(&mut self) {
        // Variable declaration
        if self.current.kind == TokenKind::Int || self.current.kind == TokenKind::Char {
            println!("Parsing variable declaration");
            self.next_token(); // consume 'int' or 'char'

            while let TokenKind::Id(name) = &self.current.kind {
                let var_name = name.clone();
                self.locals.insert(var_name, self.next_local_offset);
                self.next_local_offset += 1;

                self.next_token(); // consume identifier

                // Optional: handle comma-separated declarations
                if self.current.kind == TokenKind::Comma {
                    self.next_token(); // continue to next variable
                } else {
                    break;
                }
            }

            if self.current.kind != TokenKind::Semicolon {
                panic!("Expected semicolon after variable declaration");
            }
            self.next_token(); // consume ';'
            return;
        }

        match &self.current.kind {
            TokenKind::If => {
                println!("Parsing 'if' statement");
                self.next_token(); // consume 'if'
            
                if self.current.kind != TokenKind::LParen {
                    panic!("Expected '(' after 'if'");
                }
                self.next_token(); // consume '('
            
                self.expr(); // parse condition expression
            
                if self.current.kind != TokenKind::RParen {
                    panic!("Expected ')' after 'if' condition");
                }
                self.next_token(); // consume ')'
            
                // ⬇️ Add instruction emission for control flow
                let false_label = self.new_label();
                self.code.push(Instruction::Jz(false_label)); // if cond == 0, jump to false_label
            
                self.stmt(); // then-body
            
                // ⬇️ Patch jump target
                self.code.push(Instruction::Label(false_label));
            }
            
            TokenKind::While => {
                println!("Parsing 'while' loop");
                self.next_token(); // consume 'while'
    
                if self.current.kind != TokenKind::LParen {
                    panic!("Expected '(' after 'while'");
                }
                self.next_token(); // consume '('
    
                let start_label = self.new_label();
                let end_label = self.new_label();
    
                self.code.push(Instruction::Label(start_label)); // loop start
    
                self.expr(); // loop condition
    
                if self.current.kind != TokenKind::RParen {
                    panic!("Expected ')' after 'while' condition");
                }
                self.next_token(); // consume ')'
    
                self.code.push(Instruction::Jz(end_label)); // break if false
    
                self.stmt(); // loop body
    
                self.code.push(Instruction::Jmp(start_label)); // jump back to start
                self.code.push(Instruction::Label(end_label)); // loop end
            }
            TokenKind::Return => {
                println!("Parsing 'return' statement");
                self.next_token();
                self.expr();
                self.code.push(Instruction::Jmp(9999)); // fake label for "exit"
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
    
        match &self.current.kind {
            TokenKind::Num(n) => {
                println!("Found number: {}", n);
                self.code.push(Instruction::Imm(*n)); // <-- Emit IMM
                self.next_token();
            }
            TokenKind::Id(name) => {
                let var_name = name.clone();
                self.next_token();
            
                // Function call: f()
                if self.current.kind == TokenKind::LParen {
                    self.next_token(); // consume '('
                    let _arg_count = 0;
            
                    let mut args = Vec::new();
                    while self.current.kind != TokenKind::RParen {
                        let start = self.code.len();
                        self.expr_bp(0);
                        let end = self.code.len();
                        args.push(self.code.drain(start..end).collect::<Vec<_>>());

                        if self.current.kind == TokenKind::Comma {
                            self.next_token();
                        } else {
                            break;
                        }
                    }

                    if self.current.kind != TokenKind::RParen {
                        panic!("Expected ')' after function arguments");
                    }
                    self.next_token(); // consume ')'

                    // Push arguments in reverse (right-to-left)
                    for arg_code in args.into_iter().rev() {
                        self.code.extend(arg_code);
                    }

                    

            

            
                    if let Some(&addr) = self.functions.get(&var_name) {
                        self.code.push(Instruction::Call(addr));
                    } else {
                        panic!("Unknown function: {}", var_name);
                    }
            
                    // args are consumed by callee
                }
                
                // Assignment or usage
                else if self.current.kind == TokenKind::Assign {
                    println!("Assignment to: {}", var_name);
                    self.next_token(); // consume '='
                    self.expr_bp(0);
            
                    if let Some(offset) = self.locals.get(&var_name) {
                        self.code.push(Instruction::StoreLocal(*offset));
                    } else {
                        panic!("Undefined variable: {}", var_name);
                    }
                } else {
                    if let Some(offset) = self.locals.get(&var_name) {
                        self.code.push(Instruction::LoadLocal(*offset));
                    } else {
                        panic!("Undefined variable: {}", var_name);
                    }
                }
            }
            
            
            
            TokenKind::LParen => {
                self.next_token();
                self.expr_bp(0);
                if self.current.kind != TokenKind::RParen {
                    panic!("Expected ')'");
                }
                self.next_token();
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
    
            self.expr_bp(op_bp + 1); // Parse right-hand side
    
            // Emit corresponding opcode
            match op {
                TokenKind::Add => self.code.push(Instruction::Add),
                TokenKind::Sub => self.code.push(Instruction::Sub),
                TokenKind::Mul => self.code.push(Instruction::Mul),
                TokenKind::Div => self.code.push(Instruction::Div),
                TokenKind::Mod => self.code.push(Instruction::Mod),
                _ => panic!("Unsupported binary operator {:?}", op),
            }
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