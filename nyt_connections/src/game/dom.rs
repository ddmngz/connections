mod button;
mod cards;

mod callbacks;
mod element_ops;
mod misc_objects;
use crate::game::puzzle::PuzzleKey;
use crate::game::Board;
use crate::game::GameFailiure;
use crate::game::GameState;
use crate::game::SelectionSuccess;
use crate::game::TranscodingError;
use std::collections::HashSet;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ops::Range;
use std::sync::LazyLock;
use std::sync::RwLock;
use thiserror::Error;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

use wasm_bindgen_futures::future_to_promise;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    console, Clipboard, Document, DomTokenList, Element, HtmlCollection, HtmlDialogElement,
    HtmlDivElement, HtmlElement, Url, UrlSearchParams, Window,
};

static GAME_STATE: LazyLock<RwLock<GameState>> = LazyLock::new(|| RwLock::new(GameState::empty()));

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

pub fn main() {
    if let Err(e) = init_state() {
        show_error(e);
    }
    if init_site().is_err() {
        show_error(PopUp::Dom);
    }
}

fn init_site() -> Result<(), ()> {
    let window = web_sys::window().ok_or(())?;
    let document = window.document().ok_or(())?;

    let submit_button = document.get_element_by_id("submit").unwrap();
    let deselect_button = document.get_element_by_id("deselect").unwrap();

    setup_onload(&document)?;
    setup_cards(
        &document,
        submit_button.class_list(),
        deselect_button.class_list(),
    );
    /*
    button: HtmlElement,
    already_guessed: HtmlDialogElement,
    one_away: HtmlDialogElement,
    won_text: DomTokenList,
    lost_text: DomTokenList,
    end_screen: HtmlDialogElement,
    selection: HtmlCollection,
    */

    setup_submit(submit_button.dyn_into().unwrap(), &document)?;
    /*
    setup_deselect(deselect_button)?;
    setup_try_again()?;
    setup_share()?;
    setup_back()?;
    setup_edit_me()?;
    */
    Ok(())
}

fn setup_onload(document: &Document) -> Result<(), ()> {
    let element = document.document_element().ok_or(())?;
    let closure: Closure<dyn FnMut()> = Closure::new(move || {
        element.remove_attribute("hidden").unwrap();
    });
    document.set_onload(Some(closure.as_ref().unchecked_ref()));
    Ok(())
}

// i can make select return the number of selections
fn setup_cards(document: &Document, submit_class: DomTokenList, deselect_class: DomTokenList) {
    let cards = HtmlCollectionVec::new(&document.get_elements_by_class_name("card"));
    for (dom_index, card) in cards.into_iter().enumerate() {
        setup_card(
            card.dyn_into().unwrap(),
            dom_index,
            deselect_class.clone(),
            submit_class.clone(),
        )
    }
}

fn on_card_click(
    card_class: &DomTokenList,
    dom_index: &usize,
    deselect_class: &DomTokenList,
    submit_class: &DomTokenList,
) {
    let Ok(selection_len) = GAME_STATE.write().unwrap().select(*dom_index) else {
        return;
    };
    match selection_len {
        0 => {
            deselect_class.add_1("hidden").unwrap();
            submit_class.add_1("hidden").unwrap();
        }
        1 => {
            deselect_class.remove_1("hidden").unwrap();
        }
        2 => (),
        3 => {
            submit_class.add_1("hidden").unwrap();
        }
        4 => {
            submit_class.remove_1("hidden").unwrap();
        }
        other => {
            console::log_1(&format!("{}", other).into());
            unreachable!()
        }
    };
    card_class.toggle("selected").unwrap();
}

fn setup_submit(button: HtmlElement, document: &Document) -> Result<(), ()> {
    let already_guessed: HtmlDialogElement = document
        .get_element_by_id("already")
        .unwrap()
        .dyn_into()
        .unwrap();
    let one_away: HtmlDialogElement = document
        .get_element_by_id("away")
        .unwrap()
        .dyn_into()
        .unwrap();

    let end_screen: HtmlDialogElement = document
        .get_element_by_id("endscreen")
        .unwrap()
        .dyn_into()
        .unwrap();

    let selection: HtmlCollection = document.get_elements_by_class_name("selected");

    let won_text = document.get_element_by_id("win").unwrap().class_list();
    let lost_text = document.get_element_by_id("lose").unwrap().class_list();

    let closure = Closure::<dyn FnMut()>::new(move || {
        on_submit(
            &button,
            &already_guessed,
            &one_away,
            &won_text,
            &lost_text,
            &end_screen,
            &selection,
        );
    });

    closure.forget();

    Ok(())
}

