#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pixel(pub u8, pub u8, pub u8, pub u8);

impl Pixel {
    pub fn brightness(&self) -> f32 {
        let Pixel(r, g, b, _) = *self;
        (r as f32 + g as f32 + b as f32) / 3.0
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Pixel(255, 255, 255, 255)
    }
}

impl From<image::Rgba<u8>> for Pixel {
    fn from(value: image::Rgba<u8>) -> Self {
        Pixel(value.0[0], value.0[1], value.0[2], value.0[3])
    }
}
