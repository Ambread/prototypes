use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    Halt,

    Push(u64),
    Pop,
    Dupe,

    Jump,
    JumpIf,

    Load,
    Store,

    Call,
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

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Halt => write!(f, "halt"),

            Instruction::Push(value) => write!(f, "push {value}"),
            Instruction::Pop => write!(f, "pop"),
            Instruction::Dupe => write!(f, "dupe"),

            Instruction::Jump => write!(f, "jump"),
            Instruction::JumpIf => write!(f, "jump_if"),

            Instruction::Load => write!(f, "load"),
            Instruction::Store => write!(f, "store"),

            Instruction::Call => write!(f, "call"),
            Instruction::Return => write!(f, "return"),

            Instruction::In => write!(f, "in"),
            Instruction::Out => write!(f, "out"),

            Instruction::Add => write!(f, "add"),
            Instruction::Sub => write!(f, "sub"),
            Instruction::Mul => write!(f, "mul"),
            Instruction::Div => write!(f, "div"),

            Instruction::BitAnd => write!(f, "bit_and"),
            Instruction::BitOr => write!(f, "bit_or"),
            Instruction::BitNot => write!(f, "bit_not"),

            Instruction::BoolAnd => write!(f, "bool_and"),
            Instruction::BoolOr => write!(f, "bool_or"),
            Instruction::BoolNot => write!(f, "bool_not"),

            Instruction::Eq => write!(f, "eq"),
            Instruction::Gt => write!(f, "gt"),
            Instruction::Geq => write!(f, "geq"),
        }
    }
}
