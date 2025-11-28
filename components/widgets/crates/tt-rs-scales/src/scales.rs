//! Scales struct and operations.

use tt_rs_core::WidgetId;

/// Result of a comparison on the scales.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CompareResult {
    #[default]
    Balanced,
    LeftHeavier,
    RightHeavier,
}

/// A scales widget for comparing two values.
///
/// Drop items on the left or right pan. The scales tip toward the heavier side.
#[derive(Debug, Clone)]
pub struct Scales {
    pub(crate) id: WidgetId,
    pub(crate) left_value: Option<i64>,
    pub(crate) right_value: Option<i64>,
    pub(crate) result: CompareResult,
}

impl Scales {
    /// Creates empty scales.
    pub fn new() -> Self {
        Self {
            id: WidgetId::new(),
            left_value: None,
            right_value: None,
            result: CompareResult::Balanced,
        }
    }

    /// Returns the comparison result.
    pub fn result(&self) -> CompareResult {
        self.result
    }

    /// Returns the left pan value.
    pub fn left_value(&self) -> Option<i64> {
        self.left_value
    }

    /// Returns the right pan value.
    pub fn right_value(&self) -> Option<i64> {
        self.right_value
    }

    /// Sets the left pan value and recalculates result.
    pub fn set_left(&mut self, value: i64) {
        self.left_value = Some(value);
        self.recalculate();
    }

    /// Sets the right pan value and recalculates result.
    pub fn set_right(&mut self, value: i64) {
        self.right_value = Some(value);
        self.recalculate();
    }

    /// Clears both pans.
    pub fn clear(&mut self) {
        self.left_value = None;
        self.right_value = None;
        self.result = CompareResult::Balanced;
    }

    fn recalculate(&mut self) {
        self.result = match (self.left_value, self.right_value) {
            (Some(l), Some(r)) if l > r => CompareResult::LeftHeavier,
            (Some(l), Some(r)) if r > l => CompareResult::RightHeavier,
            _ => CompareResult::Balanced,
        };
    }
}

impl Default for Scales {
    fn default() -> Self {
        Self::new()
    }
}
