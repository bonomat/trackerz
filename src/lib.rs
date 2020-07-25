#![recursion_limit = "256"]
extern crate console_error_panic_hook;

mod app;
mod components;
mod data;

use wasm_bindgen::prelude::*;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}
#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let mounting_div = document.get_element_by_id("trackerz_app").unwrap();
    yew::App::<app::App>::new().mount(mounting_div);

    Ok(())
}
