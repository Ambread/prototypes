use chumsky::prelude::*;

use crate::lexer::Token;

#[derive(Debug, Clone)]
pub struct Ast {
    pub name: String,
    pub number: u64,
}

pub fn parser() -> impl Parser<Token, Ast, Error = Simple<Token>> {
    use Token::*;

    just(FuncKeyword)
        .ignore_then(select! { Identifier(name) => name })
        .then_ignore(just(OpenParen))
        .then_ignore(just(CloseParen))
        .then_ignore(just(OpenBrace))
        .then(select! { Number(number) => number })
        .then_ignore(just(CloseBrace))
        .then_ignore(just(Semicolon))
        .map(|(name, number)| Ast { name, number })
}
