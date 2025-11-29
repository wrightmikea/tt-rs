//! Box state struct and methods.

use std::collections::HashMap;
use tt_rs_core::WidgetId;

/// A box with its holes that can contain widgets.
#[derive(Clone)]
pub struct BoxState {
    id: WidgetId,
    pub num_holes: usize,
    pub contents: HashMap<usize, WidgetId>,
    pub erased: bool,
}

impl BoxState {
    pub fn new(num_holes: usize) -> Self {
        Self {
            id: WidgetId::new(),
            num_holes,
            contents: HashMap::new(),
            erased: false,
        }
    }

    #[allow(dead_code)]
    pub fn erased() -> Self {
        Self {
            id: WidgetId::new(),
            num_holes: 0,
            contents: HashMap::new(),
            erased: true,
        }
    }

    pub fn id(&self) -> WidgetId {
        self.id
    }

    pub fn place_in_hole(&mut self, hole: usize, wid: WidgetId) {
        self.contents.insert(hole, wid);
    }

    pub fn widget_in_hole(&self, hole: usize) -> Option<WidgetId> {
        self.contents.get(&hole).copied()
    }

    pub fn clear_hole(&mut self, hole: usize) -> Option<WidgetId> {
        self.contents.remove(&hole)
    }

    pub fn copy_box(&self) -> Self {
        Self {
            id: WidgetId::new(),
            num_holes: self.num_holes,
            contents: HashMap::new(),
            erased: self.erased,
        }
    }
}
