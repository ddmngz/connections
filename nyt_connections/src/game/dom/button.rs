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

pub fn init_buttons(document: &Document, window: Window) -> Result<[Button; 8], ButtonError> {
    button_builder::build(document, window)
}

#[derive(Clone)]
pub struct Button{
    inner:HtmlDivElement,
    callback:Option<Function>,
};

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
    }

    pub fn enable(&self) {
        let _ = self.inner.class_list().remove_1("hidden");
    }

    pub fn new(document: &Document, id: ButtonId) -> Result<Self, DomError> {
        let button = element_ops::new(document, id)?;
        Ok(Self{inner:button, callback:None})
    }

    pub fn register(&mut self, function: Function) {
        let _ = self.inner.add_event_listener_with_callback("click", &function);
        self.callback = Some(function);
    }

    pub fn reregister(&mut self) -> bool{
        if let Some(function) = &self.callback{
            let _ = self.inner.add_event_listener_with_callback("click", function);
            true
        }else{
            false
        }
    }

    pub fn deregister(&mut self) {
        if let Some(function) = &self.callback{
            self.inner.remove_event_listener_with_callback("click", function);
        }
    }
}

mod button_builder {

    use super::super::{
        cards::Cards,
        cards::Selection,
        misc_objects::dots::Dots,
        misc_objects::{end_screen::EndScreen, pop_up::PopUp, pop_up::PopUpId, Clipboard, Url},
    };

    use super::element_ops::DomError;
    use super::Button;
    use super::ButtonError;
    use super::ButtonId;
    use super::Document;
    use super::Window;
    use crate::dom::callbacks;
    use callbacks::ShareCallback;

    struct Builder {
        submit: Button,
        deselect: Button,
        shuffle: Button,
        already_guessed: PopUp,
        one_away: PopUp,
        copied: PopUp,
        end_screen: EndScreen,
        selection: Selection,
        cards: Cards,
        dots: Dots,
        url: Url,
        clipboard: Clipboard,
        window: Window,
        document: Document,
    }

    pub fn build(document: &Document, window: Window) -> Result<[Button; 8], ButtonError> {
        let builder = Builder::init(document, window)?;
        let mut vec = Vec::<Button>::with_capacity(8);
        builder
            .edit_me(&mut vec)?
            .new_puzzle(&mut vec)?
            .shuffle(&mut vec)?
            .try_again(&mut vec)?
            .share(&mut vec)?
            .submit(&mut vec)?
            .deselect(&mut vec)?
            .see_board(vec)
    }

    impl Builder {
        fn init(document: &Document, window: Window) -> Result<Self, DomError> {
            Ok(Self {
                submit: Button::new(document, ButtonId::Submit)?,
                deselect: Button::new(document, ButtonId::DeselectAll)?,
                shuffle: Button::new(document, ButtonId::Shuffle)?,
                already_guessed: PopUp::new(document, PopUpId::AlreadyGuessed)?,
                one_away: PopUp::new(document, PopUpId::OneAway)?,
                copied: PopUp::new(document, PopUpId::CopyToClipboard)?,
                end_screen: EndScreen::new(document)?,
                selection: Selection::new(document),
                cards: Cards::new_handle(document).unwrap(),
                dots: Dots::new(document),
                url: Url::new(document),
                clipboard: Clipboard::new(&window),
                window,
                document: document.clone(),
            })
        }

        fn edit_me(self, vec: &mut Vec<Button>) -> Result<State2, DomError> {
            let mut edit_me = Button::new(&self.document, ButtonId::EditMe)?;
            let window = self.window.clone();
            let mut url = self.url.clone();
            let callback =
                callbacks::to_function_mut(move || callbacks::edit_me(&window, &mut url));
            edit_me.register(callback);
            vec.push(edit_me);

            Ok(self.next_state())
        }

        fn next_state(self) -> State2 {
            State2 {
                submit: self.submit,
                shuffle: self.shuffle,
                deselect: self.deselect,
                already_guessed: self.already_guessed,
                one_away: self.one_away,
                copied: self.copied,
                end_screen: self.end_screen,
                selection: self.selection,
                cards: self.cards,
                dots: self.dots,
                url: self.url,
                clipboard: self.clipboard,
                window: self.window,
                document: self.document,
            }
        }
    }

