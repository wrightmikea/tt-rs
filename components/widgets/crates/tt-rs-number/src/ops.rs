//! Accessor methods for Number.

use crate::number::{ErasureLevel, Number};
use crate::ArithOperator;

impl Number {
    /// Returns the numerator.
    pub fn numerator(&self) -> i64 {
        self.numerator
    }

    /// Returns the denominator.
    pub fn denominator(&self) -> u64 {
        self.denominator
    }

    /// Returns the current operator.
    pub fn operator(&self) -> ArithOperator {
        self.operator
    }

    /// Returns the erasure level.
    pub fn erasure(&self) -> ErasureLevel {
        self.erasure
    }

    /// Returns true if this number is a copy source.
    pub fn is_copy_source(&self) -> bool {
        self.is_copy_source
    }

    /// Returns true if this is an integer.
    pub fn is_integer(&self) -> bool {
        self.denominator == 1
    }

    /// Returns true if this number acts as a tool.
    pub fn is_tool(&self) -> bool {
        self.operator != ArithOperator::Add
    }

    /// Returns the effective numerator value accounting for the operator.
    /// For Subtract tools, the effective value is negated.
    /// This is the actual numeric value the number represents.
    pub fn effective_numerator(&self) -> i64 {
        match self.operator {
            ArithOperator::Subtract => -self.numerator,
            _ => self.numerator,
        }
    }
}
