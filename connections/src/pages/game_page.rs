use crate::console_log;
use crate::dom::button;
use crate::dom::cards;
use crate::dom::element_ops;
use crate::dom::misc_objects::Url;
use crate::game::GameState;
use std::sync::RwLock;
use web_sys::{console, Document, Element, Window};
pub static GAME_STATE: RwLock<GameState> = RwLock::new(GameState::empty());

pub fn setup() {
    let Some(window) = web_sys::window() else {
        show_error(PopUp::Dom);
        return;
    };
    let Some(document) = window.document() else {
        show_error(PopUp::Dom);
        return;
    };

    let code = get_code(&document);
    start_game(code.as_ref().map(|x| x.as_str()), document, window);
}

fn start_game(code: Option<&str>, document: Document, window: Window) {
    console_log!("start game!");
    match code {
        None => default_init_state(),
        Some(code) => {
            if let Err(e) = init_state(code) {
                show_error(e);
            };
        }
    }

    if init_site(document, window).is_err() {
        show_error(PopUp::Dom);
    }
}

use web_sys::HtmlDivElement;
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

pub fn init_state(code: &str) -> Result<(), PopUp> {
    match GameState::from_code(code) {
        Ok(state) => {
            console::log_1(&format!("{:?}", state).into());
            *GAME_STATE.write().unwrap() = state;
            Ok(())
        }
        Err(_) => Err(PopUp::Decoding),
    }
}

fn default_init_state() {
    *GAME_STATE.write().unwrap() = GameState::default();
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
    let _cards = cards::init_cards(document, &GAME_STATE.read().unwrap()).unwrap();
    let _buttons = button::init_buttons(document, window).unwrap();
}

#[derive(Debug)]
pub enum PopUp {
    Dom,
    Decoding,
    Network,
}

fn get_code(document: &Document) -> Option<String> {
    let url = Url::new(document);
    url.game_code()
}

fn show_error(_kind: PopUp) {}
