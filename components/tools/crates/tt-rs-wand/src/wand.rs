//! Wand (magic wand) tool struct.

use tt_rs_core::WidgetId;

/// A magic wand tool for copying widgets.
///
/// Drop the wand on a widget to create a copy of it. The original remains
/// and a new widget with a unique ID is created.
#[derive(Debug, Clone)]
pub struct Wand {
    pub(crate) id: WidgetId,
}

impl Wand {
    /// Creates a new magic wand tool.
    pub fn new() -> Self {
        Self {
            id: WidgetId::new(),
        }
    }

    /// Creates a wand as a copy source (for the toolbox).
    pub fn as_copy_source() -> Self {
        Self {
            id: WidgetId::new(),
        }
    }

    /// Creates a copy of this wand with a new ID.
    pub fn copy_wand(&self) -> Wand {
        Wand {
            id: WidgetId::new(),
        }
    }
}

impl Default for Wand {
    fn default() -> Self {
        Self::new()
    }
}
