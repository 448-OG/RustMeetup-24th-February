use wasm_bindgen::prelude::*;
use web_sys::{js_sys::Function, Document, Element, HtmlElement};

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("There needs to be a window for this to work");
    let document = window.document().expect("We need a HTML document");
    let body = document
        .body()
        .expect("The body was not found in this document");

    let first_paragraph = document.create_element("p").unwrap();
    first_paragraph.set_text_content(Some("0"));
    first_paragraph.set_id("counter");

    body.append_child(&first_paragraph)?;

    let our_button = button_event(document)?;
    body.append_child(&our_button)?;

    Ok(())
}

fn button_event(document: Document) -> Result<Element, JsValue> {
    let our_button = document.create_element("button")?;
    our_button.set_text_content(Some("Click Me"));

    let target = document.get_element_by_id("counter").unwrap();

    let a = Closure::<dyn FnMut()>::new(move || {
        let mut outcome = target.text_content().unwrap().parse::<u32>().unwrap();
        outcome += 1;
        target.set_text_content(&outcome);
    });

    our_button
        .dyn_ref::<HtmlElement>()
        .unwrap()
        .set_onclick(Some(a.as_ref().unchecked_ref()));

    a.forget();

    Ok(our_button)
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[wasm_bindgen]
pub fn concat(value: &str) -> String {
    format!("Hello {}! I am calling you from Rust", value)
}
