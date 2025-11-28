//! Mutation methods for ToonBox.

use crate::{Hole, ToonBox};
use tt_rs_core::WidgetId;
use yew::Html;

impl ToonBox {
    /// Gets a mutable reference to the hole at the given index.
    pub fn hole_mut(&mut self, index: usize) -> Option<&mut Hole> {
        self.holes.get_mut(index)
    }

    /// Sets the content of a hole at the given index.
    pub fn set_hole(&mut self, index: usize, widget_id: WidgetId, html: Html) -> bool {
        if let Some(hole) = self.holes.get_mut(index) {
            hole.set_content(widget_id, html);
            true
        } else {
            false
        }
    }

    /// Clears the content of a hole at the given index.
    pub fn clear_hole(&mut self, index: usize) -> bool {
        if let Some(hole) = self.holes.get_mut(index) {
            hole.clear();
            true
        } else {
            false
        }
    }
}
