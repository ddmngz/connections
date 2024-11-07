use super::board::Board;
use super::board::SelectState;
use super::color::Color;
use super::puzzle::Card;
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

        Self {
            document,
            cards,
            dots,
            submit_button,
            deselect_button,
        }
    }

    pub fn render_cards(&mut self, board: &Board) {
        self.cards = self.document.get_elements_by_class_name("card");
        let card_iter = CollectionVec::new(&self.cards).into_iter().enumerate();
        console::log_1(&format!("{:?} ", self.cards).into());
        for (index, div) in card_iter {
            console::log_1(&format!("index {index} ").into());
            let card = board.get_card(index);
            let div = unsafe { Self::elem_to_div_elem(div) };
            self.render_card(div, card);
        }
    }

    unsafe fn elem_to_div_elem(elem: Element) -> HtmlDivElement {
        let handle: JsValue = elem.into();
        HtmlDivElement::from(handle)
    }

    pub fn rerender_by_handle(&self, card_div: HtmlDivElement, state: SelectState) {
        let card_style = card_div.style();
        let (text_color, background_color) = match state {
            SelectState::Normal => ("black", "var(--connections-light-beige)"),
            SelectState::Selected => ("white", "var(--connections-darker-beige)"),
        };
        card_style
            .set_property("color", text_color)
            .expect("error setting color");
        card_style
            .set_property("background-color", background_color)
            .expect("error setting bg color");
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

    pub fn render_card(&self, card_div: HtmlDivElement, card: Card) {
        // fix this later
        card_div.set_text_content(Some(card.word));
        let card_style = card_div.style();
        let text_color = card.text_color();
        let background_color = card.background_color();
        card_style
            .set_property("color", text_color)
            .expect("error setting color");
        card_style
            .set_property("background-color", background_color)
            .expect("error setting bg color");
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

        //collection[0].set_text_content(Some("0"));
        //collection[1].set_text_content(Some("1"));
        //collection[2].set_text_content(Some("2"));
        //collection[3].set_text_content(Some("3"));
        /*
        collection[0].remove();
        collection[1].remove();
        collection[2].remove();
        */
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
