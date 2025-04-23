use crate::vectors::Vector;

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

impl<const M: usize, const N: usize> From<[[f64; N]; M]> for Matrix<M, N> {
    fn from(array: [[f64; N]; M]) -> Self {
        Self(array)
    }
}
