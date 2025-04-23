use rasterizing::Screen;
use shapes::Mesh;
use std::cell::Cell;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::Event;
use web_sys::KeyboardEvent;
use web_sys::MouseEvent;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

mod matrix;
mod rasterizing;
mod rendering;
mod shapes;
mod textures;
mod transformations;
mod vector;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn canvas() -> HtmlCanvasElement {
    document()
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap()
}

fn context() -> CanvasRenderingContext2d {
    canvas()
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap()
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let width = canvas().client_width() as u32;
    let height = canvas().client_height() as u32;

    let screen = Screen::new(width, height);
    let camera = Camera {
        position: (0.0, 0.0, 5.0).into(),
        target: (0.0, 0.0, 0.0).into(),
        up: (0.0, 1.0, 0.0).into(),
        fov: std::f64::consts::FRAC_PI_4,
        aspect_ratio: width as f64 / height as f64,
        near: 0.1,
        far: 100.0,
    };
    let textures = textures::init();

    let mut cube = shapes::cube();
    cube.texture = "cube".into();

    let transformation = transformations::translation(&(0.0, 0.0, 2.0).into())
        * transformations::rotation_x(std::f64::consts::FRAC_PI_4)
        * transformations::scaling(&(0.2, 0.2, 0.2).into());
    let mut cube2 = shapes::cube().transformed(&transformation);
    cube2.texture = "cube".into();

    let objects = Vec::from([cube, cube2]);
    let scene = Rc::new(RefCell::new(Scene {
        screen,
        camera,
        textures,
        objects,
    }));

    resize_display(&scene);
    {
        let scene = Rc::clone(&scene);
        register_event_listener("resize", move |_: Event| {
            resize_display(&scene);
        });
    }

    {
        let scene = Rc::clone(&scene);
        let transformation = transformations::rotation_y(0.1);
        let transformation2 = transformations::rotation_x(0.1);
        register_timer(50, move || {
            scene.borrow_mut().objects[0].transform(&transformation);
            scene.borrow_mut().objects[1].transform(&transformation2);
        });
    }

    {
        let scene = Rc::clone(&scene);
        register_event_listener("keydown", move |event: KeyboardEvent| {
            let key = event.key();
            web_sys::console::log_1(&format!("Key pressed: {}", key).into());
            match key.as_ref() {
                "ArrowUp" => {
                    scene.borrow_mut().camera.position += (0.0, 0.1, 0.0).into();
                }
                "ArrowDown" => {
                    scene.borrow_mut().camera.position += (0.0, -0.1, 0.0).into();
                }
                "ArrowLeft" => {
                    scene.borrow_mut().camera.position += (-0.1, 0.0, 0.0).into();
                }
                "ArrowRight" => {
                    scene.borrow_mut().camera.position += (0.1, 0.0, 0.0).into();
                }
                _ => {}
            }
        });
    }

    {
        let is_held = Rc::new(Cell::new(false));
        let last_mouse = Rc::new(RefCell::new((0.0f64, 0.0f64)));
        let azimuth = Rc::new(RefCell::new(0.0f64));
        let elevation = Rc::new(RefCell::new(0.0f64));

        {
            let is_held = Rc::clone(&is_held);
            let last_mouse = Rc::clone(&last_mouse);
            register_event_listener("mousedown", move |event: MouseEvent| {
                is_held.set(true);
                *last_mouse.borrow_mut() = (event.client_x() as f64, event.client_y() as f64);
            });
        }
        {
            let is_held = Rc::clone(&is_held);
            register_event_listener("mouseup", move |_: MouseEvent| {
                is_held.set(false);
            });
        }

        {
            let scene = Rc::clone(&scene);
            let is_held = Rc::clone(&is_held);
            let last_mouse = Rc::clone(&last_mouse);
            let azimuth = Rc::clone(&azimuth);
            let elevation = Rc::clone(&elevation);

            register_event_listener("mousemove", move |event: MouseEvent| {
                if !is_held.get() {
                    return;
                }

                let (last_x, last_y) = *last_mouse.borrow();
                let x = event.client_x() as f64;
                let y = event.client_y() as f64;
                let delta_x = x - last_x;
                let delta_y = y - last_y;
                *last_mouse.borrow_mut() = (x, y);

                let sensitivity = 0.005;
                *azimuth.borrow_mut() += delta_x * sensitivity;
                *elevation.borrow_mut() += delta_y * sensitivity;

                let elev = elevation.borrow().clamp(
                    -std::f64::consts::FRAC_PI_2 + 0.01,
                    std::f64::consts::FRAC_PI_2 - 0.01,
                );
                *elevation.borrow_mut() = elev;

                let radius =
                    (scene.borrow().camera.position - scene.borrow().camera.target).magnitude();

                let az = *azimuth.borrow();
                let x = radius * elev.cos() * az.sin();
                let y = radius * elev.sin();
                let z = radius * elev.cos() * az.cos();

                let new_pos =
                    Into::<vector::Vector<3>>::into((x, y, z)) + scene.borrow().camera.target;
                scene.borrow_mut().camera.position = new_pos;
            });
        }
    }

    start_animation_loop(&scene);

    Ok(())
}

