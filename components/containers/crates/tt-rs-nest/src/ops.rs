//! Accessor methods for Nest.

use crate::{Nest, NestColor};

impl Nest {
    /// Returns the nest's color.
    pub fn color(&self) -> NestColor {
        self.color
    }

    /// Returns the number of messages in the nest.
    pub fn message_count(&self) -> usize {
        self.messages.len()
    }

    /// Returns true if the nest has messages.
    pub fn has_messages(&self) -> bool {
        !self.messages.is_empty()
    }

    /// Returns true if this is a copy source (palette item).
    pub fn is_copy_source(&self) -> bool {
        self.is_copy_source
    }
}

impl NestColor {
    /// Returns the CSS color string for this nest color.
    pub fn css_color(&self) -> &'static str {
        match self {
            NestColor::Blue => "#4a90d9",
            NestColor::Red => "#d94a4a",
            NestColor::Green => "#4ad94a",
            NestColor::Yellow => "#d9d94a",
        }
    }
}
