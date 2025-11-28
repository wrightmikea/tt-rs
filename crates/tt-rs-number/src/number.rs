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
    /// If true, dragging creates a copy instead of moving the original.
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
        let (n, d) = arithmetic::reduce(numerator, denominator);
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

    /// Sets the arithmetic operator.
    pub fn with_operator(mut self, op: ArithOperator) -> Self {
        self.operator = op;
        self
    }

    /// Sets this number as a copy source (dragging produces copies).
    pub fn as_copy_source(mut self) -> Self {
        self.is_copy_source = true;
        self
    }

    /// Returns true if this number is a copy source.
    pub fn is_copy_source(&self) -> bool {
        self.is_copy_source
    }

    /// Creates a copy of this number with a new ID.
    /// The copy is NOT a copy source (only the original stack produces copies).
    pub fn copy_number(&self) -> Number {
        Number {
            id: WidgetId::new(),
            numerator: self.numerator,
            denominator: self.denominator,
            operator: self.operator,
            erasure: self.erasure,
            is_copy_source: false,
        }
    }

    /// Apply another number to this one using the dropped number's operator.
    /// Returns None if division by zero is attempted.
    /// The dropped number determines the operation via its operator.
    pub fn apply(&mut self, dropped: &Number) -> Option<()> {
        let (new_num, new_den) = match dropped.operator {
            ArithOperator::Add => arithmetic::add(
                dropped.numerator,
                dropped.denominator,
                self.numerator,
                self.denominator,
            ),
            ArithOperator::Subtract => arithmetic::subtract(
                dropped.numerator,
                dropped.denominator,
                self.numerator,
                self.denominator,
            ),
            ArithOperator::Multiply => arithmetic::multiply(
                dropped.numerator,
                dropped.denominator,
                self.numerator,
                self.denominator,
            ),
            ArithOperator::Divide => arithmetic::divide(
                dropped.numerator,
                dropped.denominator,
                self.numerator,
                self.denominator,
            )?,
        };
        self.numerator = new_num;
        self.denominator = new_den;
        Some(())
    }
}
