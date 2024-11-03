use rand::prelude::SliceRandom;
use rand::rngs::ThreadRng;
use std::collections::HashSet;

use super::color::Color;
use super::puzzle::Card;
use super::puzzle::PuzzleKey;
use super::ConnectionPuzzle;

pub struct Board {
    pub puzzle: ConnectionPuzzle,
    pub selection: HashSet<PuzzleKey>,
    // the cards that are matched
    // ideally this would be in sets of 4
    pub matched_cards: HashSet<Color>,
    order: [PuzzleKey; 16],
    rng: ThreadRng,
}

pub enum SelectState {
    Selected,
    Normal,
    Matched,
}

pub enum SelectionFailiure {
    Mismatch,
    NotEnough,
    OneAway,
}

impl Board {
    pub fn select(&mut self, index: usize) -> Result<(), ()> {
        let key = self.get_key(index);
        // if it's selected already, get rid of it
        if self.selection.contains(&key) {
            self.selection.remove(&key);
            Ok(())
        }
        // ugly elif, if it's not matched then it's normal, in which case it should be inserted if
        // and only if the selection hashset isn't full
        else if !self.matched_cards.contains(&key.color) {
            if self.selection.len() == 4 {
                Err(())
            } else {
                self.selection.insert(key);
                Ok(())
            }
        } else {
            Err(())
        }
    }

    pub fn test_selection(&self) -> Result<Color, SelectionFailiure> {
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

    fn move_matched(&mut self) {
        let mut matched_position = self.matched_cards.len() * 4;
        //start..end.iter();
        for key in &self.selection {
            let index = self.order.iter().position(|x| x == key).unwrap();
            self.order.swap(index, matched_position);
            matched_position += 1;
        }
        //for i in start..end {}
    }

    pub fn check_selection(&mut self) -> Result<Color, SelectionFailiure> {
        match self.test_selection() {
            Ok(color) => {
                self.move_matched();
                self.matched_cards.insert(color);
                self.selection.clear();
                //do some stuff
                //and then
                Ok(color)
            }
            e => e,
        }
    }

    pub fn shuffle(&mut self) {
        let starting_point = self.matched_cards.len() * 4;
        self.order[starting_point..].shuffle(&mut rand::thread_rng());
    }

    pub fn new(puzzle: ConnectionPuzzle) -> Self {
        let selection = HashSet::new();
        let matched_cards = HashSet::new();
        let mut order = puzzle.all_keys();
        let mut rng = rand::thread_rng();
        order.shuffle(&mut rng);
        Self {
            puzzle,
            selection,
            matched_cards,
            order,
            rng,
        }
    }

    pub fn get_card(&self, index: usize) -> Card {
        let card_key = &self.order[index];
        Card::from_key(card_key, self)
    }

    fn get_key(&self, index: usize) -> PuzzleKey {
        self.order[index]
    }

    pub fn clear_selection(&mut self) {
        self.selection.clear();
    }
}
