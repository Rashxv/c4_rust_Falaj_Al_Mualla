use crate::instruction::Instruction;

pub struct VM {
    stack: Vec<i64>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
        }
    }

    pub fn run(&mut self, code: &[Instruction]) -> Option<i64> {
        for instr in code {
            match instr {
                Instruction::Imm(n) => {
                    self.stack.push(*n);
                }
                Instruction::Add => {
                    let b = self.stack.pop().expect("Stack underflow (Add)");
                    let a = self.stack.pop().expect("Stack underflow (Add)");
                    self.stack.push(a + b);
                }
                Instruction::Sub => {
                    let b = self.stack.pop().expect("Stack underflow (Sub)");
                    let a = self.stack.pop().expect("Stack underflow (Sub)");
                    self.stack.push(a - b);
                }
                Instruction::Mul => {
                    let b = self.stack.pop().expect("Stack underflow (Mul)");
                    let a = self.stack.pop().expect("Stack underflow (Mul)");
                    self.stack.push(a * b);
                }
                Instruction::Div => {
                    let b = self.stack.pop().expect("Stack underflow (Div)");
                    let a = self.stack.pop().expect("Stack underflow (Div)");
                    self.stack.push(a / b);
                }
                Instruction::Mod => {
                    let b = self.stack.pop().expect("Stack underflow (Mod)");
                    let a = self.stack.pop().expect("Stack underflow (Mod)");
                    self.stack.push(a % b);
                }
            }
        }

        self.stack.pop()
    }
}
