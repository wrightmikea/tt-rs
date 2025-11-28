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
}
