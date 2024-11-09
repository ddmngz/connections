mod board;
pub mod color;
mod dom;
mod puzzle;
use board::Board;
use board::SelectionFailiure;
use color::Color;
use dom::Dom;
use puzzle::Card;
pub use puzzle::ConnectionPuzzle;
use puzzle::PuzzleKey;
use puzzle::TranscodingError;
use std::collections::HashSet;
use std::io::prelude::*;
use wasm_bindgen::prelude::*;
#[allow(unused_imports)]
use web_sys::console;
use web_sys::HtmlDivElement;

#[wasm_bindgen]
pub struct GameState {
    mistakes: u8,
    successes: u8,
    board: Board,
    prev_attempts: Vec<HashSet<PuzzleKey>>,
    dom: Dom,
}

#[wasm_bindgen]
pub fn start_state() -> GameState {
    GameState::default()
}

#[wasm_bindgen]
impl GameState {
    pub fn puzzle_code(&self) {
        let string = self.board.puzzle.encode();
        console::log_1(&string.into());
    }

    pub fn render_cards(&mut self) {
        self.dom
            .render_cards(&self.board, self.board.matched_cards.len() * 4);
    }

    fn default() -> Self {
        let puzzle = ConnectionPuzzle::default();
        Self::new(puzzle)
    }

    pub fn select(&mut self, card: HtmlDivElement, card_id: usize) -> bool {
        let card_index = card_id - (self.board.matched_cards.len() * 4);
        console::log_1(&format!("index {}", card_index).into());
        let Ok(state) = self.board.select(card_id) else {
            return false;
        };
        self.dom.toggle_select(&card);
        //self.dom.rerender_selected_card(index, state);

        if self.board.selection.is_empty() {
            self.dom.disable_deselect();
            self.dom.disable_submit();
        } else {
            // trying to only rerender when necessary
            match self.board.selection.len() {
                1 => self.dom.enable_deselect(),
                3 => self.dom.disable_submit(),
                4 => self.dom.enable_submit(),
                _ => {}
            }
        }
        true
    }

    fn record_mistake(&mut self) {
        self.mistakes += 1;
        self.dom.deactivate_dot();
    }

    pub fn check_selection(&mut self) -> Result<SelectionSuccess, GameFailiure> {
        use GameFailiure::*;
        //use SelectionFailiure::*;
        use SelectionSuccess::*;

        if self.prev_attempts.contains(&self.board.selection) {
            return Err(AlreadyTried);
        }

        let almost_won = self.successes == 3;

        let almost_lost = self.mistakes == 3;

        match self.board.check_selection() {
            Ok(color) => {
                self.board.matched_cards.insert(color);
                self.successes += 1;
                self.dom.disable_deselect();
                self.dom
                    .render_cards(&self.board, (self.board.matched_cards.len() - 1) * 4);
                let (theme, words) = self.match_set_strings(color);
                self.dom.render_match(color, theme, &words);
                //self.dom.render_cards(&self.board);
                if almost_won {
                    Ok(Won)
                } else {
                    Ok(Matched)
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
        self.render_cards();
    }

    pub fn clear_selection(&mut self) {
        self.board.clear_selection();
        //self.render_cards();
        self.dom.clear_selections();
        self.dom.disable_deselect();
        self.dom.disable_submit();
    }

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

    pub fn from_code(code: String) -> Result<Self, TranscodingError> {
        let puzzle = ConnectionPuzzle::decode(&code)?;
        Ok(Self::new(puzzle))
    }

    pub fn start_over(&mut self) {
        self.mistakes = 0;
        self.successes = 0;
        self.board.reset();
        self.prev_attempts.clear();
        self.dom.reset();
        self.render_cards();
    }
}

impl GameState {
    pub fn new(puzzle: ConnectionPuzzle) -> Self {
        let board = Board::new(puzzle);
        let prev_attempts = Vec::new();
        let dom = Dom::init();
        Self {
            mistakes: 0,
            successes: 0,
            board,
            prev_attempts,
            dom,
        }
    }

    pub fn get_card(&self, index: usize) -> Card {
        self.board.get_card(index)
    }

    // string for matched set
    fn match_set_strings(&self, color: Color) -> (&str, String) {
        match color {
            Color::Yellow => (
                self.board.puzzle.yellow().theme(),
                self.board.puzzle.yellow().words(),
            ),
            Color::Blue => (
                self.board.puzzle.blue().theme(),
                self.board.puzzle.blue().words(),
            ),
            Color::Purple => (
                self.board.puzzle.purple().theme(),
                self.board.puzzle.purple().words(),
            ),
            Color::Green => (
                self.board.puzzle.green().theme(),
                self.board.puzzle.green().words(),
            ),
        }
    }
}
#[wasm_bindgen]
#[repr(u8)]
pub enum SelectionSuccess {
    Won,
    Matched,
}

#[wasm_bindgen]
#[repr(u8)]
pub enum GameFailiure {
    Mismatch,
    NotEnough,
    OneAway,
    Lost,
    AlreadyTried,
}
