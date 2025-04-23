use crate::vectors::Vector;

#[derive(Debug, Copy, Clone)]
pub struct Triangle<const S: usize>(pub Vector<S>, pub Vector<S>, pub Vector<S>);

impl<const S: usize, V: Into<Vector<S>>> From<(V, V, V)> for Triangle<S> {
    fn from((a, b, c): (V, V, V)) -> Self {
        Self(a.into(), b.into(), c.into())
    }
}
