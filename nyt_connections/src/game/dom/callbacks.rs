use crate::dom::button::ButtonId;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::js_sys::Function;
use web_sys::{
    console, Clipboard, Document, DomTokenList, Element, HtmlCollection, HtmlDialogElement,
    HtmlDivElement, HtmlElement, Url, UrlSearchParams, Window,
};

pub fn card_callback(document: &Document, index: usize, card: HtmlDivElement) -> Function {
    /*
    button: &HtmlElement,
    already_guessed: &HtmlDialogElement,
    one_away: &HtmlDialogElement,
    won_text: &DomTokenList,
    lost_text: &DomTokenList,
    end_screen: &HtmlDialogElement,
    selection: &HtmlCollection,
    */

    // get card env
    // get create closure
    todo!()
}

pub fn button_callback(id: &ButtonId) -> Function {
    // basic button env
    // get create closure
    todo!();
}

fn to_function(closure: impl FnMut() + 'static) -> Function {
    let closure = Closure::new(closure);
    closure.into_js_value().into()
}

use crate::dom::element_ops::collection_vec::CollectionVec;
use crate::game::GameFailiure;
use crate::game::SelectionSuccess;
use wasm_bindgen_futures::JsFuture;

use crate::dom::GAME_STATE;
/*
pub enum ButtonId {
    #[strum(serialize = "shuffle")]
    Shuffle,
    #[strum(serialize = "submit")]
    Submit,
    #[strum(serialize = "deselect")]
    DeselectAll,
    #[strum(serialize = "again")]
    TryAgain,
    #[strum(serialize = "share")]
    Share,
    #[strum(serialize = "back")]
    Back,
    #[strum(serialize = "edit-me")]
    EditMe,
    #[strum(serialize = "see-board")]
    SeeBoard,
}
*/

fn submit(
    button: &HtmlElement,
    already_guessed: &HtmlDialogElement,
    one_away: &HtmlDialogElement,
    won_text: &DomTokenList,
    lost_text: &DomTokenList,
    end_screen: &HtmlDialogElement,
    selection: &HtmlCollection,
) {
    button.class_list().add_1("hidden").unwrap();
    let selection = CollectionVec::<HtmlDivElement>::new(&selection);
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

fn shuffle() {}

fn see_board() {}

fn deselect() {
    GAME_STATE.write().unwrap().clear_selection();
}

fn try_again(
    document: &Document,
    submit_class: DomTokenList,
    deselect_class: DomTokenList,
    end_screen: HtmlDialogElement,
) {
    GAME_STATE.write().unwrap().start_over();
    //setup_cards(document, submit_class, deselect_class);
    end_screen.close();
}

async fn share(url: Url, clipboard: Clipboard, copied: HtmlDialogElement) {
    let code = GAME_STATE.read().unwrap().puzzle_code();
    url.search_params().set("game", &code);

    let future = JsFuture::from(clipboard.write_text(&url.href()));
    future.await.unwrap();
    animate_modal(&copied);
}

fn edit_me(cur_location: &str, window: Window) {
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

fn card_click(
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
