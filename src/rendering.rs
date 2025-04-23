use crate::{
    Camera, Pixel, Scene,
    rasterizing::Screen,
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
            crate::rasterizing::rasterize_triangle(&mut scene.screen, triangle, pixel);
        }
    }
}

fn projection_matrix(
    Camera {
        fov,
        aspect_ratio,
        near,
        far,
        ..
    }: &Camera,
) -> Matrix<4, 4> {
    let f = 1.0 / f64::tan(fov / 2.0);
    [
        [f / aspect_ratio, 0.0, 0.0, 0.0],
        [0.0, f, 0.0, 0.0],
        [
            0.0,
            0.0,
            (near + far) / (near - far),
            (2.0 * near * far) / (near - far),
        ],
        [0.0, 0.0, -1.0, 0.0],
    ]
    .into()
}

fn view_matrix(
    Camera {
        position,
        target,
        up,
        ..
    }: &Camera,
) -> Matrix<4, 4> {
    let f = (*target - *position).normalize();
    let r = up.cross(f).normalize();
    let u = f.cross(r);
    let p = *position;
    [
        [r[0], r[1], r[2], -r.dot(p)],
        [u[0], u[1], u[2], -u.dot(p)],
        [-f[0], -f[1], -f[2], f.dot(p)],
        [0.0, 0.0, 0.0, 1.0],
    ]
    .into()
}

fn viewport_matrix(Screen { width, height, .. }: &Screen) -> Matrix<4, 4> {
    [
        [*width as f64 / 2.0, 0.0, 0.0, *width as f64 / 2.0],
        [0.0, -(*height as f64) / 2.0, 0.0, *height as f64 / 2.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
    .into()
}

fn transformation_matrix(scene: &Scene, mesh: &Mesh) -> Matrix<4, 4> {
    viewport_matrix(&scene.screen)
        * projection_matrix(&scene.camera)
        * view_matrix(&scene.camera)
        * mesh.transformation_matrix().clone()
}
