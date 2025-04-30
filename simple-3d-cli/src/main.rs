use simple_3d_core::types::{pixel::Pixel, textures::Image};

mod interface;

fn main() {
    let image = Image::load(100, 100, &[Pixel(255, 0, 0, 255); 100 * 100]);

    simple_3d_core::init::<interface::CliInterface>(Box::new([("crate".to_string(), image)]));
}
