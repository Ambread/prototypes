use std::{fmt::Display, str::FromStr};

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

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "halt" => Instruction::Halt,

            "push" => panic!("Don't use Instruction::FromStr for Push"),
            "pop" => Instruction::Pop,
            "dupe" => Instruction::Dupe,

            "jump" => Instruction::Jump,
            "jump_if" => Instruction::JumpIf,

            "load" => Instruction::Load,
            "store" => Instruction::Store,

            "call" => Instruction::Call,
            "return" => Instruction::Return,

            "in" => Instruction::In,
            "out" => Instruction::Out,

            "add" => Instruction::Add,
            "sub" => Instruction::Sub,
            "mul" => Instruction::Mul,
            "div" => Instruction::Div,

            "bit_and" => Instruction::BitAnd,
            "bit_or" => Instruction::BitOr,
            "bit_not" => Instruction::BitNot,

            "bool_and" => Instruction::BoolAnd,
            "bool_or" => Instruction::BoolOr,
            "bool_not" => Instruction::BoolNot,

            "eq" => Instruction::Eq,
            "gt" => Instruction::Gt,
            "geq" => Instruction::Geq,

            _ => return Err(()),
        })
    }
}
