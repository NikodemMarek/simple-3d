use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::Event;
use web_sys::KeyboardEvent;
use web_sys::MouseEvent;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

mod matrix;
mod rendering;
mod shapes;
mod vectors;

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
    let width = canvas().client_width() as f64;
    let height = canvas().client_height() as f64;
    let camera = Camera {
        screen_size: (width, height),
        position: (0.0, 0.0, 3.0).into(),
        target: (0.0, 0.0, -1.0).into(),
        up: (0.0, 1.0, 0.0).into(),
        fov: std::f64::consts::PI / 2.0,
        aspect_ratio: width / height,
        near: 0.1,
        far: 100.0,
    };
    let camera = Rc::new(RefCell::new(camera));

    resize_display(&camera);
    {
        let camera = Rc::clone(&camera);
        register_event_listener("resize", move |_: Event| {
            resize_display(&camera);
        });
    }

    {
        let camera = Rc::clone(&camera);
        register_timer(50, move || {
            camera.borrow_mut().position += (-0.01, 0.01, 0.0).into();
        });
    }

    {
        let camera = Rc::clone(&camera);
        register_event_listener("keydown", move |event: KeyboardEvent| {
            let key = event.key();
            web_sys::console::log_1(&format!("Key pressed: {}", key).into());
            match key.as_ref() {
                "ArrowUp" => {
                    camera.borrow_mut().position += (0.1, 0.0, 0.0).into();
                }
                "ArrowDown" => {
                    camera.borrow_mut().position += (-0.1, 0.0, 0.0).into();
                }
                "ArrowLeft" => {
                    camera.borrow_mut().position += (-0.1, 0.0, 0.0).into();
                }
                "ArrowRight" => {
                    camera.borrow_mut().position += (0.1, 0.0, 0.0).into();
                }
                _ => {}
            }
        });
    }

    {
        let is_held = Rc::new(Cell::new(false));
        {
            let is_held = Rc::clone(&is_held);
            register_event_listener("mousedown", move |_: MouseEvent| {
                is_held.set(true);
            });
        }
        {
            let is_held = Rc::clone(&is_held);
            register_event_listener("mouseup", move |_: MouseEvent| {
                is_held.set(false);
            });
        }

        let camera = Rc::clone(&camera);
        register_event_listener("mousemove", move |event: MouseEvent| {
            if !is_held.get() {
                return;
            }

            let x = event.client_x() as f64;
            let y = event.client_y() as f64;

            let (width, height) = camera.borrow().screen_size;
            camera.borrow_mut().target +=
                ((x - width / 2.0) / 100.0, (y - height / 2.0) / 100.0, -1.0).into();
        });
    }

    start_animation_loop(&camera);

    Ok(())
}

fn resize_display(camera: &Rc<RefCell<Camera>>) {
    let width = window().inner_width().unwrap().as_f64().unwrap() as u32;
    let height = window().inner_height().unwrap().as_f64().unwrap() as u32;

    let canvas = canvas();
    canvas.set_width(width);
    canvas.set_height(height);

    camera.borrow_mut().screen_size = (width as f64, height as f64);
    camera.borrow_mut().aspect_ratio = width as f64 / height as f64;
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
struct Camera {
    screen_size: (f64, f64),

    position: vectors::Vector<3>,
    target: vectors::Vector<3>,
    up: vectors::Vector<3>,

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

fn start_animation_loop(camera: &Rc<RefCell<Camera>>) {
    let f: Rc<RefCell<_>> = Rc::new(RefCell::new(None));
    let g = Rc::clone(&f);
    let context = context();
    let camera = Rc::clone(camera);

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        rendering::render(&context, &camera.borrow());

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}
