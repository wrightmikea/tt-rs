//! Number struct and constructors.

use crate::{operator, ArithOperator};
use tt_rs_core::WidgetId;

/// Level of erasure for pattern matching.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ErasureLevel {
    #[default]
    None,
    Value,
}

/// A number widget representing a rational value.
///
/// # Operations
///
/// - Accessors: see [`crate::accessors`]
/// - Builders: see [`crate::builders`]
/// - Operations: see [`crate::operations`]
#[derive(Debug, Clone)]
pub struct Number {
    pub(crate) id: WidgetId,
    pub(crate) numerator: i64,
    pub(crate) denominator: u64,
    pub(crate) operator: ArithOperator,
    pub(crate) erasure: ErasureLevel,
    pub(crate) is_copy_source: bool,
}

impl Number {
    /// Creates a new integer number.
    pub fn new(value: i64) -> Self {
        Self {
            id: WidgetId::new(),
            numerator: value,
            denominator: 1,
            operator: ArithOperator::default(),
            erasure: ErasureLevel::default(),
            is_copy_source: false,
        }
    }

    /// Creates a new rational number.
    pub fn rational(numerator: i64, denominator: u64) -> Self {
        assert!(denominator != 0, "Denominator cannot be zero");
        let (n, d) = operator::reduce(numerator, denominator);
        Self {
            id: WidgetId::new(),
            numerator: n,
            denominator: d,
            operator: ArithOperator::default(),
            erasure: ErasureLevel::default(),
            is_copy_source: false,
        }
    }

    /// Creates an erased number pattern.
    pub fn erased() -> Self {
        Self {
            id: WidgetId::new(),
            numerator: 0,
            denominator: 1,
            operator: ArithOperator::default(),
            erasure: ErasureLevel::Value,
            is_copy_source: false,
        }
    }
}
