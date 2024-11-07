mod game;
use wasm_bindgen::prelude::*;
//use web_sys::console;

#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();
}

#[allow(dead_code)]
fn hello_world() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    //val.set_text_content(Some("Hello from Rust!"));

    body.append_child(&val)?;
    Ok(())
}
