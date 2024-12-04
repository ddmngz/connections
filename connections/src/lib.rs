mod game;
mod pages;
use wasm_bindgen::prelude::*;
mod dom;
use pages::editor_page;
use pages::game_page;

//use web_sys::console;

fn start() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn start_editor(code: Option<String>) {
    match code {
        Some(code) => editor_page::setup(Some(&code)),
        None => editor_page::setup(None),
    }
}

#[wasm_bindgen]
pub fn start_game() {
    game_page::setup();
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
