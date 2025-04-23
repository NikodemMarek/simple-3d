use ndarray::arr2;
use web_sys::CanvasRenderingContext2d;

use crate::{
    Camera,
    types3d::{Point3d, Triangle3d, Vector},
};

pub fn render(context: &CanvasRenderingContext2d, camera: &Camera) {
    let (width, height) = camera.screen_size;
    context.clear_rect(0.0, 0.0, width, height);

    let cube = [
        // Front face (z = -1.0)
        (
            [(0.0, 0.0, -1.0), (1.0, 0.0, -1.0), (0.0, 1.0, -1.0)].into(),
            "red",
        ),
        (
            [(1.0, 0.0, -1.0), (1.0, 1.0, -1.0), (0.0, 1.0, -1.0)].into(),
            "red",
        ),
        // Back face (z = 0.0)
        (
            [(1.0, 0.0, 0.0), (0.0, 0.0, 0.0), (0.0, 1.0, 0.0)].into(),
            "green",
        ),
        (
            [(1.0, 0.0, 0.0), (0.0, 1.0, 0.0), (1.0, 1.0, 0.0)].into(),
            "green",
        ),
        // Left face (x = 0.0)
        (
            [(0.0, 0.0, 0.0), (0.0, 0.0, -1.0), (0.0, 1.0, -1.0)].into(),
            "blue",
        ),
        (
            [(0.0, 0.0, 0.0), (0.0, 1.0, -1.0), (0.0, 1.0, 0.0)].into(),
            "blue",
        ),
        // Right face (x = 1.0)
        (
            [(1.0, 0.0, -1.0), (1.0, 0.0, 0.0), (1.0, 1.0, 0.0)].into(),
            "yellow",
        ),
        (
            [(1.0, 0.0, -1.0), (1.0, 1.0, 0.0), (1.0, 1.0, -1.0)].into(),
            "yellow",
        ),
        // Top face (y = 1.0)
        (
            [(0.0, 1.0, -1.0), (1.0, 1.0, -1.0), (1.0, 1.0, 0.0)].into(),
            "pink",
        ),
        (
            [(0.0, 1.0, -1.0), (1.0, 1.0, 0.0), (0.0, 1.0, 0.0)].into(),
            "pink",
        ),
        // Bottom face (y = 0.0)
        (
            [(0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (1.0, 0.0, -1.0)].into(),
            "lightblue",
        ),
        (
            [(0.0, 0.0, 0.0), (1.0, 0.0, -1.0), (0.0, 0.0, -1.0)].into(),
            "lightblue",
        ),
    ];

    let mut to_draw = cube
        .iter()
        .map(|(triangle, color)| (project_triangle(camera, *triangle), color))
        .collect::<Vec<_>>();

    to_draw.sort_by(|(a, _), (b, _)| {
        let a_z = (a.0.z + a.1.z + a.2.z) / 3.0;
        let b_z = (b.0.z + b.1.z + b.2.z) / 3.0;
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
) -> [[f64; 4]; 4] {
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
}

fn view_matrix(
    Camera {
        position,
        target,
        up,
        ..
    }: &Camera,
) -> [[f64; 4]; 4] {
    let f = (target - position).normalize();
    let r = f.cross(*up).normalize();
    let u = r.cross(f);
    [
        [r.x, r.y, r.z, -r.dot(*position)],
        [u.x, u.y, u.z, u.dot(*position)],
        [-f.x, -f.y, -f.z, -f.dot(*position)],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

fn project_triangle(camera: &Camera, Triangle3d(a, b, c): Triangle3d) -> Triangle3d {
    Triangle3d(
        pipeline(camera, a),
        pipeline(camera, b),
        pipeline(camera, c),
    )
}

fn pipeline(camera: &Camera, point: Point3d) -> Point3d {
    viewport_transformation(
        camera,
        projection_transformation(camera, camera_transformation(camera, point)),
    )
}

fn projection_transformation(camera: &Camera, point: Point3d) -> Point3d {
    let projection = arr2(&projection_matrix(camera));
    let v = projection.dot(&arr2(&point.homogenous()));
    Point3d {
        x: v[[0, 0]] / v[[3, 0]],
        y: v[[1, 0]] / v[[3, 0]],
        z: v[[2, 0]] / v[[3, 0]],
    }
}

fn camera_transformation(camera: &Camera, point: Point3d) -> Point3d {
    let view = arr2(&view_matrix(camera));
    let v = view.dot(&arr2(&point.homogenous()));
    Point3d {
        x: v[[0, 0]],
        y: v[[1, 0]],
        z: v[[2, 0]],
    }
}

fn viewport_transformation(camera: &Camera, Point3d { x, y, z }: Point3d) -> Point3d {
    let (width, height) = camera.screen_size;
    Point3d {
        x: (x + 1.0) * width / 2.0,
        y: (1.0 - y) * height / 2.0,
        z,
    }
}

fn draw_triangle(context: &CanvasRenderingContext2d, triangle: Triangle3d, color: &str) {
    let Triangle3d(a, b, c) = triangle;

    context.set_fill_style_str(color);

    context.begin_path();
    context.move_to(a.x, a.y);
    context.line_to(b.x, b.y);
    context.line_to(c.x, c.y);
    context.close_path();
    context.fill();
}
