mod button;
mod cards;

mod callbacks;
mod element_ops;
mod misc_objects;
use crate::form;
use crate::game::GameState;
use misc_objects::Url;
use std::sync::RwLock;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::JsFuture;
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

    let code = get_code(&document);

    match code {
        Code::Game(code) => start_game(&code, document, window),
        Code::PuzzleBuilder => {
            start_puzzle_builder(None);
        }
        Code::PuzzleEditor(code) => {
            start_puzzle_builder(Some(&code));
        }
    };
}

fn start_puzzle_builder(code: Option<&str>) {
    form::setup(code)
}
// NOTE: need to do the redirect
fn start_game(code: &str, document: Document, window: Window) {
    console_log!("start game!");
    load_game(&document).expect("error");
    if let Err(e) = init_state(code) {
        show_error(e);
    }
    if init_site(document, window).is_err() {
        show_error(PopUp::Dom);
    }
}

use web_sys::HtmlDivElement;
use web_sys::HtmlElement;
use web_sys::HtmlTemplateElement;
fn load_game(document: &Document) -> Result<(), PopUp> {
    let template: HtmlTemplateElement = element_ops::new(document, "connections-game")
        .or(Err(PopUp::Dom))
        .unwrap();

    let editor: HtmlDivElement = element_ops::new(document, "game").unwrap();
    editor
        .replace_with_with_node_1(&template.content())
        .or(Err(PopUp::Dom))
        .unwrap();
    Ok(())
}

fn init_state(code: &str) -> Result<(), PopUp> {
    match GameState::from_code(code) {
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

#[derive(Debug)]
enum PopUp {
    Dom,
    Decoding,
    Network,
}

enum Code {
    Game(String),
    PuzzleBuilder,
    PuzzleEditor(String),
}

fn get_code(document: &Document) -> Code {
    let url = Url::new(document);
    match (url.game_code(), url.puzzle_code()) {
        (Some(game_code), _) => Code::Game(game_code),
        (_, Some(puzzle_code)) => Code::PuzzleEditor(puzzle_code),
        (None, None) => Code::PuzzleBuilder,
    }
}

fn show_error(_kind: PopUp) {}
