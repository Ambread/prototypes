use crate::ext::Spanned;

use super::ext::SelfExt;
use itertools::{Itertools, PeekingNext};
use thiserror::Error;

#[derive(Debug)]
pub enum Token<'a> {
    Number(f64),
    Ident(&'a str),
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Dot,
    Semicolon,
}

#[derive(Debug, Error)]
pub enum TokenError<'a> {
    #[error("Unexpected token \"{0}\"")]
    UnexpectedChar(&'a str),
}

pub fn parse(source: &str) -> impl Iterator<Item = Spanned<Result<Token, TokenError>>> {
    source
        .char_indices()
        .peekable()
        .batching(|input| {
            let (start, char) = input.next()?;

            'single: {
                return match char {
                    '(' => Token::OpenParen,
                    ')' => Token::CloseParen,
                    '{' => Token::OpenBrace,
                    '}' => Token::CloseBrace,
                    '.' => Token::Dot,
                    ';' => Token::Semicolon,
                    _ => break 'single,
                }
                .map_self(Ok)
                .spanned(start, start)
                .map_self(Some);
            }

            if char.is_numeric() {
                let whole = input.peeking_take_while(|c| c.1.is_numeric()).count();

                let decimal = input
                    .peeking_next(|c| c.1 == '.')
                    .map(|_| 1 + input.peeking_take_while(|c| c.1.is_numeric()).count())
                    .unwrap_or(0);

                let end = start + whole + decimal;

                return source[start..=end]
                    .parse::<f64>()
                    .unwrap()
                    .map_self(Token::Number)
                    .map_self(Ok)
                    .spanned(start, end)
                    .map_self(Some);
            }

            let symbols = "!@#$%^&*<>?|~-+";
            if char.is_alphabetic() || symbols.contains(char) {
                let end = input
                    .peeking_take_while(|c| c.1.is_alphanumeric() || symbols.contains(c.1))
                    .count()
                    + start;

                return source[start..=end]
                    .map_self(Token::Ident)
                    .map_self(Ok)
                    .spanned(start, end)
                    .map_self(Some);
            }

            source[start..=start]
                .map_self(TokenError::UnexpectedChar)
                .map_self(Err)
                .spanned(start, start)
                .map_self(Some)
        })
        .coalesce(|x, y| {
            if let (
                Spanned(x_span, Err(TokenError::UnexpectedChar(_))),
                Spanned(y_span, Err(TokenError::UnexpectedChar(_))),
            ) = (&x, &y)
            {
                source[x_span.start..=y_span.end]
                    .map_self(TokenError::UnexpectedChar)
                    .map_self(Err)
                    .spanned(x_span.start, y_span.end)
                    .map_self(Ok)
            } else {
                Err((x, y))
            }
        })
}
