use crate::transformations;

use super::{matrix::Matrix, vector::Vector};

pub type Indice = ((usize, usize), (usize, usize), (usize, usize));

#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Box<[Vector<3>]>,
    pub uvs: Box<[Vector<2>]>,
    pub indices: Box<[Indice]>,
    pub texture: Box<str>,

    scale: Vector<3>,
    rotation: Vector<3>,
    translation: Vector<3>,
    transformation: Matrix<4, 4>,
}

impl Mesh {
    pub fn new(
        vertices: impl Into<Box<[Vector<3>]>>,
        uvs: impl Into<Box<[Vector<2>]>>,
        indices: &[Indice],
        texture: Box<str>,
    ) -> Self {
        Self {
            vertices: vertices.into(),
            uvs: uvs.into(),
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
