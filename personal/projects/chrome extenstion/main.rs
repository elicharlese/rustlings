// Prepare the Extension Code
// Create a manifest
// Package the Extension
// Load the Extension in Chrome
// Publish the Extension

// There are various ways of creating a browser extension for Chrome, but one way of doing it is to use Servo, which is an open source web engine written in Rust.
// To get started with creating a Chrome extension with Rust, you'll need to first install Rust.
// Then you'll want to set up the Servo build environment, which will allow you to compile Rust code for use in your browser extension.
// Once you have Servo set up, you'll be able to use the webextension-cli tool to automatically generate the necessary JSON files that define your extension and will also provide your code with access to Chrome's APIs.
// Finally, you'll need to write the code for your extension. You can find examples and tutorials for doing this within the Servo documentation.

extern crate servo;
use std::time::Duration;

fn main() {
    let origin = "chrome-extension://your-extension-id-here";
    let main_page_url = format!("{}/main-page.html", origin);
    let mut instance = servo::ServoInstance::new();
    let mut window = servo::Window::new(origin);
    instance.add_window(window);
    instance.load_uri(&main_page_url);
    loop {
        if let Some(event) = instance.wait_for_event(Duration::from_millis(10)) {
            instance.handle_events();
        } else {
            break;
        }
}

}

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
