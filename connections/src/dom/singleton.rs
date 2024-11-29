use super::button::ButtonId;
use super::cards::Card;
use super::cards::Cards;
use super::cards::Selection;
use super::element_ops;
use super::misc_objects::dots::Dots;
use super::misc_objects::end_screen::EndScreen;
use super::misc_objects::pop_up::PopUp;
use super::misc_objects::pop_up::PopUpId;
use super::misc_objects::Clipboard;
use super::misc_objects::Url;
use super::Document;
use super::Window;
use crate::dom::button::Button;

use crate::dom::GAME_STATE;
use element_ops::DomError;

pub struct WebPage(RwLock<PageState>);

pub static WEB_PAGE: WebPage = WebPage::new();
enum PageState {
    Uninit,
    Init(WebPageInner),
}

impl PageState {
    fn get_or_init<'a>(
        &'a mut self,
        window: Window,
        document: Document,
    ) -> Result<&'a WebPageInner, DomError> {
        let reference = self.get_or_init_mut(window, document)?;
        // need this to do the implic conversion from mut to immutable
        Ok(reference)
    }

    fn get_or_init_mut<'a>(
        &'a mut self,
        window: Window,
        document: Document,
    ) -> Result<&'a mut WebPageInner, DomError> {
        match self {
            Self::Uninit => {
                let inner = WebPageInner::new(window, document)?;
                *self = Self::Init(inner);
                let inner = self.assume_init_mut();
                Ok(inner)
            }
            Self::Init(_) => Ok(self.assume_init_mut()),
        }
    }

    fn init(&mut self, window: Window, document: Document) -> Result<(), DomError> {
        let inner = WebPageInner::new(window, document)?;
        *self = Self::Init(inner);
        Ok(())
    }

    fn assume_init(&self) -> &WebPageInner {
        if let Self::Init(inner) = &self {
            inner
        } else {
            unreachable!()
        }
    }

    fn assume_init_mut(&mut self) -> &mut WebPageInner {
        let Self::Init(inner) = self else {
            unreachable!()
        };
        inner
    }
}

struct WebPageInner {
    submit: Button,
    deselect: Button,
    shuffle: Button,
    see_board: Button,
    try_again: Button,
    edit_me: Button,
    share: Button,
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

use crate::dom::misc_objects::end_screen::EndState;
use crate::game::GameFailiure;
use crate::game::SelectionSuccess;
use wasm_bindgen_futures::spawn_local;
impl WebPageInner {
    fn new(window: Window, document: Document) -> Result<Self, DomError> {
        Ok(Self {
            submit: Button::new(&document, ButtonId::Submit)?,
            deselect: Button::new(&document, ButtonId::DeselectAll)?,
            see_board: Button::new(&document, ButtonId::SeeBoard)?,
            try_again: Button::new(&document, ButtonId::TryAgain)?,
            edit_me: Button::new(&document, ButtonId::EditMe)?,
            share: Button::new(&document, ButtonId::Share)?,
            shuffle: Button::new(&document, ButtonId::Shuffle)?,
            already_guessed: PopUp::new(&document, PopUpId::AlreadyGuessed)?,
            one_away: PopUp::new(&document, PopUpId::OneAway)?,
            copied: PopUp::new(&document, PopUpId::CopyToClipboard)?,
            end_screen: EndScreen::new(&document)?,
            selection: Selection::new(&document),
            cards: Cards::new(&document).unwrap(),
            dots: Dots::new(&document),
            url: Url::new(&document),
            clipboard: Clipboard::new(&window),
            window,
            document,
        })
    }

    async fn submit(&mut self) {
        self.submit.disable();
        self.selection.jump_later().await;
        let res = { GAME_STATE.write().unwrap().check_selection() };
        match res {
            Ok(success) => {
                self.selection.clear();
                let state = GAME_STATE.read().unwrap();
                match success {
                    SelectionSuccess::Won(color) => {
                        self.cards.add_set(color, &state);
                        self.end_screen.show(EndState::Win);
                    }
                    SelectionSuccess::Matched(color) => {
                        self.cards.add_set(color, &state);
                    }
                };
            }
            Err(failiure) => {
                self.submit.enable();
                self.dots.hide_one();
                match failiure {
                    GameFailiure::Mismatch | GameFailiure::NotEnough => self.selection.shake(),
                    GameFailiure::OneAway => self.one_away.pop_up().await,
                    GameFailiure::AlreadyTried => self.already_guessed.pop_up().await,
                    GameFailiure::Lost => self.end_screen.show(EndState::Lost),
                }
            }
        }
    }

    fn shuffle(&mut self) {
        let mut game_state = GAME_STATE.write().unwrap();
        game_state.shuffle();
        self.cards.rerender_on_shuffle(&game_state);
    }
    fn see_board(&self) {
        self.shuffle.disable();
        self.deselect.disable();
        self.submit.disable();
        self.end_screen.close();
    }
    fn deselect(&mut self) {
        GAME_STATE.write().unwrap().clear_selection();
        self.selection.clear();
        self.deselect.disable();
        self.submit.disable();
    }
    fn try_again(&mut self) {
        {
            GAME_STATE.write().unwrap().start_over();
        }
        // this needs to be changed, instead give them static references
        self.cards.reset(&GAME_STATE.read().unwrap());
        self.dots.reset();
        self.submit.disable();
        self.deselect.disable();
        self.end_screen.close();
    }

