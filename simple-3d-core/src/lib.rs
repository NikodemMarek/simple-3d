#![feature(test)]
extern crate test;

pub use loader::{load_image, load_obj};
use std::cell::RefCell;
use std::sync::mpsc;
use std::sync::mpsc::channel;
use types::camera::Camera;
use types::camera::CameraProperties;
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
    Nop,
    AddObject(Box<Mesh>),
    Resize(u32, u32),
    MoveCamera(Vector<3>),
    RotateObject(usize, Vector<3>),
}

pub type Objects = Vec<Mesh>;

pub struct App<I: Interface> {
    event_bus: (mpsc::Sender<Action>, mpsc::Receiver<Action>),
    _i: std::marker::PhantomData<I>,
}

impl<'a, I: Interface + 'a> App<I> {
    fn new() -> Self {
        let event_bus = channel::<Action>();
        Self {
            event_bus,
            _i: std::marker::PhantomData,
        }
    }

    fn add_object(&mut self, object: Mesh) {
        self.event_bus
            .0
            .send(Action::AddObject(Box::new(object)))
            .unwrap();
    }

    fn move_camera(&mut self, delta: Vector<3>) {
        self.event_bus.0.send(Action::MoveCamera(delta)).unwrap();
    }

    #[inline]
    fn sender(&self) -> mpsc::Sender<Action> {
        self.event_bus.0.clone()
    }

    fn handle_resize(&self) -> impl Guard + 'a {
        let sender = self.sender();
        I::handle_resize(move |width, height| {
            let _ = sender.send(Action::Resize(width, height));
        })
    }
    fn register_timer(&self, interval: i32, action: Action) -> impl Guard + 'a {
        let sender = self.sender();
        I::register_timer(interval, move || {
            let _ = sender.send(action.clone());
        })
    }
    fn on_key_hold(&self, key: &'a str, action: Action) -> impl Guard + 'a {
        let sender = self.sender();
        I::handle_key_hold(key, move || {
            let _ = sender.send(action.clone());
        })
    }
    fn start(
        self,
        mut screen: Screen,
        mut camera: Camera,
        textures: RefCell<Textures>,
    ) -> impl Guard {
        let mut objects = Vec::new();
        I::start(move || {
            for action in self.event_bus.1.try_iter() {
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
                    Action::Nop => (),
                }
            }

            screen.clear_buffer();
            screen.clear_depth();

            let camera_viewport_transformation =
                screen.transformation_matrix().clone() * camera.transformation_matrix().clone();

            let textures = textures.borrow();
            let transformed =
                transform::transform(&textures, &objects, camera_viewport_transformation);
            for (triangles, texture) in transformed {
                for p in rasterize::rasterize(texture, triangles) {
                    screen.put_pixel(p.0, p.1, p.2);
                }
            }

            I::draw(&screen);
        })
    }
}

pub trait Interface {
    fn get_screen_size() -> (u32, u32);

    fn handle_resize<C: Fn(u32, u32) + 'static>(on_resize: C) -> impl Guard;
    fn register_timer<C: FnMut() + Send + 'static>(interval: i32, on_tick: C) -> impl Guard;
    fn handle_key_hold<C: Fn() + Send + 'static>(key: &str, on_hold: C) -> impl Guard;
    fn start<C: FnMut() + Send + 'static>(on_frame: C) -> impl Guard;

    fn draw(screen: &Screen);
}

fn setup_screen<I: Interface>() -> (Screen, Camera) {
    let (width, height) = I::get_screen_size();
    let camera_properties = CameraProperties::new(FOV, width as f64 / height as f64, NEAR, FAR);
    let camera = Camera::new(camera_properties);
    let screen = Screen::new(width, height);
    (screen, camera)
}

pub fn init<'a, I: Interface + 'a + 'static>(
    objects: Box<[Mesh]>,
    images: Box<[(String, Image)]>,
) -> impl Guard + 'a {
    let mut textures = Textures::init();
    for (name, image) in images.into_iter() {
        textures.add(&name, types::textures::Texture::Image { image });
    }
    let textures = RefCell::new(textures);

    let mut interface = Box::new(App::<I>::new());

    for object in objects.into_iter() {
        interface.add_object(object);
    }

    interface.move_camera((0.0, 0.0, 5.0).into());

    let _resize_guard = interface.handle_resize();

    let _timer_guard =
        interface.register_timer(50, Action::RotateObject(0, (0.01, 0.02, 0.03).into()));

    let _key_guards = [
        interface.on_key_hold("ArrowUp", Action::MoveCamera((0.0, 0.1, 0.0).into())),
        interface.on_key_hold("ArrowDown", Action::MoveCamera((0.0, -0.1, 0.0).into())),
        interface.on_key_hold("ArrowLeft", Action::MoveCamera((-0.1, 0.0, 0.0).into())),
        interface.on_key_hold("ArrowRight", Action::MoveCamera((0.1, 0.0, 0.0).into())),
    ];

    let (screen, camera) = setup_screen::<I>();
    interface.start(screen, camera, textures)
}

pub trait Guard {
    fn is_finished(&self) -> bool;
    fn stop(self);
}
