use crate::{Camera, Pixel, matrix::Matrix, rasterizing::Screen, shapes::Triangle, vector::Vector};

pub fn render(camera: &mut Camera) {
    let cube: [(Triangle<3>, &str); 12] = [
        // Front face (z = -1.0)
        (
            ((0.0, 0.0, -1.0), (1.0, 0.0, -1.0), (0.0, 1.0, -1.0)).into(),
            "red",
        ),
        (
            ((1.0, 0.0, -1.0), (1.0, 1.0, -1.0), (0.0, 1.0, -1.0)).into(),
            "red",
        ),
        // Back face (z = 0.0)
        (
            ((1.0, 0.0, 0.0), (0.0, 0.0, 0.0), (0.0, 1.0, 0.0)).into(),
            "green",
        ),
        (
            ((1.0, 0.0, 0.0), (0.0, 1.0, 0.0), (1.0, 1.0, 0.0)).into(),
            "green",
        ),
        // Left face (x = 0.0)
        (
            ((0.0, 0.0, 0.0), (0.0, 0.0, -1.0), (0.0, 1.0, -1.0)).into(),
            "blue",
        ),
        (
            ((0.0, 0.0, 0.0), (0.0, 1.0, -1.0), (0.0, 1.0, 0.0)).into(),
            "blue",
        ),
        // Right face (x = 1.0)
        (
            ((1.0, 0.0, -1.0), (1.0, 0.0, 0.0), (1.0, 1.0, 0.0)).into(),
            "yellow",
        ),
        (
            ((1.0, 0.0, -1.0), (1.0, 1.0, 0.0), (1.0, 1.0, -1.0)).into(),
            "yellow",
        ),
        // Top face (y = 1.0)
        (
            ((0.0, 1.0, -1.0), (1.0, 1.0, -1.0), (1.0, 1.0, 0.0)).into(),
            "cyan",
        ),
        (
            ((0.0, 1.0, -1.0), (1.0, 1.0, 0.0), (0.0, 1.0, 0.0)).into(),
            "cyan",
        ),
        // Bottom face (y = 0.0)
        (
            ((0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (1.0, 0.0, -1.0)).into(),
            "pink",
        ),
        (
            ((0.0, 0.0, 0.0), (1.0, 0.0, -1.0), (0.0, 0.0, -1.0)).into(),
            "pink",
        ),
    ];

    let mut to_draw = cube
        .iter()
        .map(|(triangle, color)| (transform_triangle(camera, *triangle), color))
        .collect::<Vec<_>>();

    to_draw.sort_by(|a, b| {
        let a_z = (a.0.0[2] + a.0.1[2] + a.0.2[2]) / 3.0;
        let b_z = (b.0.0[2] + b.0.1[2] + b.0.2[2]) / 3.0;
        a_z.partial_cmp(&b_z).unwrap()
    });

    for (triangle, color) in to_draw {
        let pixel = match *color {
            "red" => Pixel(255, 0, 0, 255),
            "green" => Pixel(0, 255, 0, 255),
            "blue" => Pixel(0, 0, 255, 255),
            "yellow" => Pixel(255, 255, 0, 255),
            "cyan" => Pixel(0, 255, 255, 255),
            "pink" => Pixel(255, 192, 203, 255),
            _ => Pixel(255, 255, 255, 255),
        };
        let Triangle(a, b, c) = triangle;
        let triangle = Triangle(
            (a[0], a[1]).into(),
            (b[0], b[1]).into(),
            (c[0], c[1]).into(),
        );
        crate::rasterizing::render_triangle(&mut camera.screen, triangle, pixel);
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
    let r = f.cross(*up).normalize();
    let u = r.cross(f);
    [
        [r[0], r[1], r[2], -r.dot(*position)],
        [u[0], u[1], u[2], u.dot(*position)],
        [-f[0], -f[1], -f[2], -f.dot(*position)],
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

fn transform_triangle(camera: &Camera, Triangle(a, b, c): Triangle<3>) -> Triangle<3> {
    let transformation =
        viewport_matrix(&camera.screen) * projection_matrix(camera) * view_matrix(camera);

    (
        pipeline(&transformation, a),
        pipeline(&transformation, b),
        pipeline(&transformation, c),
    )
        .into()
}

fn pipeline(vp: &Matrix<4, 4>, point: Vector<3>) -> Vector<3> {
    let v = vp.dot(&point.homogenous());
    (v[0] / v[3], v[1] / v[3], v[2] / v[3]).into()
}
