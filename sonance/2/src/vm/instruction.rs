#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    Halt,

    Push(usize),
    Pop,
    Dupe,

    Jump(usize),
    JumpIf(usize),

    Load(usize),
    Store(usize),

    Call(usize),
    Return,

    Add,
    Sub,
    Mul,
    Div,

    And,
    Or,
    Not,

    Eq,
    Gt,
    Geq,
}
