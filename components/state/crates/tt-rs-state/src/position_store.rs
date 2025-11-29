//! Position store trait - manages widget positions only.

use crate::Position;
use tt_rs_core::WidgetId;

/// Trait for managing widget positions (single responsibility).
pub trait PositionStore {
    fn get(&self, id: WidgetId) -> Option<Position>;
    fn set(&mut self, id: WidgetId, pos: Position);
    fn remove(&mut self, id: WidgetId);
    fn contains(&self, id: WidgetId) -> bool;
}
