use std::{fmt::Debug, ops::Index};

pub trait SelfExt: Sized {
    fn map_self<T>(self, body: impl FnOnce(Self) -> T) -> T {
        body(self)
    }

    fn spanned(self, start: usize, end: usize) -> Spanned<Self> {
        Spanned(Span { start, end }, self)
    }
}

impl<T> SelfExt for T {}

#[derive(Clone, Copy)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

impl Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

impl Index<Span> for str {
    type Output = str;

    fn index(&self, span: Span) -> &Self::Output {
        &self[span.start..=span.end]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Spanned<T>(pub Span, pub T);
