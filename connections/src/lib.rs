mod game;
use wasm_bindgen::prelude::*;

//use web_sys::console;

#[wasm_bindgen(main)]
fn main() {
    console_error_panic_hook::set_once();
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
