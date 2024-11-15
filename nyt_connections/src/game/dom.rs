pub mod old_dom;
use crate::game::puzzle::PuzzleKey;
use crate::game::GameState;
use crate::game::TranscodingError;
use std::collections::HashSet;
use std::mem::MaybeUninit;
use web_sys::HtmlDivElement;
//static mut

static mut GAME_STATE: Option<GameState> = None;

fn main() {
    let code = get_code();
    init_game(&code);
}

fn get_code() -> String {
    todo!();
}

fn init_game(code: &str) -> Result<(), TranscodingError> {
    let state = GameState::from_code(code)?;
    unsafe { GAME_STATE = Some(state) };
    // add event listener for each card which uses a callback that references GAME_STATE
    todo!();
    // init state
    // setup cards with listener that owns board
    // setup Submit with listener that owns
}

pub enum ConnectionsError {
    NotInit,
}

fn game_state() -> Result<&'static GameState, ConnectionsError> {
    unsafe { GAME_STATE.as_ref() }.ok_or(ConnectionsError::NotInit)
}

fn click_card(_card: HtmlDivElement, _card_id: usize) {
    let _state = unsafe { GAME_STATE.as_ref() };
    let Some(_state) = _state else {
        return;
    };
    // do some shit
}

// get code from url and then based on url either use default code or url code
//
// Show Document OnLoad
//
// main with code:
//  setup cards:
//      get all the cards and then add a listener that on click calls state.select(card, card_key)

// For Init Buttons: Ideally we want to not go from here to game back to dom so we can figure that
// out

// init buttons
// Set Event Listeners for:
// Submit
// Shuffle
// Deselect
// Try_again
// Share
// Back
// Edit_me
//
//
//
