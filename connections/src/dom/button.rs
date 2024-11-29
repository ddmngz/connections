use crate::dom::element_ops;
use element_ops::DomError;
use js_sys::Function;
use strum::AsRefStr;
use strum::EnumIter;
use strum::VariantArray;
use thiserror::Error;

use web_sys::{Document, HtmlDivElement, Window};

#[derive(AsRefStr, EnumIter, Clone, Copy, VariantArray)]
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
    #[strum(serialize = "new-puzzle")]
    NewPuzzle,
    #[strum(serialize = "edit-me")]
    EditMe,
    #[strum(serialize = "see-board")]
    SeeBoard,
}

#[derive(Clone)]
pub struct Button {
    inner: HtmlDivElement,
    callback: Option<Function>,
}

#[derive(Debug, Error)]
pub enum ButtonError {
    #[error("expected 8 buttons, got {0}")]
    Miscount(usize),
    #[error(transparent)]
    Dom(#[from] DomError),
}

impl Button {
    pub fn disable(&self) {
        let _ = self.inner.class_list().add_1("hidden");
        self.deregister();
    }

    pub fn enable(&self) {
        let _ = self.inner.class_list().remove_1("hidden");
        self.reregister();
    }

    pub fn new(document: &Document, id: ButtonId) -> Result<Self, DomError> {
        let button = element_ops::new(document, id)?;
        Ok(Self {
            inner: button,
            callback: None,
        })
    }

    pub fn register(&mut self, function: Function) {
        let _ = self
            .inner
            .add_event_listener_with_callback("click", &function);
        self.callback = Some(function);
    }

    pub fn reregister(&self) -> bool {
        if let Some(function) = &self.callback {
            let _ = self
                .inner
                .add_event_listener_with_callback("click", function);
            true
        } else {
            false
        }
    }

    pub fn deregister(&self) {
        if let Some(function) = &self.callback {
            let _ = self
                .inner
                .remove_event_listener_with_callback("click", function);
        }
    }
}
