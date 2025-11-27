//! Number accessor methods and operations.

use crate::number::{ErasureLevel, Number};
use crate::{ArithOperator, arithmetic};

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

    /// Returns true if this is an integer.
    pub fn is_integer(&self) -> bool {
        self.denominator == 1
    }

    /// Applies this number's operator to another.
    pub fn apply_to(&self, other: &Number) -> Option<Number> {
        let (n, d) = apply_op(self.operator, self, other)?;
        Some(Number::rational(n, d))
    }
}

fn apply_op(op: ArithOperator, a: &Number, b: &Number) -> Option<(i64, u64)> {
    match op {
        ArithOperator::Add => Some(arithmetic::add(
            a.numerator,
            a.denominator,
            b.numerator,
            b.denominator,
        )),
        ArithOperator::Subtract => Some(arithmetic::subtract(
            a.numerator,
            a.denominator,
            b.numerator,
            b.denominator,
        )),
        ArithOperator::Multiply => Some(arithmetic::multiply(
            a.numerator,
            a.denominator,
            b.numerator,
            b.denominator,
        )),
        ArithOperator::Divide => {
            arithmetic::divide(a.numerator, a.denominator, b.numerator, b.denominator)
        }
    }
}
