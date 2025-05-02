// src/vm.rs
use crate::instruction::Instruction;
use std::collections::HashMap;

pub struct VM {
    pub stack: Vec<i64>,
    pub fp: usize, // frame pointer
    pub call_stack: Vec<usize>,
    pub function_arity: HashMap<usize, usize>, // or name → usize
}

impl VM {
    pub fn new(function_arity: HashMap<usize, usize>) -> Self {
        Self {
            stack: Vec::new(),
            fp: 0,
            call_stack: Vec::new(),
            function_arity, // ✅ now it's coming from the function argument
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
                Neg => {
                    // added: unary -
                    let v = self.stack.pop().unwrap();
                    self.stack.push(-v);
                }
                Not => {
                    // added: logical !
                    let v = self.stack.pop().unwrap();
                    self.stack.push((v == 0) as i64);
                }
                Deref => {
                    // added: unary *
                    let addr = self.stack.pop().unwrap() as usize;
                    let v = self.stack[addr];
                    self.stack.push(v);
                }
                Addr(offset) => {
                    // now `offset` is a usize you can use:
                    let addr = (self.fp + *offset) as i64;
                    self.stack.push(addr);
                }
                Cast => { // added: no-op for C-style cast
                    // nothing to do at runtime
                }
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

                Eq => {
                    // added: ==
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push((a == b) as i64);
                }
                Ne => {
                    // added: !=
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push((a != b) as i64);
                }
                Lt => {
                    // added: <
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push((a < b) as i64);
                }
                Gt => {
                    // added: >
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push((a > b) as i64);
                }
                Le => {
                    // added: <=
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push((a <= b) as i64);
                }
                Ge => {
                    // added: >=
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push((a >= b) as i64);
                }

                BitAnd => {
                    // added: bitwise &
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a & b);
                }
                BitOr => {
                    // added: bitwise |
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a | b);
                }
                BitXor => {
                    // added: bitwise ^
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a ^ b);
                }
                Shl => {
                    // added: <<
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push((a << b) as i64);
                }
                Shr => {
                    // added: >>
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push((a >> b) as i64);
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

                Instruction::Call(addr) => {
                    // Get number of arguments for this function
                    let num_args = self
                        .function_arity
                        .get(addr)
                        .copied()
                        .expect("Missing function arity");

                    // Pop args from stack (right to left), reverse them
                    let mut args = Vec::with_capacity(num_args);
                    for _ in 0..num_args {
                        args.push(self.stack.pop().expect("Missing argument"));
                    }
                    args.reverse();

                    // ✅ Push args first
                    for arg in args {
                        self.stack.push(arg);
                    }

                    // ✅ Now push old FP and return address
                    self.stack.push(self.fp as i64);
                    self.stack.push((ip + 1) as i64);

                    // ✅ Set new FP (points to base of new frame)
                    self.fp = self.stack.len();

                    // ✅ Jump to function start
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
                    // load a local (offset into frame)
                    let v = self.stack[self.fp + *offset];
                    self.stack.push(v);
                }
                StoreLocal(offset) => {
                    // store to a local
                    let v = self.stack.pop().unwrap();
                    self.stack[self.fp + *offset] = v;
                }

                Leave => {
                    // 1) pull the return value off the top
                    let ret_val = self.stack.pop().unwrap();
                    // 2) pop any locals (everything above the saved FP)
                    while self.stack.len() > self.fp {
                        self.stack.pop();
                    }
                    // 3) pop return address
                    let ret_addr = self.stack.pop().unwrap() as usize;
                    // 4) restore old FP
                    let old_fp = self.stack.pop().unwrap() as usize;
                    self.fp = old_fp;
                    // 5) push the return value back on
                    self.stack.push(ret_val);
                    // 6) jump back to caller
                    ip = ret_addr;
                    continue;
                }
            }

            ip += 1;
        }

        self.stack.pop()
    }

    /// mirror the same fp-logic in run_from
    pub fn run_from(&mut self, code: &[Instruction], start_ip: usize) -> Option<i64> {
        use Instruction::*;

        // build label→index map
        let mut labels = HashMap::new();
        for (i, instr) in code.iter().enumerate() {
            if let Label(id) = instr {
                labels.insert(*id, i);
            }
        }

        let mut ip = start_ip;
        while ip < code.len() {
            match &code[ip] {
                Neg => {
                    let v = self.stack.pop().unwrap();
                    self.stack.push(-v);
                }
                Not => {
                    let v = self.stack.pop().unwrap();
                    self.stack.push((v == 0) as i64);
                }
                Deref => {
                    let addr = self.stack.pop().unwrap() as usize;
                    let v = self.stack[addr];
                    self.stack.push(v);
                }
                Addr(o) => {
                    let o = *o;
                    let addr = (self.fp + o) as i64;
                    self.stack.push(addr);
                }
                Cast => { /* no-op */ }
                Imm(n) => self.stack.push(*n),
                Add => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a + b)
                }
                Sub => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a - b)
                }
                Mul => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a * b)
                }
                Div => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a / b)
                }
                Mod => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a % b)
                }

                Eq => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push((a == b) as i64)
                }
                Ne => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push((a != b) as i64)
                }
                Lt => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push((a < b) as i64)
                }
                Gt => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push((a > b) as i64)
                }
                Le => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push((a <= b) as i64)
                }
                Ge => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push((a >= b) as i64)
                }

                BitAnd => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a & b)
                }
                BitOr => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a | b)
                }
                BitXor => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a ^ b)
                }
                Shl => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a << b)
                }
                Shr => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a >> b)
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
                    // 1) grab how many args this function expects
                    let num_args = *self
                        .function_arity
                        .get(addr)
                        .expect("Missing function arity");

                    // 2) pop them off (right-to-left), then reverse into correct order
                    let mut args = Vec::with_capacity(num_args);
                    for _ in 0..num_args {
                        args.push(self.stack.pop().expect("Missing argument"));
                    }
                    args.reverse();

                    // 3) push old FP and return address
                    self.stack.push(self.fp as i64);
                    self.stack.push((ip + 1) as i64);

                    // 4) set new frame pointer
                    self.fp = self.stack.len();

                    // 5) re-push the argument values into the parameter slots
                    for arg in args {
                        self.stack.push(arg);
                    }

                    // 6) jump into the function
                    ip = *addr;
                    continue;
                }

                Enter(n_total_slots) => {
                        // parser used n_total_slots = params + locals.
                    // we only want to allocate the *locals* here.
                    let param_count = *self.function_arity.get(&ip).unwrap_or(&0);
                    let local_only = if *n_total_slots > param_count {
                        *n_total_slots - param_count
                    } else {
                        0
                    };
                    for _ in 0..local_only {
                        self.stack.push(0);
                    }
                }
                LoadLocal(o) => {
                    let v = self.stack[self.fp + *o];
                    self.stack.push(v);
                }
                StoreLocal(o) => {
                    let v = self.stack.pop().unwrap();
                    self.stack[self.fp + *o] = v;
                }

                Leave => {
                    // 1) pull the return value off the top
                    let ret_val = self.stack.pop().unwrap();
                    // 2) pop any locals (everything above the saved FP)
                    while self.stack.len() > self.fp {
                        self.stack.pop();
                    }
                    // 3) pop return address
                    let ret_addr = self.stack.pop().unwrap() as usize;
                    // 4) restore old FP
                    let old_fp = self.stack.pop().unwrap() as usize;
                    self.fp = old_fp;
                    // 5) push the return value back on
                    self.stack.push(ret_val);
                    // 6) jump back to caller
                    ip = ret_addr;
                    continue;
                }
            }
            ip += 1;
        }
        self.stack.pop()
    }
}
