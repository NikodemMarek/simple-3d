use crate::{matrix::Matrix, vector::Vector};

pub fn rotation(axis: &Vector<3>, angle: f64) -> Matrix<4, 4> {
    let c = f64::cos(angle);
    let s = f64::sin(angle);
    let t = 1.0 - c;
    let x = axis[0];
    let y = axis[1];
    let z = axis[2];
    [
        [t * x * x + c, t * x * y - s * z, t * x * z + s * y, 0.0],
        [t * y * x + s * z, t * y * y + c, t * y * z - s * x, 0.0],
        [t * z * x - s * y, t * z * y + s * x, t * z * z + c, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
    .into()
}
#[inline]
pub fn rotation_x(angle: f64) -> Matrix<4, 4> {
    rotation(&(1.0, 0.0, 0.0).into(), angle)
}
#[inline]
pub fn rotation_y(angle: f64) -> Matrix<4, 4> {
    rotation(&(0.0, 1.0, 0.0).into(), angle)
}
#[inline]
pub fn rotation_z(angle: f64) -> Matrix<4, 4> {
    rotation(&(0.0, 0.0, 1.0).into(), angle)
}
pub fn translation(vector: &Vector<3>) -> Matrix<4, 4> {
    [
        [1.0, 0.0, 0.0, vector[0]],
        [0.0, 1.0, 0.0, vector[1]],
        [0.0, 0.0, 1.0, vector[2]],
        [0.0, 0.0, 0.0, 1.0],
    ]
    .into()
}
pub fn scaling(vector: &Vector<3>) -> Matrix<4, 4> {
    [
        [vector[0], 0.0, 0.0, 0.0],
        [0.0, vector[1], 0.0, 0.0],
        [0.0, 0.0, vector[2], 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
    .into()
}
