//! Accessor methods for Text.

use crate::text::{ErasureLevel, Text};

impl Text {
    /// Returns the text value.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Returns the erasure level.
    pub fn erasure(&self) -> ErasureLevel {
        self.erasure
    }

    /// Returns the length of the text.
    pub fn len(&self) -> usize {
        self.value.len()
    }

    /// Returns true if the text is empty.
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    /// Returns the first character, if any.
    pub fn first_char(&self) -> Option<char> {
        self.value.chars().next()
    }

    /// Creates a copy of this text widget with a new ID.
    pub fn copy_text(&self) -> Text {
        Text {
            id: tt_rs_core::WidgetId::new(),
            value: self.value.clone(),
            erasure: self.erasure,
        }
    }
}
