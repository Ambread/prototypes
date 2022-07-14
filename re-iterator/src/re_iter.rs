trait ReIterator {
    type Item;

    fn next(&mut self) -> Self::Item;

    fn collect<T>(self) -> T
    where
        Self: Sized,
        T: FromReIterator<Self::Item>,
    {
        T::from_iter(self)
    }

    fn map<J, F>(self, f: F) -> Map<Self::Item, J, Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> J,
    {
        Map { iter: self, f }
    }
}

struct Map<I, J, T, F>
where
    T: ReIterator<Item = I>,
    F: FnMut(I) -> J,
{
    iter: T,
    f: F,
}

impl<I, J, T, F> ReIterator for Map<I, J, T, F>
where
    T: ReIterator<Item = I>,
    F: FnMut(I) -> J,
{
    type Item = J;

    fn next(&mut self) -> Self::Item {
        (self.f)(self.iter.next())
    }
}

impl<I, T> ReIterator for T
where
    T: FnMut() -> I,
{
    type Item = I;

    fn next(&mut self) -> Self::Item {
        self()
    }
}

trait IntoReIterator {
    type Item;
    type IntoIter: ReIterator<Item = Self::Item>;
    fn into_iter(self) -> Self::IntoIter;
}

impl<T> IntoReIterator for T
where
    T: ReIterator,
{
    type Item = T::Item;

    type IntoIter = T;

    fn into_iter(self) -> Self::IntoIter {
        self
    }
}

trait FromReIterator<I> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoReIterator<Item = I>;
}

impl<I, const N: usize> FromReIterator<I> for [I; N] {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoReIterator<Item = I>,
    {
        let mut iter = iter.into_iter();
        [(); N].map(|()| iter.next())
    }
}

impl<I> FromReIterator<Option<I>> for Vec<I> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoReIterator<Item = Option<I>>,
    {
        let mut iter = iter.into_iter();
        let mut output = Vec::new();
        while let Some(value) = iter.next() {
            output.push(value);
        }
        output
    }
}
