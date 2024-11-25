use super::button::Button;
use super::button::ButtonId;
use super::callbacks;
#[allow(unused_imports)]
use crate::dom::console_log;
use crate::game::color::Color;
use crate::game::dom::element_ops;
use crate::game::dom::element_ops::CollectionVec;
use crate::game::GameState;
use std::collections::VecDeque;
use std::rc::Rc;
use std::time::Duration;
#[allow(unused_imports)]
use web_sys::console;

use web_sys::{Document, HtmlCollection, HtmlDivElement};

#[derive(Clone)]
pub struct Cards {
    collection: HtmlCollection,
    cards: VecDeque<Card>,
    document: Document,
}

pub fn init_cards(document: &Document, state: &GameState) -> Option<Cards> {
    let cards = Cards::new(document)?;
    cards.render_text(state);
    Some(cards)
}

impl Cards {
    pub fn new(document: &Document) -> Option<Self> {
        let deselect = Button::new(document, ButtonId::DeselectAll).ok()?;
        let submit = Button::new(document, ButtonId::Submit).ok()?;
        let selection = Selection::new(document);
        let mut cards = Self::init(document)?;

        cards.register_callbacks(deselect, submit, selection);
        Some(cards)
    }

    pub fn add_set(&mut self, color: Color, state: &GameState) {
        self.update_cards_list();
        let set = MatchedSet::new(color, &self.document, state);
        //let cards: [Card; 12] = self.cards[0..12].try_into().unwrap();

        //let (last_card, cards) = shrink_cards::<8, 4>(self.cards);
        for card in self.cards.drain(0..3) {
            card.delete();
        }
        let last_card = self.cards.pop_front().unwrap();
        set.attatch_to_dom(last_card);
        self.render_text(state);
    }

    pub fn new_handle(document: &Document) -> Option<Self> {
        Self::init(document)
    }

    fn init(document: &Document) -> Option<Self> {
        let collection = document.get_elements_by_class_name("card");
        let cards = CollectionVec::new(&collection)
            .into_iter()
            .map(Card)
            .collect::<VecDeque<Card>>();
        Some(Self {
            collection,
            cards,
            document: document.clone(),
        })
    }

    pub fn reset(&mut self) {
        //let cards = Self::init(document)?;
        // need way to represent that the board could be fewer cards
        //self.register_callbacks();
    }

    fn register_callbacks(&mut self, deselect: Button, submit: Button, selection: Selection) {
        self.update_cards_list();
        let deselect = Rc::new(deselect);
        let submit = Rc::new(submit);
        for (index, card) in self.new_cards_list().into_iter().enumerate() {
            let mut selection_handle = selection.clone();
            let card_ref = card.clone();
            let deselect = deselect.clone();
            let submit = submit.clone();
            let closure = move || card.on_click(index, &mut selection_handle, &deselect, &submit);
            let function = callbacks::to_function_mut(closure);
            card_ref.register_callback(&function);
        }
    }

    fn new_cards_list(&mut self) -> VecDeque<Card> {
        CollectionVec::new(&self.collection)
            .into_iter()
            .map(Card)
            .collect()
    }

    fn update_cards_list(&mut self) {
        self.cards = self.new_cards_list();
    }

    pub fn render_text(&self, state: &GameState) {
        let offset = 16 - self.cards.len();
        let offset_msg = offset.to_string();
        console_log!(&offset_msg);
        for (index, card) in self.cards.iter().enumerate() {
            card.update_text(state, index + offset);
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

use element_ops::AnimationType;
impl Selection {
    pub fn new(document: &Document) -> Self {
        let handle = document.get_elements_by_class_name("selected");
        let vec = CollectionVec::new(&handle).into_iter().map(Card).collect();
        Self { handle, vec }
    }

    pub fn update_vec(&mut self) {
        console_log!("updating!");
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

    pub async fn update_jump_later(&mut self) {
        self.update_vec();
        self.jump_later().await;
    }

    pub fn clear(&mut self) {
        self.update_vec();
        for card in self.vec.drain(..) {
            card.toggle_selected();
        }
        self.update_vec();
    }

    pub async fn jump_later(&self) {
        assert_eq!(self.vec.len(), 4);
        self.vec[0].jump().await;
        self.vec[1].jump().await;
        self.vec[2].jump().await;
        self.vec[3].jump_last().await;
    }
}

#[derive(Clone, Debug)]
pub struct Card(HtmlDivElement);

use super::GAME_STATE;
use web_sys::js_sys::Function;
impl Card {
    pub fn toggle_selected(&self) {
        let _ = self.0.class_list().toggle("selected");
    }

    fn delete(self) {
        self.0.remove()
    }

    fn toggle_shuffling(&self) {
        let _ = self.0.class_list().toggle("shuffling");
    }

    pub fn update_text(&self, state: &GameState, index: usize) {
        let card = state.get(index);
        self.0.set_text_content(Some(card.word));
    }

    fn shake(&self) {
        element_ops::animate(&self.0, AnimationType::Shake);
    }

    async fn jump(&self) {
        element_ops::animate_with_timeout(&self.0, AnimationType::Jump, Duration::from_millis(75))
            .await;
    }

    async fn jump_last(&self) {
        element_ops::animate_then(&self.0, AnimationType::Jump).await;
    }

    fn on_click(
        &self,
        index: usize,
        selection: &mut Selection,
        deselect: &Button,
        submit: &Button,
    ) {
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
                selection.update_vec();
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
        self.0
            .add_event_listener_with_callback("click", callback)
            .unwrap();
    }

    fn inner(&self) -> &HtmlDivElement {
        &self.0
    }
}

use wasm_bindgen::JsCast;
use web_sys::HtmlBrElement;
use web_sys::HtmlElement;
use web_sys::Text;
pub struct MatchedSet(HtmlDivElement);
impl MatchedSet {
    fn new(color: Color, document: &Document, state: &GameState) -> Self {
        let (theme, words) = state.matched_set_text(color);
        let new_div: HtmlDivElement = document.create_element("div").unwrap().dyn_into().unwrap();
        let newline: HtmlBrElement = document.create_element("br").unwrap().dyn_into().unwrap();
        let bold_text: HtmlElement = document.create_element("b").unwrap().dyn_into().unwrap();
        bold_text.set_text_content(Some(theme));
        let text: Text = Text::new_with_data(&words).unwrap();
        let _ = new_div.append_with_node_3(&bold_text, &newline, &text);
        let class = new_div.class_list();
        let _ = class.add_2("matched-set", color.as_ref());
        Self(new_div)
    }

    fn attatch_to_dom(&self, replacing: Card) {
        let _ = replacing.inner().replace_with_with_node_1(&self.0);
    }
}
