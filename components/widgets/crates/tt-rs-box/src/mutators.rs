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

    /// Resizes the box to have the specified number of holes.
    ///
    /// If increasing size, new empty holes are appended.
    /// If decreasing size, excess holes are removed from the end.
    pub fn resize(&mut self, new_size: usize) {
        let current_size = self.holes.len();
        if new_size > current_size {
            // Add new empty holes
            for i in current_size..new_size {
                self.holes.push(Hole::new(i));
            }
        } else if new_size < current_size {
            // Remove excess holes from the end
            self.holes.truncate(new_size);
        }
        // If new_size == current_size, do nothing
    }
}
