use std::io::Read;

use simple_3d_core::{
    load_obj,
    types::{pixel::Pixel, textures::Image},
};

mod interface;

fn main() {
    let image = Image::load(100, 100, &[Pixel(255, 0, 0, 255); 100 * 100]);
    let object = load_obj(&load_binary_asset("cube.obj"));

    simple_3d_core::init::<interface::CliInterface>(
        Box::new([object]),
        Box::new([("crate.jpg".to_string(), image)]),
    );
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
