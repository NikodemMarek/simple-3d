#![feature(test)]
extern crate test;

pub use loader::{load_image, load_obj};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::channel;
use types::camera::Camera;
use types::camera::CameraProperties;
use types::keys::Key;
use types::mesh::Mesh;
use types::screen::Screen;
use types::textures::Image;
use types::textures::Textures;
use types::vector;
use types::vector::Vector;

mod loader;
mod rasterize;
mod transform;
mod transformations;
pub mod types;

const NEAR: f64 = 0.1;
const FAR: f64 = 100.0;
const FOV: f64 = std::f64::consts::FRAC_PI_4;

#[derive(Clone, Debug)]
pub enum Action {
    AddObject(Box<Mesh>),
    Resize(u32, u32),
    MoveCamera(Vector<3>),
    RotateObject(usize, Vector<3>),
    End,
}

pub type Objects = Vec<Mesh>;

pub trait Interface {
    fn start<
        F: FnMut() -> Option<Screen> + Send + 'static,
        R: Fn(u32, u32) + Send + 'static,
        T: Fn() + Send + 'static,
        K: Fn() + Send + 'static,
    >(
        on_frame: F,
        on_resize: R,
        timers: Vec<(u64, T)>,
        keys: Vec<(Key, K)>,
    ) -> Self;

    fn get_screen_size() -> (u32, u32);
    fn wait(self);
}

fn setup_screen<I: Interface>() -> (Screen, Camera) {
    let (width, height) = I::get_screen_size();
    let camera_properties = CameraProperties::new(FOV, width as f64 / height as f64, NEAR, FAR);
    let camera = Camera::new(camera_properties);
    let screen = Screen::new(width, height);
    (screen, camera)
}

pub fn init<'a, I: Interface + 'a>(
    objects: Box<[Mesh]>,
    images: HashMap<Box<str>, Image>,
) -> App<I> {
    let timers = [(50, Action::RotateObject(0, (0.01, 0.02, 0.03).into()))];
    let keys = [
        (Key::ArrowUp, Action::MoveCamera((0.0, 0.1, 0.0).into())),
        (Key::ArrowDown, Action::MoveCamera((0.0, -0.1, 0.0).into())),
        (Key::ArrowLeft, Action::MoveCamera((-0.1, 0.0, 0.0).into())),
        (Key::ArrowRight, Action::MoveCamera((0.1, 0.0, 0.0).into())),
    ];

    let (tx, rx) = channel::<Action>();
    let (mut screen, mut camera) = setup_screen::<I>();

    let textures = RefCell::new(Textures::new(images));
    let mut objects = Vec::from(objects);

    let sender = tx.clone();
    let on_resize = move |width, height| {
        sender.send(Action::Resize(width, height)).unwrap();
    };
    let timers = timers
        .into_iter()
        .map(|(interval, action)| {
            let sender = tx.clone();
            (interval, move || {
                sender.send(action.clone()).unwrap();
            })
        })
        .collect::<Vec<_>>();
    let keys = keys
        .into_iter()
        .map(|(key, action)| {
            let sender = tx.clone();
            (key, move || {
                sender.send(action.clone()).unwrap();
            })
        })
        .collect::<Vec<_>>();

    let on_frame = move || {
        for action in rx.try_iter() {
            match action {
                Action::AddObject(object) => {
                    objects.push(*object);
                }
                Action::Resize(width, height) => {
                    screen = Screen::new(width, height);
                    let camera_properties = CameraProperties::inherit(
                        camera.properties().to_owned(),
                        width as f64 / height as f64,
                    );
                    camera = Camera::inherit(camera.clone(), camera_properties);
                }
                Action::MoveCamera(delta) => {
                    camera.r#move(*delta);
                }
                Action::RotateObject(index, delta) => {
                    if let Some(object) = objects.get_mut(index) {
                        object.rotate(*delta);
                    }
                }
                Action::End => {
                    return None;
                }
            }
        }

        screen.clear_buffer();
        screen.clear_depth();

        let camera_viewport_transformation =
            screen.transformation_matrix().clone() * camera.transformation_matrix().clone();

        let textures = textures.borrow();
        let transformed = transform::transform(&textures, &objects, camera_viewport_transformation);
        for (triangles, texture) in transformed {
            for p in rasterize::rasterize(texture, triangles) {
                screen.put_pixel(p.0, p.1, p.2);
            }
        }

        Some(screen.clone())
    };

    let interface = I::start(on_frame, on_resize, timers.to_vec(), keys);

    App(interface, tx)
}

pub struct App<I: Interface>(I, mpsc::Sender<Action>);
impl<I: Interface> App<I> {
    pub fn wait(self) {
        self.0.wait();
    }
    pub fn add_object(&self, object: Mesh) {
        self.1.send(Action::AddObject(Box::new(object))).unwrap();
    }
    pub fn resize(&self, width: u32, height: u32) {
        self.1.send(Action::Resize(width, height)).unwrap();
    }
    pub fn move_camera(&self, delta: Vector<3>) {
        self.1.send(Action::MoveCamera(delta)).unwrap();
    }
}
