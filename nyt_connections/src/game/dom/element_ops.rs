pub mod collection_vec;
pub use collection_vec::CollectionVec;
use thiserror::Error;
use wasm_bindgen::JsCast;

use web_sys::Document;
use web_sys::Element;
use web_sys::HtmlCollection;

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

#[derive(Error, Debug)]
pub enum DomError {
    #[error("Element Not Found")]
    NoElem,
    #[error("Error in conversion of element {0}")]
    Conversion(String),
}
