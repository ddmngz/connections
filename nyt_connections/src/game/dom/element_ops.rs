pub mod collection_vec;
pub use collection_vec::CollectionVec;
use thiserror::Error;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

use web_sys::js_sys::Function;
use web_sys::Document;
use web_sys::Element;

pub fn new_element<T: JsCast>(id: impl AsRef<str>) -> Result<T, DomError> {
    let document = (|| {
        let window = web_sys::window()?;
        window.document()
    })()
    .ok_or(DomError::NoElem)?;
    new_with_doc(&document, id)
}

fn get_elem(document: &Document, id: impl AsRef<str>) -> Result<Element, DomError> {
    document
        .get_element_by_id(id.as_ref())
        .ok_or(DomError::NoElem)
}

pub fn new_with_doc<T: JsCast>(document: &Document, id: impl AsRef<str>) -> Result<T, DomError> {
    let element = get_elem(&document, &id)?;
    element.dyn_into().map_err(|_| {
        let id = id.as_ref().into();
        DomError::Conversion(id)
    })
}

pub fn register_callback<T: AsRef<Element>, U: Fn() + 'static>(elem: &T, callback: U) {
    let closure = Closure::<dyn Fn()>::new(callback);
    let js_fn: Function = closure.into_js_value().into();
    let _ = elem
        .as_ref()
        .add_event_listener_with_callback("click", &js_fn);
}

pub fn deregister_callback<T: AsRef<Element>, U: Fn() + 'static>(elem: &T, callback: U) {
    let closure = Closure::<dyn Fn()>::new(callback);
    let js_fn: Function = closure.into_js_value().into();
    let _ = elem
        .as_ref()
        .remove_event_listener_with_callback("click", &js_fn);
}

#[derive(Error, Debug)]
pub enum DomError {
    #[error("Element Not Found")]
    NoElem,
    #[error("Error in conversion of element {0}")]
    Conversion(String),
}
