use std::collections::HashMap;

use super::pixel::Pixel;

pub struct Textures(HashMap<Box<str>, Texture>);

impl Textures {
    pub fn get(&self, name: &str) -> &Texture {
        self.0.get(name).unwrap_or(&Texture::None)
    }

    pub fn new(images: HashMap<Box<str>, Image>) -> Self {
        let mut textures = images
            .into_iter()
            .map(|(name, image)| (name, Texture::Image { image }))
            .collect::<HashMap<Box<str>, Texture>>();
        textures.insert("none".into(), Texture::None);
        textures.insert("solid_red".into(), Texture::Solid(255, 0, 0, 255));

        Self(textures)
    }

    pub fn add(&mut self, name: &str, texture: Texture) {
        self.0.insert(name.into(), texture);
    }
}

#[derive(Debug)]
pub enum Texture {
    None,
    Solid(u8, u8, u8, u8),
    Image { image: Image },
}

impl Texture {
    pub fn get(&self, x: u32, y: u32) -> Pixel {
        match self {
            Texture::None => Pixel::default(),
            Texture::Solid(r, g, b, a) => Pixel(*r, *g, *b, *a),
            Texture::Image { image } => image.get(x, y),
        }
    }

    pub fn width(&self) -> u32 {
        match self {
            Texture::None => 1,
            Texture::Solid(_, _, _, _) => 1,
            Texture::Image { image } => image.width,
        }
    }
    pub fn height(&self) -> u32 {
        match self {
            Texture::None => 1,
            Texture::Solid(_, _, _, _) => 1,
            Texture::Image { image } => image.height,
        }
    }
}

#[derive(Debug)]
pub struct Image {
    data: Box<[Pixel]>,
    width: u32,
    height: u32,
}

impl Image {
    pub fn load(width: u32, height: u32, data: &[Pixel]) -> Self {
        Self {
            data: data.into(),
            width,
            height,
        }
    }

    #[inline]
    pub fn get(&self, x: u32, y: u32) -> Pixel {
        let y = y * self.width;
        self.data[(x + y) as usize]
    }
}
