#[derive(Debug, Clone)]
pub enum Instruction {
    Imm(i64),
    Add, Sub, Mul, Div, Mod,
    Neg, Not, Deref, Addr(usize), Cast,
    Eq, Ne, Lt, Gt, Le, Ge,
    BitAnd, BitOr, BitXor,
    Shl, Shr,
    Jmp(usize),
    Jz(usize),
    Label(usize),
    LoadLocal(usize),
    StoreLocal(usize),
    Call(usize),
    Enter(usize),
    Leave,
}
