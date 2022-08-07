use std::str::FromStr;

use crate::instruction::Instruction;

#[derive(Debug, Clone)]
enum Line {
    Instruction(Instruction, Vec<Argument>),
    Label(String),
    Constant(String, Vec<Literal>),
}

#[derive(Debug, Clone)]
enum Argument {
    Literal(Literal),
    Constant(String),
    Label(String),
}

#[derive(Debug, Clone)]
enum Literal {
    Number(Ty),
    Char(Ty),
}

#[derive(Debug, Clone)]
enum Ty {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}

// fn parse_literal(prefix: &str, src: &str, suffix: &str) {
//     (src.starts_with(prefix) && src.starts_with(suffix)).then(|| src.stri)
// }

impl FromStr for Literal {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
