//! Box contents trait - manages what's in box holes only.

use tt_rs_core::WidgetId;

/// Trait for managing box hole contents (single responsibility).
pub trait BoxContents {
    fn widget_in_hole(&self, box_id: WidgetId, hole: usize) -> Option<WidgetId>;
    fn place_in_hole(&mut self, box_id: WidgetId, hole: usize, widget_id: WidgetId);
    fn clear_hole(&mut self, box_id: WidgetId, hole: usize) -> Option<WidgetId>;
    fn box_for_widget(&self, widget_id: WidgetId) -> Option<(WidgetId, usize)>;
}
