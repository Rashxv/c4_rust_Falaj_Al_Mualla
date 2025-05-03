// src/vm.rs
use crate::instruction::Instruction;
use std::collections::HashMap;
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Flt(f64),
}

pub struct VM {
    pub stack: Vec<Value>,
    pub fp: usize, // frame pointer
    pub call_stack: Vec<usize>,
    pub function_arity: HashMap<usize, usize>, // or name → usize
    pub float_stack: Vec<f64>,
}

impl VM {
    pub fn new(function_arity: HashMap<usize, usize>) -> Self {
        Self {
            stack: Vec::new(),
            float_stack: Vec::new(),
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
                PushF(f) => {
                    self.float_stack.push(*f);
                }
                PrintF => {
                    let f = self.float_stack.pop().unwrap();
                    println!("{}", f);
                }
                Print => {
                    let v = self.stack.pop().unwrap();
                    match v {
                        Value::Int(i) => println!("{}", i),
                        Value::Flt(f) => println!("{}", f),
                    }
                }
                PrintStr(s) => {
                    println!("{}", s);
                }
                Neg => {
                    let v = self.stack.pop().unwrap();
                    let result = match v {
                        Value::Int(n) => Value::Int(-n),
                        Value::Flt(f) => Value::Flt(-f),
                    };
                    self.stack.push(result);
                }

                Not => {
                    let v = self.stack.pop().unwrap();
                    let result = match v {
                        Value::Int(n) => Value::Int((n == 0) as i64),
                        Value::Flt(f) => Value::Int((f == 0.0) as i64),
                    };
                    self.stack.push(result);
                }
                Deref => {
                    let addr = self.stack.pop().unwrap();
                    let addr = match addr {
                        Value::Int(i) => i as usize,
                        _ => panic!("Cannot dereference non-integer address"),
                    };
                    let v = self.stack[addr].clone(); // clone to avoid borrow checker issues
                    self.stack.push(v);
                }
                Addr(offset) => {
                    let addr = (self.fp + *offset) as i64;
                    self.stack.push(Value::Int(addr));
                }
                Cast => { // added: no-op for C-style cast
                    // nothing to do at runtime
                }
                Imm(n) => {
                    self.stack.push(Value::Int(*n));
                }
                ImmF(f) => {
                    self.stack.push(Value::Flt(*f));
                }

                Add => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    let res = match (a, b) {
                        (Value::Int(x), Value::Int(y)) => Value::Int(x + y),
                        (Value::Flt(x), Value::Flt(y)) => Value::Flt(x + y),
                        (Value::Int(x), Value::Flt(y)) => Value::Flt(x as f64 + y),
                        (Value::Flt(x), Value::Int(y)) => Value::Flt(x + y as f64),
                    };
                    self.stack.push(res);
                }
                Sub => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(match (a, b) {
                        (Value::Int(x), Value::Int(y)) => Value::Int(x - y),
                        (Value::Flt(x), Value::Flt(y)) => Value::Flt(x - y),
                        (Value::Int(x), Value::Flt(y)) => Value::Flt(x as f64 - y),
                        (Value::Flt(x), Value::Int(y)) => Value::Flt(x - y as f64),
                    });
                }
                Mul => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(match (a, b) {
                        (Value::Int(x), Value::Int(y)) => Value::Int(x * y),
                        (Value::Flt(x), Value::Flt(y)) => Value::Flt(x * y),
                        (Value::Int(x), Value::Flt(y)) => Value::Flt(x as f64 * y),
                        (Value::Flt(x), Value::Int(y)) => Value::Flt(x * y as f64),
                    });
                }
                Div => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(match (a, b) {
                        (Value::Int(x), Value::Int(y)) => Value::Int(x / y),
                        (Value::Flt(x), Value::Flt(y)) => Value::Flt(x / y),
                        (Value::Int(x), Value::Flt(y)) => Value::Flt(x as f64 / y),
                        (Value::Flt(x), Value::Int(y)) => Value::Flt(x / y as f64),
                    });
                }
                Mod => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(match (a, b) {
                        (Value::Int(x), Value::Int(y)) => Value::Int(x % y),
                        _ => panic!("Modulo is not defined for floating-point numbers"),
                    });
                }

                Eq => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::Int((a == b) as i64));
                }
                Ne => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::Int((a != b) as i64));
                }
                Lt => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::Int(match (a, b) {
                        (Value::Int(x), Value::Int(y)) => (x < y) as i64,
                        (Value::Flt(x), Value::Flt(y)) => (x < y) as i64,
                        (Value::Int(x), Value::Flt(y)) => ((x as f64) < y) as i64,
                        (Value::Flt(x), Value::Int(y)) => (x < (y as f64)) as i64,
                    }));
                }
                Gt => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::Int(match (a, b) {
                        (Value::Int(x), Value::Int(y)) => (x > y) as i64,
                        (Value::Flt(x), Value::Flt(y)) => (x > y) as i64,
                        (Value::Int(x), Value::Flt(y)) => ((x as f64) > y) as i64,
                        (Value::Flt(x), Value::Int(y)) => (x > (y as f64)) as i64,
                    }));
                }
                Le => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::Int(match (a, b) {
                        (Value::Int(x), Value::Int(y)) => (x <= y) as i64,
                        (Value::Flt(x), Value::Flt(y)) => (x <= y) as i64,
                        (Value::Int(x), Value::Flt(y)) => ((x as f64) <= y) as i64,
                        (Value::Flt(x), Value::Int(y)) => (x <= (y as f64)) as i64,
                    }));
                }
                Ge => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::Int(match (a, b) {
                        (Value::Int(x), Value::Int(y)) => (x >= y) as i64,
                        (Value::Flt(x), Value::Flt(y)) => (x >= y) as i64,
                        (Value::Int(x), Value::Flt(y)) => ((x as f64) >= y) as i64,
                        (Value::Flt(x), Value::Int(y)) => (x >= (y as f64)) as i64,
                    }));
                }

                BitAnd => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Int(x & y)),
                        _ => panic!("Bitwise AND only supports integers"),
                    }
                }

                BitOr => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Int(x | y)),
                        _ => panic!("Bitwise OR only supports integers"),
                    }
                }

                BitXor => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Int(x ^ y)),
                        _ => panic!("Bitwise XOR only supports integers"),
                    }
                }

                Shl => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Int(x << y)),
                        _ => panic!("Shift operations only support integers"),
                    }
                }

                Shr => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Int(x >> y)),
                        _ => panic!("Shift operations only support integers"),
                    }
                }

                Jmp(lbl) => {
                    ip = labels[lbl];
                    continue;
                }

                Jz(lbl) => {
                    let v = self.stack.pop().unwrap();
                    let cond = match v {
                        Value::Int(i) => i == 0,
                        Value::Flt(f) => f == 0.0,
                    };
                    if cond {
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

                    // ✅ Now push old FP and return address (as Int values)
                    self.stack.push(Value::Int(self.fp as i64));
                    self.stack.push(Value::Int((ip + 1) as i64));

                    // ✅ Set new FP (points to base of new frame)
                    self.fp = self.stack.len();

                    // ✅ Jump to function start
                    ip = *addr;
                    continue;
                }

                Enter(n_locals) => {
                    // Allocate space for locals with default int value
                    for _ in 0..*n_locals {
                        self.stack.push(Value::Int(0));
                    }
                }

                LoadLocal(offset) => {
                    // load a local (offset into frame)
                    let v = self.stack[self.fp + *offset].clone();
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

                    // 3) pop return address (must be an Int)
                    let ret_addr = match self.stack.pop().unwrap() {
                        Value::Int(i) => i as usize,
                        _ => panic!("Expected integer return address"),
                    };

                    // 4) restore old FP
                    let old_fp = match self.stack.pop().unwrap() {
                        Value::Int(i) => i as usize,
                        _ => panic!("Expected integer frame pointer"),
                    };
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

        match self.stack.pop() {
            Some(Value::Int(i)) => Some(i),
            Some(Value::Flt(f)) => Some(f as i64), // convert float to int for return
            None => None,
        }
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
                PushF(f) => {
                    self.float_stack.push(*f);
                }
                PrintF => {
                    let f = self.float_stack.pop().unwrap();
                    println!("{}", f);
                }
                Print => {
                    let v = self.stack.pop().unwrap();
                    match v {
                        Value::Int(i) => println!("{}", i),
                        Value::Flt(f) => println!("{}", f),
                    }
                }
                PrintStr(s) => {
                    println!("{}", s);
                }
                Neg => {
                    let v = self.stack.pop().unwrap();
                    let result = match v {
                        Value::Int(n) => Value::Int(-n),
                        Value::Flt(f) => Value::Flt(-f),
                    };
                    self.stack.push(result);
                }

                Not => {
                    let v = self.stack.pop().unwrap();
                    let result = match v {
                        Value::Int(n) => Value::Int((n == 0) as i64),
                        Value::Flt(f) => Value::Int((f == 0.0) as i64),
                    };
                    self.stack.push(result);
                }
                Deref => {
                    let addr = self.stack.pop().unwrap();
                    let addr = match addr {
                        Value::Int(i) => i as usize,
                        _ => panic!("Cannot dereference non-integer address"),
                    };
                    let v = self.stack[addr].clone(); // clone to avoid borrow checker issues
                    self.stack.push(v);
                }
                Addr(offset) => {
                    let addr = (self.fp + *offset) as i64;
                    self.stack.push(Value::Int(addr));
                }
                Cast => { // added: no-op for C-style cast
                    // nothing to do at runtime
                }
                Imm(n) => {
                    self.stack.push(Value::Int(*n));
                }
                ImmF(f) => {
                    self.stack.push(Value::Flt(*f));
                }

                Add => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    let res = match (a, b) {
                        (Value::Int(x), Value::Int(y)) => Value::Int(x + y),
                        (Value::Flt(x), Value::Flt(y)) => Value::Flt(x + y),
                        (Value::Int(x), Value::Flt(y)) => Value::Flt(x as f64 + y),
                        (Value::Flt(x), Value::Int(y)) => Value::Flt(x + y as f64),
                    };
                    self.stack.push(res);
                }
                Sub => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(match (a, b) {
                        (Value::Int(x), Value::Int(y)) => Value::Int(x - y),
                        (Value::Flt(x), Value::Flt(y)) => Value::Flt(x - y),
                        (Value::Int(x), Value::Flt(y)) => Value::Flt(x as f64 - y),
                        (Value::Flt(x), Value::Int(y)) => Value::Flt(x - y as f64),
                    });
                }
                Mul => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(match (a, b) {
                        (Value::Int(x), Value::Int(y)) => Value::Int(x * y),
                        (Value::Flt(x), Value::Flt(y)) => Value::Flt(x * y),
                        (Value::Int(x), Value::Flt(y)) => Value::Flt(x as f64 * y),
                        (Value::Flt(x), Value::Int(y)) => Value::Flt(x * y as f64),
                    });
                }
                Div => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(match (a, b) {
                        (Value::Int(x), Value::Int(y)) => Value::Int(x / y),
                        (Value::Flt(x), Value::Flt(y)) => Value::Flt(x / y),
                        (Value::Int(x), Value::Flt(y)) => Value::Flt(x as f64 / y),
                        (Value::Flt(x), Value::Int(y)) => Value::Flt(x / y as f64),
                    });
                }
                Mod => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(match (a, b) {
                        (Value::Int(x), Value::Int(y)) => Value::Int(x % y),
                        _ => panic!("Modulo is not defined for floating-point numbers"),
                    });
                }

                Eq => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::Int((a == b) as i64));
                }
                Ne => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::Int((a != b) as i64));
                }
                Lt => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::Int(match (a, b) {
                        (Value::Int(x), Value::Int(y)) => (x < y) as i64,
                        (Value::Flt(x), Value::Flt(y)) => (x < y) as i64,
                        (Value::Int(x), Value::Flt(y)) => ((x as f64) < y) as i64,
                        (Value::Flt(x), Value::Int(y)) => (x < (y as f64)) as i64,
                    }));
                }
                Gt => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::Int(match (a, b) {
                        (Value::Int(x), Value::Int(y)) => (x > y) as i64,
                        (Value::Flt(x), Value::Flt(y)) => (x > y) as i64,
                        (Value::Int(x), Value::Flt(y)) => ((x as f64) > y) as i64,
                        (Value::Flt(x), Value::Int(y)) => (x > (y as f64)) as i64,
                    }));
                }
                Le => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::Int(match (a, b) {
                        (Value::Int(x), Value::Int(y)) => (x <= y) as i64,
                        (Value::Flt(x), Value::Flt(y)) => (x <= y) as i64,
                        (Value::Int(x), Value::Flt(y)) => ((x as f64) <= y) as i64,
                        (Value::Flt(x), Value::Int(y)) => (x <= (y as f64)) as i64,
                    }));
                }
                Ge => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::Int(match (a, b) {
                        (Value::Int(x), Value::Int(y)) => (x >= y) as i64,
                        (Value::Flt(x), Value::Flt(y)) => (x >= y) as i64,
                        (Value::Int(x), Value::Flt(y)) => ((x as f64) >= y) as i64,
                        (Value::Flt(x), Value::Int(y)) => (x >= (y as f64)) as i64,
                    }));
                }

                BitAnd => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Int(x & y)),
                        _ => panic!("Bitwise AND only supports integers"),
                    }
                }

                BitOr => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Int(x | y)),
                        _ => panic!("Bitwise OR only supports integers"),
                    }
                }

                BitXor => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Int(x ^ y)),
                        _ => panic!("Bitwise XOR only supports integers"),
                    }
                }

                Shl => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Int(x << y)),
                        _ => panic!("Shift operations only support integers"),
                    }
                }

                Shr => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Int(x >> y)),
                        _ => panic!("Shift operations only support integers"),
                    }
                }

                Jmp(lbl) => {
                    ip = labels[lbl];
                    continue;
                }

                Jz(lbl) => {
                    let v = self.stack.pop().unwrap();
                    let cond = match v {
                        Value::Int(i) => i == 0,
                        Value::Flt(f) => f == 0.0,
                    };
                    if cond {
                        ip = labels[lbl];
                        continue;
                    }
                }

                Label(_) => { /* no-op */ }

                Instruction::Call(addr) => {
                    // how many args this function expects
                    let num_args = *self
                        .function_arity
                        .get(addr)
                        .expect("Missing function arity");

                    // pop them off (right-to-left), then reverse to left-to-right
                    let mut args = Vec::with_capacity(num_args);
                    for _ in 0..num_args {
                        args.push(self.stack.pop().unwrap());
                    }
                    args.reverse();

                    // first push the old frame pointer and return address
                    self.stack.push(Value::Int(self.fp as i64));
                    self.stack.push(Value::Int((ip + 1) as i64));

                    // update the frame pointer
                    self.fp = self.stack.len();

                    // now push the arguments into the new frame
                    for arg in args {
                        self.stack.push(arg);
                    }

                    // jump into the function
                    ip = *addr;
                    continue;
                }

                Enter(n_locals) => {
                    // Allocate space for locals with default int value
                    for _ in 0..*n_locals {
                        self.stack.push(Value::Int(0));
                    }
                }

                LoadLocal(offset) => {
                    // load a local (offset into frame)
                    let v = self.stack[self.fp + *offset].clone();
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

                    // 3) pop return address (must be an Int)
                    let ret_addr = match self.stack.pop().unwrap() {
                        Value::Int(i) => i as usize,
                        _ => panic!("Expected integer return address"),
                    };

                    // 4) restore old FP
                    let old_fp = match self.stack.pop().unwrap() {
                        Value::Int(i) => i as usize,
                        _ => panic!("Expected integer frame pointer"),
                    };
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
        match self.stack.pop() {
            Some(Value::Int(i)) => Some(i),
            Some(Value::Flt(f)) => Some(f as i64), // convert float to int for return
            None => None,
        }
    }
}