fn resize_display(scene: &Rc<RefCell<Scene>>) {
    let width = window().inner_width().unwrap().as_f64().unwrap() as u32;
    let height = window().inner_height().unwrap().as_f64().unwrap() as u32;

    let canvas = canvas();
    canvas.set_width(width);
    canvas.set_height(height);

    let mut scene = scene.borrow_mut();
    scene.screen = Screen::new(width, height);
    scene.camera.aspect_ratio = width as f64 / height as f64;
}

fn register_timer<C: Fn() + 'static>(interval: i32, closure: C) {
    let closure = Closure::wrap(Box::new(closure) as Box<dyn FnMut()>);

    window()
        .set_interval_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            interval,
        )
        .unwrap();
    closure.forget(); // Keep it alive
}

fn register_event_listener<E, C>(event: &str, closure: C)
where
    E: JsCast + 'static,
    C: Fn(E) + 'static,
{
    let closure = Closure::wrap(Box::new(move |e: web_sys::Event| {
        closure(e.dyn_into::<E>().unwrap());
    }) as Box<dyn FnMut(_)>);

    window()
        .add_event_listener_with_callback(event, closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget(); // Keep it alive
}

#[derive(Debug)]
struct Scene {
    screen: Screen,
    camera: Camera,
    textures: HashMap<Box<str>, textures::Texture>,
    objects: Vec<Mesh>,
}

#[derive(Debug)]
struct Camera {
    position: vector::Vector<3>,
    target: vector::Vector<3>,
    up: vector::Vector<3>,

    fov: f64, // in radians
    aspect_ratio: f64,

    near: f64,
    far: f64,
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn start_animation_loop(scene: &Rc<RefCell<Scene>>) {
    let f: Rc<RefCell<_>> = Rc::new(RefCell::new(None));
    let g = Rc::clone(&f);
    let context = context();
    let scene = Rc::clone(scene);

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let mut scene = scene.borrow_mut();

        rendering::render(&mut scene);
        draw(&scene.screen, &context);
        scene.screen.clear_buffer();

        web_sys::console::log_1(&"Rendering frame".into());

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}

#[derive(Debug, Copy, Clone)]
pub struct Pixel(pub u8, pub u8, pub u8, pub u8);
impl Default for Pixel {
    fn default() -> Self {
        Pixel(255, 255, 255, 255)
    }
}

fn draw(
    Screen {
        width,
        height,
        buffer,
        ..
    }: &Screen,
    context: &CanvasRenderingContext2d,
) {
    context.clear_rect(0.0, 0.0, *width as f64, *height as f64);

    let image_data = mk_image_data(*width, *height, buffer);
    context.put_image_data(&image_data, 0.0, 0.0).unwrap();
}

fn mk_image_data(width: u32, height: u32, data: &[Pixel]) -> web_sys::ImageData {
    let data = data
        .iter()
        .flat_map(|Pixel(r, g, b, a)| vec![*r, *g, *b, *a])
        .collect::<Vec<_>>();
    web_sys::ImageData::new_with_u8_clamped_array_and_sh(
        wasm_bindgen::Clamped(&data),
        width,
        height,
    )
    .unwrap()
}
