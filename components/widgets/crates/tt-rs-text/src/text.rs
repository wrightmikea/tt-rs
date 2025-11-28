//! Text struct and constructors.

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

    /// Returns the text without its first character.
    pub fn rest(&self) -> Text {
        let rest: String = self.value.chars().skip(1).collect();
        Text::new(rest)
    }
}
