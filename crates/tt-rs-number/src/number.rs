//! Number struct and constructors.

use crate::{ArithOperator, arithmetic};
use tt_rs_core::WidgetId;

/// Level of erasure for pattern matching.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ErasureLevel {
    #[default]
    None,
    Value,
}

/// A number widget representing a rational value.
#[derive(Debug, Clone)]
pub struct Number {
    pub(crate) id: WidgetId,
    pub(crate) numerator: i64,
    pub(crate) denominator: u64,
    pub(crate) operator: ArithOperator,
    pub(crate) erasure: ErasureLevel,
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
        }
    }

    /// Creates a new rational number.
    pub fn rational(numerator: i64, denominator: u64) -> Self {
        assert!(denominator != 0, "Denominator cannot be zero");
        let (n, d) = arithmetic::reduce(numerator, denominator);
        Self {
            id: WidgetId::new(),
            numerator: n,
            denominator: d,
            operator: ArithOperator::default(),
            erasure: ErasureLevel::default(),
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
        }
    }

    /// Sets the arithmetic operator.
    pub fn with_operator(mut self, op: ArithOperator) -> Self {
        self.operator = op;
        self
    }
}
