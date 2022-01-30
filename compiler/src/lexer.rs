use chumsky::{prelude::*, text};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    FuncKeyword,
    Identifier(String),
    OpenParen,
    CloseParen,
    OpenBrace,
    Number(u64),
    CloseBrace,
    Semicolon,
}

pub fn lexer() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
    use Token::*;

    let num = text::int(10)
        .chain::<char, _, _>(just('.').chain(text::digits(10)).or_not().flatten())
        .collect::<String>()
        .map(|num| Number(num.parse().unwrap()));

    let ctrl = just('(')
        .to(OpenParen)
        .or(just(')').to(CloseParen))
        .or(just('{').to(OpenBrace))
        .or(just('}').to(CloseBrace))
        .or(just(';').to(Semicolon));

    let ident = text::ident().map(|ident: String| match ident.as_str() {
        "func" => FuncKeyword,
        _ => Identifier(ident),
    });

    let token = num
        .or(ctrl)
        .or(ident)
        .recover_with(skip_then_retry_until([]));

    token.padded().repeated()
}
