use crate::vector::Vector;

pub struct Triangle(pub Vertex, pub Vertex, pub Vertex);

#[derive(Debug, Clone)]
pub enum Texture {
    None,
    Solid(u8, u8, u8, u8),
    Triangles(Box<[(u8, u8, u8, u8)]>),
}

#[derive(Debug, Clone)]
pub struct Mesh<'a> {
    pub vertices: Box<[Vertex]>,
    pub indices: Box<[(usize, usize, usize)]>,
    pub texture: &'a Texture,
}

impl<'a> Mesh<'a> {
    pub fn new(vertices: impl Into<Box<[Vertex]>>, indices: &[(usize, usize, usize)]) -> Self {
        Self {
            vertices: vertices.into(),
            indices: indices.into(),
            texture: &Texture::None,
        }
    }
    pub fn textured(
        vertices: impl Into<Box<[Vertex]>>,
        indices: &[(usize, usize, usize)],
        texture: &'a Texture,
    ) -> Self {
        Self {
            vertices: vertices.into(),
            indices: indices.into(),
            texture,
        }
    }
}

impl<'a> IntoIterator for &'a Mesh<'_> {
    type Item = Triangle;

    type IntoIter = MeshIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        MeshIterator {
            vertices: &self.vertices,
            indices: &self.indices,
            indice: 0,
        }
    }
}

pub struct MeshIterator<'a> {
    vertices: &'a [Vertex],
    indices: &'a [(usize, usize, usize)],
    indice: usize,
}
impl Iterator for MeshIterator<'_> {
    type Item = Triangle;

    fn next(&mut self) -> Option<Self::Item> {
        let (a, b, c) = self.indices.get(self.indice)?;
        self.indice += 1;

        Some(Triangle(
            self.vertices[*a],
            self.vertices[*b],
            self.vertices[*c],
        ))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: Vector<3>,
    pub texture: Vector<2>,
}

impl Vertex {
    pub fn new(position: impl Into<Vector<3>>) -> Self {
        Self {
            position: position.into(),
            texture: (0.0, 0.0).into(),
        }
    }
    pub fn textured(position: impl Into<Vector<3>>, texture: impl Into<Vector<2>>) -> Self {
        Self {
            position: position.into(),
            texture: texture.into(),
        }
    }
}
