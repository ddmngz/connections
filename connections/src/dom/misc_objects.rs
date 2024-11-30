use super::element_ops;
pub use element_ops::CollectionVec;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::Promise;
use web_sys::Document;
use web_sys::HtmlSpanElement;
use web_sys::Window;

#[allow(unused_imports)]
use crate::dom::console_log;

#[derive(Clone)]
pub struct Url(web_sys::Url);

#[derive(Clone)]
pub struct Clipboard(web_sys::Clipboard);

impl Url {
    pub fn new(document: &Document) -> Self {
        let url = document.url().unwrap();
        let raw_url = web_sys::Url::new(&url).unwrap();
        Self(raw_url)
    }

    pub fn parent(&self) -> Self {
        let url = self.to_string();
        let raw_url = web_sys::Url::new_with_base("..", &url).unwrap();
        Self(raw_url)
    }

    pub fn game_code(&self) -> Option<String> {
        self.0.search_params().get("game")
    }

    pub fn puzzle_code(&self) -> Option<String> {
        self.0.search_params().get("puzzle")
    }

    pub fn set_game(&mut self, code: &str) {
        self.0.search_params().set("game", code);
    }

    pub fn remove_game(&mut self) {
        self.0.search_params().delete("game");
    }

    pub fn set_puzzle(&mut self, code: &str) {
        self.0.search_params().set("puzzle", code);
    }

    pub fn remove_puzzle(&mut self) {
        self.0.search_params().delete("puzzle");
    }
}

use std::fmt;
impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.0.href())
    }
}
impl Clipboard {
    pub fn new(window: &Window) -> Self {
        Self(window.navigator().clipboard())
    }

    pub fn copy_raw(&self, string: &str) -> Promise {
        self.0.write_text(string)
    }

    pub async fn copy_async(&self, string: &str) -> JsFuture {
        JsFuture::from(self.copy_raw(string))
    }
}

pub mod dots {
    use super::*;

    use web_sys::HtmlCollection;

    #[derive(Clone)]
    pub struct Dots {
        dots: Vec<Dot>,
        handle: HtmlCollection,
    }

    impl Dots {
        pub fn new(document: &Document) -> Self {
            let handle = document.get_elements_by_class_name("dot");
            let dots: Vec<Dot> = CollectionVec::<HtmlSpanElement>::new(&handle)
                .into_iter()
                .map(Dot)
                .collect();
            assert!(dots.len() == 4);
            Self { dots, handle }
        }

        fn update(&mut self) {
            self.dots = CollectionVec::<HtmlSpanElement>::new(&self.handle)
                .into_iter()
                .map(Dot)
                .collect();
        }

        pub fn hide_one(&mut self) {
            self.update();
            let Some(dot) = self.last() else { return };
            dot.hide()
        }

        fn last(&self) -> Option<&Dot> {
            self.dots
                .iter()
                .rfind(|&dot| dot.state() == DotState::Enabled)
        }

        pub fn reset(&mut self) {
            self.update();
            for dot in &self.dots {
                dot.show();
            }
        }
    }

    #[derive(Clone)]
    struct Dot(HtmlSpanElement);
    impl Dot {
        fn show(&self) {
            let _ = self.0.class_list().remove_1("hidden");
        }

        fn hide(&self) {
            let _ = self.0.class_list().add_1("hidden");
        }

        fn state(&self) -> DotState {
            if self.0.class_list().contains("hidden") {
                DotState::Disabled
            } else {
                DotState::Enabled
            }
        }
    }
    #[derive(PartialEq)]
    enum DotState {
        Enabled,
        Disabled,
    }
}

pub mod pop_up {
    use super::element_ops;
    use super::element_ops::AnimationType;
    use element_ops::DomError;
    use strum::AsRefStr;
    use web_sys::Document;
    use web_sys::HtmlDialogElement;
    #[derive(Clone)]
    pub struct PopUp(HtmlDialogElement);

    #[derive(AsRefStr)]
    pub enum PopUpId {
        #[strum(serialize = "copied")]
        CopyToClipboard,
        #[strum(serialize = "away")]
        OneAway,
        #[strum(serialize = "already")]
        AlreadyGuessed,
    }

    impl PopUp {
        pub fn new(document: &Document, id: PopUpId) -> Result<Self, DomError> {
            let popup = element_ops::new::<HtmlDialogElement>(document, id.as_ref())?;
            Ok(Self(popup))
        }

        pub async fn pop_up(&self) {
            self.0.show();
            let _ = element_ops::animate_later(&self.0, AnimationType::PopUp).await;
            self.0.close();
        }

        pub async fn slide_in(&self) {
            self.0.show();
            let _ = element_ops::animate_later(&self.0, AnimationType::SlideIn).await;
            self.0.close();
        }
    }
}

pub mod end_screen {
    use super::element_ops;
    use web_sys::Document;
    use web_sys::DomTokenList;
    use web_sys::HtmlDialogElement;
    use web_sys::HtmlDivElement;

    #[derive(Clone)]
    pub struct EndScreen {
        modal: HtmlDialogElement,
        win_div: DomTokenList,
        lose_div: DomTokenList,
    }

    #[derive(Copy, Clone)]
    pub enum EndState {
        Win,
        Lost,
    }

    impl EndScreen {
        pub fn new(document: &Document) -> Result<Self, element_ops::DomError> {
            let modal: HtmlDialogElement = element_ops::new(document, "endscreen")?;
            let win_div = element_ops::new::<HtmlDivElement>(document, "win")?.class_list();
            let lose_div = element_ops::new::<HtmlDivElement>(document, "lose")?.class_list();
            Ok(Self {
                modal,
                win_div,
                lose_div,
            })
        }

        pub fn show(&self, state: EndState) {
            let _ = self.modal.show_modal();
            let shown_div = match state {
                EndState::Win => &self.win_div,
                EndState::Lost => &self.lose_div,
            };
            let _ = shown_div.add_1("enabled");
        }

        pub fn close(&self) {
            self.modal.close();
        }
    }
}
