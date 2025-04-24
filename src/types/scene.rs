use super::{camera::Camera, mesh::Mesh, screen::Screen, textures::Textures};

pub struct Scene {
    pub screen: Screen,
    pub camera: Camera,
    pub textures: Textures,
    pub objects: Vec<Mesh>,
}
