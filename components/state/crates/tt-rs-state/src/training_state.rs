//! Training state trait - manages robot training mode only.

use tt_rs_core::WidgetId;

/// Trait for managing robot training state (single responsibility).
pub trait TrainingState {
    fn training_robot(&self) -> Option<WidgetId>;
    fn set_training_robot(&mut self, id: Option<WidgetId>);
    fn is_training(&self) -> bool;
}
