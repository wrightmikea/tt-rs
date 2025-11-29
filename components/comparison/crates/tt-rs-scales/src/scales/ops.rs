//! Scales accessor methods.

use super::{CompareResult, Scales};

impl Scales {
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

    /// Creates a copy of this scales widget with a new ID.
    pub fn copy_scales(&self) -> Scales {
        Scales::new_with(self.left_value, self.right_value, self.result)
    }
}
