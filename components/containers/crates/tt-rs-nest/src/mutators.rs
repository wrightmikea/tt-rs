//! Mutator methods for Nest.

use crate::Nest;
use tt_rs_core::Widget;

impl Nest {
    /// Adds a message to the nest (called when a bird delivers).
    pub fn receive(&mut self, message: Box<dyn Widget>) {
        self.messages.push_back(message);
    }

    /// Takes the oldest message from the nest, if any.
    pub fn take(&mut self) -> Option<Box<dyn Widget>> {
        self.messages.pop_front()
    }

    /// Marks this nest as a copy source (for palette).
    pub fn as_copy_source(mut self) -> Self {
        self.is_copy_source = true;
        self
    }

    /// Creates a copy of this nest with a new ID.
    pub fn copy_nest(&self) -> Self {
        Self {
            id: tt_rs_core::WidgetId::new(),
            color: self.color,
            messages: VecDeque::new(), // New nest starts empty
            is_copy_source: false,
        }
    }
}

use std::collections::VecDeque;