    async fn share(&mut self) {
        let code = GAME_STATE.read().unwrap().puzzle_code();
        self.url.set_game(&code);
        let new_url = self.url.to_string();
        self.clipboard.copy_async(&new_url).await;
        self.copied.slide_in().await;
    }
    fn edit_me(&self) {
        let code = GAME_STATE.read().unwrap().puzzle_code();
        let mut url = self.url.parent();
        url.remove_game();
        url.set_puzzle(&code);
        self.window.location().assign(&url.to_string()).unwrap();
    }

    fn card_click(&mut self, card: &Card, index: usize) {
        let Ok(selection_len) = GAME_STATE.write().unwrap().select(index) else {
            return;
        };
        match selection_len {
            0 => {
                self.deselect.disable();
                self.submit.disable();
            }
            1 => {
                self.deselect.enable();
            }
            2 => (),
            3 => {
                self.submit.disable();
            }
            4 => {
                self.selection.update_vec();
                self.submit.enable();
            }
            _ => {
                unreachable!()
            }
        };
        card.toggle_selected();
    }
}

// Even though there are raw pointers in the web sys types, since they're only accessed through
// Singleton, and Singleton has a RwLock around it, this should be okay
unsafe impl Sync for WebPageInner {}
unsafe impl Sync for WebPage {}

use std::sync::RwLock;
impl WebPage {
    const fn new() -> Self {
        Self(RwLock::new(PageState::Uninit))
    }

    pub fn init(&self, window: Window, document: Document) -> Result<(), DomError> {
        self.0.write().unwrap().init(window, document)?;
        Ok(())
    }

    pub fn setup_callbacks(&'static self) {
        self.register_cards();
        self.register_submit();
        self.register_shuffle();
        self.register_see_board();
        self.register_deselect();
        self.register_try_again();
        self.register_share();
        self.register_edit_me();
    }

    fn inner(&self) -> PageRef {
        let inner: PageRef = self.0.read().unwrap().into();
        inner
    }

    fn inner_mut(&self) -> PageRefMut {
        let inner: PageRefMut = self.0.write().unwrap().into();
        inner
    }

    fn register_submit(&'static self) {
        let f = || {
            spawn_local(async {
                self.inner_mut().submit().await;
            })
        };
        let mut inner = self.inner_mut();
        Self::register_button(&mut inner.submit, f)
    }

    fn register_button(button: &mut Button, f: impl Fn() + 'static) {
        let f = crate::dom::callbacks::to_function(f);
        button.register(f);
    }

    fn register_shuffle(&'static self) {
        let f = || self.inner_mut().shuffle();
        Self::register_button(&mut self.inner_mut().shuffle, f);
    }

    fn register_see_board(&'static self) {
        let f = || self.inner_mut().see_board();
        Self::register_button(&mut self.inner_mut().see_board, f);
    }

    fn register_deselect(&'static self) {
        let f = || self.inner_mut().deselect();
        Self::register_button(&mut self.inner_mut().deselect, f);
    }

    fn register_try_again(&'static self) {
        let f = || {
            self.inner_mut().try_again();
            // need to reregister cards
            self.register_cards();
        };
        Self::register_button(&mut self.inner_mut().try_again, f);
    }

    fn register_share(&'static self) {
        let f = || {
            spawn_local(async {
                self.inner_mut().share().await;
            });
        };

        Self::register_button(&mut self.inner_mut().share, f);
    }

    fn register_edit_me(&'static self) {
        let f = || self.inner().edit_me();
        Self::register_button(&mut self.inner_mut().edit_me, f);
    }

    fn register_cards(&'static self) {
        for (index, card) in self.inner().cards.into_iter().enumerate() {
            self.register_card(card, index)
        }
    }

    fn register_card(&'static self, card: Card, index: usize) {
        let card_handle = card.clone();
        let f = move || self.inner_mut().card_click(&card, index);
        card_handle.register(f)

        //let f = || self.inner().deselect();
    }
}

use std::sync::RwLockReadGuard;
struct PageRef<'a>(RwLockReadGuard<'a, PageState>);

impl<'a> From<RwLockReadGuard<'a, PageState>> for PageRef<'a> {
    fn from(guard: RwLockReadGuard<'a, PageState>) -> Self {
        Self(guard)
    }
}

impl<'a> PageRef<'a> {
    fn new(guard: RwLockReadGuard<'a, PageState>) -> Self {
        Self(guard)
    }
}

use std::ops::Deref;
use std::ops::DerefMut;
impl<'a> Deref for PageRef<'a> {
    type Target = WebPageInner;

    fn deref(&self) -> &Self::Target {
        self.0.assume_init()
    }
}

use std::sync::RwLockWriteGuard;
struct PageRefMut<'a>(RwLockWriteGuard<'a, PageState>);

impl<'a> From<RwLockWriteGuard<'a, PageState>> for PageRefMut<'a> {
    fn from(guard: RwLockWriteGuard<'a, PageState>) -> Self {
        Self(guard)
    }
}

impl<'a> PageRefMut<'a> {
    fn new(guard: RwLockWriteGuard<'a, PageState>) -> Self {
        Self(guard)
    }
}
impl<'a> Deref for PageRefMut<'a> {
    type Target = WebPageInner;

    fn deref(&self) -> &Self::Target {
        self.0.assume_init()
    }
}
impl<'a> DerefMut for PageRefMut<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.assume_init_mut()
    }
}
