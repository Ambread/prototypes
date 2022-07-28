use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    Halt,

    Push,
    Pop,
    Dupe,

    Jump,
    JumpIf,

    Load,
    Store,

    Call,
    Return,

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

impl PartialEq<Instruction> for u8 {
    fn eq(&self, other: &Instruction) -> bool {
        *self == *other as u8
    }
}

impl TryFrom<u8> for Instruction {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            _ if value == Instruction::Halt => Instruction::Halt,

            _ if value == Instruction::Push => Instruction::Push,
            _ if value == Instruction::Pop => Instruction::Pop,
            _ if value == Instruction::Dupe => Instruction::Dupe,

            _ if value == Instruction::Jump => Instruction::Jump,
            _ if value == Instruction::JumpIf => Instruction::JumpIf,

            _ if value == Instruction::Load => Instruction::Load,
            _ if value == Instruction::Store => Instruction::Store,

            _ if value == Instruction::Call => Instruction::Call,
            _ if value == Instruction::Return => Instruction::Return,

            _ if value == Instruction::Add => Instruction::Add,
            _ if value == Instruction::Sub => Instruction::Sub,
            _ if value == Instruction::Mul => Instruction::Mul,
            _ if value == Instruction::Div => Instruction::Div,

            _ if value == Instruction::BitAnd => Instruction::BitAnd,
            _ if value == Instruction::BitOr => Instruction::BitOr,
            _ if value == Instruction::BitNot => Instruction::BitNot,

            _ if value == Instruction::BoolAnd => Instruction::BoolAnd,
            _ if value == Instruction::BoolOr => Instruction::BoolOr,
            _ if value == Instruction::BoolNot => Instruction::BoolNot,

            _ if value == Instruction::Eq => Instruction::Eq,
            _ if value == Instruction::Gt => Instruction::Gt,
            _ if value == Instruction::Geq => Instruction::Geq,

            _ => return Err(()),
        })
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Halt => write!(f, "halt"),

            Instruction::Push => write!(f, "push"),
            Instruction::Pop => write!(f, "pop"),
            Instruction::Dupe => write!(f, "dupe"),

            Instruction::Jump => write!(f, "jump"),
            Instruction::JumpIf => write!(f, "jump_if"),

            Instruction::Load => write!(f, "load"),
            Instruction::Store => write!(f, "store"),

            Instruction::Call => write!(f, "call"),
            Instruction::Return => write!(f, "return"),

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

            "push" => Instruction::Push,
            "pop" => Instruction::Pop,
            "dupe" => Instruction::Dupe,

            "jump" => Instruction::Jump,
            "jump_if" => Instruction::JumpIf,

            "load" => Instruction::Load,
            "store" => Instruction::Store,

            "call" => Instruction::Call,
            "return" => Instruction::Return,

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
