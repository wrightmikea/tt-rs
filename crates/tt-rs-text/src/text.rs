//! Text struct and operations.

use tt_rs_core::WidgetId;

/// Level of erasure for pattern matching.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ErasureLevel {
    #[default]
    None,
    /// Matches any text value.
    Value,
}

/// A text widget representing a string value.
#[derive(Debug, Clone)]
pub struct Text {
    pub(crate) id: WidgetId,
    pub(crate) value: String,
    pub(crate) erasure: ErasureLevel,
}

impl Text {
    /// Creates a new text widget with the given value.
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            id: WidgetId::new(),
            value: value.into(),
            erasure: ErasureLevel::default(),
        }
    }

    /// Creates an erased text pattern that matches any text.
    pub fn erased() -> Self {
        Self {
            id: WidgetId::new(),
            value: String::new(),
            erasure: ErasureLevel::Value,
        }
    }

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

    /// Returns the text without its first character.
    pub fn rest(&self) -> Text {
        let rest: String = self.value.chars().skip(1).collect();
        Text::new(rest)
    }
}
