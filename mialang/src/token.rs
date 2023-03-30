use std::{iter::Peekable, str::CharIndices};

use crate::ext::{Span, Spanned};

use itertools::{Itertools, PeekingNext};
use thiserror::Error;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Error)]
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
    error_state: ErrorState<'a>,
}

#[derive(Debug, Clone, Default)]
enum ErrorState<'a> {
    #[default]
    None,
    UnexpectedToken(Span),
    SavedItem(Item<'a>),
}

impl<'a> ErrorState<'a> {
    fn take_saved(&mut self) -> Option<Item<'a>> {
        match std::mem::take(self) {
            ErrorState::SavedItem(item) => Some(item),
            state => {
                *self = state;
                None
            }
        }
    }

    fn take_any(&mut self, source: &'a str) -> Option<Item<'a>> {
        match std::mem::take(self) {
            ErrorState::None => None,
            ErrorState::UnexpectedToken(span) => Some(Spanned(
                span,
                Err(TokenError::UnexpectedToken(&source[span])),
            )),
            ErrorState::SavedItem(item) => Some(item),
        }
    }

    fn save_if_needed(&mut self, item: Item<'a>, source: &'a str) -> Option<Item<'a>> {
        match std::mem::take(self) {
            ErrorState::None => Some(item),
            ErrorState::SavedItem(_) => unreachable!("item already saved"),
            ErrorState::UnexpectedToken(span) => {
                *self = ErrorState::SavedItem(item);
                Some(Spanned(
                    span,
                    Err(TokenError::UnexpectedToken(&source[span])),
                ))
            }
        }
    }

    fn update(&mut self, span: Span) {
        match self {
            ErrorState::None => *self = ErrorState::UnexpectedToken(span),
            ErrorState::UnexpectedToken(err) => err.end = span.end,
            ErrorState::SavedItem(_) => unreachable!("should have return saved item earlier"),
        }
    }
}

type Item<'a> = Spanned<Result<Token<'a>, TokenError<'a>>>;

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            input: source.char_indices().peekable(),
            span: Span { start: 0, end: 0 },
            error_state: ErrorState::None,
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

    fn ok(&mut self, token: Token<'a>) -> Option<Item<'a>> {
        let item = Spanned(self.span, Ok(token));
        self.error_state.save_if_needed(item, self.source)
    }

    fn err(&mut self, err: TokenError<'a>) -> Option<Item<'a>> {
        let item = Spanned(self.span, Err(err));
        self.error_state.save_if_needed(item, self.source)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Item<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let char = {
            let saved = self.error_state.take_saved();
            if saved.is_some() {
                return saved;
            }

            let Some((start, char)) = self.input.next() else {
                return self.error_state.take_any(self.source);
            };

            self.span = Span::new(start, start);
            char
        };

        if char.is_whitespace() {
            self.munch(|c| c.is_whitespace());

            return self.next();
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

        self.error_state.update(self.span);
        self.next()
    }
}
