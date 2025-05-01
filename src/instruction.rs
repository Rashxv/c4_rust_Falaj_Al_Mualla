#[derive(Debug, Clone)]
pub enum Instruction {
    Imm(i64),
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    // New control flow ops
    Jmp(usize),      // unconditional jump
    Jz(usize),       // jump if zero
    Label(usize),    // pseudo-op for labeling
}