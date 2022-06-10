use std::{
    array,
    ops::{Add, Mul},
};

pub struct Vector<T, const N: usize>([T; N]);

impl<T, const N: usize> Vector<T, N> {
    pub fn from_fn<F>(cb: F) -> Self
    where
        F: FnMut(usize) -> T,
    {
        Vector(array::from_fn(cb))
    }

    pub fn map<U, F>(self, f: F) -> Vector<U, N>
    where
        F: FnMut(T) -> U,
    {
        Vector(self.0.map(f))
    }

    pub fn zip<U>(self, rhs: Vector<U, N>) -> Vector<(T, U), N> {
        Vector(self.0.zip(rhs.0))
    }

    pub fn reduce<F>(self, f: F) -> T
    where
        F: FnMut(T, T) -> T,
    {
        self.0.into_iter().reduce(f).expect("N != 0")
    }

    pub fn fold<U, F>(self, init: U, f: F) -> U
    where
        F: FnMut(U, T) -> U,
    {
        self.0.into_iter().fold(init, f)
    }
}

impl<T: Default, const N: usize> Default for Vector<T, N> {
    fn default() -> Self {
        Self::from_fn(|_| T::default())
    }
}

impl<T: Add, const N: usize> Add for Vector<T, N> {
    type Output = Vector<T::Output, N>;

    fn add(self, rhs: Self) -> Self::Output {
        self.zip(rhs).map(|(x, y)| x + y)
    }
}

impl<T: Mul + Clone, const N: usize> Mul<T> for Vector<T, N> {
    type Output = Vector<T::Output, N>;

    fn mul(self, rhs: T) -> Self::Output {
        self.map(|x| x * rhs.clone())
    }
}
