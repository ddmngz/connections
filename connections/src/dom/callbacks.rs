use crate::game::GameFailiure;
use crate::game::SelectionSuccess;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use web_sys::js_sys::Function;
use web_sys::Window;

#[allow(unused_imports)]
use web_sys::console;

#[allow(unused_imports)]
use crate::console_log;

use super::button::Button;
use super::cards::Cards;
use super::cards::Selection;
use super::misc_objects::dots::Dots;
use super::misc_objects::end_screen::EndScreen;
use super::misc_objects::end_screen::EndState;
use super::misc_objects::Clipboard;
use super::misc_objects::Url;
use crate::dom::GAME_STATE;

use super::misc_objects::pop_up::PopUp;

pub fn new_puzzle(window: &Window, url: &mut Url) {
    let mut url = url.parent();
    url.remove_puzzle();
    url.remove_game();
    window.location().assign(&url.to_string()).unwrap();
}

pub fn to_function_mut(closure: impl FnMut() + 'static) -> Function {
    let closure: Closure<dyn FnMut()> = Closure::new(closure);
    closure.into_js_value().into()
}

pub fn to_function(closure: impl Fn() + 'static) -> Function {
    let closure: Closure<dyn Fn()> = Closure::new(closure);
    closure.into_js_value().into()
}

#[derive(Clone)]
pub struct SubmitCallback {
    submit_button: Button,
    already_guessed: PopUp,
    one_away: PopUp,
    end_screen: EndScreen,
    selection: Selection,
    dots: Dots,
    cards: Cards,
}

use wasm_bindgen_futures::spawn_local;
impl SubmitCallback {
    pub fn boxed_click(mut self: Box<Self>) {
        spawn_local(async move { self.submit().await })
    }

    pub fn submit_callback(self: Box<Self>) {
        let mut button_handle = self.submit_button.clone();
        let function = to_function_mut(move || Self::boxed_click(self.clone()));
        button_handle.register(function);
    }

    pub fn new(
        submit_button: Button,
        already_guessed: PopUp,
        one_away: PopUp,
        end_screen: EndScreen,
        selection: Selection,
        dots: Dots,
        cards: Cards,
    ) -> Self {
        Self {
            submit_button,
            already_guessed,
            one_away,
            end_screen,
            selection,
            dots,
            cards,
        }
    }

    async fn submit(&mut self) {
        self.submit_button.disable();
        self.selection.update_jump_later().await;
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
                self.submit_button.enable();
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
}

pub fn shuffle(cards: &Cards) {
    let mut game_state = GAME_STATE.write().unwrap();
    game_state.shuffle();
    cards.rerender_on_shuffle(&game_state);
}

pub fn see_board(end_screen: &EndScreen, shuffle: &Button, deselect: &Button, submit: &Button) {
    shuffle.disable();
    deselect.disable();
    submit.disable();
    end_screen.close();
}

pub fn deselect(selection: &mut Selection, deselect_button: &Button, submit_button: &Button) {
    GAME_STATE.write().unwrap().clear_selection();
    selection.clear();
    deselect_button.disable();
    submit_button.disable();
}

pub fn try_again(
    cards: &mut Cards,
    end_screen: &EndScreen,
    dots: &mut Dots,
    submit: &Button,
    deselect: &Button,
) {
    {
        GAME_STATE.write().unwrap().start_over();
    }
    cards.reset(&GAME_STATE.read().unwrap());
    dots.reset();
    submit.disable();
    deselect.disable();
    end_screen.close();
}

#[derive(Clone)]
pub struct ShareCallback {
    url: Url,
    clipboard: Clipboard,
    copied: PopUp,
}

impl ShareCallback {
    pub fn register(button: &mut Button, url: Url, clipboard: Clipboard, copied: PopUp) {
        let items = Box::new(Self {
            url,
            clipboard,
            copied,
        });
        let function = to_function_mut(move || Self::on_click(items.clone()));
        button.register(function);
    }

    fn on_click(mut self: Box<Self>) {
        spawn_local(async move { self.share().await })
    }

    async fn share(&mut self) {
        let code = GAME_STATE.read().unwrap().puzzle_code();
        self.url.set_game(&code);
        let new_url = self.url.to_string();
        self.clipboard.copy_async(&new_url).await;
        self.copied.slide_in().await;
    }
}

pub fn edit_me(window: &Window, cur_url: &mut Url) {
    let code = GAME_STATE.read().unwrap().puzzle_code();
    let mut url = cur_url.parent();
    url.remove_game();
    url.set_puzzle(&code);
    window.location().assign(&url.to_string()).unwrap();
}
