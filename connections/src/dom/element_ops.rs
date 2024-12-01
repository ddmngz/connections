pub mod collection_vec;
pub use collection_vec::CollectionVec;
pub mod animations;

pub use animations::{
    animate, animate_in_background, animate_later, animate_then, animate_with_timeout,
    AnimationType,
};

use thiserror::Error;
use wasm_bindgen::JsCast;

use js_sys::Object;
use web_sys::Document;
use web_sys::Element;
use web_sys::HtmlCollection;
use web_sys::HtmlTemplateElement;

use std::time::Duration;

fn get_elem(document: &Document, id: impl AsRef<str>) -> Result<Element, DomError> {
    document
        .get_element_by_id(id.as_ref())
        .ok_or(DomError::NoElem)
}

#[allow(dead_code)]
fn get_collection(document: &Document, id: impl AsRef<str>) -> HtmlCollection {
    let collection = document.get_elements_by_class_name(id.as_ref());
    assert!(
        collection.length() > 0,
        "collection with id {} is empty",
        id.as_ref()
    );
    collection
}

pub fn new<T: JsCast>(document: &Document, id: impl AsRef<str>) -> Result<T, DomError> {
    let element = get_elem(document, &id)?;
    element.dyn_into().map_err(|_| {
        let id = id.as_ref().into();
        DomError::Conversion(id)
    })
}

pub enum CustomElem {
    Board,
    Game,
}

impl AsRef<str> for CustomElem {
    fn as_ref(&self) -> &'static str {
        match self {
            Self::Board => "board-template",
            Self::Game => "connections-game",
        }
    }
}
use web_sys::Node;
pub fn create(elem: CustomElem, document: &Document) -> Node {
    let template: HtmlTemplateElement = new(document, &elem).unwrap();
    template.content().into()
}

#[derive(Error, Debug)]
pub enum DomError {
    #[error("Element Not Found")]
    NoElem,
    #[error("Error in conversion of element {0}")]
    Conversion(String),
}
