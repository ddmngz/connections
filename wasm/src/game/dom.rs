use super::board::Board;
use super::board::SelectState;
use super::color::Color;
use super::puzzle::Card;
use super::puzzle::CardState;
use std::ops::Range;

use wasm_bindgen::JsValue;
use web_sys::console;
use web_sys::Document;
use web_sys::Element;
use web_sys::HtmlCollection;
use web_sys::HtmlDivElement;

pub struct Dom {
    document: Document,
    cards: HtmlCollection,
    dots: HtmlCollection,
    selection: HtmlCollection,
    submit_button: Element,
    deselect_button: Element,
}

impl Dom {
    pub fn init() -> Self {
        let window = web_sys::window().expect("no window found");
        let document = window.document().expect("no document found");
        let cards = document.get_elements_by_class_name("card");
        let dots = document.get_elements_by_class_name("dot");
        let submit_button = document.get_element_by_id("submit").unwrap();
        let deselect_button = document.get_element_by_id("deselect").unwrap();
        let selection = document.get_elements_by_class_name("selected");

        Self {
            document,
            cards,
            dots,
            selection,
            submit_button,
            deselect_button,
        }
    }

    pub fn render_cards(&mut self, board: &Board, offset: usize) {
        self.cards = self.document.get_elements_by_class_name("card");
        let card_iter = CollectionVec::new(&self.cards).into_iter().enumerate();
        for (index, div) in card_iter {
            let card = board.get_card(index + offset);
            self.render_card(div, card);
        }
    }

    pub fn enable_submit(&self) {
        set_button(&self.submit_button, ButtonState::Enable);
    }

    pub fn disable_submit(&self) {
        set_button(&self.submit_button, ButtonState::Disable);
    }

    pub fn enable_deselect(&self) {
        set_button(&self.deselect_button, ButtonState::Enable);
    }

    pub fn disable_deselect(&self) {
        set_button(&self.deselect_button, ButtonState::Disable);
    }

    pub fn reset(&mut self) {
        self.reset_dots();
        self.disable_submit();
        self.disable_deselect();
        self.document
            .get_element_by_id("board")
            .unwrap()
            .replace_with_with_node_1(&self.fresh_board())
            .unwrap();
    }

    fn reset_dots(&self) {
        let deactivated_dots = self.document.get_elements_by_class_name("deactivated-dot");
        let deactivated_dots = CollectionVec::new(&deactivated_dots);
        for dot in deactivated_dots {
            dot.set_class_name("dot");
        }
    }

    fn fresh_board(&self) -> Element {
        let board = self.document.create_element("div").unwrap();
        board.set_id("board");
        board.set_class_name("board");
        for _ in 0..16 {
            let new_card = self.document.create_element("div").unwrap();
            new_card.set_class_name("card");
            board.append_with_node_1(&new_card).unwrap();
        }
        board
    }

    pub fn toggle_select(&self, card_div: &Element) {
        card_div
            .class_list()
            .toggle("selected")
            .expect("Error selecting classlist");
    }

    pub fn render_card(&self, card_div: Element, card: Card) {
        if card.state == CardState::Normal {
            card_div
                .class_list()
                .remove_1("selected")
                .expect("removing selected");
        } else {
            card_div
                .class_list()
                .add_1("selected")
                .expect("removing selected");
        }
        // fix this later
        card_div.set_text_content(Some(card.word));
    }

    pub fn clear_selections(&self) {
        for selection in CollectionVec::new(&self.selection) {
            self.toggle_select(&selection);
        }
    }

    pub fn deactivate_dot(&self) {
        let dots = CollectionVec::new(&self.dots);
        if let Some(life) = dots.last() {
            life.set_class_name("deactivated-dot");
        }
    }

    fn create_color_set(&self, color: Color, theme: &str, words: &str) -> Element {
        let color_set = self.document.create_element("div").unwrap();
        let newline = self.document.create_element("br").unwrap();
        let theme_div = self.document.create_element("b").unwrap();
        theme_div.set_text_content(Some(theme));
        let words = self.document.create_text_node(words);
        color_set
            .append_with_node_3(&theme_div, &newline, &words)
            .expect("error adding theme, newline, and words");
        color_set.set_class_name("matched-set");
        color_set.set_id(color.as_ref());
        color_set
    }

    //pub fn update_card_color(&self, card: HtmlDivElement, color: Color) {}

    pub fn render_match(&self, color: Color, theme: &str, words: &str) {
        console::log_1(&"render match start".into());
        let collection = CollectionVec::new(&self.cards);
        collection[0].remove();
        collection[1].remove();
        collection[2].remove();
        let final_div = &collection[3];
        let color_set = self.create_color_set(color, theme, words);
        final_div
            .replace_with_with_node_1(&color_set)
            .expect("error with replace with");
        console::log_1(&"render match end".into());
    }
}

enum ButtonState {
    Enable,
    Disable,
}

fn set_button(button: &Element, state: ButtonState) {
    let new = match state {
        ButtonState::Enable => "button",
        ButtonState::Disable => "hidden_button",
    };
    button.set_class_name(new);
}

struct CollectionVec {
    array: Vec<Element>,
}

impl CollectionVec {
    fn new(collection: &HtmlCollection) -> Self {
        let mut array = Vec::new();
        let mut i = 0;
        while let Some(elem) = collection.get_with_index(i) {
            array.push(elem);
            i += 1;
        }
        Self { array }
    }

    fn last(&self) -> Option<&Element> {
        self.array.last()
    }
}

impl std::ops::Index<usize> for CollectionVec {
    type Output = Element;
    fn index(&self, at: usize) -> &Element {
        &self.array[at]
    }
}

impl std::ops::Index<std::ops::Range<usize>> for CollectionVec {
    type Output = [Element];
    fn index(&self, at: Range<usize>) -> &[Element] {
        &self.array[at]
    }
}

impl IntoIterator for CollectionVec {
    type Item = Element;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.array.into_iter()
    }
}
