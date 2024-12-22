use super::color::Color;
use super::puzzle::PuzzleRef;
use super::ConnectionPuzzle;
use crate::console_log;
use crate::game::ConnectionSet;
use rand::prelude::SliceRandom;
use std::mem::MaybeUninit;
use std::ops::Index;
use wasm_bindgen::prelude::*;
#[allow(unused_imports)]
use web_sys::console;

#[derive(Debug)]
pub struct Board {
    pub selection: Selection,
    matched_cards: MatchedCards,
    puzzle: ConnectionPuzzle,
    order: [PuzzleRef; 16],
}

impl Board {
    pub fn get(&self, index: usize) -> Card {
        let card = self.order[index];
        Card {
            color: card.color(),
            word: self.card_word(card),
            theme: self.card_theme(card),
            state: self.card_state(card),
        }
    }

    pub fn get_word(&self, index: usize) -> &str {
        let card = self.order[index];
        self.card_word(card)
    }

    pub fn set(&self, color: Color) -> &ConnectionSet {
        self.puzzle.by_color(color)
    }
    pub fn reset(&mut self) {
        self.selection.clear();
        self.matched_cards.clear();
        self.shuffle();
    }

    pub fn select(&mut self, index: usize) -> Result<usize, SelectionFailiure> {
        let reference = self.order[index];
        if self.matched_cards.contains(reference.color()) {
            CardState::Matched
        } else {
            self.selection.toggle(reference)?.into()
        };
        Ok(self.selection.len())
    }

    pub fn deselect_all(&mut self) {
        self.selection.clear()
    }

    pub const fn empty() -> Self {
        let puzzle = ConnectionPuzzle::empty();
        let selection = Selection::new();
        let matched_cards = MatchedCards::new();
        let order = PuzzleRef::new_set();
        Self {
            puzzle,
            selection,
            matched_cards,
            order,
        }
    }

    pub fn encode(&self) -> String {
        self.puzzle.encode()
    }

    pub fn test_selection(&mut self) -> Result<Color, SelectionFailiure> {
        if self.selection.len() != 4 {
            return Err(SelectionFailiure::NotEnough);
        }
        let color = self.selection[0].color();

        let matches = self.selection.iter().filter(|x| x.color() == color).count();
        match matches {
            4 => {
                self.move_matched();
                self.matched_cards.mark_match(color);
                self.selection.clear();
                Ok(color)
            }
            3 => Err(SelectionFailiure::OneAway),
            _ => Err(SelectionFailiure::Mismatch),
        }
    }

    fn move_matched(&mut self) {
        console_log!("move matched");
        let mut top_of_board = self.matched_cards.num_matched() * 4;
        for reference in self.selection.iter() {
            let index = self.order.iter().position(|&x| x == reference).unwrap();
            self.order.swap(top_of_board, index);
            top_of_board += 1;
        }
    }

    pub fn shuffle(&mut self) {
        let starting_point = self.matched_cards.num_matched() * 4;
        let mut rng = rand::thread_rng();
        self.order[starting_point..].shuffle(&mut rng);
    }

    pub fn new(puzzle: ConnectionPuzzle) -> Self {
        let selection = Selection::new();
        let matched_cards = MatchedCards::default();
        let mut rng = rand::thread_rng();
        let mut order = PuzzleRef::new_set();
        order.shuffle(&mut rng);
        Self {
            puzzle,
            selection,
            matched_cards,
            order,
        }
    }

    fn card_state(&self, card: PuzzleRef) -> CardState {
        if self.matched_cards.contains(card.color()) {
            CardState::Matched
        } else if self.selection.contains(card) {
            CardState::Selected
        } else {
            CardState::Normal
        }
    }

    fn card_theme(&self, card: PuzzleRef) -> &str {
        self.puzzle.by_color(card.color()).theme_ref()
    }

    fn card_word(&self, card: PuzzleRef) -> &str {
        &self.puzzle[card]
    }

    pub fn matched_set_text(&self, color: Color) -> (&str, String) {
        let set = self.puzzle.by_color(color);
        (set.theme_ref(), set.words())
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        self.order.swap(a, b);
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
#[wasm_bindgen]
pub enum CardState {
    Selected,
    Normal,
    Matched,
}

impl From<SelectState> for CardState {
    fn from(select: SelectState) -> Self {
        match select {
            SelectState::Normal => Self::Normal,
            SelectState::Selected => Self::Selected,
        }
    }
}
use js_sys::JsString;
#[wasm_bindgen]
pub struct OwnedCard {
    #[wasm_bindgen(getter_with_clone)]
    pub word: JsString,
    #[wasm_bindgen(getter_with_clone)]
    pub theme: JsString,
    pub color: Color,
    pub state: CardState,
}

impl From<Card<'_>> for OwnedCard {
    fn from(card: Card<'_>) -> Self {
        Self {
            word: card.word.into(),
            theme: card.theme.into(),
            color: card.color,
            state: card.state,
        }
    }
}

pub struct Card<'a> {
    pub color: Color,
    pub word: &'a str,
    pub theme: &'a str,
    pub state: CardState,
}

