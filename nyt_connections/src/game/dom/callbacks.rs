use crate::dom::element_ops::collection_vec::CollectionVec;
use crate::game::GameFailiure;
use crate::game::SelectionSuccess;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use web_sys::js_sys::Function;
use web_sys::{Document, DomTokenList, HtmlDialogElement, HtmlDivElement, Window};

#[allow(unused_imports)]
use web_sys::console;

use super::button::Button;
use super::cards::Card;
use super::cards::Cards;
use super::cards::Selection;
use super::misc_objects::dots::Dots;
use super::misc_objects::end_screen::EndScreen;
use super::misc_objects::end_screen::EndState;
use super::misc_objects::Clipboard;
use super::misc_objects::Url;
use crate::dom::GAME_STATE;

use super::misc_objects::pop_up::PopUp;
use crate::dom::console_log;

pub fn new_puzzle(window: &Window, url: &mut Url) {
    let mut url = url.parent();
    url.remove_puzzle();
    url.remove_game();
    window.location().assign(&url.to_string()).unwrap();
}

pub fn to_function(closure: impl Fn() + 'static) -> Function {
    let closure: Closure<dyn Fn()> = Closure::new(closure);
    closure.into_js_value().into()
}

pub fn to_function_mut(closure: impl FnMut() + 'static) -> Function {
    let closure: Closure<dyn FnMut()> = Closure::new(closure);
    closure.into_js_value().into()
}

pub fn submit(
    submit_button: &Button,
    already_guessed: &PopUp,
    one_away: &PopUp,
    end_screen: &EndScreen,
    selection: &Selection,
) {
    submit_button.disable();
    selection.jump();
    match GAME_STATE.write().unwrap().check_selection() {
        Ok(SelectionSuccess::Won) => end_screen.show_relaxed(EndState::Win),
        Ok(_) => (),
        Err(GameFailiure::Mismatch | GameFailiure::NotEnough) => selection.shake(),
        Err(GameFailiure::OneAway) => one_away.pop_up(),
        Err(GameFailiure::Lost) => end_screen.show_relaxed(EndState::Lost),
        Err(GameFailiure::AlreadyTried) => already_guessed.pop_up(),
    };
    submit_button.enable();
}

async fn jump_selection(selection: &CollectionVec<HtmlDivElement>) {
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

fn shake_selection(selection: CollectionVec<HtmlDivElement>) {
    for card in selection {
        card.style().remove_property("animation");
        card.style().set_property("animation", "shake linear .25s");
    }
}

fn animate_modal(element: &HtmlDialogElement) {
    element.style().remove_property("animation");
    element
        .style()
        .set_property("animation", "show_modal 5s ease-in");
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

pub fn shuffle(cards: &Cards) {
    let mut game_state = GAME_STATE.write().unwrap();
    game_state.shuffle();
    cards.rerender_on_shuffle(&game_state);
}

pub fn see_board(end_screen: &EndScreen, shuffle: &Button, deselect: &Button, submit: &Button) {
    shuffle.disable();
    deselect.disable();
    submit.disable();
    end_screen.close();
}

pub fn deselect(selection: &mut Selection, button: &Button) {
    GAME_STATE.write().unwrap().clear_selection();
    selection.clear();
    button.disable();
}

pub fn try_again(
    cards: &mut Cards,
    end_screen: &EndScreen,
    dots: &mut Dots,
    submit: &Button,
    deselect: &Button,
) {
    GAME_STATE.write().unwrap().start_over();
    cards.reset();
    dots.reset();
    submit.disable();
    deselect.disable();
    end_screen.close();
}

pub fn share(url: &mut Url, clipboard: &Clipboard, copied: PopUp) {
    let code = GAME_STATE.read().unwrap().puzzle_code();
    url.set_game(&code);
    let new_url = url.to_string();
    let future = clipboard.copy_raw(&new_url);
    let next = move |_: JsValue| copied.pop_up();
    let closure = Closure::<dyn FnMut(JsValue)>::new(next);
    future.then(&closure);
}

pub fn edit_me(window: &Window, cur_url: &mut Url) {
    let code = GAME_STATE.read().unwrap().puzzle_code();
    let mut url = cur_url.parent();
    url.remove_game();
    url.set_puzzle(&code);
    window.location().assign(&url.to_string()).unwrap();
}

enum ErrorPopUp {
    InvalidCode,
    Dom,
    Decoding,
    Other,
}
