use crate::{
    Pixel,
    types::{mesh::Triangle, screen::Screen},
    vector::Vector,
};

pub fn rasterize_triangle(screen: &mut Screen, triangle: &Triangle, pixel: Pixel) {
    for point in triangle_points(triangle) {
        screen.put_pixel(point, pixel);
    }
}

pub fn triangle_points<'a>(
    triangle @ Triangle(a, b, c): &'a Triangle,
) -> Box<dyn Iterator<Item = (u32, u32, f32)> + 'a> {
    #[inline]
    fn det(a: Vector<3>, b: Vector<3>, c: Vector<3>) -> f64 {
        a[0] * (b[1] - c[1]) + b[0] * (c[1] - a[1]) + c[0] * (a[1] - b[1])
    }
    #[inline]
    fn bounds(Triangle(a, b, c): &Triangle) -> (u32, u32, u32, u32) {
        let l = a.position[0].min(b.position[0]).min(c.position[0]) as u32;
        let r = a.position[0].max(b.position[0]).max(c.position[0]) as u32;
        let t = a.position[1].min(b.position[1]).min(c.position[1]) as u32;
        let b = a.position[1].max(b.position[1]).max(c.position[1]) as u32;
        (l, t, r, b)
    }

    let det_abc = det(a.position, b.position, c.position);
    if det_abc <= 0.0 {
        return Box::new(std::iter::empty()); // Cull backfaces
    }

    let (lb, tb, rb, bb) = bounds(triangle);
    let points = (lb..=rb).flat_map(move |x| (tb..=bb).map(move |y| (x, y)));
    Box::new(points.filter_map(move |(x, y)| {
        let p = (x as f64, y as f64, 0.0).into();
        let det_abp = det(a.position, b.position, p);
        let det_bcp = det(b.position, c.position, p);
        let det_cap = det(c.position, a.position, p);

        if det_abp >= 0.0 && det_bcp >= 0.0 && det_cap >= 0.0 {
            let alpha = det_bcp / det_abc;
            let beta = det_cap / det_abc;
            let gamma = det_abp / det_abc;

            let z_a = 1.0 / a.position[2];
            let z_b = 1.0 / b.position[2];
            let z_c = 1.0 / c.position[2];

            let inv_z = alpha * z_a + beta * z_b + gamma * z_c;
            let z = 1.0 / inv_z;

            Some((x, y, z as f32))
        } else {
            None
        }
    }))
}

#[cfg(test)]
mod test {
    use test::Bencher;

    use crate::{
        Pixel,
        rasterizing::Screen,
        types::{mesh::Triangle, vertex::Vertex},
    };

    use super::rasterize_triangle;

    #[bench]
    fn test_rasterize_triangle(b: &mut Bencher) {
        let mut screen = Screen::new(10, 10);

        let triangle = Triangle(
            Vertex::textured((0.0, 0.0, 1.0), (0.0, 0.0)),
            Vertex::textured((5.0, 5.0, 1.0), (0.0, 0.0)),
            Vertex::textured((10.0, 0.0, 1.0), (0.0, 0.0)),
        );
        let pixel = Pixel(255, 0, 0, 255);

        b.iter(|| {
            rasterize_triangle(&mut screen, &triangle, pixel);
        });
    }
}
