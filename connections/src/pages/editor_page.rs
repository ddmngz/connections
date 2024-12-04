use crate::game::color::Color;
use crate::game::ConnectionPuzzle;
use crate::game::ConnectionSet;
use std::sync::RwLock;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::JsFuture;
use web_sys::console;
use web_sys::Clipboard;
use web_sys::Document;
use web_sys::Element;
use web_sys::HtmlAnchorElement;
use web_sys::HtmlDialogElement;
use web_sys::HtmlDivElement;
use web_sys::HtmlInputElement;
use web_sys::HtmlTemplateElement;
use web_sys::Url;
use web_sys::Window;

thread_local! {
    pub static DOM: Dom = Dom::new().unwrap();
    pub static SUBMIT_CALLBACK: Closure<dyn FnMut()> = Closure::<dyn FnMut()>::new(encode);
    pub static TRY_GAME: Closure<dyn FnMut()> = Closure::<dyn FnMut()>::new(move ||{
        DOM.with(|dom|{
            dom.try_game();

        })
    });
    pub static CLIPBOARD_CALLBACK: Closure<dyn FnMut()> = Closure::<dyn FnMut()>::new(move || {
        DOM.with(|dom|{
            let future = dom.copy_to_clipboard().unwrap();
            let handle = dom.copy_handle();
            let window = dom.window_handle();
            spawn_local(async move {
                future.await.unwrap();
                Dom::show_copied(&handle);

                console::log_1(&"registering callback".into());
                HIDE_CLIPBOARD.with(|closure|{
                    window.set_timeout_with_callback_and_timeout_and_arguments_1(closure.as_ref().unchecked_ref(),500, &handle).unwrap();
                });
            })
        });
    });

    pub static HIDE_CLIPBOARD: Closure<dyn FnMut(Element)> = Closure::<dyn FnMut(Element)>::new(move |dialog| {
        console::log_1(&"hide clipboard callback".into());
        Dom::hide_copied(&dialog);
    });
}

pub fn setup(code: Option<&str>) {
    match code {
        Some(code) => DOM.with(|dom| dom.init_with_code(code)),
        None => DOM.with(Dom::init),
    };
}

fn encode() {
    let args = DOM.with(|dom| {
        let yellow = dom.yellow.to_args();
        let purple = dom.purple.to_args();
        let green = dom.green.to_args();
        let blue = dom.blue.to_args();
        [yellow, blue, purple, green]
    });
    let args_ref = args.each_ref().map(get_ref);
    let puzzle = ConnectionPuzzle::new(args_ref[0], args_ref[1], args_ref[2], args_ref[3]);
    let code = puzzle.encode();
    DOM.with(|dom| {
        dom.update_url(code);
    });
}

type StringTuple = (String, [String; 4]);

