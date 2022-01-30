mod generate;

use chumsky::{prelude::*, text};
use generate::generate;

pub fn compile(input: &str) -> String {
    let tokens = lexer().parse(input).unwrap();
    let ast = parser().parse(tokens).unwrap();
    generate(ast)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Token {
    FuncKeyword,
    Identifier(String),
    OpenParen,
    CloseParen,
    OpenBrace,
    Number(u64),
    CloseBrace,
    Semicolon,
}

fn lexer() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
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

#[derive(Debug, Clone)]
pub struct Ast {
    name: String,
    number: u64,
}

fn parser() -> impl Parser<Token, Ast, Error = Simple<Token>> {
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

#[cfg(test)]
mod test {
    use indoc::formatdoc;
    use proptest::prelude::*;

    use crate::compile;

    proptest! {
        #[test]
        fn compile_number(name in "[a-zA-Z_]+", number: u64) {
            let input = format!("func {name}() {{ {number} }};");

            let expected = formatdoc!(
                "
                    export function w ${name}() {{
                    @start
                        ret {number}
                    }}
                "
            );

            assert_eq!(expected, compile(&input));
        }
    }
}
