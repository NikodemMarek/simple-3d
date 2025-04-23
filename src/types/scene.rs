use std::collections::HashMap;

use super::{camera::Camera, mesh::Mesh, screen::Screen, textures::Texture};

#[derive(Debug)]
pub struct Scene {
    pub screen: Screen,
    pub camera: Camera,
    pub textures: HashMap<Box<str>, Texture>,
    pub objects: Vec<Mesh>,
}