type StrTuple<'a> = (&'a str, [&'a str; 4]);

fn get_ref(tuple: &StringTuple) -> StrTuple {
    let theme: &str = &tuple.0;
    let words: [&str; 4] = tuple.1.each_ref().map(|x| x.as_ref());
    (theme, words)
}

struct Dom {
    blue: InputSet,
    purple: InputSet,
    yellow: InputSet,
    green: InputSet,
    create_game: HtmlDivElement,
    try_game: HtmlDivElement,
    copy_link: HtmlDivElement,
    url: Url,
    clipboard: Clipboard,
    code: RwLock<Option<String>>,
    copied: HtmlDialogElement,
    window: Window,
    template: HtmlTemplateElement,
    game_div: HtmlDivElement,
}

use crate::dom::element_ops;
impl Dom {
    fn window_handle(&self) -> Window {
        self.window.clone()
    }

    fn new() -> Result<Self, ()> {
        let window = web_sys::window().expect("no window found");
        let document = window.document().expect("no document found");

        let create_game = element_ops::new(&document, "submit").expect("no button found");
        let try_game = element_ops::new(&document, "go_to_game").expect("no button found");

        let copy_link = element_ops::new(&document, "copy_link").expect("no button found");

        let template: HtmlTemplateElement =
            crate::dom::element_ops::new(&document, "connections-game").unwrap();
        let game_div: HtmlDivElement = crate::dom::element_ops::new(&document, "game").unwrap();

        let url = Url::new(&document.url().map_err(|_| ())?).map_err(|_| ())?;
        let clipboard = window.navigator().clipboard();
        let (blue, purple, yellow, green) = InputSet::new_set(&document).map_err(|_| ())?;
        let code = RwLock::new(None);
        let copied = document
            .get_element_by_id("copier")
            .expect("no button found")
            .dyn_into()
            .map_err(|_| ())?;
        Ok(Self {
            create_game,
            copy_link,
            try_game,
            blue,
            purple,
            yellow,
            game_div,
            green,
            url,
            clipboard,
            code,
            copied,
            window,
            template,
        })
    }

    fn error_decoding(&self) {
        todo!()
    }

    fn init_with_code(&self, code: &str) {
        self.init();
        let puzzle = ConnectionPuzzle::decode(code).unwrap();
        /*
        let Ok(puzzle) = ConnectionPuzzle::decode(code) else {
            self.error_decoding();
            return;
        };
        */

        self.blue.set_with_set(puzzle.blue());
        self.yellow.set_with_set(puzzle.yellow());
        self.purple.set_with_set(puzzle.purple());
        self.green.set_with_set(puzzle.green());
    }

    fn init(&self) {
        SUBMIT_CALLBACK.with(|closure| {
            self.create_game
                .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                .expect("listener error")
        });

        CLIPBOARD_CALLBACK.with(|closure| {
            self.copy_link
                .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                .expect("listener error")
        })
    }

    fn update_url(&self, code: String) {
        let url = format!("?game={}", code);
        self.url.set_search(&url);
        *self.code.write().unwrap() = Some(code);
        console::log_1(&format!("updating code to {}", url).into());
    }

    fn enable_try_game(&self) {
        self.try_game.set_class_name("button");
    }

    fn try_game(&self) {
        let code = self.code.read().unwrap();
        let code = code.as_ref().unwrap();

        self.game_div
            .replace_with_with_node_1(&self.template.content())
            .unwrap();
        let _ = crate::pages::game_page::init_state(code);
    }

    fn disable_try_game(&self) {}

    fn copy_to_clipboard(&self) -> Option<JsFuture> {
        let code = self.code.read().unwrap();

        let code = code.as_ref()?;

        let url = self.url.clone();
        let code = format!("?game={}", code);
        url.set_search(&code);
        Some(JsFuture::from(self.clipboard.write_text(&url.href())))
    }

    fn copy_handle(&self) -> HtmlDialogElement {
        self.copied.clone()
    }

    fn show_copied(copied: &Element) {
        copied.class_list().add_1("visible").unwrap();
    }

    fn hide_copied(copied: &Element) {
        copied.class_list().remove_1("visible").unwrap();
    }
}

struct InputSet {
    theme_input: HtmlInputElement,
    other_inputs: [HtmlInputElement; 4],
}

impl InputSet {
    fn new_set(document: &Document) -> Result<(Self, Self, Self, Self), ()> {
        let blue = Self::new(Color::Blue, document)?;
        let purple = Self::new(Color::Purple, document)?;
        let yellow = Self::new(Color::Yellow, document)?;
        let green = Self::new(Color::Green, document)?;
        Ok((blue, purple, yellow, green))
    }

    fn set_with_set(&self, set: &ConnectionSet) {
        self.theme_input.set_value(&set.theme());
        use std::iter::zip;
        for (input, word) in zip(&self.other_inputs, set.word_list()) {
            input.set_value(&word);
        }
    }

    fn new(color: Color, document: &Document) -> Result<Self, ()> {
        let div = document.get_element_by_id(color.as_ref()).ok_or(())?;
        let theme_input = div.get_elements_by_class_name("theme").item(0).ok_or(())?;
        let other_input_set = div
            .get_elements_by_class_name("word-set")
            .item(0)
            .ok_or(())?
            .children();

        let theme_input = get_input_elem(theme_input);
        let other_inputs: [HtmlInputElement; 4] = std::array::from_fn(|index| {
            get_input_elem(other_input_set.item(index as u32).unwrap())
        });

        Ok(Self {
            theme_input,
            other_inputs,
        })
    }

    fn to_args(&self) -> (String, [String; 4]) {
        let theme = self.theme_input.value();
        let other_inputs = self.other_inputs.each_ref().map(|x| x.value());
        (theme, other_inputs)
    }
}

fn get_anchor_elem(elem: Element) -> HtmlAnchorElement {
    elem.dyn_into().unwrap()
}

fn get_input_elem(elem: Element) -> HtmlInputElement {
    elem.dyn_into().unwrap()
}
