use crate::{
    Camera, Pixel,
    matrix::Matrix,
    rasterizing::Screen,
    shapes::{Mesh, Texture, Vertex},
    vector::Vector,
};

pub fn render(camera: &mut Camera) {
    let texture = Texture::Triangles(Box::new([
        (255, 0, 0, 255),
        (255, 0, 0, 255),
        (0, 255, 0, 255),
        (0, 255, 0, 255),
        (0, 0, 255, 255),
        (0, 0, 255, 255),
        (255, 255, 0, 255),
        (255, 255, 0, 255),
        (255, 0, 255, 255),
        (255, 0, 255, 255),
        (0, 255, 255, 255),
        (0, 255, 255, 255),
    ]));
    let cube = Mesh::textured(
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
        &texture,
    );

    camera.screen.clear_depth();

    let to_draw = transform_mesh(camera, &cube);
    let texture = to_draw.texture;
    for (i, triangle) in to_draw.into_iter().enumerate() {
        let pixel = match texture {
            Texture::None => Pixel::default(),
            Texture::Solid(r, g, b, a) => Pixel(*r, *g, *b, *a),
            Texture::Triangles(texture) => {
                let (r, g, b, a) = texture[i % texture.len()];
                Pixel(r, g, b, a)
            }
        };
        crate::rasterizing::rasterize_triangle(&mut camera.screen, triangle, pixel);
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

fn transform_mesh<'a>(
    camera: &Camera,
    Mesh {
        vertices,
        indices,
        texture,
    }: &'a Mesh,
) -> Mesh<'a> {
    let transformation =
        viewport_matrix(&camera.screen) * projection_matrix(camera) * view_matrix(camera);

    Mesh::textured(
        vertices
            .iter()
            .map(|v| Vertex::new(pipeline(&transformation, v.position)))
            .collect::<Vec<_>>(),
        indices,
        texture,
    )
}

fn pipeline(vp: &Matrix<4, 4>, point: Vector<3>) -> Vector<3> {
    let v = vp.dot(&point.homogenous());
    (v[0] / v[3], v[1] / v[3], v[2] / v[3]).into()
}
