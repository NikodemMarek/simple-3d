use crate::{Pixel, shapes::Triangle, vector::Vector};

#[derive(Debug, Clone)]
pub struct Screen {
    pub width: u32,
    pub height: u32,

    pub buffer: Box<[Pixel]>,
}
impl Screen {
    pub fn clear_buffer(&mut self) {
        self.buffer.fill(Pixel::default());
    }
}

pub fn rasterize_triangle(screen: &mut Screen, Triangle(a, b, c): Triangle<2>, pixel: Pixel) {
    #[inline]
    fn det(a: Vector<2>, b: Vector<2>, c: Vector<2>) -> f64 {
        a[0] * (b[1] - c[1]) + b[0] * (c[1] - a[1]) + c[0] * (a[1] - b[1])
    }
    #[inline]
    fn bounds(Triangle(a, b, c): &Triangle<2>) -> (u32, u32, u32, u32) {
        let l = a[0].min(b[0]).min(c[0]) as u32;
        let r = a[0].max(b[0]).max(c[0]) as u32;
        let t = a[1].min(b[1]).min(c[1]) as u32;
        let b = a[1].max(b[1]).max(c[1]) as u32;
        (l, t, r, b)
    }

    let (lb, tb, rb, bb) = bounds(&Triangle(a, b, c));
    for x in lb..rb {
        for y in tb..bb {
            let p = (x as f64, y as f64).into();
            if det(a, b, p) >= 0.0 && det(b, c, p) >= 0.0 && det(c, a, p) >= 0.0 {
                rasterize_point(screen, (x, y), pixel);
            }
        }
    }
}

fn rasterize_point(Screen { width, buffer, .. }: &mut Screen, (x, y): (u32, u32), pixel: Pixel) {
    let index = (x + y * *width) as usize;
    if index < buffer.len() {
        buffer[index] = pixel;
    }
}