impl<'a> Card<'a> {
    pub fn text_color(&self) -> &str {
        match self.state {
            CardState::Selected => "white",
            _ => "black",
        }
    }

    pub fn class_name(&self) -> &str {
        match self.state {
            CardState::Normal => "card",
            CardState::Selected => "selected",
            CardState::Matched => match self.color {
                Color::Yellow => "matched_yellow",
                Color::Green => "matched_green",
                Color::Blue => "matched_blue",
                Color::Purple => "matched_purple",
            },
        }
    }
}

pub enum SelectState {
    Selected,
    Normal,
}

#[wasm_bindgen]
pub enum SelectionFailiure {
    Mismatch,
    NotEnough,
    OneAway,
}

#[derive(Debug, Default)]
struct MatchedCards {
    yellow: bool,
    blue: bool,
    purple: bool,
    green: bool,
}

impl MatchedCards {
    fn num_matched(&self) -> usize {
        let mut num = 0;
        if self.yellow {
            num += 1
        };
        if self.blue {
            num += 1
        };
        if self.purple {
            num += 1
        };
        if self.green {
            num += 1
        };
        num
    }

    fn contains(&self, color: Color) -> bool {
        *self.by_color(color)
    }

    fn clear(&mut self) {
        self.yellow = false;
        self.blue = false;
        self.purple = false;
        self.green = false;
    }

    fn mark_match(&mut self, color: Color) {
        *self.by_color_mut(color) = true;
    }

    fn by_color(&self, color: Color) -> &bool {
        match color {
            Color::Yellow => &self.yellow,
            Color::Blue => &self.blue,
            Color::Purple => &self.purple,
            Color::Green => &self.green,
        }
    }

    fn by_color_mut(&mut self, color: Color) -> &mut bool {
        match color {
            Color::Yellow => &mut self.yellow,
            Color::Blue => &mut self.blue,
            Color::Purple => &mut self.purple,
            Color::Green => &mut self.green,
        }
    }

    const fn new() -> Self {
        Self {
            yellow: false,
            blue: false,
            purple: false,
            green: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Selection {
    selection: [MaybeUninit<PuzzleRef>; 4],
    len: SelectionSize,
}

impl Selection {
    const fn new() -> Self {
        let selection = [MaybeUninit::uninit(); 4];
        let len = SelectionSize::Empty;
        Self { selection, len }
    }

    fn toggle(&mut self, card: PuzzleRef) -> Result<SelectState, SelectionFailiure> {
        let index = self.iter().position(|selected_card| selected_card == card);
        match index {
            Some(index) => {
                console::log_1(&"found, removing".into());
                self.remove(index);
                Ok(SelectState::Normal)
            }
            None => {
                if self.push(card).is_some() {
                    console::log_1(&"not found, adding".into());
                    Ok(SelectState::Selected)
                } else {
                    console::log_1(&"Selection Full".into());
                    Err(SelectionFailiure::NotEnough)
                }
            }
        }
    }

    fn remove(&mut self, index: usize) {
        let end_index = self.len() - 1;
        self.selection.swap(index, end_index);
        console::log_1(&format!("before remove len is {}", self.len()).into());
        self.len = match self.len {
            SelectionSize::Empty => unreachable!(),
            SelectionSize::One => SelectionSize::Empty,
            SelectionSize::Two => SelectionSize::One,
            SelectionSize::Three => SelectionSize::Two,
            SelectionSize::Four => SelectionSize::Three,
        };
        console::log_1(&format!("after remove len is {}", self.len()).into());
    }

    fn push(&mut self, card: PuzzleRef) -> Option<usize> {
        if self.len() == 4 {
            return None;
        }

        self.selection[self.len()].write(card);
        match self.len {
            SelectionSize::Empty => self.len = SelectionSize::One,
            SelectionSize::One => self.len = SelectionSize::Two,
            SelectionSize::Two => self.len = SelectionSize::Three,
            SelectionSize::Three => self.len = SelectionSize::Four,
            SelectionSize::Four => panic!("pushed too many to selection len"),
        };
        Some(self.len())
    }

    fn clear(&mut self) {
        self.len = SelectionSize::Empty;
    }

    pub const fn len(&self) -> usize {
        self.len as usize
    }

    pub fn contains(&self, target_card: PuzzleRef) -> bool {
        for card in self.iter() {
            if card == target_card {
                return true;
            }
        }
        false
    }

    pub fn iter(&self) -> impl Iterator<Item = PuzzleRef> + use<'_> {
        let slice = &self.selection[0..self.len()];
        let iter = slice.iter().map(|index| unsafe { index.assume_init() });
        iter
    }
}

impl PartialEq for Selection {
    fn eq(&self, other: &Self) -> bool {
        // kinda inneficient but both arrays are length 4 so it doesnt matter
        for card in self.iter() {
            if !other.contains(card) {
                return false;
            }
        }
        true
    }
}

impl Index<usize> for Selection {
    type Output = PuzzleRef;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < (self.len as usize));
        unsafe { self.selection[index].assume_init_ref() }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(usize)]
enum SelectionSize {
    Empty = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
}

/*
struct SelectedCards{
    cards:[]
}
*/

impl Board {}
