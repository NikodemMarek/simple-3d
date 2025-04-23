use crate::{Pixel, types::mesh::Triangle, vector::Vector};

#[derive(Debug, Clone)]
pub struct Screen {
    pub width: u32,
    pub height: u32,

    pub buffer: Box<[Pixel]>,
    depth: Box<[f32]>,
}
impl Screen {
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        Self {
            width,
            height,
            buffer: vec![Pixel::default(); size].into(),
            depth: vec![f32::MAX; size].into(),
        }
    }

    pub fn clear_buffer(&mut self) {
        self.buffer.fill(Pixel::default());
    }
    pub fn clear_depth(&mut self) {
        self.depth.fill(f32::MAX);
    }
}

pub fn rasterize_triangle(screen: &mut Screen, triangle: &Triangle, pixel: Pixel) {
    for point in triangle_points(triangle) {
        rasterize_point(screen, point, pixel);
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

fn rasterize_point(
    Screen {
        width,
        height,
        buffer,
        depth,
    }: &mut Screen,
    (x, y, z): (u32, u32, f32),
    pixel: Pixel,
) {
    if x < *width && y < *height {
        let index = (x + y * *width) as usize;
        if z.is_finite() && z < depth[index] {
            buffer[index] = pixel;
            depth[index] = z;
        }
    }
}

#[cfg(test)]
mod test {
    use test::Bencher;

    use crate::{
        Pixel,
        rasterizing::Screen,
        types::{mesh::Triangle, vertex::Vertex},
    };

    use super::{rasterize_point, rasterize_triangle};

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

    #[test]
    fn test_rasterize_point() {
        let mut screen = Screen::new(10, 10);

        let pixel = Pixel(255, 0, 0, 255);
        rasterize_point(&mut screen, (5, 5, 0.5), pixel);
        assert_eq!(screen.buffer[5 + 5 * 10], pixel);

        rasterize_point(&mut screen, (5, 5, 0.6), Pixel(0, 255, 128, 255));
        assert_eq!(screen.buffer[5 + 5 * 10], pixel);

        let pixel = Pixel(64, 255, 0, 255);
        rasterize_point(&mut screen, (5, 5, 0.4), pixel);
        assert_eq!(screen.buffer[5 + 5 * 10], pixel);

        let second_pixel = Pixel(64, 255, 0, 255);
        rasterize_point(&mut screen, (4, 6, 0.3), second_pixel);
        assert_eq!(screen.buffer[5 + 5 * 10], pixel);
        assert_eq!(screen.buffer[4 + 6 * 10], second_pixel);
    }
}
