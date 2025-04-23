use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Texture {
    None,
    Solid(u8, u8, u8, u8),
    Triangles(Box<[(u8, u8, u8, u8)]>),
}

pub fn init() -> HashMap<Box<str>, Texture> {
    let mut textures = HashMap::new();
    textures.insert("none".into(), Texture::None);
    let cube = Texture::Triangles(Box::new([
        (255, 0, 0, 255),
        (255, 0, 0, 255),
        (0, 255, 0, 255),
        (0, 255, 0, 255),
        (0, 0, 255, 255),
        (0, 0, 255, 255),
        (255, 255, 0, 255),
        (255, 255, 0, 255),
        (255, 0, 255, 255),
        (255, 0, 255, 255),
        (0, 255, 255, 255),
        (0, 255, 255, 255),
    ]));
    textures.insert("cube".into(), cube);
    textures
}
