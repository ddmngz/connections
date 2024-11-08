use crate::game::color::Color;
use crate::game::ConnectionPuzzle;
use std::sync::OnceLock;
use wasm_bindgen::prelude::*;
use web_sys::console;
use web_sys::Clipboard;
use web_sys::Document;
use web_sys::Element;
use web_sys::HtmlInputElement;
use web_sys::Url;

thread_local! {
    pub static DOM: Dom = Dom::new().unwrap();


    pub static SUBMIT_CALLBACK: Closure<dyn FnMut()> = Closure::<dyn FnMut()>::new(encode);

    pub static CLIPBOARD_CALLBACK: Closure<dyn FnMut()> = Closure::<dyn FnMut()>::new(move || {
        DOM.with(|dom| dom.copy_to_clipboard());
    });

}

#[wasm_bindgen]
pub fn setup() {
    DOM.with(Dom::init);
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
    DOM.with(|dom| dom.render_url(&code));
}

type StringTuple = (String, [String; 4]);

type StrTuple<'a> = (&'a str, [&'a str; 4]);

fn get_ref(tuple: &StringTuple) -> StrTuple {
    let theme: &str = &tuple.0;
    let words: [&str; 4] = tuple.1.each_ref().map(|x| x.as_ref());
    (theme, words)
}

struct Dom {
    document: Document,
    blue: InputSet,
    purple: InputSet,
    yellow: InputSet,
    green: InputSet,
    button: Element,
    link_button: Element,
    copy_link_button: Element,
    url: Url,
    clipboard: Clipboard,
    link: OnceLock<String>,
}

impl Dom {
    fn new() -> Result<Self, ()> {
        let window = web_sys::window().expect("no window found");
        let document = window.document().expect("no document found");
        let button = document
            .get_element_by_id("submit")
            .expect("no button found");
        let link_button = document
            .get_element_by_id("go_to_game")
            .expect("no button found");
        let copy_link_button = document
            .get_element_by_id("copy_link")
            .expect("no button found");

        let url = Url::new(&document.url().map_err(|_| ())?).map_err(|_| ())?;
        let clipboard = window.navigator().clipboard();
        let (blue, purple, yellow, green) = InputSet::new_set(&document).map_err(|_| ())?;
        let link = OnceLock::new();
        Ok(Self {
            copy_link_button,
            document,
            link_button,
            blue,
            purple,
            yellow,
            green,
            button,
            url,
            clipboard,
            link,
        })
    }

    fn init(&self) {
        SUBMIT_CALLBACK.with(|closure| {
            self.button
                .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                .expect("listener error")
        });

        CLIPBOARD_CALLBACK.with(|closure| {
            self.copy_link_button
                .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                .expect("listener error")
        })
    }

    fn render_url(&self, code: &str) {
        self.link_button.set_class_name("button");
        self.copy_link_button.set_class_name("button");
        let url = format!("{}/#{}", self.url.host(), code);
        self.link_button.set_attribute("href", &url).unwrap();
        self.link.set(url).unwrap();
        console::log_1(&code.into())
    }

    fn copy_to_clipboard(&self) {
        if let Some(link) = self.link.get() {
            use wasm_bindgen_futures::spawn_local;
            use wasm_bindgen_futures::JsFuture;
            let future = JsFuture::from(self.clipboard.write_text(link));
            spawn_local(async move {
                future.await.unwrap();
            })
        }
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

fn get_input_elem(elem: Element) -> HtmlInputElement {
    elem.dyn_into().unwrap()
}

fn get_input_elem_value(elem: Element) -> String {
    let input: HtmlInputElement = elem.dyn_into().unwrap();
    input.value()
}
