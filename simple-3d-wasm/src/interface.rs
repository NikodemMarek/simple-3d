use simple_3d_core::types::{keys::Key, pixel::Pixel, screen::Screen};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{JsCast, prelude::Closure};
use web_sys::{CanvasRenderingContext2d, Event, HtmlCanvasElement, KeyboardEvent};

use crate::{canvas, context, window};

fn register_timer<C: FnMut() + 'static>(interval: i32, closure: C) {
    let closure = Closure::wrap(Box::new(closure) as Box<dyn FnMut()>);

    window()
        .set_interval_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            interval,
        )
        .unwrap();
    closure.forget();
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
    closure.forget();
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub struct WasmInterface;
impl simple_3d_core::Interface for WasmInterface {
    fn start<
        F: FnMut() -> Option<Screen> + Send + 'static,
        R: Fn(u32, u32) + Send + 'static,
        T: Fn() + Send + 'static,
        K: Fn() + Send + 'static,
    >(
        mut on_frame: F,
        on_resize: R,
        timers: Vec<(u64, T)>,
        keys: Vec<(Key, K)>,
    ) -> Self {
        let c = canvas();
        let (width, height) = Self::get_screen_size();
        resize_screen(&c, width, height);
        on_resize(width, height);

        register_event_listener("resize", move |_: Event| {
            let (width, height) = Self::get_screen_size();
            resize_screen(&c, width, height);

            on_resize(width, height);
        });

        for (timer, on_tick) in timers {
            register_timer(timer as i32, on_tick);
        }

        for (key, on_hold) in keys {
            register_event_listener("keydown", move |event: KeyboardEvent| {
                let pressed = match event.key().as_str() {
                    "ArrowUp" => Key::ArrowUp,
                    "ArrowDown" => Key::ArrowDown,
                    "ArrowLeft" => Key::ArrowLeft,
                    "ArrowRight" => Key::ArrowRight,
                    _ => return,
                };
                if key == pressed {
                    on_hold();
                }
            });
        }

        let c = context();
        let f: Rc<RefCell<_>> = Rc::new(RefCell::new(None));
        let g = Rc::clone(&f);
        *g.borrow_mut() = Some(Closure::new(move || {
            web_sys::console::log_1(&"Rendering frame".into());

            if let Some(screen) = on_frame() {
                draw(&c, &screen);
            } else {
                return;
            }

            request_animation_frame(f.borrow().as_ref().unwrap());
        }));

        request_animation_frame(g.borrow().as_ref().unwrap());

        WasmInterface
    }

    fn get_screen_size() -> (u32, u32) {
        let width = window().inner_width().unwrap().as_f64().unwrap() as u32;
        let height = window().inner_height().unwrap().as_f64().unwrap() as u32;
        (width, height)
    }
    fn wait(self) {}

    // {
    //     let is_held = Rc::new(Cell::new(false));
    //     let last_mouse = Rc::new(RefCell::new((0.0f64, 0.0f64)));
    //     let azimuth = Rc::new(RefCell::new(0.0f64));
    //     let elevation = Rc::new(RefCell::new(0.0f64));
    //
    //     {
    //         let is_held = Rc::clone(&is_held);
    //         let last_mouse = Rc::clone(&last_mouse);
    //         register_event_listener("mousedown", move |event: MouseEvent| {
    //             is_held.set(true);
    //             *last_mouse.borrow_mut() = (event.client_x() as f64, event.client_y() as f64);
    //         });
    //     }
    //     {
    //         let is_held = Rc::clone(&is_held);
    //         register_event_listener("mouseup", move |_: MouseEvent| {
    //             is_held.set(false);
    //         });
    //     }
    //
    //     {
    //         let scene = Rc::clone(&scene);
    //         let is_held = Rc::clone(&is_held);
    //         let last_mouse = Rc::clone(&last_mouse);
    //         let azimuth = Rc::clone(&azimuth);
    //         let elevation = Rc::clone(&elevation);
    //
    //         register_event_listener("mousemove", move |event: MouseEvent| {
    //             if !is_held.get() {
    //                 return;
    //             }
    //
    //             let (last_x, last_y) = *last_mouse.borrow();
    //             let x = event.client_x() as f64;
    //             let y = event.client_y() as f64;
    //             let delta_x = x - last_x;
    //             let delta_y = y - last_y;
    //             *last_mouse.borrow_mut() = (x, y);
    //
    //             let sensitivity = 0.005;
    //             *azimuth.borrow_mut() += delta_x * sensitivity;
    //             *elevation.borrow_mut() += delta_y * sensitivity;
    //
    //             let elev = elevation.borrow().clamp(
    //                 -std::f64::consts::FRAC_PI_2 + 0.01,
    //                 std::f64::consts::FRAC_PI_2 - 0.01,
    //             );
    //             *elevation.borrow_mut() = elev;
    //
    //             let radius = scene.borrow().camera.radius();
    //
    //             let az = *azimuth.borrow();
    //             let x = radius * elev.cos() * az.sin();
    //             let y = radius * elev.sin();
    //             let z = radius * elev.cos() * az.cos();
    //
    //             scene.borrow_mut().camera.r#move((x, y, z));
    //         });
    //     }
    // }
}

fn draw(context: &CanvasRenderingContext2d, screen: &Screen) {
    let (width, height) = screen.size();
    context.clear_rect(0.0, 0.0, width as f64, height as f64);

    let image_data = mk_image_data(screen);
    context.put_image_data(&image_data, 0.0, 0.0).unwrap();
}

fn resize_screen(canvas: &HtmlCanvasElement, width: u32, height: u32) {
    canvas.set_width(width);
    canvas.set_height(height);
}

fn mk_image_data(screen: &Screen) -> web_sys::ImageData {
    let data = screen
        .buffer()
        .iter()
        .flat_map(|Pixel(r, g, b, a)| vec![*r, *g, *b, *a])
        .collect::<Vec<_>>();
    let (width, height) = screen.size();
    web_sys::ImageData::new_with_u8_clamped_array_and_sh(
        wasm_bindgen::Clamped(&data),
        width,
        height,
    )
    .unwrap()
}
