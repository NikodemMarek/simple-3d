use crate::{Pixel, shapes::Triangle, vector::Vector};

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

pub fn rasterize_triangle(screen: &mut Screen, Triangle(a, b, c): Triangle, pixel: Pixel) {
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
        return; // Cull backfaces
    }

    let (lb, tb, rb, bb) = bounds(&Triangle(a, b, c));
    for x in lb..=rb {
        for y in tb..=bb {
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

                rasterize_point(screen, (x, y, z as f32), pixel);
            }
        }
    }
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
