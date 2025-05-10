use std::{
    collections::HashMap,
    io::{Read, stdout},
};

use simple_3d_core::{
    init, load_image, load_obj,
    types::{mesh::Mesh, textures::Image},
};
use termion::raw::IntoRawMode;

mod interface;

fn main() {
    let (objects, images) = load_objects(&["cube.obj"]);

    stdout().into_raw_mode().unwrap();
    let app = init::<interface::CliInterface>(objects, images);
    app.wait();
}

fn load_objects(paths: &[&str]) -> (Box<[Mesh]>, HashMap<Box<str>, Image>) {
    let mut objects = Vec::new();
    let mut images = HashMap::new();
    for path in paths {
        let object = load_obj(&load_binary_asset(path));
        let texture = object.texture.clone();
        let image = load_image(&load_binary_asset(&texture));
        images.insert(texture, image);
        objects.push(object);
    }
    (objects.into(), images)
}

fn load_binary_asset(path: &str) -> Vec<u8> {
    let path = format!("../assets/{}", path);
    let file = std::fs::File::open(path).expect("Unable to open file");
    let mut reader = std::io::BufReader::new(file);
    let mut buffer = Vec::new();
    reader
        .read_to_end(&mut buffer)
        .expect("Unable to read file");
    buffer
}
