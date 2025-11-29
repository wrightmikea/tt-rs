//! Scales struct and operations.

mod mutators;
mod ops;

use tt_rs_core::WidgetId;

/// Result of a comparison on the scales.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CompareResult {
    /// No values to compare - scales wobble indeterminately
    #[default]
    Indeterminate,
    /// Left and right values are equal
    Balanced,
    /// Left value is greater than right
    LeftHeavier,
    /// Right value is greater than left
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

    /// Creates scales with specified values.
    pub(crate) fn new_with(left: Option<i64>, right: Option<i64>, result: CompareResult) -> Self {
        Self {
            id: WidgetId::new(),
            left_value: left,
            right_value: right,
            result,
        }
    }
}

impl Default for Scales {
    fn default() -> Self {
        Self::new()
    }
}
