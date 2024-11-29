mod dom;
mod form;
mod game;
use wasm_bindgen::prelude::*;

//use web_sys::console;

#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();
    dom::main();
}

macro_rules! console_log {
    ($expr:expr) => (web_sys::console::log_1(&(AsRef::<str>::as_ref($expr)).into()));
    ($($y:expr),+) => (
        console_log!(
            &format!($($y),+)
        )
    );
}

pub(crate) use console_log;
