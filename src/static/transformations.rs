use crate::{types::matrix::Matrix, vector::Vector};

pub fn transformation<V: Into<Vector<3>>>(
    scale_factor: V,
    rotation: V,
    translation: V,
) -> Matrix<4, 4> {
    translate(translation) * rotate(rotation) * scale(scale_factor)
}

pub fn rotate<V: Into<Vector<3>>>(angles: V) -> Matrix<4, 4> {
    let [x, y, z] = *angles.into();

    let (sx, cx) = x.sin_cos();
    let (sy, cy) = y.sin_cos();
    let (sz, cz) = z.sin_cos();

    let m00 = cz * cy;
    let m01 = cz * sy * sx - sz * cx;
    let m02 = cz * sy * cx + sz * sx;

    let m10 = sz * cy;
    let m11 = sz * sy * sx + cz * cx;
    let m12 = sz * sy * cx - cz * sx;

    let m20 = -sy;
    let m21 = cy * sx;
    let m22 = cy * cx;

    [
        [m00, m01, m02, 0.0],
        [m10, m11, m12, 0.0],
        [m20, m21, m22, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
    .into()
}
#[inline]
pub fn rotate_x(angle: f64) -> Matrix<4, 4> {
    rotate((angle, 0.0, 0.0))
}
#[inline]
pub fn rotate_y(angle: f64) -> Matrix<4, 4> {
    rotate((0.0, angle, 0.0))
}
#[inline]
pub fn rotate_z(angle: f64) -> Matrix<4, 4> {
    rotate((0.0, 0.0, angle))
}

pub fn translate<V: Into<Vector<3>>>(vector: V) -> Matrix<4, 4> {
    let [x, y, z] = *vector.into();
    [
        [1.0, 0.0, 0.0, x],
        [0.0, 1.0, 0.0, y],
        [0.0, 0.0, 1.0, z],
        [0.0, 0.0, 0.0, 1.0],
    ]
    .into()
}

pub fn scale<V: Into<Vector<3>>>(vector: V) -> Matrix<4, 4> {
    let [x, y, z] = *vector.into();
    [
        [x, 0.0, 0.0, 0.0],
        [0.0, y, 0.0, 0.0],
        [0.0, 0.0, z, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    use approx::assert_relative_eq;
    use std::f64::consts::{FRAC_PI_2, PI};

    #[test]
    fn test_rotate_x() {
        assert_relative_eq!(
            rotate_x(PI),
            [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, -1.0, 0.0, 0.0],
                [0.0, 0.0, -1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
            .into(),
        );
    }

    #[test]
    fn test_rotate_y() {
        assert_relative_eq!(
            rotate_y(PI),
            [
                [-1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, -1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
            .into(),
        );
    }

    #[test]
    fn test_rotate_z() {
        assert_relative_eq!(
            rotate_z(PI),
            [
                [-1.0, 0.0, 0.0, 0.0],
                [0.0, -1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
            .into(),
        );
    }

    #[test]
    fn test_rotate_identity() {
        assert_relative_eq!(
            rotate((0.0, 0.0, 0.0)),
            [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
            .into(),
        );
    }

    #[test]
    fn test_rotate_combined_90_deg_xyz() {
        assert_relative_eq!(
            rotate((FRAC_PI_2, FRAC_PI_2, FRAC_PI_2)),
            [
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
            .into(),
        );
    }

    #[test]
    fn test_translate() {
        assert_eq!(
            translate((1.0, 2.0, 3.0)),
            [
                [1.0, 0.0, 0.0, 1.0],
                [0.0, 1.0, 0.0, 2.0],
                [0.0, 0.0, 1.0, 3.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
            .into()
        );
    }

    #[test]
    fn test_scale() {
        assert_eq!(
            scale((2.0, 3.0, 4.0)),
            [
                [2.0, 0.0, 0.0, 0.0],
                [0.0, 3.0, 0.0, 0.0],
                [0.0, 0.0, 4.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
            .into()
        );
    }

    #[test]
    fn test_transformation() {
        let scale_factor = (1.0, 2.0, 3.0);
        let rotation = (PI, 0.0, PI);
        let translation = (4.0, 5.0, 6.0);
        assert_relative_eq!(
            transformation(scale_factor, rotation, translation),
            translate(translation) * rotate(rotation) * scale(scale_factor),
        );
    }
}
