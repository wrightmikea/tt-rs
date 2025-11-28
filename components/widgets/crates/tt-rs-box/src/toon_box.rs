//! ToonBox struct and constructors.

use crate::Hole;
use tt_rs_core::WidgetId;

/// A box widget containing numbered holes for other widgets.
///
/// In ToonTalk, boxes are containers that organize data. Each hole
/// can hold one widget, and boxes can be matched against patterns
/// based on their size and contents.
///
/// # Operations
///
/// Read-only access: see [`crate::accessors`]
/// Computed queries: see [`crate::queries`]
/// State changes: see [`crate::mutators`]
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
}
