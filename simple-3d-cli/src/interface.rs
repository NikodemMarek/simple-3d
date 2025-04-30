use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
    thread::sleep,
};

use simple_3d_core::types::{
    camera::{Camera, CameraProperties},
    scene::Scene,
    screen::Screen,
    textures::Textures,
};

pub struct CliInterface;
impl simple_3d_core::Interface for CliInterface {
    fn new_scene(fov: f64, near: f64, far: f64) -> simple_3d_core::types::scene::Scene {
        let (width, height) = get_terminal_size();

        let camera_properties = CameraProperties::new(fov, width as f64 / height as f64, near, far);
        let camera = Camera::new(camera_properties);
        let screen = Screen::new(width, height);

        Scene {
            screen,
            camera,
            textures: Textures::init(),
            objects: Vec::from([]),
        }
    }

    fn handle_resize(scene: Rc<RefCell<Scene>>) {}

    fn register_timer<C: Fn(RefMut<Scene>) + 'static>(
        interval: i32,
        scene: Rc<RefCell<Scene>>,
        closure: C,
    ) {
    }

    fn on_key_hold<C: Fn(RefMut<Scene>, String) + 'static>(scene: Rc<RefCell<Scene>>, closure: C) {}

    fn start_animation_loop(scene: Rc<RefCell<Scene>>) {
        println!("Rendering frame");

        sleep(std::time::Duration::from_millis(100));

        Self::process(&mut scene.borrow_mut());
        Self::draw(&scene.borrow().screen);
    }

    fn draw(screen: &Screen) {}
}

fn get_terminal_size() -> (u32, u32) {
    let (width, height) = (100, 100);
    (width, height)
}
