use crate::ext::Spanned;

use super::ext::SelfExt;
use itertools::Itertools;
use thiserror::Error;

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
            let (start, char) = input.next()?;

            if char.is_numeric() {
                let end = start + input.peeking_take_while(|c| c.1.is_numeric()).count();
                return source[start..end]
                    .parse::<f64>()
                    .unwrap()
                    .map_self(Token::Number)
                    .map_self(Ok)
                    .spanned(start, end)
                    .map_self(Some);
            }

            source[start..start]
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
                source[x_span.start..y_span.end]
                    .map_self(TokenError::UnexpectedChar)
                    .map_self(Err)
                    .spanned(x_span.start, y_span.end)
                    .map_self(Ok)
            } else {
                Err((x, y))
            }
        })
}
