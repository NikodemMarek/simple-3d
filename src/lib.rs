use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::KeyboardEvent;
use web_sys::MouseEvent;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

mod rendering;
mod types3d;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let (canvas, context) = get_canvas_and_context();
    let canvas = Rc::new(canvas);
    let context = Rc::new(context);

    let width = canvas.client_width() as f64;
    let height = canvas.client_height() as f64;
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

    resize_canvas_to_display_size(&canvas, &camera);

    register_resize_listener(&canvas, &camera);
    register_timer(50, &camera);
    register_keypress_listener(&camera);
    register_mouse_drag_listener(&camera);

    start_animation_loop(context, &camera);

    Ok(())
}

fn get_canvas_and_context() -> (HtmlCanvasElement, CanvasRenderingContext2d) {
    let canvas = document()
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    (canvas, context)
}

fn resize_canvas_to_display_size(canvas: &HtmlCanvasElement, camera: &Rc<RefCell<Camera>>) {
    let width = window().inner_width().unwrap().as_f64().unwrap() as u32;
    let height = window().inner_height().unwrap().as_f64().unwrap() as u32;

    canvas.set_width(width);
    canvas.set_height(height);

    camera.borrow_mut().screen_size = (width as f64, height as f64);
    camera.borrow_mut().aspect_ratio = width as f64 / height as f64;
}

fn register_resize_listener(canvas: &Rc<HtmlCanvasElement>, camera: &Rc<RefCell<Camera>>) {
    let canvas = Rc::clone(canvas);
    let camera = Rc::clone(camera);
    let closure = Closure::wrap(Box::new(move || {
        resize_canvas_to_display_size(&canvas, &camera);
    }) as Box<dyn Fn()>);

    window()
        .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget(); // Keep it alive
}

fn register_timer(interval: i32, camera: &Rc<RefCell<Camera>>) {
    let camera = Rc::clone(camera);
    let closure = Closure::wrap(Box::new(move || {
        camera.borrow_mut().position.x -= 0.01;
        camera.borrow_mut().position.y += 0.01;
    }) as Box<dyn FnMut()>);

    window()
        .set_interval_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            interval,
        )
        .unwrap();
    closure.forget(); // Keep it alive
}

fn register_keypress_listener(camera: &Rc<RefCell<Camera>>) {
    let camera = Rc::clone(camera);
    let closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
        let key = event.key();
        web_sys::console::log_1(&format!("Key pressed: {}", key).into());
        match key.as_ref() {
            "ArrowUp" => {
                camera.borrow_mut().position.y += 0.1;
            }
            "ArrowDown" => {
                camera.borrow_mut().position.y -= 0.1;
            }
            "ArrowLeft" => {
                camera.borrow_mut().position.x -= 0.1;
            }
            "ArrowRight" => {
                camera.borrow_mut().position.x += 0.1;
            }
            _ => {}
        }
    }) as Box<dyn FnMut(_)>);

    window()
        .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget(); // Keep it alive
}

fn register_mouse_drag_listener(camera: &Rc<RefCell<Camera>>) {
    let camera = Rc::clone(camera);

    let is_held = Rc::new(Cell::new(false));
    let md_is_held = Rc::clone(&is_held);
    let mousedown_closure = Closure::wrap(Box::new(move |_: MouseEvent| {
        md_is_held.set(true);
    }) as Box<dyn FnMut(_)>);
    let mu_is_held = Rc::clone(&is_held);
    let mouseup_closure = Closure::wrap(Box::new(move |_: MouseEvent| {
        mu_is_held.set(false);
    }) as Box<dyn FnMut(_)>);

    let move_closure = Closure::wrap(Box::new(move |event: MouseEvent| {
        if !is_held.get() {
            return;
        }

        let x = event.client_x() as f64;
        let y = event.client_y() as f64;

        let (width, height) = camera.borrow().screen_size;
        camera.borrow_mut().target.x = (x - width / 2.0) / 100.0;
        camera.borrow_mut().target.y = (y - height / 2.0) / 100.0;
        camera.borrow_mut().target.z = -1.0;
    }) as Box<dyn FnMut(_)>);

    window()
        .add_event_listener_with_callback("mousedown", mousedown_closure.as_ref().unchecked_ref())
        .unwrap();
    window()
        .add_event_listener_with_callback("mouseup", mouseup_closure.as_ref().unchecked_ref())
        .unwrap();
    window()
        .add_event_listener_with_callback("mousemove", move_closure.as_ref().unchecked_ref())
        .unwrap();
    mousedown_closure.forget(); // Keep it alive
    mouseup_closure.forget(); // Keep it alive
    move_closure.forget(); // Keep it alive
}

#[derive(Debug)]
struct Camera {
    screen_size: (f64, f64),

    position: types3d::Point3d,
    target: types3d::Point3d,
    up: types3d::Point3d,

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

fn start_animation_loop(context: Rc<CanvasRenderingContext2d>, camera: &Rc<RefCell<Camera>>) {
    let f: Rc<RefCell<_>> = Rc::new(RefCell::new(None));
    let g = Rc::clone(&f);
    let camera = Rc::clone(camera);

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        rendering::render(&context, &camera.borrow());

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}
