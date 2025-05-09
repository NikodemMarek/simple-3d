use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{JsCast, prelude::Closure};
use web_sys::{Event, KeyboardEvent};

use simple_3d_core::{
    Guard,
    types::{pixel::Pixel, screen::Screen},
};

use crate::{canvas, context, window};

fn register_timer<C: FnMut() + 'static>(interval: i32, closure: C) -> impl Guard {
    let closure = Closure::wrap(Box::new(closure) as Box<dyn FnMut()>);

    let handle = window()
        .set_interval_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            interval,
        )
        .unwrap();
    WasmTimerGuard(handle)
}

fn register_event_listener<E, C>(event: &str, closure: C) -> impl Guard
where
    E: JsCast + 'static,
    C: Fn(E) + 'static,
{
    let closure = Closure::wrap(Box::new(move |e: web_sys::Event| {
        closure(e.dyn_into::<E>().unwrap());
    }) as Box<dyn FnMut(_)>);

    let guard = WasmEventGuard::new(event.to_string(), closure);
    window()
        .add_event_listener_with_callback(event, guard.closure.as_ref().unchecked_ref())
        .unwrap();
    guard
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub struct WasmInterface;
impl simple_3d_core::Interface for WasmInterface {
    fn get_screen_size() -> (u32, u32) {
        let width = window().inner_width().unwrap().as_f64().unwrap() as u32;
        let height = window().inner_height().unwrap().as_f64().unwrap() as u32;
        (width, height)
    }

    fn handle_resize<C: Fn(u32, u32) + 'static>(on_resize: C) -> impl Guard {
        register_event_listener("resize", move |_: Event| {
            let (width, height) = Self::get_screen_size();
            resize_screen(width, height);

            on_resize(width, height);
        })
    }

    fn register_timer<C: FnMut() + 'static>(interval: i32, on_tick: C) -> impl Guard {
        register_timer(interval, on_tick)
    }

    fn handle_key_hold<C: Fn() + 'static>(key: &str, on_hold: C) -> impl Guard {
        let key = key.to_owned();
        register_event_listener("keydown", move |event: KeyboardEvent| {
            if key == event.key() {
                on_hold();
            }
        })
    }

    fn start<C: FnMut() + Send + 'static>(mut on_frame: C) -> impl Guard {
        let (width, height) = Self::get_screen_size();
        resize_screen(width, height);

        let f: Rc<RefCell<_>> = Rc::new(RefCell::new(None));
        let g = Rc::clone(&f);

        let guard = WasmAnimationFrameGuard::default();
        let gc = Rc::clone(&guard.0);
        *g.borrow_mut() = Some(Closure::new(move || {
            web_sys::console::log_1(&"Rendering frame".into());

            on_frame();

            if gc.get() {
                request_animation_frame(f.borrow().as_ref().unwrap());
            }
        }));

        request_animation_frame(g.borrow().as_ref().unwrap());
        guard
    }

    fn draw(screen: &Screen) {
        let context = context();
        let (width, height) = screen.size();
        context.clear_rect(0.0, 0.0, width as f64, height as f64);

        let image_data = mk_image_data(screen);
        context.put_image_data(&image_data, 0.0, 0.0).unwrap();
    }

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

fn resize_screen(width: u32, height: u32) -> Screen {
    let canvas = canvas();
    canvas.set_width(width);
    canvas.set_height(height);

    Screen::new(width, height)
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

struct WasmAnimationFrameGuard(Rc<std::cell::Cell<bool>>);
impl Default for WasmAnimationFrameGuard {
    fn default() -> Self {
        Self(Rc::new(std::cell::Cell::new(true)))
    }
}
impl Guard for WasmAnimationFrameGuard {
    fn is_finished(&self) -> bool {
        false
    }
    fn stop(self) {
        self.0.set(false);
    }
}

struct WasmTimerGuard(i32);
impl Guard for WasmTimerGuard {
    fn is_finished(&self) -> bool {
        false
    }
    fn stop(self) {
        window().clear_interval_with_handle(self.0);
    }
}

struct WasmEventGuard {
    event: String,
    closure: Closure<dyn FnMut(Event)>,
}
impl WasmEventGuard {
    fn new(event: String, closure: Closure<dyn FnMut(Event)>) -> Self {
        Self { event, closure }
    }
}
impl Guard for WasmEventGuard {
    fn is_finished(&self) -> bool {
        false
    }
    fn stop(self) {
        window()
            .remove_event_listener_with_callback(&self.event, self.closure.as_ref().unchecked_ref())
            .unwrap();
        self.closure.forget();
    }
}
