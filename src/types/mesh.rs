use crate::r#static::transformations;

use super::{matrix::Matrix, vector::Vector, vertex::Vertex};

#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Box<[Vertex]>,
    pub indices: Box<[(usize, usize, usize)]>,
    pub texture: Box<str>,

    scale: Vector<3>,
    rotation: Vector<3>,
    translation: Vector<3>,
    transformation: Matrix<4, 4>,
}

impl Mesh {
    pub fn new(vertices: impl Into<Box<[Vertex]>>, indices: &[(usize, usize, usize)]) -> Self {
        Self::textured(vertices, indices, "none".into())
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

            scale: (1.0, 1.0, 1.0).into(),
            rotation: (0.0, 0.0, 0.0).into(),
            translation: (0.0, 0.0, 0.0).into(),
            transformation: Matrix::identity(),
        }
    }

    pub fn scale(&mut self, scale: impl Into<Vector<3>>) {
        let [s_x, s_y, s_z] = *scale.into();
        let [x, y, z] = *self.scale;
        self.scale = (s_x * x, s_y * y, s_z * z).into();
        self.update_transformation();
    }
    pub fn rotate(&mut self, rotation: impl Into<Vector<3>>) {
        self.rotation += rotation.into();
        self.update_transformation();
    }
    pub fn translate(&mut self, translation: impl Into<Vector<3>>) {
        self.translation += translation.into();
        self.update_transformation();
    }

    pub fn transformation_matrix(&self) -> &Matrix<4, 4> {
        &self.transformation
    }

    #[inline]
    fn update_transformation(&mut self) {
        self.transformation =
            transformations::transformation(self.scale, self.rotation, self.translation);
    }
}

pub struct TriangleIterator<'a, I: Iterator<Item = &'a (usize, usize, usize)>> {
    vertices: Box<[Vertex]>,
    indices: I,
}
impl<'a, I: Iterator<Item = &'a (usize, usize, usize)>> Iterator for TriangleIterator<'a, I> {
    type Item = Triangle;

    fn next(&mut self) -> Option<Self::Item> {
        let (a, b, c) = self.indices.next()?;

        Some(Triangle(
            self.vertices[*a],
            self.vertices[*b],
            self.vertices[*c],
        ))
    }
}

impl<'a, I: Iterator<Item = &'a (usize, usize, usize)>> TriangleIterator<'a, I> {
    pub fn new(
        vertices: &[Vertex],
        indices: impl IntoIterator<Item = &'a (usize, usize, usize), IntoIter = I>,
    ) -> Self {
        Self {
            vertices: vertices.into(),
            indices: indices.into_iter(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Triangle(pub Vertex, pub Vertex, pub Vertex);
