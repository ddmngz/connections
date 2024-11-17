pub mod old_dom;
use crate::game::puzzle::PuzzleKey;
use crate::game::Board;
use crate::game::GameState;
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
use web_sys::{
    console, Document, DomTokenList, Element, HtmlCollection, HtmlDivElement, HtmlElement, Url,
    UrlSearchParams, Window,
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
    setup_submit(submit_button)?;
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
        element.remove_attribute("hidden");
    });
    document.set_onload(Some(closure.as_ref().unchecked_ref()));
    Ok(())
}

// i can make select return the number of selections
fn setup_cards(document: &Document, submit_class: DomTokenList, deselect_class: DomTokenList) {
    let cards = CollectionVec::new(&document.get_elements_by_class_name("card"));
    for (dom_index, card) in cards.into_iter().enumerate() {
        setup_card(
            card.dyn_into().unwrap(),
            dom_index,
            deselect_class.clone(),
            submit_class.clone(),
        )
    }
}

fn setup_card(
    card: HtmlDivElement,
    dom_index: usize,
    deselect_class: DomTokenList,
    submit_class: DomTokenList,
) {
    let list = card.class_list();
    let closure: Closure<dyn FnMut()> =
        Closure::new(move || on_card_click(&list, &dom_index, &deselect_class, &submit_class));

    card.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget();
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
        2 => {}
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

fn setup_submit(button: Element) -> Result<(), ()> {
    todo!();
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

fn on_submit() {
    /*
    *
           const classList = submit.classList;
           if(classList.contains("hidden")){
               return
           }
           classList.add("hidden");
               try{
                   await jump_selection();
                   if(state.check_selection() == SelectionSuccess.Won){
                       display_won();
                   }
               }catch (exceptionVar){
               //console.log(GameFailiure[exceptionVar]);
                   switch (exceptionVar){
                       case GameFailiure.Mismatch: // MISMATCH
                           shake_selection();
                           break;
                       case GameFailiure.NotEnough: // NOT ENOUGH
                           shake_selection();
                           break;
                       case GameFailiure.OneAway: //One Away
                           shake_selection();
                           one_away();
                           break;
                       case GameFailiure.Lost:
                           shake_selection();
                           display_lost();
                           break;
                       case GameFailiure.AlreadyTried:
                           already_guessed();
                   }
               }finally{
               submit.classList.remove("hidden");
           }
    */
}

fn on_deselect() {
    /*
            if(deselect.classList.contains("hidden")){
                return
            }
                state.clear_selection();
    */
}

fn on_try_again() {

    /*
    *  state.start_over();
           setup_cards();
               hide_overlay();

    *
    */
}

fn on_share() {
    /*
    *
    *const code = state.puzzle_code();
           const url = new URL(document.location.href);
           // is this secure,,
           url.searchParams.set("game",code);
           await window.navigator.clipboard.writeText(url.href);
           const copied = document.getElementById("copied");
           animate(copied);

    *
    */
}

fn on_edit_me() {
    /*
    *	    const code = state.puzzle_code();
           console.log(code);
           const url = new URL("../",document.location.href);
           url.searchParams.delete("game");
           url.searchParams.set("puzzle",code);
           console.log(url);
               self.window.location.assign(url);

    *
    *
    */
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

/*
fn _site() -> Result<&'static mut GameState, InitError> {
    let code = get_code();
    let state = GameState::from_code(&code)?;
    let state = unsafe { GAME_STATE.set(state) };
    init_dom()?;
    Ok(state)
}
*/

fn init_dom() -> Result<(), InitError> {
    todo!();
}

#[derive(Error, Debug)]
pub enum InitError {
    #[error(transparent)]
    Transcode(#[from] TranscodingError),
    #[error("missing component")]
    Dom(),
}

/*
fn init_game(code: &str) -> Result<(), TranscodingError> {
    let state = GameState::from_code(code)?;
    unsafe { GAME_STATE = Some(state) };
    // add event listener for each card which uses a callback that references GAME_STATE
    todo!();
    // init state
}
*/

pub enum ConnectionsError {
    NotInit,
}

struct CollectionVec {
    array: Vec<Element>,
}

impl CollectionVec {
    fn new(collection: &HtmlCollection) -> Self {
        let mut array = Vec::new();
        let mut i = 0;
        while let Some(elem) = collection.get_with_index(i) {
            array.push(elem);
            i += 1;
        }
        Self { array }
    }

    fn last(&self) -> Option<&Element> {
        self.array.last()
    }
}

impl std::ops::Index<usize> for CollectionVec {
    type Output = Element;
    fn index(&self, at: usize) -> &Element {
        &self.array[at]
    }
}

impl std::ops::Index<Range<usize>> for CollectionVec {
    type Output = [Element];
    fn index(&self, at: Range<usize>) -> &[Element] {
        &self.array[at]
    }
}

impl IntoIterator for CollectionVec {
    type Item = Element;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.array.into_iter()
    }
}
