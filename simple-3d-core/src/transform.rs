use std::slice::Iter;

use crate::types::{
    matrix::Matrix,
    mesh::{Mesh, TriangleIterator},
    textures::{Texture, Textures},
    vertex,
};

pub fn transform<'a>(
    textures: &'a Textures,
    objects: &'a [Mesh],
    camera_viewport_transformation: Matrix<4, 4>,
) -> impl Iterator<
    Item = (
        TriangleIterator<'a, Iter<'a, (usize, usize, usize)>>,
        &'a Texture,
    ),
> + 'a {
    objects.iter().map(move |mesh| {
        let matrix = camera_viewport_transformation.clone() * mesh.transformation_matrix().clone();

        let texture = textures.get(&mesh.texture);
        (transform_mesh(mesh, matrix), texture)
    })
}

pub fn transform_mesh<'a>(
    Mesh {
        vertices, indices, ..
    }: &'a Mesh,
    matrix: Matrix<4, 4>,
) -> TriangleIterator<'a, Iter<'a, (usize, usize, usize)>> {
    let transformed_vertices =
        vertex::transform(&matrix, vertices.iter().copied()).collect::<Vec<_>>();
    TriangleIterator::new(&transformed_vertices, indices)
}
