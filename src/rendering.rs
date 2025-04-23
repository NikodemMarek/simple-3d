use ndarray::arr2;
use web_sys::CanvasRenderingContext2d;

use crate::{
    Camera,
    types2d::{Point2d, Triangle2d},
    types3d::{Point3d, Triangle3d, Vector},
};

pub fn render(context: &CanvasRenderingContext2d, camera: &Camera) {
    let (width, height) = camera.screen_size;
    context.clear_rect(0.0, 0.0, width, height);

    let square = [
        [(0.0, 0.0, -1.0), (1.0, 0.0, -1.0), (0.0, 1.0, -1.0)].into(),
        [(1.0, 0.0, -1.0), (1.0, 1.0, -1.0), (0.0, 1.0, -1.0)].into(),
    ];

    draw_triangle(context, project_triangle(camera, square[0]));
    draw_triangle(context, project_triangle(camera, square[1]));
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
        [u.x, u.y, u.z, -u.dot(*position)],
        [-f.x, -f.y, -f.z, f.dot(*position)],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

fn project_triangle(camera: &Camera, Triangle3d(a, b, c): Triangle3d) -> Triangle2d {
    Triangle2d(
        pipeline(camera, a),
        pipeline(camera, b),
        pipeline(camera, c),
    )
}

fn pipeline(camera: &Camera, point: Point3d) -> Point2d {
    viewport_transformation(
        camera,
        projection_transformation(camera, camera_transformation(camera, point)),
    )
}

fn projection_transformation(camera: &Camera, point: Point3d) -> Point2d {
    let projection = arr2(&projection_matrix(camera));
    let v = projection.dot(&arr2(&point.homogenous()));
    Point2d {
        x: v[[0, 0]] / v[[3, 0]],
        y: v[[1, 0]] / v[[3, 0]],
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

fn viewport_transformation(camera: &Camera, Point2d { x, y }: Point2d) -> Point2d {
    let (width, height) = camera.screen_size;
    Point2d {
        x: (x + 1.0) * width / 2.0,
        y: (1.0 - y) * height / 2.0,
    }
}

fn draw_triangle(context: &CanvasRenderingContext2d, triangle: Triangle2d) {
    let Triangle2d(a, b, c) = triangle;

    context.set_fill_style_str("red");

    context.begin_path();
    context.move_to(a.x, a.y);
    context.line_to(b.x, b.y);
    context.line_to(c.x, c.y);
    context.close_path();
    context.fill();
}
