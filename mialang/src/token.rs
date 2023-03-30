use std::{iter::Peekable, str::CharIndices};

use crate::ext::{Span, Spanned};

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
    span: Span,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            input: source.char_indices().peekable(),
            span: Span { start: 0, end: 0 },
        }
    }

    fn munch(&mut self, mut accept: impl FnMut(char) -> bool) {
        self.span.end = self
            .input
            .peeking_take_while(|c| accept(c.1))
            .last()
            .map_or(self.span.start, |c| c.0);
    }

    fn ok(&self, token: Token<'a>) -> Option<Spanned<Result<Token<'a>, TokenError<'a>>>> {
        Some(Spanned(self.span, Ok(token)))
    }

    fn err(&self, err: TokenError<'a>) -> Option<Spanned<Result<Token<'a>, TokenError<'a>>>> {
        Some(Spanned(self.span, Err(err)))
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Spanned<Result<Token<'a>, TokenError<'a>>>;

    fn next(&mut self) -> Option<Self::Item> {
        let (start, char) = self.input.next()?;
        self.span = Span::new(start, start);

        if char.is_whitespace() {
            self.munch(|c| c.is_whitespace());

            return self.ok(Token::Whitespace);
        }

        'single: {
            return self.ok(match char {
                '(' => Token::OpenParen,
                ')' => Token::CloseParen,
                '{' => Token::OpenBrace,
                '}' => Token::CloseBrace,
                '.' => Token::Dot,
                ';' => Token::Semicolon,
                _ => break 'single,
            });
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
            self.munch(|c| c.is_alphanumeric() || symbols.contains(c));

            return self.ok(Token::Ident(&self.source[self.span]));
        }

        self.err(TokenError::UnexpectedToken(&self.source[self.span]))
    }
}
