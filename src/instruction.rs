/// The `Instruction` enum defines the complete set of virtual machine instructions
/// used by the C4 Rust compiler backend. These instructions are interpreted by the VM
/// and represent low-level operations such as arithmetic, memory access, control flow,
/// function calls, and printing. Each variant corresponds to a specific behavior that
/// the virtual machine must implement.
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
