//! ToonBox struct and operations.

use crate::Hole;
use tt_rs_core::WidgetId;
use yew::Html;

/// A box widget containing numbered holes for other widgets.
///
/// In ToonTalk, boxes are containers that organize data. Each hole
/// can hold one widget, and boxes can be matched against patterns
/// based on their size and contents.
#[derive(Debug, Clone)]
pub struct ToonBox {
    pub(crate) id: WidgetId,
    pub(crate) holes: Vec<Hole>,
    pub(crate) erased: bool,
}

impl ToonBox {
    /// Creates a new box with the specified number of empty holes.
    pub fn new(num_holes: usize) -> Self {
        let holes = (0..num_holes).map(Hole::new).collect();
        Self {
            id: WidgetId::new(),
            holes,
            erased: false,
        }
    }

    /// Creates an erased box pattern that matches any box.
    pub fn erased() -> Self {
        Self {
            id: WidgetId::new(),
            holes: Vec::new(),
            erased: true,
        }
    }

    /// Creates an erased box pattern that matches boxes with a specific size.
    pub fn erased_with_size(num_holes: usize) -> Self {
        let holes = (0..num_holes).map(Hole::new).collect();
        Self {
            id: WidgetId::new(),
            holes,
            erased: true,
        }
    }

    /// Returns the number of holes in this box.
    pub fn len(&self) -> usize {
        self.holes.len()
    }

    /// Returns true if this box has no holes.
    pub fn is_empty(&self) -> bool {
        self.holes.is_empty()
    }

    /// Returns true if this is an erased pattern.
    pub fn is_erased(&self) -> bool {
        self.erased
    }

    /// Gets a reference to the hole at the given index.
    pub fn hole(&self, index: usize) -> Option<&Hole> {
        self.holes.get(index)
    }

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

    /// Returns an iterator over the holes.
    pub fn holes(&self) -> impl Iterator<Item = &Hole> {
        self.holes.iter()
    }

    /// Returns the number of filled holes.
    pub fn filled_count(&self) -> usize {
        self.holes.iter().filter(|h| !h.is_empty()).count()
    }

    /// Returns the number of empty holes.
    pub fn empty_count(&self) -> usize {
        self.holes.iter().filter(|h| h.is_empty()).count()
    }
}
