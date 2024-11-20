use super::button::Button;
use super::button::ButtonId;
use super::callbacks;
use crate::game::dom::element_ops::CollectionVec;
use crate::game::GameState;
use std::rc::Rc;
#[allow(unused_imports)]
use web_sys::console;
use web_sys::{Document, HtmlCollection, HtmlDivElement};

#[derive(Clone)]
pub struct Cards {
    collection: HtmlCollection,
    cards: Vec<Card>,
}

pub fn init_cards(document: &Document, state: &GameState) -> Option<Cards> {
    let cards = Cards::new(document)?;
    cards.render_text(state);
    Some(cards)
}

impl Cards {
    pub fn new(document: &Document) -> Option<Self> {
        let deselect = Button::new(document, ButtonId::DeselectAll).ok()?;
        let submit = Button::new(document, ButtonId::DeselectAll).ok()?;
        let mut cards = Self::init(document)?;
        cards.register_callbacks(deselect, submit);
        let deselect = Button::new(document, ButtonId::DeselectAll).ok()?;
        let submit = Button::new(document, ButtonId::DeselectAll).ok()?;

        cards.register_callbacks(deselect, submit);
        Some(cards)
    }

    fn init(document: &Document) -> Option<Self> {
        let collection = document.get_elements_by_class_name("card");
        let cards = CollectionVec::new(&collection)
            .into_iter()
            .map(Card)
            .collect();
        Some(Self { collection, cards })
    }

    pub fn reset(&mut self) {
        //let cards = Self::init(document)?;
        // need way to represent that the board could be fewer cards
        //self.register_callbacks();
    }

    fn register_callbacks(&mut self, deselect: Button, submit: Button) {
        self.update_cards_list();
        let deselect = Rc::new(deselect);
        let submit = Rc::new(submit);
        for (index, card) in self.new_cards_list().into_iter().enumerate() {
            let card_ref = card.clone();
            let deselect = deselect.clone();
            let submit = submit.clone();
            let closure = move || card.on_click(index, &deselect, &submit);
            let function = callbacks::to_function(closure);
            card_ref.register_callback(&function);
        }
    }

    fn new_cards_list(&mut self) -> Vec<Card> {
        CollectionVec::new(&self.collection)
            .into_iter()
            .map(|div| Card(div))
            .collect()
    }

    fn update_cards_list(&mut self) {
        self.cards = CollectionVec::new(&self.collection)
            .into_iter()
            .map(|div| Card(div))
            .collect();
    }

    pub fn render_text(&self, state: &GameState) {
        for (index, card) in self.cards.iter().enumerate() {
            card.update_text(state, index);
        }
    }

    pub fn rerender_on_shuffle(&self, state: &GameState) {
        self.cards.iter().for_each(Card::toggle_shuffling);
        self.render_text(state);
        self.cards.iter().for_each(Card::toggle_shuffling);
    }
}

#[derive(Clone)]
pub struct Selection {
    handle: HtmlCollection,
    vec: Vec<Card>,
}

impl Selection {
    pub fn new(document: &Document) -> Self {
        let handle = document.get_elements_by_class_name("selected");
        let vec = CollectionVec::new(&handle).into_iter().map(Card).collect();
        Self { handle, vec }
    }

    pub fn update_vec(&mut self) {
        self.vec = CollectionVec::new(&self.handle)
            .into_iter()
            .map(Card)
            .collect();
    }

    pub fn shake(&self) {
        self.vec.iter().for_each(Card::shake);
    }

    pub fn update_shake(&mut self) {
        self.update_vec();
        self.shake();
    }

    pub fn update_jump(&mut self) {
        self.update_vec();
        self.jump();
    }

    pub fn add_card(&mut self, card: HtmlDivElement) {
        self.vec.push(Card(card))
    }

    pub fn clear(&mut self) {
        for card in self.vec.drain(..) {
            card.toggle_selected();
        }
        self.update_vec();
    }

    pub fn jump(&self) {
        self.vec.iter().for_each(Card::shake);
    }
}

#[derive(Clone)]
pub struct Card(HtmlDivElement);

use super::GAME_STATE;
use web_sys::js_sys::Function;
impl Card {
    pub fn toggle_selected(&self) {
        self.0.class_list().toggle("selected");
    }

    fn toggle_shuffling(&self) {
        self.0.class_list().toggle("shuffling");
    }

    pub fn update_text(&self, state: &GameState, index: usize) {
        let card = state.get(index);
        self.0.set_text_content(Some(card.word));
    }

    fn shake(&self) {}

    fn jump(&self) {}

    fn on_click(&self, index: usize, deselect: &Button, submit: &Button) {
        let Ok(selection_len) = GAME_STATE.write().unwrap().select(index) else {
            return;
        };
        match selection_len {
            0 => {
                deselect.disable();
                submit.disable();
            }
            1 => {
                deselect.enable();
            }
            2 => (),
            3 => {
                submit.disable();
            }
            4 => {
                submit.enable();
            }
            other => {
                console::log_1(&format!("{}", other).into());
                unreachable!()
            }
        };
        self.toggle_selected();
    }

    fn register_callback(&self, callback: &Function) {
        self.0.add_event_listener_with_callback("click", callback);
    }
}

pub struct MatchedSet(HtmlDivElement);
