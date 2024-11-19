use web_sys::{
    console, Clipboard, Document, DomTokenList, Element, HtmlCollection, HtmlDialogElement,
    HtmlDivElement, HtmlElement, Url, UrlSearchParams, Window,
};

pub struct Cards {
    inner: HtmlCollection,
}

impl Cards {
    fn new_with_doc() -> Option<Self> {
        todo!()
    }
    fn new() -> Option<Self> {
        todo!()
    }

    fn toggle() {}
}

pub struct Selection {
    inner: HtmlCollection,
}

impl Selection {
    fn shake(&self) {}

    fn jump(&self) {}
}

struct Card {
    inner: HtmlDivElement,
}
