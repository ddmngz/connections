mod form;
mod game;
use wasm_bindgen::prelude::*;
//use web_sys::console;

#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();
}
