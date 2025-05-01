use crate::instruction::Instruction;
use std::collections::HashMap;

pub struct VM {
    pub stack: Vec<i64>,
    pub fp: usize,  // frame pointer
}

impl VM {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            fp: 0,
        }
    }

    pub fn run(&mut self, code: &[Instruction]) -> Option<i64> {
        use Instruction::*;

        // build label→index map
        let mut labels = HashMap::new();
        for (i, instr) in code.iter().enumerate() {
            if let Label(id) = instr {
                labels.insert(*id, i);
            }
        }

        let mut ip = 0;
        while ip < code.len() {
            match &code[ip] {
                Imm(n) => {
                    self.stack.push(*n);
                }
                Add => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a + b);
                }
                Sub => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a - b);
                }
                Mul => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a * b);
                }
                Div => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a / b);
                }
                Mod => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a % b);
                }

                Jmp(lbl) => {
                    ip = labels[lbl];
                    continue;
                }
                Jz(lbl) => {
                    let v = self.stack.pop().unwrap();
                    if v == 0 {
                        ip = labels[lbl];
                        continue;
                    }
                }
                Label(_) => { /* no-op */ }

                Call(addr) => {
                    // push old frame pointer
                    self.stack.push(self.fp as i64);
                    // push return address
                    self.stack.push((ip + 1) as i64);
                    // set new frame pointer
                    self.fp = self.stack.len();
                    // jump into function
                    ip = *addr;
                    continue;
                }

                Enter(n_locals) => {
                    // allocate space for locals
                    for _ in 0..*n_locals {
                        self.stack.push(0);
                    }
                }

                LoadLocal(offset) => {
                    // simply push stack[offset]
                    let val = self.stack[*offset];
                    self.stack.push(val);
                }
                StoreLocal(offset) => {
                    let val = self.stack.pop().unwrap();
                    self.stack[*offset] = val;
                }
                

                Leave => {
                    // pop locals
                    while self.stack.len() > self.fp {
                        self.stack.pop();
                    }
                    // pop return address
                    let ret_addr = self.stack.pop().unwrap() as usize;
                    // restore old frame pointer
                    let old_fp = self.stack.pop().unwrap() as usize;
                    self.fp = old_fp;
                    // jump back
                    ip = ret_addr;
                    continue;
                }
            }

            ip += 1;
        }

        self.stack.pop()
    }

    /// If you still need `run_from`, mirror the same fp-logic there:
    pub fn run_from(&mut self, code: &[Instruction], start_ip: usize) -> Option<i64> {
        use Instruction::*;

        let mut labels = HashMap::new();
        for (i, instr) in code.iter().enumerate() {
            if let Label(id) = instr {
                labels.insert(*id, i);
            }
        }

        let mut ip = start_ip;
        while ip < code.len() {
            match &code[ip] {
                Imm(n)   => self.stack.push(*n),
                Add      => { let b = self.stack.pop().unwrap(); let a = self.stack.pop().unwrap(); self.stack.push(a + b); }
                Sub      => { let b = self.stack.pop().unwrap(); let a = self.stack.pop().unwrap(); self.stack.push(a - b); }
                Mul      => { let b = self.stack.pop().unwrap(); let a = self.stack.pop().unwrap(); self.stack.push(a * b); }
                Div      => { let b = self.stack.pop().unwrap(); let a = self.stack.pop().unwrap(); self.stack.push(a / b); }
                Mod      => { let b = self.stack.pop().unwrap(); let a = self.stack.pop().unwrap(); self.stack.push(a % b); }

                Jmp(lbl) => { ip = labels[lbl]; continue; }
                Jz(lbl)  => { let v = self.stack.pop().unwrap(); if v == 0 { ip = labels[lbl]; continue; } }
                Label(_) => {}

                Call(addr) => {
                    self.stack.push(self.fp as i64);
                    self.stack.push((ip + 1) as i64);
                    self.fp = self.stack.len();
                    ip = *addr;
                    continue;
                }
                Enter(n_locals) => {
                    for _ in 0..*n_locals {
                        self.stack.push(0);
                    }
                }
                LoadLocal(offset) => {
                    // simply push stack[offset]
                    let val = self.stack[*offset];
                    self.stack.push(val);
                }
                StoreLocal(offset) => {
                    let val = self.stack.pop().unwrap();
                    self.stack[*offset] = val;
                }
                
                Leave => {
                    while self.stack.len() > self.fp {
                        self.stack.pop();
                    }
                    let ret_addr = self.stack.pop().unwrap() as usize;
                    let old_fp = self.stack.pop().unwrap() as usize;
                    self.fp = old_fp;
                    ip = ret_addr;
                    continue;
                }
            }

            ip += 1;
        }

        self.stack.pop()
    }
}
