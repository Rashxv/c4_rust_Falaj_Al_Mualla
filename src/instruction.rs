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
    LoadLocal(usize),   // Load local variable by stack offset
    StoreLocal(usize),  // Store to local variable by stack offset
    Call(usize),     // new: call function at instruction index
    Enter(usize),    // new: allocate space for locals
    Leave,           // new: return from function
}