#![recursion_limit = "256"]
extern crate console_error_panic_hook;

mod app;
mod components;
mod data;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    let children = body.children();
    let mounting_div = children.named_item("yewapp").expect("missing element with 'yewapp' id");

    yew::App::<app::App>::new().mount(mounting_div);

    Ok(())
}
