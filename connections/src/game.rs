mod board;
pub mod color;
mod puzzle;
#[allow(unused_imports)]
use crate::console_log;
use board::Board;
use board::Card;
use board::OwnedCard;
use board::Selection;
use board::SelectionFailiure;
use color::Color;
pub use puzzle::ConnectionPuzzle;
pub use puzzle::ConnectionSet;
use puzzle::TranscodingError;
use wasm_bindgen::prelude::*;

#[derive(Debug)]
#[wasm_bindgen]
pub struct GameState {
    mistakes: u8,
    successes: u8,
    board: Board,
    prev_attempts: Vec<Selection>,
}

#[wasm_bindgen]
impl GameState {
    pub fn puzzle_code(&self) -> String {
        self.board.encode()
    }

    pub fn default() -> Self {
        let puzzle = ConnectionPuzzle::default();
        Self::new(puzzle)
    }

    pub fn select(&mut self, card_id: usize) -> Result<usize, SelectionFailiure> {
        self.board.select(card_id)
    }

    fn record_mistake(&mut self) {
        self.mistakes += 1;
        //self.dom.deactivate_dot();
    }

    pub fn card_text(&self, index: usize) -> String {
        self.board.get_word(index).to_string()
    }

    pub fn check_selection(&mut self) -> Result<JsSelectionSuccess, GameFailiure> {
        use GameFailiure::*;
        use SelectionSuccess::*;

        if self.prev_attempts.contains(&self.board.selection) {
            return Err(AlreadyTried);
        }

        let almost_won = self.successes == 3;

        let almost_lost = self.mistakes == 3;

        match self.board.test_selection() {
            Ok(color) => {
                self.successes += 1;

                if almost_won {
                    Ok(Won(color).into())
                } else {
                    Ok(Matched(color).into())
                }
            }
            Err(SelectionFailiure::Mismatch) => {
                self.prev_attempts.push(self.board.selection.clone());
                self.record_mistake();

                if almost_lost {
                    Err(Lost)
                } else {
                    Err(Mismatch)
                }
            }
            Err(SelectionFailiure::OneAway) => {
                self.prev_attempts.push(self.board.selection.clone());
                self.record_mistake();
                if almost_lost {
                    Err(Lost)
                } else {
                    Err(OneAway)
                }
            }
            Err(SelectionFailiure::NotEnough) => Err(NotEnough),
        }
    }
    pub fn shuffle(&mut self) {
        self.board.shuffle();
    }

    pub fn clear_selection(&mut self) {
        self.board.deselect_all();
        //self.render_cards();
        /*
        self.dom.clear_selections();
        self.dom.disable_deselect();
        self.dom.disable_submit();
        */
    }

    /*
    pub fn get_selection_indices(&self) -> Box<[u32]> {
        //let card_index = card_id - (self.board.matched_cards.len() * 4);
        let selection: Vec<PuzzleKey> = self.board.selection.iter().cloned().collect();
        let mut elems = Vec::new();
        for (pos, key) in self.board.order.iter().enumerate() {
            for selection_key in &selection {
                if *key == *selection_key {
                    let real_index = pos - (self.board.matched_cards.len() * 4);
                    elems.push(real_index as u32);
                }
            }
        }
        elems.into_boxed_slice()
    }
    */

    pub fn from_code(code: &str) -> Result<Self, TranscodingError> {
        let puzzle = ConnectionPuzzle::decode(code)?;
        Ok(Self::new(puzzle))
    }

    pub fn start_over(&mut self) {
        self.mistakes = 0;
        self.successes = 0;
        self.board.reset();
        self.prev_attempts.clear();
        //self.dom.reset();
    }

    pub fn connection_set(&self, color: Color) -> ConnectionSet {
        self.board.set(color).clone()
    }

    pub fn clipboard_copied(&self) {
        console_log!("copied to clipboard");
    }

    pub fn get_owned(&self, index: usize) -> OwnedCard {
        self.board.get(index).into()
    }

    pub fn new(puzzle: ConnectionPuzzle) -> Self {
        let board = Board::new(puzzle);
        let prev_attempts = Vec::new();
        Self {
            mistakes: 0,
            successes: 0,
            board,
            prev_attempts,
        }
    }
}

impl GameState {
    pub const fn empty() -> GameState {
        let board = Board::empty();

        Self {
            board,
            mistakes: 0,
            successes: 0,
            prev_attempts: Vec::new(),
        }
    }

    pub fn get(&self, index: usize) -> Card {
        self.board.get(index)
    }
}

#[repr(u8)]
pub enum SelectionSuccess {
    Won(Color),
    Matched(Color),
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum SelectionSuccessTags {
    Won,
    Matched,
}

#[wasm_bindgen]
pub struct JsSelectionSuccess {
    #[wasm_bindgen(getter_with_clone)]
    pub color: js_sys::JsString,
    pub result: SelectionSuccessTags,
}

impl From<SelectionSuccess> for JsSelectionSuccess {
    fn from(other: SelectionSuccess) -> Self {
        match other {
            SelectionSuccess::Won(color) => Self {
                color: color.into(),
                result: SelectionSuccessTags::Won,
            },
            SelectionSuccess::Matched(color) => Self {
                color: color.into(),
                result: SelectionSuccessTags::Matched,
            },
        }
    }
}

#[wasm_bindgen]
pub enum GameFailiure {
    Mismatch,
    NotEnough,
    OneAway,
    Lost,
    AlreadyTried,
}

impl From<SelectionFailiure> for GameFailiure {
    fn from(failiure: SelectionFailiure) -> Self {
        match failiure {
            SelectionFailiure::Mismatch => Self::Mismatch,
            SelectionFailiure::NotEnough => Self::NotEnough,
            SelectionFailiure::OneAway => Self::OneAway,
        }
    }
}
