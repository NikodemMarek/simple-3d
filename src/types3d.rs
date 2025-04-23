use std::ops::Sub;

pub trait Vector {
    fn magnitude(&self) -> f64;
    fn normalize(&self) -> Self;
    fn dot(&self, other: Self) -> f64;
    fn cross(&self, other: Self) -> Self;
}

#[derive(Debug, Copy, Clone)]
pub struct Point3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3d {
    pub fn homogenous(&self) -> [[f64; 1]; 4] {
        [
            [self.x],
            [self.y],
            [if self.z == 0.0 { 0.0001 } else { self.z }],
            [1.0],
        ]
    }
}

impl Sub for &Point3d {
    type Output = Point3d;
    fn sub(self, other: &Point3d) -> Self::Output {
        Point3d {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Vector for Point3d {
    fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
    fn normalize(&self) -> Self {
        let mag = self.magnitude();
        Point3d {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }
    fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    fn cross(&self, other: Self) -> Self {
        Point3d {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl From<(f64, f64, f64)> for Point3d {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Triangle3d(pub Point3d, pub Point3d, pub Point3d);

impl From<[(f64, f64, f64); 3]> for Triangle3d {
    fn from([a, b, c]: [(f64, f64, f64); 3]) -> Self {
        Self(a.into(), b.into(), c.into())
    }
}
