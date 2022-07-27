#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    Halt,

    Push(u64),
    Pop,
    Dupe,

    Jump,
    JumpIf,

    Load(u64),
    Store(u64),

    Call(usize),
    Return,

    In,
    Out,

    Add,
    Sub,
    Mul,
    Div,

    BitAnd,
    BitOr,
    BitNot,

    BoolAnd,
    BoolOr,
    BoolNot,

    Eq,
    Gt,
    Geq,
}
