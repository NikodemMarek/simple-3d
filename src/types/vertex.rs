use super::{matrix::Matrix, vector::Vector};

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: Vector<3>,
    pub texture: Vector<2>,
}

impl Vertex {
    pub fn new(position: impl Into<Vector<3>>) -> Self {
        Self::textured(position, (0.0, 0.0))
    }
    pub fn textured(position: impl Into<Vector<3>>, texture: impl Into<Vector<2>>) -> Self {
        Self {
            position: position.into(),
            texture: texture.into(),
        }
    }

    pub fn transformed(&self, transformation: &Matrix<4, 4>) -> Self {
        let v = transformation.dot(&self.position.homogenous());
        Self {
            position: (v[0] / v[3], v[1] / v[3], v[2] / v[3]).into(),
            texture: self.texture,
        }
    }
}

pub fn transform(
    transformation: &Matrix<4, 4>,
    vertexes: impl Iterator<Item = Vertex>,
) -> impl Iterator<Item = Vertex> {
    vertexes.map(|v| v.transformed(transformation))
}
