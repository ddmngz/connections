use super::color::Color;
use super::puzzle::PuzzleIndex;
use super::ConnectionPuzzle;
use rand::prelude::SliceRandom;
use std::mem::MaybeUninit;
use std::ops::Index;
#[allow(unused_imports)]
use web_sys::console;

#[derive(Debug)]
pub struct Board {
    pub selection: Selection,
    matched_cards: MatchedCards,
    puzzle: ConnectionPuzzle,
    order: [PuzzleIndex; 16],
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
    pub fn reset(&mut self) {
        /*
        self.selection.clear();
        self.matched_cards.clear();
        self.shuffle();
        */
    }
    pub fn select(&mut self, index: usize) -> Result<Card, ()> {
        /*    pub fn select(&mut self, index: usize) -> Result<SelectState, ()> {
            let key = self.get_key(index);
            // if it's selected already, get rid of it
            if self.selection.contains(&key) {
                self.selection.remove(&key);
                Ok(SelectState::Normal)
            }
            // ugly elif, if it's not matched then it's normal, in which case it should be inserted if
            // and only if the selection hashset isn't full
            else if !self.matched_cards.contains(&key.color) {
                if self.selection.len() == 4 {
                    Err(())
                } else {
                    self.selection.insert(key);
                    Ok(SelectState::Selected)
                }
            } else {
                Err(())
            }
        }*/
        todo!()
    }

    pub fn deselect_all(&mut self) {
        self.selection.clear()
    }

    pub const fn empty() -> Self {
        todo!();
    }

    pub fn encode(&self) -> String {
        self.puzzle.encode()
    }

    pub fn test_selection(&mut self) -> Result<Color, SelectionFailiure> {
        // DIFFERENCE FROM BEFORE: Now if there's a match we should add it to our list
        /*
             *    pub fn test_selection(&self) -> Result<Color, SelectionFailiure> {
            if self.selection.len() != 4 {
                return Err(SelectionFailiure::NotEnough);
            }
            let color: Color = self.selection.iter().next().unwrap().color;
            let matches = self.selection.iter().filter(|x| x.color == color).count();
            if matches == 4 {
                Ok(color)
            } else if matches == 3 {
                Err(SelectionFailiure::OneAway)
            } else {
                Err(SelectionFailiure::Mismatch)
            }
        }
             */
        todo!()
    }

    fn check(&self, index: usize) -> CardState {
        /*
        pub fn check_selection(&mut self) -> Result<Color, SelectionFailiure> {
            match self.test_selection() {
                Ok(color) => {
                    self.move_matched();
                    self.matched_cards.insert(color);
                    console::log_1(&format!("length: {}", self.matched_cards.len()).into());
                    self.selection.clear();
                    Ok(color)
                }
                e => e,
            }
        }
            */
        todo!()
    }

    fn move_matched(&mut self) {
        /*
        //start..end.iter();
        let mut top_of_board = self.matched_cards.len() * 4;
        console::log_1(&format!("moving selection into index {top_of_board}").into());
        for key in &self.selection {
            let index = self.order.iter().position(|x| x == key).unwrap();
            self.order.swap(top_of_board, index);
            top_of_board += 1;
        }
        console::log_1(&"move matched done".into());
        */
    }

    pub fn shuffle(&mut self) {
        /*
         * let starting_point = self.matched_cards.len() * 4;
         * let mut rng = rand::thread_rng();
         * self.order[starting_point..].shuffle(&mut rng);
         */
    }

    pub fn new(puzzle: ConnectionPuzzle) -> Self {
        todo!()
        /*
         *  let selection = HashSet::new();
         *  let matched_cards = HashSet::new();
         *  let mut rng = rand::thread_rng();
         *  let mut order = puzzle.all_keys();
         *  order.shuffle(&mut rng);
         *  Self {
         *      puzzle,
         *      selection,
         *      matched_cards,
         *      order,
         *  }
         *
         */
    }

    fn card_state(&self, card: PuzzleIndex) -> CardState {
        if self.matched_cards.contains(card.color()) {
            CardState::Matched
        } else if self.selection.contains(card) {
            CardState::Selected
        } else {
            CardState::Normal
        }
    }

    fn card_theme(&self, card: PuzzleIndex) -> &str {
        self.puzzle.by_color(card.color()).theme()
    }

    fn card_word(&self, card: PuzzleIndex) -> &str {
        &self.puzzle[card]
    }

    pub fn matched_set_strings(&self, color: Color) -> (&str, String) {
        let set = self.puzzle.by_color(color);
        (set.theme(), set.words())
    }
}

#[derive(PartialEq, Eq)]
pub enum CardState {
    Selected,
    Normal,
    Matched,
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

pub enum SelectionFailiure {
    Mismatch,
    NotEnough,
    OneAway,
}

#[derive(Debug)]
struct MatchedCards {
    yellow: bool,
    blue: bool,
    purple: bool,
    green: bool,
}

impl MatchedCards {
    fn contains(&self, color: Color) -> bool {
        match color {
            Color::Yellow => self.yellow,
            Color::Blue => self.blue,
            Color::Purple => self.purple,
            Color::Green => self.green,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Selection {
    selection: [MaybeUninit<PuzzleIndex>; 4],
    len: SelectionSize,
}

impl Selection {
    fn push(&mut self, card: PuzzleIndex) -> Option<usize> {
        match self.len {
            SelectionSize::Empty => self.len = SelectionSize::One,
            SelectionSize::One => self.len = SelectionSize::Two,
            SelectionSize::Two => self.len = SelectionSize::Three,
            SelectionSize::Three => self.len = SelectionSize::Four,
            SelectionSize::Four => return None,
        };
        self.selection[self.len as usize].write(card);
        Some(self.len as usize)
    }

    fn clear(&mut self) {
        self.len = SelectionSize::Empty;
    }

    pub const fn len(&self) -> usize {
        self.len as usize
    }

    pub fn contains(&self, target_card: PuzzleIndex) -> bool {
        for card in self.iter() {
            if card == target_card {
                return true;
            }
        }
        false
    }

    pub fn iter(&self) -> impl Iterator<Item = PuzzleIndex> + use<'_> {
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
    type Output = PuzzleIndex;

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
