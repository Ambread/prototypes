use chumsky::prelude::*;

use crate::lexer::Token;

#[derive(Debug, Clone)]
pub struct Ast {
    pub name: String,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub body: Vec<u32>,
    pub trailing: u32,
}

pub fn parser() -> impl Parser<Token, Ast, Error = Simple<Token>> {
    use Token::*;

    let number = select! { Number(number) => number };

    let block = just(OpenBrace)
        .ignore_then(
            number
                .then_ignore(just(Semicolon))
                .repeated()
                .collect::<Vec<_>>(),
        )
        .then(number)
        .then_ignore(just(CloseBrace))
        .map(|(body, trailing)| Block { body, trailing });

    just(FuncKeyword)
        .ignore_then(select! { Identifier(name) => name })
        .then_ignore(just(OpenParen))
        .then_ignore(just(CloseParen))
        .then(block)
        .then_ignore(just(Semicolon))
        .map(|(name, body)| Ast { name, body })
}
