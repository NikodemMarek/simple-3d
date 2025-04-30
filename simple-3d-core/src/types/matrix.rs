use super::vector::Vector;

#[derive(Debug, Clone, PartialEq)]
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

impl<const N: usize> Matrix<N, N> {
    pub fn identity() -> Self {
        use std::array::from_fn;
        from_fn(|i| from_fn(|j| if i == j { 1.0 } else { 0.0 })).into()
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

#[cfg(test)]
mod tests {
    use approx::{AbsDiffEq, RelativeEq};

    use super::*;
    use crate::vector::Vector;

    impl RelativeEq for Matrix<4, 4> {
        fn relative_eq(
            &self,
            other: &Self,
            epsilon: Self::Epsilon,
            max_relative: Self::Epsilon,
        ) -> bool {
            self.0
                .iter()
                .zip(other.0.iter())
                .all(|(a, b)| a.relative_eq(b, epsilon, max_relative))
        }
        fn default_max_relative() -> Self::Epsilon {
            1e-12
        }
    }
    impl AbsDiffEq for Matrix<4, 4> {
        type Epsilon = f64;
        fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
            self.0
                .iter()
                .zip(other.0.iter())
                .all(|(a, b)| a.abs_diff_eq(b, epsilon))
        }
        fn default_epsilon() -> Self::Epsilon {
            1e-12
        }
    }

    #[test]
    fn test_identity() {
        assert_eq!(
            Matrix::<3, 3>::identity(),
            Matrix::from([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
        );
        assert_eq!(
            Matrix::<4, 4>::identity(),
            Matrix::from([
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ])
        );
    }

    #[test]
    fn test_dot() {
        assert_eq!(
            Matrix::from([[1.0, 2.0], [3.0, 4.0]]).dot(&Vector::from([5.0, 6.0])),
            Vector::from([17.0, 39.0])
        );

        assert_eq!(
            Matrix::from([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]).dot(&Vector::from([7.0, 8.0, 9.0])),
            Vector::from([50.0, 122.0])
        );
    }

    #[test]
    fn test_mul() {
        assert_eq!(
            Matrix::from([[1.0, 2.0], [3.0, 4.0]]) * Matrix::from([[5.0, 6.0], [7.0, 8.0]]),
            Matrix::from([[19.0, 22.0], [43.0, 50.0]])
        );
        assert_eq!(
            Matrix::from([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]])
                * Matrix::from([[7.0, 8.0], [9.0, 10.0], [11.0, 12.0]]),
            Matrix::from([[58.0, 64.0], [139.0, 154.0]])
        );
    }
}
