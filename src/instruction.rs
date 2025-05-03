#[derive(Debug, Clone)]
pub enum Instruction {
    Imm(i64),
    Add, Sub, Mul, Div, Mod,
    Neg, Not, Deref, Addr(usize), Cast,
    Eq, Ne, Lt, Gt, Le, Ge,
    BitAnd, BitOr, BitXor,
    Shl, Shr,Print,
    PrintStr(String),// prints a Rust string literal
    PushF(f64),
    PrintF,
    ImmF(f64),
    Jmp(usize),
    Jz(usize),
    Label(usize),
    LoadLocal(usize),
    StoreLocal(usize),
    Call(usize),
    Enter(usize),
    Leave,
}
