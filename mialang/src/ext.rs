use std::fmt::Debug;

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

impl Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Spanned<T>(pub Span, pub T);