fn setup_deselect(button: Element) -> Result<(), ()> {
    todo!();
}

fn setup_try_again() -> Result<(), ()> {
    todo!();
}

fn setup_back() -> Result<(), ()> {
    todo!();
}

fn setup_share() -> Result<(), ()> {
    todo!();
}

fn setup_edit_me() -> Result<(), ()> {
    todo!();
}

async fn jump_selection(selection: &HtmlCollectionVec) {
    let animations = [
        "jump linear .25s",
        "jump2 linear .25s",
        "jump3 linear .25s",
        "jump4 linear .25s",
    ];
    for (card, animation) in selection.iter().zip(animations) {
        card.style().remove_property("animation");
        card.style().set_property("animation", animation);
    }
}

fn shake_selection(selection: HtmlCollectionVec) {
    for card in selection {
        card.style().remove_property("animation");
        card.style().set_property("animation", "shake linear .25s");
    }
}

fn done(end_text: &DomTokenList, end_screen: &HtmlDialogElement) {
    end_screen.show_modal();
    end_text.add_1("enabled");

    /*
     * might need this
    function show_end_buttons(){
        document.getElementById("again").classList.add("enabled");
        document.getElementById("share").classList.add("enabled");
        document.getElementById("back").classList.add("enabled");
        document.getElementById("edit-me").classList.add("enabled");
    }
    */
}

fn on_submit(
    button: &HtmlElement,
    already_guessed: &HtmlDialogElement,
    one_away: &HtmlDialogElement,
    won_text: &DomTokenList,
    lost_text: &DomTokenList,
    end_screen: &HtmlDialogElement,
    selection: &HtmlCollection,
) {
    button.class_list().add_1("hidden").unwrap();
    let selection = HtmlCollectionVec::new(&selection);
    jump_selection(&selection);
    match GAME_STATE.write().unwrap().check_selection() {
        Ok(SelectionSuccess::Won) => done(won_text, end_screen),
        Ok(_) => (),
        Err(GameFailiure::Mismatch | GameFailiure::NotEnough) => shake_selection(selection),
        Err(GameFailiure::OneAway) => animate_modal(one_away),
        Err(GameFailiure::Lost) => done(lost_text, end_screen),
        Err(GameFailiure::AlreadyTried) => animate_modal(already_guessed),
    };
    button.class_list().remove_1("hidden").unwrap();
}

fn on_deselect() {
    GAME_STATE.write().unwrap().clear_selection();
}

fn on_try_again(
    document: &Document,
    submit_class: DomTokenList,
    deselect_class: DomTokenList,
    end_screen: HtmlDialogElement,
) {
    GAME_STATE.write().unwrap().start_over();
    setup_cards(document, submit_class, deselect_class);
    end_screen.close();
}

fn animate_modal(element: &HtmlDialogElement) {
    element.style().remove_property("animation");
    element
        .style()
        .set_property("animation", "show_modal 5s ease-in");
}

async fn on_share(url: Url, clipboard: Clipboard, copied: HtmlDialogElement) {
    let code = GAME_STATE.read().unwrap().puzzle_code();
    url.search_params().set("game", &code);

    let future = JsFuture::from(clipboard.write_text(&url.href()));
    future.await.unwrap();
    animate_modal(&copied);
}

fn on_edit_me(cur_location: &str, window: Window) {
    let code = GAME_STATE.read().unwrap().puzzle_code();
    let url = Url::new_with_base("..", cur_location).unwrap();
    let params = url.search_params();
    params.delete("game");
    params.set("puzzle", &code);
    window.location().assign(&url.href()).unwrap();
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

fn show_error(kind: PopUp) {}

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
