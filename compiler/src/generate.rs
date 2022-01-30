use std::fmt::Display;

use crate::parser::Ast;

pub fn generate(ast: Ast) -> String {
    Function {
        name: ast.name,
        body: Instruction::Ret(ast.number),
    }
    .to_string()
}

#[derive(Debug, Clone)]
struct Function {
    name: String,
    body: Instruction,
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "export function w ${}() {{\n@start", self.name)?;
        writeln!(f, "{}", self.body)?;
        writeln!(f, "}}")
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Ret(u32),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "    ")?;

        match self {
            Self::Ret(number) => write!(f, "ret {number}"),
        }
    }
}
