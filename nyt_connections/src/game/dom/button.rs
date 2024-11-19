use crate::dom::callbacks;
use crate::dom::element_ops;
use element_ops::DomError;
use js_sys::Function;
use std::sync::Mutex;
use strum::AsRefStr;
use strum::EnumIter;
use strum::IntoEnumIterator;
use web_sys::{
    console, Clipboard, Document, DomTokenList, Element, HtmlCollection, HtmlDialogElement,
    HtmlDivElement, HtmlElement, Url, UrlSearchParams, Window,
};

#[derive(AsRefStr, EnumIter, Clone, Copy)]
pub enum ButtonId {
    #[strum(serialize = "shuffle")]
    Shuffle,
    #[strum(serialize = "submit")]
    Submit,
    #[strum(serialize = "deselect")]
    DeselectAll,
    #[strum(serialize = "again")]
    TryAgain,
    #[strum(serialize = "share")]
    Share,
    #[strum(serialize = "back")]
    Back,
    #[strum(serialize = "edit-me")]
    EditMe,
    #[strum(serialize = "see-board")]
    SeeBoard,
}

impl ButtonId {
    fn id(&self) -> &str {
        self.as_ref()
    }
}

fn init_buttons(doc: &Document) -> Result<(), DomError> {
    for id in ButtonId::iter() {
        let button = Button::new_with_doc(doc, id)?;
        button.register_callback();
    }
    Ok(())
}

pub struct Button {
    id: ButtonId,
    button: HtmlDivElement,
}

impl Button {
    fn new_with_doc(document: &Document, id: ButtonId) -> Result<Self, DomError> {
        let button = element_ops::new_with_doc(document, &id)?;
        Ok(Self { button, id })
    }

    fn new(id: ButtonId) -> Result<Self, DomError> {
        let button = element_ops::new_element(&id)?;
        Ok(Self { button, id })
    }

    fn register_callback(&self) {
        let callback = callbacks::button_callback(&self.id);
        let _ = self
            .button
            .add_event_listener_with_callback("click", &callback);
    }

    fn deregister_callback(&self) {
        let callback = callbacks::button_callback(&self.id);
        let _ = self
            .button
            .remove_event_listener_with_callback("click", &callback);
    }
}
