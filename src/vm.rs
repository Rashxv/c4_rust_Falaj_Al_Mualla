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
        use Instruction::*;
    
        // Step 1: Build label-to-index mapping
        let mut labels = std::collections::HashMap::new();
        for (i, instr) in code.iter().enumerate() {
            if let Label(id) = instr {
                labels.insert(*id, i);
            }
        }
    
        // Step 2: Start execution
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
                Jmp(label) => {
                    ip = *labels.get(label).expect("Invalid JMP label");
                    continue;
                }
                Jz(label) => {
                    let val = self.stack.pop().unwrap();
                    if val == 0 {
                        ip = *labels.get(label).expect("Invalid JZ label");
                        continue;
                    }
                }
                Label(_) => {
                    // Labels are handled during label resolution — skip
                }
            }
    
            ip += 1;
        }
    
        self.stack.pop()
    }
    
}