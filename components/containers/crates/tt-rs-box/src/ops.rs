//! Operations on ToonBox: accessors, queries, and mutators.

use crate::{Hole, ToonBox};

// === Accessors ===

impl ToonBox {
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
