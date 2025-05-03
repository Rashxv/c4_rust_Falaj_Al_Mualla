use crate::instruction::Instruction;
use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};
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
    in_function: bool,              // new field
    pub current_fn: Option<String>, // name of the function we're parsing, or None
    pub function_arity: HashMap<String, usize>,
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
            in_function: false,
            current_fn: None,
            function_arity: HashMap::new(),
        }
    }

    fn next_token(&mut self) {
        self.current = self.lexer.next_token();
    }

    pub fn parse(&mut self) {
        while self.current.kind != TokenKind::Eof {
            // —— Named-function prologue detection ——
            if !self.in_function && self.current.kind == TokenKind::Int {
                self.next_token(); // consume `int`

                if let TokenKind::Id(func_name) = &self.current.kind {
                    let name = func_name.clone();
                    self.next_token(); // consume the identifier

                    if self.current.kind == TokenKind::LParen {
                        // + mark that we’re inside this named function
                        self.in_function = true;
                        self.current_fn = Some(name.clone());

                        // —— parse parameter list ——
                        self.next_token(); // consume '('
                        let mut params = Vec::new();
                        while self.current.kind == TokenKind::Int
                            || self.current.kind == TokenKind::Char
                        {
                            self.next_token(); // consume type
                            if let TokenKind::Id(p) = &self.current.kind {
                                params.push(p.clone());
                                self.next_token(); // consume name
                                if self.current.kind == TokenKind::Comma {
                                    self.next_token(); // consume ','
                                }
                            } else {
                                panic!("Expected identifier in parameter list");
                            }
                        }
                        if self.current.kind != TokenKind::RParen {
                            panic!("Expected ')' after parameters");
                        }
                        self.next_token(); // consume ')'

                        // —— function body open ——
                        if self.current.kind != TokenKind::LBrace {
                            panic!("Expected '{{' to start function body");
                        }
                        self.next_token(); // consume '{'

                        // record entry point
                        let entry = self.code.len();
                        self.functions.insert(name.clone(), entry);
                        self.function_arity.insert(name.clone(), params.len());

                        if name == "main" {
                            self.main_label = Some(entry);
                        }

                        // reset locals & assign parameter slots
                        self.locals.clear();
                        self.next_local_offset = 0;
                        for (i, p) in params.into_iter().enumerate() {
                            self.locals.insert(p, i);
                            self.next_local_offset += 1;
                        }

                        // placeholder Enter, we'll patch after the body
                        self.code.push(Instruction::Enter(0));

                        // —— parse the function body ——
                        while self.current.kind != TokenKind::RBrace {
                            self.stmt();
                        }
                        self.next_token(); // consume '}'

                        // now that all locals (params + any `int x;` inside) are in self.locals:
                        let total_slots = self.locals.len();
                        // patch Enter with correct slot count
                        if let Instruction::Enter(ref mut cnt) = self.code[entry] {
                            *cnt = total_slots;
                        }

                        // leave the function
                        self.code.push(Instruction::Leave);

                        // + done with this function
                        self.in_function = false;
                        self.current_fn = None;

                        continue;
                    }
                }
                // if it looked like “int foo” but not “int foo(”, fall through
            }

            // —— Top-level anonymous block ——
            if !self.in_function && self.current.kind == TokenKind::LBrace {
                self.next_token(); // consume '{'
                while self.current.kind != TokenKind::RBrace {
                    self.stmt();
                }
                self.next_token(); // consume '}'
                continue;
            }

            // —— Everything else is just a statement ——
            self.stmt();
        }

        // fake exit label for top-level Jmp(9999)
        self.code.push(Instruction::Label(9999));
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

            // + skip any '*' (pointer declarators) before the identifier
            while self.current.kind == TokenKind::Mul {
                self.next_token(); // consume '*'
            }

            while let TokenKind::Id(name) = &self.current.kind {
                let var_name = name.clone();
                self.locals.insert(var_name, self.next_local_offset);
                self.next_local_offset += 1;

                self.next_token(); // consume identifier

                // Optional: handle comma-separated declarations
                if self.current.kind == TokenKind::Comma {
                    self.next_token(); // consume ','
                    // + also skip '*' on subsequent pointer declarations
                    while self.current.kind == TokenKind::Mul {
                        self.next_token();
                    }
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
                self.next_token(); // consume `return`
                self.expr(); // emit the return-value

                // if we're in a named function that's NOT `main`, emit Leave,
                // otherwise (main or top-level) jump to exit label:
                match &self.current_fn {
                    Some(name) if name != "main" => {
                        self.code.push(Instruction::Leave);
                    }
                    _ => {
                        self.code.push(Instruction::Jmp(9999));
                    }
                }

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
        // ——— Prefix / "nud" ———
        let prefix_op = matches!(
            self.current.kind,
            TokenKind::Sub
                | TokenKind::Not
                | TokenKind::Mul
                | TokenKind::And
                | TokenKind::Sizeof
                | TokenKind::String(_)
                | TokenKind::CharLiteral(_)
                | TokenKind::Num(_)
                | TokenKind::Float(_) 
                | TokenKind::Id(_)
                | TokenKind::LParen
        );
        if prefix_op {
            // handle prefix expressions exactly once

            println!("Parsing expression starting with {:?}", self.current.kind);
            match &self.current.kind {
                TokenKind::Float(f) => {
                    let v = *f;
                    self.next_token();
                    self.code.push(Instruction::ImmF(v));
                }
                TokenKind::Num(n) => {
                    let v = *n;
                    self.next_token();
                    self.code.push(Instruction::Imm(v));
                }
                TokenKind::Sub => {
                    self.next_token();
                    self.expr_bp(11);
                    self.code.push(Instruction::Neg);
                }
                TokenKind::Not => {
                    self.next_token();
                    self.expr_bp(11);
                    self.code.push(Instruction::Not);
                }
                TokenKind::Mul => {
                    self.next_token();
                    self.expr_bp(11);
                    self.code.push(Instruction::Deref);
                }
                TokenKind::And => {
                    self.next_token();
                    if let TokenKind::Id(name) = &self.current.kind {
                        let &offset = self
                            .locals
                            .get(name)
                            .unwrap_or_else(|| panic!("Undefined variable `{}`", name));
                        self.code.push(Instruction::Addr(offset));
                        self.next_token();
                    } else {
                        panic!("Expected identifier after '&'");
                    }
                }
                TokenKind::Sizeof => {
                    self.next_token();
                    if self.current.kind != TokenKind::LParen {
                        panic!("Expected '(' after sizeof");
                    }
                    self.next_token();

                    let mut size = match self.current.kind {
                        TokenKind::Int => std::mem::size_of::<i64>(),
                        TokenKind::Char => std::mem::size_of::<i8>(),
                        _ => panic!("Invalid type in sizeof"),
                    } as i64;
                    self.next_token();

                    while self.current.kind == TokenKind::Mul {
                        size = std::mem::size_of::<usize>() as i64;
                        self.next_token();
                    }

                    if self.current.kind != TokenKind::RParen {
                        panic!("Expected ')' after sizeof");
                    }
                    self.next_token();

                    self.code.push(Instruction::Imm(size));
                }
                TokenKind::String(s) => {
                    let lit = s.clone();
                    self.next_token();
                    let addr = self.emit_string_literal(&lit);
                    self.code.push(Instruction::Imm(addr));
                }
                TokenKind::CharLiteral(c) => {
                    self.code.push(Instruction::Imm(*c as i64));
                    self.next_token();
                }
                TokenKind::Num(n) => {
                    self.code.push(Instruction::Imm(*n));
                    self.next_token();
                }
                TokenKind::Id(name) => {
                    let var_name = name.clone();
                    self.next_token();

                    if var_name == "print" {
                        // must see '('
                        if self.current.kind != TokenKind::LParen {
                            panic!("Expected '(' after print");
                        }
                        self.next_token();

                        // string literal?
                        if let TokenKind::String(s) = &self.current.kind {
                            let s_lit = s.clone();
                            self.next_token(); // consume the literal
                            if self.current.kind != TokenKind::RParen {
                                panic!("Expected ')' after print string");
                            }
                            self.next_token(); // consume ')'
                            self.code.push(Instruction::PrintStr(s_lit));
                        } else {
                            // otherwise parse an integer expression
                            self.expr_bp(0);
                            if self.current.kind != TokenKind::RParen {
                                panic!("Expected ')' after print expr");
                            }
                            self.next_token(); // consume ')'

                            // Check if last emitted instruction was a float
                            if let Some(Instruction::PushF(_)) = self.code.last() {
                                self.code.push(Instruction::PrintF);
                            } else {
                                self.code.push(Instruction::Print);
                            }
                        }
                        return;
                    }

                    // ——— everything else is a normal call ———
                    if self.current.kind == TokenKind::LParen {
                        self.next_token(); // consume '('
                        let mut args = Vec::new();
                        while self.current.kind != TokenKind::RParen {
                            let start = self.code.len();
                            self.expr_bp(0);
                            let end = self.code.len();
                            args.push(self.code.drain(start..end).collect::<Vec<_>>());
                            if self.current.kind == TokenKind::Comma {
                                self.next_token();
                            }
                        }
                        self.next_token(); // consume ')'

                        // push args right-to-left
                        for arg in args.into_iter().rev() {
                            self.code.extend(arg);
                        }

                        let &addr = self
                            .functions
                            .get(&var_name)
                            .unwrap_or_else(|| panic!("Unknown function `{}`", var_name));

                        self.code.push(Instruction::Call(addr));
                    } else if self.current.kind == TokenKind::Assign {
                        self.next_token();
                        self.expr_bp(0);
                        let &offset = self
                            .locals
                            .get(&var_name)
                            .unwrap_or_else(|| panic!("Undefined variable `{}`", var_name));
                        self.code.push(Instruction::StoreLocal(offset));
                    } else {
                        let &offset = self
                            .locals
                            .get(&var_name)
                            .unwrap_or_else(|| panic!("Undefined variable `{}`", var_name));
                        self.code.push(Instruction::LoadLocal(offset));
                    }
                }
                TokenKind::LParen => {
                    self.next_token();
                    let is_type = matches!(
                        self.current.kind,
                        TokenKind::Int | TokenKind::Char | TokenKind::Mul
                    );
                    if is_type {
                        while matches!(
                            self.current.kind,
                            TokenKind::Int | TokenKind::Char | TokenKind::Mul
                        ) {
                            self.next_token();
                        }
                        if self.current.kind != TokenKind::RParen {
                            panic!("Expected ')' after cast type");
                        }
                        self.next_token(); // consume ')'
                        self.expr_bp(11);
                        self.code.push(Instruction::Cast);
                    } else {
                        self.expr_bp(0);
                        if self.current.kind != TokenKind::RParen {
                            panic!("Expected ')' after expression");
                        }
                        self.next_token();
                    }
                }
                other => panic!("Unexpected token in expression: {:?}", other),
            }
        }
        // ——— Infix / "led" ———
        loop {
            let op_bp = self.get_precedence();
            if op_bp < min_bp || op_bp == 0 {
                break;
            }

            let op = self.current.kind.clone();
            self.next_token();

            if op != TokenKind::Cond {
                self.expr_bp(op_bp + 1);
            }

            match op {
                TokenKind::Cond => {
                    let else_lbl = self.new_label();
                    self.code.push(Instruction::Jz(else_lbl));

                    self.expr_bp(1);

                    let end_lbl = self.new_label();
                    self.code.push(Instruction::Jmp(end_lbl));
                    self.code.push(Instruction::Label(else_lbl));

                    if self.current.kind != TokenKind::Colon {
                        panic!("Expected ':' in conditional expression");
                    }
                    self.next_token();

                    self.expr_bp(1);
                    self.code.push(Instruction::Label(end_lbl));
                }

                TokenKind::Add => self.code.push(Instruction::Add),
                TokenKind::Sub => self.code.push(Instruction::Sub),
                TokenKind::Mul => self.code.push(Instruction::Mul),
                TokenKind::Div => self.code.push(Instruction::Div),
                TokenKind::Mod => self.code.push(Instruction::Mod),

                TokenKind::Eq => self.code.push(Instruction::Eq),
                TokenKind::Ne => self.code.push(Instruction::Ne),
                TokenKind::Lt => self.code.push(Instruction::Lt),
                TokenKind::Gt => self.code.push(Instruction::Gt),
                TokenKind::Le => self.code.push(Instruction::Le),
                TokenKind::Ge => self.code.push(Instruction::Ge),

                TokenKind::And => self.code.push(Instruction::BitAnd),
                TokenKind::Or => self.code.push(Instruction::BitOr),
                TokenKind::Xor => self.code.push(Instruction::BitXor),

                TokenKind::Shl => self.code.push(Instruction::Shl),
                TokenKind::Shr => self.code.push(Instruction::Shr),

                _ => panic!("Unsupported infix operator: {:?}", op),
            }
        }
    }

    ///  need this helper to place string literals into your data segment
    fn emit_string_literal(&mut self, _s: &str) -> i64 {
        // e.g. push into a Vec<u8>, track its offset, then return that offset
        unimplemented!("string literal emission");
    }

    // Simple precedence rules based on C4
    fn get_precedence(&self) -> u8 {
        match self.current.kind {
            TokenKind::Assign => 1,
            TokenKind::Cond => 2, // + support `?:`
            TokenKind::Lor => 3,
            TokenKind::Lan => 4,
            TokenKind::Or => 5,
            TokenKind::Xor => 6,
            TokenKind::And => 7,
            TokenKind::Eq | TokenKind::Ne => 8,
            TokenKind::Lt | TokenKind::Gt | TokenKind::Le | TokenKind::Ge => 9,
            TokenKind::Shl | TokenKind::Shr => 10,
            TokenKind::Add | TokenKind::Sub => 11,
            TokenKind::Mul | TokenKind::Div | TokenKind::Mod => 12,
            _ => 0,
        }
    }
}
