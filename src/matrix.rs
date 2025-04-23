use crate::vector::Vector;

#[derive(Debug, Clone)]
pub struct Matrix<const M: usize, const N: usize>([[f64; N]; M]);

impl<const M: usize, const N: usize> Matrix<M, N> {
    pub fn dot(&self, vector: &Vector<N>) -> Vector<M> {
        std::array::from_fn(|i| {
            self.0[i]
                .iter()
                .zip(vector.iter())
                .map(|(a, b)| a * b)
                .sum()
        })
        .into()
    }
}

impl<const M: usize, const N: usize, const K: usize> std::ops::Mul<Matrix<N, K>> for Matrix<M, N> {
    type Output = Matrix<M, K>;
    fn mul(self, other: Matrix<N, K>) -> Self::Output {
        use std::array::from_fn;
        Matrix(from_fn(|i| {
            from_fn(|j| {
                self.0[i]
                    .iter()
                    .zip(&other.0)
                    .map(|(a, col)| a * col[j])
                    .sum()
            })
        }))
    }
}

impl<const M: usize, const N: usize> From<[[f64; N]; M]> for Matrix<M, N> {
    fn from(array: [[f64; N]; M]) -> Self {
        Self(array)
    }
}
