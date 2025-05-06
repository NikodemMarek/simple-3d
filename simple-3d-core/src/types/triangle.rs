use super::{mesh::Indice, vector::Vector};

pub struct TriangleIterator<'a, I: Iterator<Item = &'a Indice>> {
    vertices: Box<[Vector<3>]>,
    uvs: Box<[Vector<2>]>,
    indices: I,
}
impl<'a, I: Iterator<Item = &'a Indice>> Iterator for TriangleIterator<'a, I> {
    type Item = Triangle;

    fn next(&mut self) -> Option<Self::Item> {
        let (a, b, c) = self.indices.next()?;

        Some(Triangle(
            Vertex::new(self.vertices[a.0], self.uvs[a.1]),
            Vertex::new(self.vertices[b.0], self.uvs[b.1]),
            Vertex::new(self.vertices[c.0], self.uvs[c.1]),
        ))
    }
}

impl<'a, I: Iterator<Item = &'a Indice>> TriangleIterator<'a, I> {
    pub fn new(
        vertices: &[Vector<3>],
        uvs: &[Vector<2>],
        indices: impl IntoIterator<Item = &'a Indice, IntoIter = I>,
    ) -> Self {
        Self {
            vertices: vertices.into(),
            uvs: uvs.into(),
            indices: indices.into_iter(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Triangle(pub Vertex, pub Vertex, pub Vertex);

pub fn transform(
    transformation: &super::matrix::Matrix<4, 4>,
    vertexes: impl Iterator<Item = Vector<3>>,
) -> impl Iterator<Item = Vector<3>> {
    vertexes.map(|v| v.transformed(transformation))
}

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: Vector<3>,
    pub uv: Vector<2>,
}

impl Vertex {
    pub fn new(position: impl Into<Vector<3>>, texture: impl Into<Vector<2>>) -> Self {
        Self {
            position: position.into(),
            uv: texture.into(),
        }
    }
}
