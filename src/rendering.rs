use crate::{
    Pixel, Scene,
    textures::Texture,
    types::{
        matrix::Matrix,
        mesh::{Mesh, TriangleIterator},
        vertex,
    },
};

pub fn render(scene: &mut Scene) {
    scene.screen.clear_depth();

    for mesh @ Mesh {
        vertices,
        indices,
        texture,
        ..
    } in scene.objects.iter()
    {
        let matrix = transformation_matrix(scene, mesh);

        let transformed_vertices: Vec<_> =
            vertex::transform(&matrix, vertices.iter().copied()).collect();
        let mut indices = indices.into_iter();
        let triangles = TriangleIterator::new(&transformed_vertices, &mut indices);

        for (i, triangle) in triangles.enumerate() {
            let pixel = match scene.textures.get(texture.as_ref()).unwrap() {
                Texture::None => Pixel::default(),
                Texture::Solid(r, g, b, a) => Pixel(*r, *g, *b, *a),
                Texture::Triangles(texture) => {
                    let (r, g, b, a) = texture[i % texture.len()];
                    Pixel(r, g, b, a)
                }
            };
            crate::rasterizing::rasterize_triangle(&mut scene.screen, &triangle, pixel);
        }
    }
}

fn transformation_matrix(scene: &Scene, mesh: &Mesh) -> Matrix<4, 4> {
    scene.screen.transformation_matrix().clone()
        * scene.camera.transformation_matrix().clone()
        * mesh.transformation_matrix().clone()
}
