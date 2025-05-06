use simple_3d_core::{
    load_obj,
    types::{pixel::Pixel, textures::Image},
};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageBitmap};

mod interface;

pub(crate) fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub(crate) fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

pub(crate) fn canvas() -> HtmlCanvasElement {
    document()
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap()
}

pub(crate) fn context() -> CanvasRenderingContext2d {
    canvas()
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap()
}

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    let image = load_and_process_image("./assets/crate.jpg").await;
    let mesh = load_obj(&fetch_binary_data("./assets/cube.obj").await);

    simple_3d_core::init::<interface::WasmInterface>(
        Box::new([mesh]),
        Box::new([("crate.jpg".to_string(), image)]),
    );

    Ok(())
}

async fn load_and_process_image(url: &str) -> Image {
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

    let data = image_data.data().to_vec();
    let data = data
        .chunks_exact(4)
        .map(|chunk| Pixel(chunk[0], chunk[1], chunk[2], chunk[3]))
        .collect::<Box<[Pixel]>>();

    Image::load(width, height, &data)
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

async fn fetch_binary_data(url: &str) -> Box<[u8]> {
    let resp_value = window().fetch_with_str(url);
    let resp = JsFuture::from(resp_value)
        .await
        .expect("Failed to fetch data")
        .dyn_into::<web_sys::Response>()
        .expect("Failed to convert to Response");

    if !resp.ok() {
        panic!("Failed to fetch data");
    }

    let blob_promise = resp.blob().expect("Failed to get blob");
    let blob = JsFuture::from(blob_promise)
        .await
        .expect("Failed to convert to Blob")
        .dyn_into::<web_sys::Blob>()
        .expect("Failed to convert to Blob");

    let array_buffer_promise = blob.array_buffer();
    let array_buffer = JsFuture::from(array_buffer_promise)
        .await
        .expect("Failed to get ArrayBuffer");

    let uint8_array = web_sys::js_sys::Uint8Array::new(&array_buffer);

    let mut vec = vec![0; uint8_array.length() as usize];
    uint8_array.copy_to(&mut vec[..]);

    vec.into()
}
