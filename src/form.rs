use crate::game::color::Color;
use crate::game::ConnectionPuzzle;
use wasm_bindgen::prelude::*;
use web_sys::console;
use web_sys::Document;
use web_sys::Element;
use web_sys::HtmlCollection;
use web_sys::HtmlInputElement;

thread_local! {
    pub static DOM: Dom = Dom::new().unwrap();
    pub static CLOSURE: Closure<dyn FnMut()> = Closure::<dyn FnMut()>::new(move || {
        encode();
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
}

impl Dom {
    fn new() -> Result<Self, ()> {
        let window = web_sys::window().expect("no window found");
        let document = window.document().expect("no document found");
        let button = document.get_element_by_id("submit").unwrap();
        let (blue, purple, yellow, green) = InputSet::new_set(&document).map_err(|_| ())?;
        Ok(Self {
            document,
            blue,
            purple,
            yellow,
            green,
            button,
        })
    }

    fn init(&self) {
        CLOSURE.with(|closure| {
            self.button
                .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                .expect("listener error")
        })
    }

    fn render_url(&self, code: &str) {
        console::log_1(&code.into())
    }
}

struct InputSet {
    theme_input: Element,
    other_inputs: HtmlCollection,
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
        let other_inputs = div.get_elements_by_class_name("word");
        Ok(Self {
            theme_input,
            other_inputs,
        })
    }

    fn to_args(&self) -> (String, [String; 4]) {
        let theme = get_input_elem_value(self.theme_input.clone());
        let other_inputs = std::array::from_fn(|index| {
            get_input_elem_value(self.other_inputs.item(index as u32).unwrap())
        });
        (theme, other_inputs)
    }
}

fn get_input_elem_value(elem: Element) -> String {
    let input: HtmlInputElement = elem.dyn_into().unwrap();
    input.value()
}
