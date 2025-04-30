#[derive(Debug, Clone)]
pub enum Instruction {
    Imm(i64),   // Push immediate value
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    // You’ll add more later: EQ, JSR, etc.
}
