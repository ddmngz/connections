mod button;
mod cards;

mod callbacks;
mod element_ops;
mod misc_objects;
use crate::game::GameState;
use crate::game::TranscodingError;
use std::sync::LazyLock;
use std::sync::RwLock;
use thiserror::Error;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use web_sys::{console, Document, Element, Url, Window};

static GAME_STATE: LazyLock<RwLock<GameState>> = LazyLock::new(|| RwLock::new(GameState::empty()));

macro_rules! console_log {
    ($expr:expr) => (console::log_1(&(AsRef::<str>::as_ref($expr)).into()));
    ($($y:expr),+) => (
        console_log!(
            &format!($($y),+)
        )
    );
}

pub(crate) use console_log;

pub fn main() {
    console_log!("test");
    console_log!("testing format string {}", 4);
    if let Err(e) = init_state() {
        show_error(e);
    }
    console_log!("initialized state");
    if init_site().is_err() {
        show_error(PopUp::Dom);
    }
    console_log!("initialized site");
}

fn init_state() -> Result<(), PopUp> {
    let Ok(code) = get_code() else {
        return Err(PopUp::InvalidCode);
    };
    match GameState::from_code(&code) {
        Ok(state) => {
            console::log_1(&format!("{:?}", state).into());
            *GAME_STATE.write().unwrap() = state;
            Ok(())
        }
        Err(_) => Err(PopUp::Decoding),
    }
}

fn init_site() -> Result<(), ()> {
    let window = web_sys::window().ok_or(())?;
    let document = window.document().ok_or(())?;
    setup_onload(document, window)?;

    Ok(())
}

fn setup_onload(document: Document, window: Window) -> Result<(), ()> {
    let element = document.document_element().ok_or(())?;
    let document_handle = document.clone();
    let window_2 = window.clone();
    let closure: Closure<dyn FnMut()> =
        Closure::new(move || on_load(&document_handle, &element, &window));
    document.set_onload(Some(&closure.into_js_value().into()));
    console_log!("set onload");
    let element = document.document_element().ok_or(())?;
    on_load(&document, &element, &window_2);
    Ok(())
}

fn on_load(document: &Document, document_element: &Element, window: &Window) {
    console_log!(
        "loaded, trying to remove attribute hidden from element {:?}",
        document_element
    );
    document_element.remove_attribute("hidden").unwrap();
    let _cards = cards::init_cards(document, &GAME_STATE.read().unwrap()).unwrap();
    let _buttons = button::init_buttons(document, window.clone()).unwrap();
}

enum PopUp {
    InvalidCode,
    Dom,
    Decoding,
    Other,
}

fn get_code() -> Result<String, ()> {
    let window = web_sys::window().ok_or(())?;
    let document = window.document().ok_or(())?;
    let url = document.url().or(Err(()))?;
    let params = Url::new(&url).or(Err(()))?.search_params();
    params.get("game").ok_or(())
}

fn show_error(_kind: PopUp) {}

#[derive(Error, Debug)]
pub enum InitError {
    #[error(transparent)]
    Transcode(#[from] TranscodingError),
    #[error("missing component")]
    Dom(),
}

pub enum ConnectionsError {
    NotInit,
}
