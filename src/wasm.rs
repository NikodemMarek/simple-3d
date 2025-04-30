use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use wasm_bindgen::{JsCast, prelude::Closure};
use wasm_bindgen_futures::JsFuture;
use web_sys::{CanvasRenderingContext2d, Event, HtmlCanvasElement, ImageBitmap, KeyboardEvent};

use crate::types::{
    camera::{Camera, CameraProperties},
    pixel::Pixel,
    scene::Scene,
    screen::Screen,
    textures::Textures,
};

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

async fn fetch_image_data(url: &str) -> ImageBitmap {
    let resp_value = window().fetch_with_str(url);
    let resp = JsFuture::from(resp_value)
        .await
        .expect("Failed to fetch image data")
        .dyn_into::<web_sys::Response>()
        .expect("Failed to convert to Response");

    if !resp.ok() {
        panic!("Failed to fetch image data");
    }

    if !resp
        .headers()
        .get("Content-Type")
        .expect("Failed to get Content-Type header")
        .expect("Failed to get Content-Type header")
        .starts_with("image/")
    {
        panic!("Invalid MIME type");
    }

    let blob = JsFuture::from(resp.blob().expect("Failed to get blob"))
        .await
        .expect("Failed to convert to Blob")
        .dyn_into::<web_sys::Blob>()
        .expect("Failed to convert to Blob");

    let obj_url =
        web_sys::Url::create_object_url_with_blob(&blob).expect("Failed to create object URL");

    let promise = window()
        .create_image_bitmap_with_blob(&blob)
        .expect("Failed to create ImageBitmap");
    let image_bitmap = JsFuture::from(promise)
        .await
        .expect("Failed to convert to ImageBitmap");

    web_sys::Url::revoke_object_url(&obj_url).unwrap();
    image_bitmap.into()
}

pub async fn load_and_process_image(url: &str) -> crate::types::textures::Image {
    let image_bitmap = fetch_image_data(url).await;

    let width = image_bitmap.width();
    let height = image_bitmap.height();

    let canvas = canvas();

    canvas.set_width(width);
    canvas.set_height(height);

    context()
        .draw_image_with_image_bitmap(&image_bitmap, 0.0, 0.0)
        .expect("Failed to draw image");

    let image_data = context()
        .get_image_data(0.0, 0.0, width as f64, height as f64)
        .expect("Failed to get image data");

    use crate::types::pixel::Pixel;
    let data = image_data.data().to_vec();
    let data = data
        .chunks_exact(4)
        .map(|chunk| Pixel(chunk[0], chunk[1], chunk[2], chunk[3]))
        .collect::<Box<[Pixel]>>();

    crate::types::textures::Image::load(width, height, &data)
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

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub struct WasmInterface;
impl crate::Interface for WasmInterface {
    fn new_scene(fov: f64, near: f64, far: f64) -> Scene {
        let (width, height) = get_display_size();

        let camera_properties = CameraProperties::new(fov, width as f64 / height as f64, near, far);
        let camera = Camera::new(camera_properties);
        let (camera, screen) = resize_screen(width, height, camera);

        Scene {
            screen,
            camera,
            textures: Textures::init(),
            objects: Vec::from([]),
        }
    }

    fn handle_resize(scene: Rc<RefCell<Scene>>) {
        register_event_listener("resize", move |_: Event| {
            let (width, height) = get_display_size();
            let (camera, screen) = resize_screen(width, height, scene.borrow().camera.clone());

            scene.borrow_mut().camera = camera;
            scene.borrow_mut().screen = screen;
        });
    }

    fn register_timer<C: Fn(RefMut<Scene>) + 'static>(
        interval: i32,
        scene: Rc<RefCell<Scene>>,
        closure: C,
    ) {
        register_timer(interval, move || {
            closure(scene.borrow_mut());
        });
    }

    fn on_key_hold<C: Fn(RefMut<Scene>, String) + 'static>(scene: Rc<RefCell<Scene>>, closure: C) {
        register_event_listener("keydown", move |event: KeyboardEvent| {
            let key = event.key();
            web_sys::console::log_1(&format!("Key held: {}", key).into());

            let scene = scene.borrow_mut();
            closure(scene, key.clone());
        });
    }

    fn start_animation_loop(scene: Rc<RefCell<Scene>>) {
        let f: Rc<RefCell<_>> = Rc::new(RefCell::new(None));
        let g = Rc::clone(&f);

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            web_sys::console::log_1(&"Rendering frame".into());

            Self::process(&mut scene.borrow_mut());
            Self::draw(&scene.borrow().screen);

            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));

        request_animation_frame(g.borrow().as_ref().unwrap());
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

fn resize_screen(width: u32, height: u32, camera: Camera) -> (Camera, Screen) {
    let canvas = canvas();
    canvas.set_width(width);
    canvas.set_height(height);

    let screen = Screen::new(width, height);
    let camera_properties =
        CameraProperties::inherit(camera.properties().to_owned(), width as f64 / height as f64);
    let camera = Camera::inherit(camera, camera_properties);

    (camera, screen)
}

fn get_display_size() -> (u32, u32) {
    let width = window().inner_width().unwrap().as_f64().unwrap() as u32;
    let height = window().inner_height().unwrap().as_f64().unwrap() as u32;
    (width, height)
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
