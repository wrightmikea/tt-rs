//! Nest struct and constructors.

use std::collections::VecDeque;
use tt_rs_core::{Widget, WidgetId};

/// A nest widget that receives messages from birds.
///
/// Nests are the receiving end of ToonTalk's message passing system.
/// Birds deliver messages to their paired nest, where they queue up
/// until retrieved.
#[derive(Debug)]
pub struct Nest {
    pub(crate) id: WidgetId,
    pub(crate) color: NestColor,
    pub(crate) messages: VecDeque<Box<dyn Widget>>,
    pub(crate) is_copy_source: bool,
}

impl Clone for Nest {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            color: self.color,
            // Deep copy each message using Widget::copy()
            messages: self.messages.iter().map(|m| m.copy()).collect(),
            is_copy_source: self.is_copy_source,
        }
    }
}

/// Colors for nest/bird pairs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NestColor {
    #[default]
    Blue,
    Red,
    Green,
    Yellow,
}

impl Nest {
    /// Creates a new empty nest with the default color.
    pub fn new() -> Self {
        Self {
            id: WidgetId::new(),
            color: NestColor::default(),
            messages: VecDeque::new(),
            is_copy_source: false,
        }
    }

    /// Creates a new nest with a specific color.
    pub fn with_color(color: NestColor) -> Self {
        Self {
            id: WidgetId::new(),
            color,
            messages: VecDeque::new(),
            is_copy_source: false,
        }
    }
}

impl Default for Nest {
    fn default() -> Self {
        Self::new()
    }
}
