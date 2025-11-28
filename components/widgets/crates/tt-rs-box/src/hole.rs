//! Hole type for box slots.

use tt_rs_core::WidgetId;
use yew::Html;

/// A hole in a box that can optionally contain a widget.
#[derive(Debug, Clone)]
pub struct Hole {
    /// The index of this hole in the box.
    index: usize,
    /// The content of the hole, if any.
    content: Option<HoleContent>,
}

/// Content that can be placed in a hole.
#[derive(Debug, Clone)]
pub struct HoleContent {
    /// The ID of the widget in this hole.
    pub widget_id: WidgetId,
    /// The rendered HTML of the widget.
    pub html: Html,
}

impl Hole {
    /// Creates a new empty hole at the given index.
    pub fn new(index: usize) -> Self {
        Self {
            index,
            content: None,
        }
    }

    /// Creates a new hole with content.
    pub fn with_content(index: usize, widget_id: WidgetId, html: Html) -> Self {
        Self {
            index,
            content: Some(HoleContent { widget_id, html }),
        }
    }

    /// Returns the index of this hole.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Returns true if this hole is empty.
    pub fn is_empty(&self) -> bool {
        self.content.is_none()
    }

    /// Returns the content of this hole, if any.
    pub fn content(&self) -> Option<&HoleContent> {
        self.content.as_ref()
    }

    /// Sets the content of this hole.
    pub fn set_content(&mut self, widget_id: WidgetId, html: Html) {
        self.content = Some(HoleContent { widget_id, html });
    }

    /// Clears the content of this hole.
    pub fn clear(&mut self) {
        self.content = None;
    }

    /// Takes the content out of this hole, leaving it empty.
    pub fn take(&mut self) -> Option<HoleContent> {
        self.content.take()
    }
}
