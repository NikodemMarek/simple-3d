use web_sys::CanvasRenderingContext2d;

use crate::{Camera, matrix::Matrix, shapes::Triangle, vectors::Vector};

pub fn render(context: &CanvasRenderingContext2d, camera: &Camera) {
    let (width, height) = camera.screen_size;
    context.clear_rect(0.0, 0.0, width, height);

    let cube = [
        // Front face (z = -1.0)
        (
            (
                (0.0, 0.0, -1.0).into(),
                (1.0, 0.0, -1.0).into(),
                (0.0, 1.0, -1.0).into(),
            )
                .into(),
            "red",
        ),
        (
            (
                (1.0, 0.0, -1.0).into(),
                (1.0, 1.0, -1.0).into(),
                (0.0, 1.0, -1.0).into(),
            )
                .into(),
            "red",
        ),
        // Back face (z = 0.0)
        (
            (
                (1.0, 0.0, 0.0).into(),
                (0.0, 0.0, 0.0).into(),
                (0.0, 1.0, 0.0).into(),
            )
                .into(),
            "green",
        ),
        (
            (
                (1.0, 0.0, 0.0).into(),
                (0.0, 1.0, 0.0).into(),
                (1.0, 1.0, 0.0).into(),
            )
                .into(),
            "green",
        ),
        // Left face (x = 0.0)
        (
            (
                (0.0, 0.0, 0.0).into(),
                (0.0, 0.0, -1.0).into(),
                (0.0, 1.0, -1.0).into(),
            )
                .into(),
            "blue",
        ),
        (
            (
                (0.0, 0.0, 0.0).into(),
                (0.0, 1.0, -1.0).into(),
                (0.0, 1.0, 0.0).into(),
            )
                .into(),
            "blue",
        ),
        // Right face (x = 1.0)
        (
            (
                (1.0, 0.0, -1.0).into(),
                (1.0, 0.0, 0.0).into(),
                (1.0, 1.0, 0.0).into(),
            )
                .into(),
            "yellow",
        ),
        (
            (
                (1.0, 0.0, -1.0).into(),
                (1.0, 1.0, 0.0).into(),
                (1.0, 1.0, -1.0).into(),
            )
                .into(),
            "yellow",
        ),
        // Top face (y = 1.0)
        (
            (
                (0.0, 1.0, -1.0).into(),
                (1.0, 1.0, -1.0).into(),
                (1.0, 1.0, 0.0).into(),
            )
                .into(),
            "pink",
        ),
        (
            (
                (0.0, 1.0, -1.0).into(),
                (1.0, 1.0, 0.0).into(),
                (0.0, 1.0, 0.0).into(),
            )
                .into(),
            "pink",
        ),
        // Bottom face (y = 0.0)
        (
            (
                (0.0, 0.0, 0.0).into(),
                (1.0, 0.0, 0.0).into(),
                (1.0, 0.0, -1.0).into(),
            )
                .into(),
            "lightblue",
        ),
        (
            (
                (0.0, 0.0, 0.0).into(),
                (1.0, 0.0, -1.0).into(),
                (0.0, 0.0, -1.0).into(),
            )
                .into(),
            "lightblue",
        ),
    ];

    let mut to_draw = cube
        .iter()
        .map(|(triangle, color)| (project_triangle(camera, *triangle), color))
        .collect::<Vec<_>>();

    to_draw.sort_by(|(a, _), (b, _)| {
        let a_z = (a.0[2] + a.1[2] + a.2[2]) / 3.0;
        let b_z = (b.0[2] + b.1[2] + b.2[2]) / 3.0;
        a_z.partial_cmp(&b_z).unwrap()
    });

    for (triangle, color) in to_draw {
        draw_triangle(context, triangle, color);
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

fn project_triangle(camera: &Camera, Triangle(a, b, c): Triangle<3>) -> Triangle<3> {
    Triangle(
        pipeline(camera, a),
        pipeline(camera, b),
        pipeline(camera, c),
    )
}

fn pipeline(camera: &Camera, point: Vector<3>) -> Vector<3> {
    viewport_transformation(
        camera,
        projection_transformation(camera, camera_transformation(camera, point)),
    )
}

fn projection_transformation(camera: &Camera, point: Vector<3>) -> Vector<3> {
    let projection = projection_matrix(camera);
    let v = projection.dot(&point.homogenous());
    (v[0] / v[3], v[1] / v[3], v[2] / v[3]).into()
}

fn camera_transformation(camera: &Camera, point: Vector<3>) -> Vector<3> {
    let view = view_matrix(camera);
    let v = view.dot(&point.homogenous());
    (v[0], v[1], v[2]).into()
}

fn viewport_transformation(camera: &Camera, vec: Vector<3>) -> Vector<3> {
    let [x, y, z] = *vec;
    let (width, height) = camera.screen_size;
    ((x + 1.0) * width / 2.0, (1.0 - y) * height / 2.0, z).into()
}

fn draw_triangle(context: &CanvasRenderingContext2d, Triangle(a, b, c): Triangle<3>, color: &str) {
    let [a_x, a_y, _] = *a;
    let [b_x, b_y, _] = *b;
    let [c_x, c_y, _] = *c;

    context.set_fill_style_str(color);

    context.begin_path();
    context.move_to(a_x, a_y);
    context.line_to(b_x, b_y);
    context.line_to(c_x, c_y);
    context.close_path();
    context.fill();
}
