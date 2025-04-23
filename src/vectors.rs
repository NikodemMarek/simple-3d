use std::iter::zip;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Vector<const S: usize>([f64; S]);

impl<const S: usize> Vector<S> {
    pub fn magnitude(&self) -> f64 {
        self.0.iter().map(|x| x.powi(2)).sum::<f64>().sqrt()
    }
    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        self.0.iter().map(|x| x / mag).collect()
    }
    pub fn dot(&self, other: Self) -> f64 {
        zip(self.0.iter(), other.0.iter()).map(|(a, b)| a * b).sum()
    }
}

impl Vector<3> {
    pub fn cross(&self, other: Self) -> Self {
        Self([
            self.0[1] * other.0[2] - self.0[2] * other.0[1],
            self.0[2] * other.0[0] - self.0[0] * other.0[2],
            self.0[0] * other.0[1] - self.0[1] * other.0[0],
        ])
    }

    pub fn homogenous(&self) -> Vector<4> {
        [
            self.0[0],
            self.0[1],
            if self.0[2] == 0.0 { 0.0001 } else { self.0[2] },
            1.0,
        ]
        .into()
    }
}

impl<const S: usize> std::ops::Index<usize> for Vector<S> {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        if index >= S {
            panic!("Index out of bounds");
        }
        &self.0[index]
    }
}
impl<const S: usize> std::ops::IndexMut<usize> for Vector<S> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= S {
            panic!("Index out of bounds");
        }
        &mut self.0[index]
    }
}

impl<const S: usize> std::ops::Deref for Vector<S> {
    type Target = [f64; S];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const S: usize> std::ops::Add for Vector<S> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() {
            panic!("Vectors must be of the same length");
        }
        if self.0.len() != S {
            panic!("Vector length mismatch");
        }

        zip(self.0.iter(), other.0.iter())
            .map(|(a, b)| a + b)
            .collect()
    }
}
impl<const S: usize> std::ops::AddAssign for Vector<S> {
    fn add_assign(&mut self, other: Self) {
        if self.0.len() != other.0.len() {
            panic!("Vectors must be of the same length");
        }
        if self.0.len() != S {
            panic!("Vector length mismatch");
        }
        for (a, b) in zip(self.0.iter_mut(), other.0.iter()) {
            *a += *b;
        }
    }
}
impl<const S: usize> std::ops::Sub for Vector<S> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() {
            panic!("Vectors must be of the same length");
        }
        if self.0.len() != S {
            panic!("Vector length mismatch");
        }

        zip(self.0.iter(), other.0.iter())
            .map(|(a, b)| a - b)
            .collect()
    }
}

impl<const S: usize> From<[f64; S]> for Vector<S> {
    fn from(arr: [f64; S]) -> Self {
        Self(arr)
    }
}
impl<const S: usize> From<Vec<f64>> for Vector<S> {
    fn from(vec: Vec<f64>) -> Self {
        Self(vec.try_into().unwrap())
    }
}
impl From<(f64, f64, f64)> for Vector<3> {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Self([x, y, z])
    }
}
impl<const S: usize> FromIterator<f64> for Vector<S> {
    fn from_iter<I: IntoIterator<Item = f64>>(iter: I) -> Self {
        iter.into_iter().collect::<Vec<_>>().into()
    }
}
