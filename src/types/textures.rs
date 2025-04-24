use std::collections::HashMap;

use super::vector::Vector;

pub struct Textures(HashMap<Box<str>, Texture>);

impl Textures {
    pub fn get(&self, name: &str) -> &Texture {
        self.0.get(name).unwrap_or(&Texture::None)
    }

    pub fn init() -> Self {
        let mut textures = HashMap::new();
        textures.insert("none".into(), Texture::None);
        let cube = Texture::Triangles(Box::new([
            // FRONT (Red)
            (
                (((1.0, 1.0).into(), (2.0, 1.0).into(), (2.0, 2.0).into())),
                (255, 0, 0, 255),
            ),
            (
                (((1.0, 1.0).into(), (2.0, 2.0).into(), (1.0, 2.0).into())),
                (255, 0, 0, 255),
            ),
            // BACK (Green)
            (
                (((3.0, 1.0).into(), (4.0, 1.0).into(), (4.0, 2.0).into())),
                (0, 255, 0, 255),
            ),
            (
                (((3.0, 1.0).into(), (4.0, 2.0).into(), (3.0, 2.0).into())),
                (0, 255, 0, 255),
            ),
            // LEFT (Blue)
            (
                (((0.0, 1.0).into(), (1.0, 1.0).into(), (1.0, 2.0).into())),
                (0, 0, 255, 255),
            ),
            (
                (((0.0, 1.0).into(), (1.0, 2.0).into(), (0.0, 2.0).into())),
                (0, 0, 255, 255),
            ),
            // RIGHT (Yellow)
            (
                (((2.0, 1.0).into(), (3.0, 1.0).into(), (3.0, 2.0).into())),
                (255, 255, 0, 255),
            ),
            (
                (((2.0, 1.0).into(), (3.0, 2.0).into(), (2.0, 2.0).into())),
                (255, 255, 0, 255),
            ),
            // TOP (Magenta)
            (
                (((1.0, 2.0).into(), (2.0, 2.0).into(), (2.0, 3.0).into())),
                (255, 0, 255, 255),
            ),
            (
                (((1.0, 2.0).into(), (2.0, 3.0).into(), (1.0, 3.0).into())),
                (255, 0, 255, 255),
            ),
            // BOTTOM (Cyan)
            (
                (((1.0, 0.0).into(), (2.0, 0.0).into(), (2.0, 1.0).into())),
                (0, 255, 255, 255),
            ),
            (
                (((1.0, 0.0).into(), (2.0, 1.0).into(), (1.0, 1.0).into())),
                (0, 255, 255, 255),
            ),
        ]));
        textures.insert("cube".into(), cube);
        Self(textures)
    }
}

#[derive(Debug, Clone)]
pub enum Texture {
    None,
    Solid(u8, u8, u8, u8),
    Triangles(Box<[((Vector<2>, Vector<2>, Vector<2>), (u8, u8, u8, u8))]>),
}
