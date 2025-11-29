//! Scales mutation methods.

use super::{CompareResult, Scales};

impl Scales {
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
        self.result = CompareResult::Indeterminate; // After clearing, wobble
    }

    pub(super) fn recalculate(&mut self) {
        self.result = match (self.left_value, self.right_value) {
            (Some(l), Some(r)) if l > r => CompareResult::LeftHeavier,
            (Some(l), Some(r)) if r > l => CompareResult::RightHeavier,
            (Some(_), Some(_)) => CompareResult::Balanced,
            (Some(_), None) | (None, Some(_)) => CompareResult::Indeterminate, // One value = wobble
            (None, None) => CompareResult::Balanced, // No values = stationary
        };
    }
}
