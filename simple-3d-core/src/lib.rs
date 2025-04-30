#![feature(test)]
extern crate test;

use r#static::shapes;
use std::cell::RefCell;
use std::cell::RefMut;
use std::rc::Rc;
use types::scene::Scene;
use types::screen::Screen;
use types::textures::Image;
use types::vector;

mod rasterize;
mod r#static;
mod transform;
pub mod types;

const NEAR: f64 = 0.1;
const FAR: f64 = 100.0;
const FOV: f64 = std::f64::consts::FRAC_PI_4;

pub trait Interface {
    fn new_scene(fov: f64, near: f64, far: f64) -> Scene;

    fn handle_resize(scene: Rc<RefCell<Scene>>);
    fn register_timer<C: Fn(RefMut<Scene>) + 'static>(
        interval: i32,
        scene: Rc<RefCell<Scene>>,
        closure: C,
    );
    fn on_key_hold<C: Fn(RefMut<Scene>, String) + 'static>(scene: Rc<RefCell<Scene>>, closure: C);
    fn start_animation_loop(scene: Rc<RefCell<Scene>>);

    fn process(scene: &mut Scene) {
        scene.screen.clear_depth();
        scene.screen.clear_buffer();

        let camera_viewport_transformation = scene.screen.transformation_matrix().clone()
            * scene.camera.transformation_matrix().clone();

        let transformed = transform::transform(
            &scene.textures,
            &scene.objects,
            camera_viewport_transformation,
        );
        for (triangles, texture) in transformed {
            for p in rasterize::rasterize(texture, triangles) {
                scene.screen.put_pixel(p.0, p.1, p.2);
            }
        }
    }
    fn draw(screen: &Screen);
}

pub async fn init<I: Interface>(images: Box<[(String, Image)]>) -> Result<(), ()> {
    let mut scene = I::new_scene(FOV, NEAR, FAR);

    for (name, image) in images.into_iter() {
        scene
            .textures
            .add(&name, types::textures::Texture::Image { image });
    }

    let mut cube = shapes::cube();
    cube.texture = "crate".into();
    scene.objects.push(cube);

    scene.camera.r#move((0.0, 0.0, 5.0));

    let scene = Rc::new(RefCell::new(scene));

    I::handle_resize(Rc::clone(&scene));

    I::register_timer(50, Rc::clone(&scene), |mut scene| {
        scene.objects[0].rotate((0.01, 0.02, 0.03));
    });

    I::on_key_hold(Rc::clone(&scene), |mut scene, key| match key.as_ref() {
        "ArrowUp" => {
            scene.camera.r#move((0.0, 0.1, 0.0));
        }
        "ArrowDown" => {
            scene.camera.r#move((0.0, -0.1, 0.0));
        }
        "ArrowLeft" => {
            scene.camera.r#move((-0.1, 0.0, 0.0));
        }
        "ArrowRight" => {
            scene.camera.r#move((0.1, 0.0, 0.0));
        }
        _ => {}
    });

    I::start_animation_loop(scene);

    Ok(())
}
