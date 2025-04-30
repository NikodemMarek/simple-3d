#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pixel(pub u8, pub u8, pub u8, pub u8);

impl Default for Pixel {
    fn default() -> Self {
        Pixel(255, 255, 255, 255)
    }
}