    struct State2 {
        submit: Button,
        shuffle: Button,
        deselect: Button,
        already_guessed: PopUp,
        one_away: PopUp,
        copied: PopUp,
        end_screen: EndScreen,
        selection: Selection,
        cards: Cards,
        dots: Dots,
        url: Url,
        clipboard: Clipboard,
        window: Window,
        document: Document,
    }

    impl State2 {
        fn new_puzzle(self, vec: &mut Vec<Button>) -> Result<State3, DomError> {
            let mut new_puzzle_url = self.url.clone();
            let mut new_puzzle = Button::new(&self.document, ButtonId::NewPuzzle)?;
            let callback = callbacks::to_function_mut(move || {
                callbacks::new_puzzle(&self.window, &mut new_puzzle_url)
            });
            new_puzzle.register(callback);
            vec.push(new_puzzle);
            Ok(State3 {
                submit: self.submit,
                shuffle: self.shuffle,
                deselect: self.deselect,
                already_guessed: self.already_guessed,
                one_away: self.one_away,
                copied: self.copied,
                end_screen: self.end_screen,
                selection: self.selection,
                cards: self.cards,
                dots: self.dots,
                url: self.url,
                clipboard: self.clipboard,
                document: self.document,
            })
        }
    }

    struct State3 {
        submit: Button,
        deselect: Button,
        shuffle: Button,
        already_guessed: PopUp,
        one_away: PopUp,
        copied: PopUp,
        end_screen: EndScreen,
        selection: Selection,
        cards: Cards,
        dots: Dots,
        url: Url,
        clipboard: Clipboard,
        document: Document,
    }

    impl State3 {
        fn shuffle(self, vec: &mut Vec<Button>) -> Result<State4, DomError> {
            let mut shuffle = Button::new(&self.document, ButtonId::Shuffle)?;
            let cards = self.cards.clone();
            let callback = callbacks::to_function_mut(move || {
                callbacks::shuffle(&cards);
            });
            shuffle.register(callback);
            vec.push(shuffle);

            Ok(self.next_state())
        }

        fn next_state(self) -> State4 {
            State4 {
                submit: self.submit,
                shuffle: self.shuffle,
                deselect: self.deselect,
                already_guessed: self.already_guessed,
                one_away: self.one_away,
                copied: self.copied,
                end_screen: self.end_screen,
                selection: self.selection,
                dots: self.dots,
                url: self.url,
                cards: self.cards,
                clipboard: self.clipboard,
                document: self.document,
            }
        }
    }

    struct State4 {
        submit: Button,
        deselect: Button,
        already_guessed: PopUp,
        one_away: PopUp,
        copied: PopUp,
        end_screen: EndScreen,
        selection: Selection,
        cards: Cards,
        dots: Dots,
        url: Url,
        shuffle: Button,
        clipboard: Clipboard,
        document: Document,
    }

    impl State4 {
        fn try_again(self, vec: &mut Vec<Button>) -> Result<State5, DomError> {
            let mut try_again = Button::new(&self.document, ButtonId::TryAgain)?;
            let submit = self.submit.clone();
            let end_screen = self.end_screen.clone();
            let deselect = self.deselect.clone();
            let mut dots = self.dots.clone();
            let selection = self.selection.clone();
            let mut cards = self.cards.clone();
            let callback = callbacks::to_function_mut(move || {
                callbacks::try_again(
                    &mut cards,
                    &end_screen,
                    &mut dots,
                    &submit,
                    &deselect,
                    selection.clone(),
                );
            });
            try_again.register(callback);
            vec.push(try_again);
            Ok(State5 {
                submit: self.submit,
                shuffle: self.shuffle,
                deselect: self.deselect,
                end_screen: self.end_screen,
                cards: self.cards,
                already_guessed: self.already_guessed,
                one_away: self.one_away,
                copied: self.copied,
                dots: self.dots,
                selection: self.selection,
                url: self.url,
                clipboard: self.clipboard,
                document: self.document,
            })
        }
    }

