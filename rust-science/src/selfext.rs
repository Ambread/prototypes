trait SelfExt {
    fn also<R>(mut self, mut body: impl FnMut(&mut Self)) -> Self
    where
        Self: Sized,
    {
        body(&mut self);
        self
    }

    fn with<R>(self, body: impl FnOnce(Self) -> R) -> R
    where
        Self: Sized,
    {
        body(self)
    }
}

impl<T> SelfExt for T {}
