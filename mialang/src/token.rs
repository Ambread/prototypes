use crate::ext::Spanned;

use super::ext::SelfExt;
use itertools::{Itertools, PeekingNext};
use thiserror::Error;

#[derive(Debug)]
pub enum Token<'a> {
    Whitespace,
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
    UnexpectedToken(&'a str),
    #[error("Missing digits after exponent symbol")]
    MissingDigitsAfterExponentSymbol,
}

pub fn parse(source: &str) -> impl Iterator<Item = Spanned<Result<Token, TokenError>>> {
    source
        .char_indices()
        .peekable()
        .batching(|input| {
            let (start, char) = input.next()?;

            if char.is_whitespace() {
                let end = start + input.peeking_take_while(|c| c.1.is_whitespace()).count();
                return Token::Whitespace
                    .map_self(Ok)
                    .spanned(start, end)
                    .map_self(Some);
            }

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

            if char.is_numeric()
                || ("+-".contains(char) && input.peek().map(|c| c.1.is_numeric()).unwrap_or(false))
            {
                let mut end = start + input.peeking_take_while(|c| c.1.is_numeric()).count();

                if input.peeking_next(|c| c.1 == '.').is_some() {
                    end += 1 + input.peeking_take_while(|c| c.1.is_numeric()).count()
                }

                if input.peeking_next(|c| "eE".contains(c.1)).is_some() {
                    if input
                        .peeking_next(|c| "+-".contains(c.1) || c.1.is_numeric())
                        .is_none()
                    {
                        return TokenError::MissingDigitsAfterExponentSymbol
                            .map_self(Err)
                            .spanned(start, 1 + end)
                            .map_self(Some);
                    }

                    end += 2 + input.peeking_take_while(|c| c.1.is_numeric()).count()
                }

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
                .map_self(TokenError::UnexpectedToken)
                .map_self(Err)
                .spanned(start, start)
                .map_self(Some)
        })
        .coalesce(|x, y| {
            if let (
                Spanned(x_span, Err(TokenError::UnexpectedToken(_))),
                Spanned(y_span, Err(TokenError::UnexpectedToken(_))),
            ) = (&x, &y)
            {
                source[x_span.start..=y_span.end]
                    .map_self(TokenError::UnexpectedToken)
                    .map_self(Err)
                    .spanned(x_span.start, y_span.end)
                    .map_self(Ok)
            } else {
                Err((x, y))
            }
        })
}
