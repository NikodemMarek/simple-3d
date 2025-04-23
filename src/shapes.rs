use crate::{matrix::Matrix, vector::Vector};

#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Box<[Vertex]>,
    pub indices: Box<[(usize, usize, usize)]>,
    pub texture: Box<str>,
}

impl Mesh {
    pub fn new(vertices: impl Into<Box<[Vertex]>>, indices: &[(usize, usize, usize)]) -> Self {
        Self {
            vertices: vertices.into(),
            indices: indices.into(),
            texture: "none".into(),
        }
    }
    pub fn textured(
        vertices: impl Into<Box<[Vertex]>>,
        indices: &[(usize, usize, usize)],
        texture: Box<str>,
    ) -> Self {
        Self {
            vertices: vertices.into(),
            indices: indices.into(),
            texture,
        }
    }

    pub fn transformed(&self, transformation: &Matrix<4, 4>) -> Self {
        let vertices = self
            .vertices
            .iter()
            .map(|v| v.transformed(transformation))
            .collect::<Vec<_>>()
            .into();
        Self {
            vertices,
            indices: self.indices.clone(),
            texture: self.texture.clone(),
        }
    }

    pub fn transform(&mut self, transformation: &Matrix<4, 4>) {
        self.vertices = self
            .vertices
            .iter()
            .map(|v| v.transformed(transformation))
            .collect::<Vec<_>>()
            .into();
    }
}

impl<'a> IntoIterator for &'a Mesh {
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

pub struct Triangle(pub Vertex, pub Vertex, pub Vertex);

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

    pub fn transformed(&self, transformation: &Matrix<4, 4>) -> Self {
        let position = self.position.homogenous();
        let transformed = transformation.dot(&position);
        Self {
            position: (
                transformed[0] / transformed[3],
                transformed[1] / transformed[3],
                transformed[2] / transformed[3],
            )
                .into(),
            texture: self.texture,
        }
    }
}

pub fn cube() -> Mesh {
    Mesh::new(
        [
            Vertex::textured((1.0, 1.0, 1.0), (0.0, 0.0)), // 0 - Front top right
            Vertex::textured((1.0, -1.0, 1.0), (0.0, 0.0)), // 1 - Front bottom right
            Vertex::textured((-1.0, -1.0, 1.0), (0.0, 0.0)), // 2 - Front bottom left
            Vertex::textured((-1.0, 1.0, 1.0), (0.0, 0.0)), // 3 - Front top left
            Vertex::textured((1.0, 1.0, -1.0), (1.0, 0.0)), // 4 - Back top right
            Vertex::textured((1.0, -1.0, -1.0), (1.0, 0.0)), // 5 - Back bottom right
            Vertex::textured((-1.0, -1.0, -1.0), (1.0, 0.0)), // 6 - Back bottom left
            Vertex::textured((-1.0, 1.0, -1.0), (1.0, 0.0)), // 7 - Back top left
        ],
        &[
            // Front face (z = +1)
            (0, 2, 1),
            (0, 3, 2),
            // Back face (z = -1)
            (4, 5, 6),
            (4, 6, 7),
            // Right face (x = +1)
            (0, 4, 5),
            (0, 5, 1),
            // Left face (x = -1)
            (2, 6, 7),
            (2, 7, 3),
            // Top face (y = +1)
            (0, 7, 4),
            (0, 3, 7),
            // Bottom face (y = -1)
            (1, 5, 6),
            (1, 6, 2),
        ],
    )
}
