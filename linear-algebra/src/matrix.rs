use std::ops::{Add, Mul};

use crate::vector::Vector;

struct Matrix<T, const N: usize, const M: usize>(Vector<Vector<T, N>, M>);

impl<T: Mul + Clone, const N: usize, const M: usize> Mul<Vector<T, M>> for Matrix<T, N, M>
where
    <<T as Mul>::Output as Add>::Output: Default,
    <T as Mul>::Output: Add<Output = <T as Mul>::Output>,
{
    type Output = Vector<<<T as Mul>::Output as Add>::Output, N>;

    fn mul(self, rhs: Vector<T, M>) -> Self::Output {
        self.0
            .zip(rhs)
            .map(|(column, x)| column * x)
            .reduce(|a, b| a + b)
    }
}
