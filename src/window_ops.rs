use wasm_bindgen::JsValue;
use web_sys::{js_sys::Object, Document, Window};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct WindowOps {
    window: Window,
    document: Document,
}

impl WindowOps {
    pub fn new() -> Self {
        let window = if let Some(window) = web_sys::window() {
            window
        } else {
            panic!("The window for the browser was not detected");
        };

        let document = if let Some(document) = window.document() {
            document
        } else {
            panic!("The `window.document` was not detected");
        };

        Self { window, document }
    }

    pub fn get_entry(&self, property: &str) -> Option<Object> {
        self.window.get(property)
    }

    pub fn as_option(value: &JsValue) -> Option<&JsValue> {
        if value.is_null() || value.is_undefined() {
            return Option::None;
        }

        Some(value)
    }
}
