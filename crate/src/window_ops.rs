use web_sys::{js_sys::Object, Document, Window};

use crate::{WalletError, WalletResult};

/// Operations on a browser window.
/// `Window` and `Document` object must be present otherwise
/// an error is thrown.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct WindowOps {
    window: Window,
    document: Document,
}

impl WindowOps {
    /// Get the `Window` and `Document` object in the current browser window
    pub fn new() -> WalletResult<Self> {
        let window = if let Some(window) = web_sys::window() {
            window
        } else {
            return Err(WalletError::MissingAccessToBrowserWindow);
        };

        let document = if let Some(document) = window.document() {
            document
        } else {
            return Err(WalletError::MissingAccessToBrowserDocument);
        };

        Ok(Self { window, document })
    }

    /// Get an entry in the `Window` object
    pub fn get_entry(&self, property: &str) -> Option<Object> {
        self.window.get(property)
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn document(&self) -> &Document {
        &self.document
    }
}
