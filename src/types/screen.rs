use super::{matrix::Matrix, pixel::Pixel};

#[derive(Debug, Clone)]
pub struct Screen {
    width: u32,
    height: u32,

    buffer: Box<[Pixel]>,
    depth: Box<[f32]>,

    transformation: Matrix<4, 4>,
}

impl Screen {
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        Self {
            width,
            height,
            buffer: vec![Pixel::default(); size].into(),
            depth: vec![f32::MAX; size].into(),
            transformation: Self::viewport_matrix(width, height),
        }
    }

    fn viewport_matrix(width: u32, height: u32) -> Matrix<4, 4> {
        [
            [width as f64 / 2.0, 0.0, 0.0, width as f64 / 2.0],
            [0.0, -(height as f64) / 2.0, 0.0, height as f64 / 2.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]
        .into()
    }

    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
    pub fn buffer(&self) -> &[Pixel] {
        &self.buffer
    }
    pub fn transformation_matrix(&self) -> &Matrix<4, 4> {
        &self.transformation
    }

    pub fn clear_buffer(&mut self) {
        self.buffer.fill(Pixel::default());
    }
    pub fn clear_depth(&mut self) {
        self.depth.fill(f32::MAX);
    }

    pub fn put_pixel(&mut self, (x, y): (u32, u32), z: f32, pixel: Pixel) {
        if x < self.width && y < self.height {
            let index = (x + y * self.width) as usize;
            if z.is_finite() && z < self.depth[index] {
                self.buffer[index] = pixel;
                self.depth[index] = z;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{pixel::Pixel, screen::Screen};

    #[test]
    fn test_rasterize_point() {
        let mut screen = Screen::new(10, 10);

        let pixel = Pixel(255, 0, 0, 255);
        screen.put_pixel((5, 5, 0.5), pixel);
        assert_eq!(screen.buffer[5 + 5 * 10], pixel);

        screen.put_pixel((5, 5, 0.6), Pixel(0, 255, 128, 255));
        assert_eq!(screen.buffer[5 + 5 * 10], pixel);

        let pixel = Pixel(64, 255, 0, 255);
        screen.put_pixel((5, 5, 0.4), pixel);
        assert_eq!(screen.buffer[5 + 5 * 10], pixel);

        let second_pixel = Pixel(64, 255, 0, 255);
        screen.put_pixel((4, 6, 0.3), second_pixel);
        assert_eq!(screen.buffer[5 + 5 * 10], pixel);
        assert_eq!(screen.buffer[4 + 6 * 10], second_pixel);
    }
}
