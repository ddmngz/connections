#[macro_use]
extern crate yoke;

mod form;
mod game;
use game::dom;
use wasm_bindgen::prelude::*;
//use web_sys::console;

#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn start_game() {
    dom::main();
}
