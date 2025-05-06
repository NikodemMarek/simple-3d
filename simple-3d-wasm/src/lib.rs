use simple_3d_core::{load_image, load_obj};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

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
    let object = load_obj(&load_binary_asset("cube.obj").await);
    let image = load_image(&load_binary_asset("crate.jpg").await);

    simple_3d_core::init::<interface::WasmInterface>(
        Box::new([object]),
        Box::new([("crate.jpg".to_string(), image)]),
    );

    Ok(())
}

async fn load_binary_asset(name: &str) -> Box<[u8]> {
    fetch_binary_data(format!("./assets/{}", name).as_str()).await
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
