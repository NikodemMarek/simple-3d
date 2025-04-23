use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::console::log_1;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

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

    resize_canvas_to_display_size(&canvas);

    register_resize_listener(&canvas);

    start_animation_loop(canvas, context);

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

fn resize_canvas_to_display_size(canvas: &HtmlCanvasElement) {
    let width = window().inner_width().unwrap().as_f64().unwrap() as u32;
    let height = window().inner_height().unwrap().as_f64().unwrap() as u32;

    canvas.set_width(width);
    canvas.set_height(height);

    log_1(&format!("resized to {}x{}", width, height).into());
}

fn register_resize_listener(canvas: &Rc<HtmlCanvasElement>) {
    let canvas = Rc::clone(canvas);
    let closure = Closure::wrap(Box::new(move || {
        resize_canvas_to_display_size(&canvas);
    }) as Box<dyn Fn()>);

    window()
        .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget(); // Keep it alive
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn start_animation_loop(canvas: Rc<HtmlCanvasElement>, context: Rc<CanvasRenderingContext2d>) {
    let f: Rc<RefCell<_>> = Rc::new(RefCell::new(None));
    let g = Rc::clone(&f);

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        draw_centered_rect(&canvas, &context);

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}

fn draw_centered_rect(canvas: &HtmlCanvasElement, context: &CanvasRenderingContext2d) {
    let width = canvas.width() as f64;
    let height = canvas.height() as f64;

    context.clear_rect(0.0, 0.0, width, height);

    let rect_width = 100.0;
    let rect_height = 100.0;

    let x = (width - rect_width) / 2.0;
    let y = (height - rect_height) / 2.0;

    context.set_fill_style_str(&"red");
    context.fill_rect(x, y, rect_width, rect_height);
}
