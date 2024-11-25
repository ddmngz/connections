mod button;
mod cards;

mod callbacks;
mod element_ops;
mod misc_objects;
use crate::game::GameState;
use misc_objects::Url;
use std::sync::RwLock;
use web_sys::{console, Document, Element, Window};

static GAME_STATE: RwLock<GameState> = RwLock::new(GameState::empty());

macro_rules! console_log {
    ($expr:expr) => (web_sys::console::log_1(&(AsRef::<str>::as_ref($expr)).into()));
    ($($y:expr),+) => (
        console_log!(
            &format!($($y),+)
        )
    );
}

pub(crate) use console_log;

pub fn main() {
    let Some(window) = web_sys::window() else {
        show_error(PopUp::Dom);
        return;
    };
    let Some(document) = window.document() else {
        show_error(PopUp::Dom);
        return;
    };

    if let Err(e) = init_state(&document) {
        show_error(e);
    }
    console_log!("initialized state");
    if init_site(document, window).is_err() {
        show_error(PopUp::Dom);
    }
    console_log!("initialized site");
}

fn init_state(document: &Document) -> Result<(), PopUp> {
    let Ok(code) = get_code(document) else {
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

fn init_site(document: Document, window: Window) -> Result<(), PopUp> {
    setup_onload(document, window)?;

    Ok(())
}

fn setup_onload(document: Document, window: Window) -> Result<(), PopUp> {
    let element = document.document_element().ok_or(PopUp::Dom)?;
    // onload didn't work so
    on_load(&document, &element, window);
    Ok(())
}

fn on_load(document: &Document, document_element: &Element, window: Window) {
    console_log!(
        "loaded, trying to remove attribute hidden from element {:?}",
        document_element
    );
    document_element.remove_attribute("hidden").unwrap();
    let _cards = cards::init_cards(document, &GAME_STATE.read().unwrap()).unwrap();
    let _buttons = button::init_buttons(document, window).unwrap();
}

enum PopUp {
    InvalidCode,
    Dom,
    Decoding,
}

fn get_code(document: &Document) -> Result<String, PopUp> {
    let url = Url::new(document);
    match (url.game_code(), url.puzzle_code()) {
        (Some(game_code), _) => Ok(game_code),
        (_, Some(puzzle_code)) => Ok(puzzle_code),
        (None, None) => Err(PopUp::InvalidCode),
    }
}

fn show_error(_kind: PopUp) {}
