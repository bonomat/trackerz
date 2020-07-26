#![recursion_limit = "256"]
extern crate console_error_panic_hook;

#[macro_use]
mod macros;
#[macro_use]
mod app;
mod components;
mod data;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let mounting_div = document.get_element_by_id("trackerz_app").unwrap();
    yew::App::<app::App>::new().mount(mounting_div);

    Ok(())
}
