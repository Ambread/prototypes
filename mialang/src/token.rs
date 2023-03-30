use std::{iter::Peekable, str::CharIndices};

use crate::ext::{Span, Spanned};

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

    fn munch_one(&mut self, accept: impl FnOnce(char) -> bool) -> bool {
        self.input.peeking_next(|c| accept(c.1)).map_or(false, |c| {
            self.span.end = c.0;
            true
        })
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

        if char.is_numeric() || ("+-".contains(char) && self.munch_one(|c| c.is_numeric())) {
            self.munch(|c| c.is_numeric());

            if self.munch_one(|c| c == '.') {
                self.munch(|c| c.is_numeric());
            }

            if self.munch_one(|c| "eE".contains(c)) {
                if !self.munch_one(|c| "+-".contains(c) || c.is_numeric()) {
                    return self.err(TokenError::MissingDigitsAfterExponentSymbol);
                }

                self.munch(|c| c.is_numeric());
            }

            let number = self.source[self.span].parse().unwrap();
            return self.ok(Token::Number(number));
        }

        let symbols = "!@#$%^&*<>?|~-+";
        if char.is_alphabetic() || symbols.contains(char) {
            self.munch(|c| c.is_alphanumeric() || symbols.contains(c));

            return self.ok(Token::Ident(&self.source[self.span]));
        }

        self.err(TokenError::UnexpectedToken(&self.source[self.span]))
    }
}
