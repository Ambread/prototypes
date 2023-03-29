use crate::ext::Spanned;

use super::ext::SelfExt;
use itertools::Itertools;
use thiserror::Error;
use Token::*;
use TokenError::*;

#[derive(Debug)]
pub enum Token {
    Number(f64),
}

#[derive(Debug, Error)]
pub enum TokenError<'a> {
    #[error("Unexpected token \"{0}\"")]
    UnexpectedChar(&'a str),
}

pub fn parse(source: &str) -> impl Iterator<Item = Spanned<Result<Token, TokenError<'_>>>> + '_ {
    source
        .char_indices()
        .peekable()
        .batching(|input| {
            let (index, char) = *input.peek()?;

            if char.is_numeric() {
                let string = input
                    .peeking_take_while(|c| c.1.is_numeric())
                    .map(|c| c.1)
                    .collect::<String>();
                return string
                    .parse::<f64>()
                    .unwrap()
                    .map_self(Token::Number)
                    .wrap_ok()
                    .spanned(index, index + string.len() - 1)
                    .wrap_some();
            }

            input.next().unwrap();
            TokenError::UnexpectedChar(&source[index..index])
                .wrap_err()
                .spanned(index, index)
                .wrap_some()
        })
        .coalesce(|x, y| {
            if let (
                Spanned(x_span, Err(UnexpectedChar(_))),
                Spanned(y_span, Err(UnexpectedChar(_))),
            ) = (&x, &y)
            {
                UnexpectedChar(&source[x_span.start..y_span.end])
                    .wrap_err()
                    .spanned(x_span.start, y_span.end)
                    .wrap_ok()
            } else {
                Err((x, y))
            }
        })
}
