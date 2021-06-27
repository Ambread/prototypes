use std::mem::MaybeUninit;

pub trait JIterator {
    type Item;
    fn next(&mut self) -> Self::Item;

    fn collect<T>(self) -> T
    where
        T: FromJIterator<Self::Item>,
        Self: Sized,
    {
        FromJIterator::from_iter(self)
    }
}

pub fn from_fn<T>(func: impl FnMut() -> T) -> impl JIterator<Item = T> {
    FromFn { func }
}

struct FromFn<T, F>
where
    F: FnMut() -> T,
{
    func: F,
}

impl<T, F> JIterator for FromFn<T, F>
where
    F: FnMut() -> T,
{
    type Item = T;

    fn next(&mut self) -> Self::Item {
        (self.func)()
    }
}

pub fn repeat<T: Clone>(value: T) -> impl JIterator<Item = T> {
    from_fn(move || value.clone())
}

pub fn once<T>(value: T) -> impl JIterator<Item = Option<T>> {
    let mut value = Some(value);
    from_fn(move || value.take())
}

pub fn unfold<T, U>(value: T, mut func: impl FnMut(T) -> (T, U)) -> impl JIterator<Item = U> {
    let mut value = Some(value);
    from_fn(move || {
        let result = func(value.take().unwrap());
        value = Some(result.0);
        result.1
    })
}

pub trait FromJIterator<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: JIterator<Item = T>;
}

impl<T, const S: usize> FromJIterator<T> for [T; S] {
    fn from_iter<I>(mut iter: I) -> Self
    where
        I: JIterator<Item = T>,
    {
        let mut data = MaybeUninit::uninit_array();

        for elem in &mut data[..] {
            *elem = MaybeUninit::new(iter.next());
        }

        unsafe { MaybeUninit::array_assume_init(data) }
    }
}

#[cfg(test)]
#[test]
fn array_test() {
    assert_eq!(repeat(1).collect(), [1, 1, 1]);
    assert_eq!(
        unfold(0, |acc| {
            let acc = acc + 1;
            (acc, acc)
        })
        .collect(),
        [1, 2, 3, 4]
    );
}
