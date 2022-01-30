mod generate;
mod lexer;
mod parser;
#[cfg(test)]
mod test;

use chumsky::prelude::*;

use generate::generate;
use lexer::lexer;
use parser::parser;

pub fn compile(input: &str) -> String {
    let tokens = lexer().parse(input).unwrap();
    let ast = parser().parse(tokens).unwrap();
    generate(ast)
}
