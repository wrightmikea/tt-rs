//! Vacuum tool struct.

use tt_rs_core::WidgetId;

/// A vacuum tool for erasing values from widgets.
///
/// Drop the vacuum on a widget to erase its value, turning it into a pattern
/// that matches any value of that type.
#[derive(Debug, Clone)]
pub struct Vacuum {
    pub(crate) id: WidgetId,
}

impl Vacuum {
    /// Creates a new vacuum tool.
    pub fn new() -> Self {
        Self {
            id: WidgetId::new(),
        }
    }

    /// Creates a vacuum as a copy source (for the toolbox).
    pub fn as_copy_source() -> Self {
        Self {
            id: WidgetId::new(),
        }
    }

    /// Creates a copy of this vacuum with a new ID.
    pub fn copy_vacuum(&self) -> Vacuum {
        Vacuum {
            id: WidgetId::new(),
        }
    }
}

impl Default for Vacuum {
    fn default() -> Self {
        Self::new()
    }
}