    struct State5 {
        url: Url,
        clipboard: Clipboard,
        copied: PopUp,
        submit: Button, //also needed by state8

        //state 6
        already_guessed: PopUp,
        one_away: PopUp,
        dots: Dots,
        cards: Cards,
        //also state 8
        end_screen: EndScreen,

        // state 8
        selection: Selection,
        deselect: Button, // also needed by state 7
        shuffle: Button,
        document: Document,
    }

    impl State5 {
        fn share(self, vec: &mut Vec<Button>) -> Result<State6, DomError> {
            let mut share = Button::new(&self.document, ButtonId::Share)?;
            ShareCallback::register(&mut share, self.url, self.clipboard, self.copied);
            vec.push(share);
            Ok(State6 {
                already_guessed: self.already_guessed,
                one_away: self.one_away,
                end_screen: self.end_screen,
                dots: self.dots,
                cards: self.cards,

                selection: self.selection,
                submit: self.submit,
                deselect: self.deselect,
                shuffle: self.shuffle,
                document: self.document,
            })
        }
    }

    struct State6 {
        already_guessed: PopUp,
        one_away: PopUp,
        //also needed by state8
        end_screen: EndScreen,
        cards: Cards,
        //needed by state7
        selection: Selection,
        //neded by state7 and state8
        deselect: Button,
        //needed by state8
        submit: Button,
        shuffle: Button,
        document: Document,
        dots: Dots,
    }

    impl State6 {
        fn submit(self, vec: &mut Vec<Button>) -> Result<State7, DomError> {
            let submit = Button::new(&self.document, ButtonId::Submit)?;
            let end_screen = self.end_screen.clone();
            let submit_button = self.submit.clone();
            let already_guessed = self.already_guessed.clone();
            let selection = self.selection.clone();
            let callback_struct = Box::new(callbacks::SubmitCallback::new(
                submit_button,
                already_guessed,
                self.one_away,
                end_screen,
                selection,
                self.dots,
                self.cards,
            ));
            callback_struct.submit_callback();
            vec.push(submit);
            Ok(State7 {
                // needed by state7
                selection: self.selection,
                //needed by both
                deselect: self.deselect,
                // needed by state8
                submit: self.submit,
                end_screen: self.end_screen,
                shuffle: self.shuffle,
                document: self.document,
            })
        }
    }

    struct State7 {
        selection: Selection,

        // needed by state 8
        submit: Button,
        deselect: Button,
        end_screen: EndScreen,
        shuffle: Button,
        document: Document,
    }

    impl State7 {
        fn deselect(mut self, vec: &mut Vec<Button>) -> Result<State8, DomError> {
            let mut deselect = Button::new(&self.document, ButtonId::DeselectAll)?;
            let deselect_button = self.deselect.clone();
            let submit_button = self.submit.clone();
            let callback = callbacks::to_function_mut(move || {
                callbacks::deselect(&mut self.selection, &deselect_button, &submit_button)
            });
            deselect.register(callback);
            vec.push(deselect);
            Ok(State8 {
                submit: self.submit,
                deselect: self.deselect,
                end_screen: self.end_screen,
                shuffle: self.shuffle,
                document: self.document,
            })
        }
    }

    struct State8 {
        document: Document,
        end_screen: EndScreen,
        shuffle: Button,
        submit: Button,
        deselect: Button,
    }

    impl State8 {
        fn see_board(self, mut vec: Vec<Button>) -> Result<[Button; 8], ButtonError> {
            let mut see_board = Button::new(&self.document, ButtonId::SeeBoard)?;
            let callback = callbacks::to_function_mut(move || {
                callbacks::see_board(
                    &self.end_screen,
                    &self.shuffle,
                    &self.deselect,
                    &self.submit,
                )
            });
            see_board.register(callback);
            vec.push(see_board);
            let len = vec.len();
            vec.try_into().map_err(|_| ButtonError::Miscount(len))
        }
    }
}
