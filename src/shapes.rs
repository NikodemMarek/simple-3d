use crate::vectors::Vector;

#[derive(Debug, Copy, Clone)]
pub struct Triangle<const S: usize>(pub Vector<S>, pub Vector<S>, pub Vector<S>);

impl<const S: usize> From<(Vector<S>, Vector<S>, Vector<S>)> for Triangle<S> {
    fn from((a, b, c): (Vector<S>, Vector<S>, Vector<S>)) -> Self {
        Self(a, b, c)
    }
}
