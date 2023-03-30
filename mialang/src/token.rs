use std::{iter::Peekable, str::CharIndices};

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

pub struct Lexer<'a> {
    source: &'a str,
    input: Peekable<CharIndices<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            input: source.char_indices().peekable(),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Spanned<Result<Token<'a>, TokenError<'a>>>;

    fn next(&mut self) -> Option<Self::Item> {
        let (start, char) = self.input.next()?;

        if char.is_whitespace() {
            let end = self
                .input
                .peeking_take_while(|c| c.1.is_whitespace())
                .last()
                .map_or(start, |c| c.0);

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
            || ("+-".contains(char) && self.input.peek().map(|c| c.1.is_numeric()).unwrap_or(false))
        {
            let mut end = self
                .input
                .peeking_take_while(|c| c.1.is_numeric())
                .last()
                .map_or(start, |c| c.0);

            if self.input.peeking_next(|c| c.1 == '.').is_some() {
                end = self
                    .input
                    .peeking_take_while(|c| c.1.is_numeric())
                    .last()
                    .map_or(start, |c| c.0);
            }

            if self.input.peeking_next(|c| "eE".contains(c.1)).is_some() {
                if self
                    .input
                    .peeking_next(|c| "+-".contains(c.1) || c.1.is_numeric())
                    .is_none()
                {
                    return TokenError::MissingDigitsAfterExponentSymbol
                        .map_self(Err)
                        .spanned(start, 1 + end)
                        .map_self(Some);
                }

                end = self
                    .input
                    .peeking_take_while(|c| c.1.is_numeric())
                    .last()
                    .map_or(start, |c| c.0);
            }

            return self.source[start..=end]
                .parse::<f64>()
                .unwrap()
                .map_self(Token::Number)
                .map_self(Ok)
                .spanned(start, end)
                .map_self(Some);
        }

        let symbols = "!@#$%^&*<>?|~-+";
        if char.is_alphabetic() || symbols.contains(char) {
            let end = self
                .input
                .peeking_take_while(|c| c.1.is_alphanumeric() || symbols.contains(c.1))
                .last()
                .map_or(start, |c| c.0);

            return self.source[start..=end]
                .map_self(Token::Ident)
                .map_self(Ok)
                .spanned(start, end)
                .map_self(Some);
        }

        self.source[start..=start]
            .map_self(TokenError::UnexpectedToken)
            .map_self(Err)
            .spanned(start, start)
            .map_self(Some)
    }
}
