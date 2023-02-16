use wasm_bindgen::prelude::*;
use web_sys::{EventTarget, HtmlButtonElement};

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // Get a reference to the button element
    let button = get_button()?;

    // Add a click event listener to the button
    let closure = Closure::wrap(Box::new(|| {
        // Display an alert with the message "Hello, World!"
        web_sys::window()
            .unwrap()
            .alert_with_message("Hello, World!")
            .unwrap();
    }) as Box<dyn FnMut()>);
    button
        .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
    closure.forget();

    Ok(())
}

fn get_button() -> Result<HtmlButtonElement, JsValue> {
    let window = web_sys::window().ok_or(JsValue::NULL)?;
    let document = window.document().ok_or(JsValue::NULL)?;
    let button = document
        .get_element_by_id("helloButton")
        .ok_or(JsValue::NULL)?
        .dyn_into::<HtmlButtonElement>()?;
    Ok(button)
}