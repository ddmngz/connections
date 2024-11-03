mod board;
mod color;
mod puzzle;
use board::Board;
use board::SelectState;
use board::SelectionFailiure;
use puzzle::Card;
use puzzle::ConnectionPuzzle;
use puzzle::PuzzleKey;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;
use web_sys::console;
use web_sys::Element;
use web_sys::HtmlCollection;
use web_sys::HtmlDivElement;

#[wasm_bindgen]
pub struct GameState {
    mistakes: u8,
    successes: u8,
    board: Board,
    prev_attempts: Vec<HashSet<PuzzleKey>>,
}

#[wasm_bindgen]
pub fn start_state() -> GameState {
    GameState::default()
}

#[wasm_bindgen]
impl GameState {
    pub fn render_card(&self, card_div: HtmlDivElement, index: usize) {
        let card = self.get_card(index);
        card_div.set_text_content(Some(card.word));
        let card_class = card.class_name();
        card_div.set_class_name(card_class);
    }

    fn default() -> Self {
        let puzzle = ConnectionPuzzle::default();
        Self::new(puzzle)
    }

    pub fn select(&mut self, index: usize) -> bool {
        let res = self.board.select(index).is_ok();
        console::log_1(&(format!("{:?}", self.board.selection)).into());
        if self.board.selection.is_empty() {
            console::log_1(&"cant select".into());
            self.cant_select();
        } else if self.board.selection.len() == 1 {
            console::log_1(&"can select".into());
            self.can_select();
        }
        res
    }

    fn get_last(collection: HtmlCollection) -> Option<Element> {
        let mut count = 4;
        while count > 0 {
            if let Some(life) = collection.item(count - 1) {
                return Some(life);
            }
            count -= 1;
        }
        None
    }

    fn record_mistake(&mut self) {
        self.mistakes += 1;
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let lives = document.get_elements_by_class_name("dot");
        if let Some(life) = Self::get_last(lives) {
            life.set_class_name("deactivated-dot");
        }
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
        // animate with keyframes here
        self.board.shuffle();
    }

    pub fn clear_selection(&mut self) {
        self.board.clear_selection();
        self.cant_select();
    }
}

impl GameState {
    fn can_select(&self) {
        self.toggle_select_button(true);
    }

    fn cant_select(&self) {
        self.toggle_select_button(false);
    }

    fn toggle_select_button(&self, make_available: bool) {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let new = match make_available {
            true => "button",
            false => "hidden_button",
        };
        let lives = document.get_element_by_id("deselect").unwrap();
        lives.set_class_name(new);
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

    pub fn get_card(&self, index: usize) -> Card {
        self.board.get_card(index)
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
