#[derive(Debug, Clone)]
pub struct Point2d {
    pub x: f64,
    pub y: f64,
}

impl From<(f64, f64)> for Point2d {
    fn from((x, y): (f64, f64)) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone)]
pub struct Triangle2d(pub Point2d, pub Point2d, pub Point2d);

impl From<[(f64, f64); 3]> for Triangle2d {
    fn from([a, b, c]: [(f64, f64); 3]) -> Self {
        Self(a.into(), b.into(), c.into())
    }
}
