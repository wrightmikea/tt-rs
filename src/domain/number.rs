//! Number widget - represents numeric values in ToonTalk.
//!
//! Numbers in ToonTalk use rational arithmetic for exact precision.
//! They can be combined using arithmetic operations by dropping one onto another.

use super::{MatchResult, Widget, WidgetId};
use yew::prelude::*;

/// The arithmetic operator displayed on a number widget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ArithOperator {
    #[default]
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl ArithOperator {
    /// Returns the symbol for this operator.
    pub fn symbol(&self) -> &'static str {
        match self {
            ArithOperator::Add => "+",
            ArithOperator::Subtract => "-",
            ArithOperator::Multiply => "*",
            ArithOperator::Divide => "/",
        }
    }
}

/// Level of erasure for pattern matching.
///
/// In ToonTalk, users can "erase" parts of widgets to create patterns.
/// - A fully erased number matches any number
/// - A partially erased number might match numbers with specific properties
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ErasureLevel {
    /// Not erased - matches only exact values.
    #[default]
    None,
    /// Value erased - matches any number.
    Value,
}

/// A number widget representing a rational value.
///
/// Numbers are the fundamental data type in ToonTalk. They support:
/// - Exact rational arithmetic (no floating point errors)
/// - Pattern matching with erasure
/// - Visual representation with operator
#[derive(Debug, Clone)]
pub struct Number {
    id: WidgetId,
    /// Numerator of the rational number.
    numerator: i64,
    /// Denominator of the rational number (always positive, never zero).
    denominator: u64,
    /// The operator shown on this number.
    operator: ArithOperator,
    /// Erasure level for pattern matching.
    erasure: ErasureLevel,
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
        let mut num = Self {
            id: WidgetId::new(),
            numerator,
            denominator,
            operator: ArithOperator::default(),
            erasure: ErasureLevel::default(),
        };
        num.reduce();
        num
    }

    /// Creates an erased number pattern that matches any number.
    pub fn erased() -> Self {
        Self {
            id: WidgetId::new(),
            numerator: 0,
            denominator: 1,
            operator: ArithOperator::default(),
            erasure: ErasureLevel::Value,
        }
    }

    /// Returns the numerator.
    pub fn numerator(&self) -> i64 {
        self.numerator
    }

    /// Returns the denominator.
    pub fn denominator(&self) -> u64 {
        self.denominator
    }

    /// Returns true if this is an integer (denominator is 1).
    pub fn is_integer(&self) -> bool {
        self.denominator == 1
    }

    /// Sets the arithmetic operator.
    pub fn with_operator(mut self, op: ArithOperator) -> Self {
        self.operator = op;
        self
    }

    /// Returns the current operator.
    pub fn operator(&self) -> ArithOperator {
        self.operator
    }

    /// Reduces the fraction to lowest terms.
    fn reduce(&mut self) {
        let g = gcd(self.numerator.unsigned_abs(), self.denominator);
        self.numerator /= g as i64;
        self.denominator /= g;
    }

    /// Applies this number's operator to another number.
    pub fn apply_to(&self, other: &Number) -> Option<Number> {
        let result = match self.operator {
            ArithOperator::Add => {
                let num = self.numerator * other.denominator as i64
                    + other.numerator * self.denominator as i64;
                let den = self.denominator * other.denominator;
                Number::rational(num, den)
            }
            ArithOperator::Subtract => {
                let num = other.numerator * self.denominator as i64
                    - self.numerator * other.denominator as i64;
                let den = self.denominator * other.denominator;
                Number::rational(num, den)
            }
            ArithOperator::Multiply => {
                let num = self.numerator * other.numerator;
                let den = self.denominator * other.denominator;
                Number::rational(num, den)
            }
            ArithOperator::Divide => {
                if self.numerator == 0 {
                    return None; // Division by zero
                }
                let num = other.numerator * self.denominator as i64;
                let den = other.denominator * self.numerator.unsigned_abs();
                let sign = if self.numerator < 0 { -1 } else { 1 };
                Number::rational(num * sign, den)
            }
        };
        Some(result)
    }
}

/// Computes the greatest common divisor.
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

impl Widget for Number {
    fn type_name(&self) -> &'static str {
        "number"
    }

    fn id(&self) -> WidgetId {
        self.id
    }

    fn copy(&self) -> Box<dyn Widget> {
        Box::new(Self {
            id: WidgetId::new(), // New copy gets new ID
            numerator: self.numerator,
            denominator: self.denominator,
            operator: self.operator,
            erasure: self.erasure,
        })
    }

    fn matches(&self, other: &dyn Widget) -> MatchResult {
        // Can only match another number
        if other.type_name() != "number" {
            return MatchResult::NoMatch;
        }

        match self.erasure {
            ErasureLevel::Value => {
                // Erased number matches any number
                MatchResult::Match
            }
            ErasureLevel::None => {
                // Must match exact value
                let other_desc = other.description();
                let self_desc = self.description();
                if other_desc == self_desc {
                    MatchResult::Match
                } else {
                    MatchResult::NoMatch
                }
            }
        }
    }

    fn render(&self) -> Html {
        let value_str = if self.erasure == ErasureLevel::Value {
            "?".to_string()
        } else if self.is_integer() {
            self.numerator.to_string()
        } else {
            format!("{}/{}", self.numerator, self.denominator)
        };

        let op_symbol = self.operator.symbol();

        html! {
            <div class="widget number">
                <div class="number-operator">{op_symbol}</div>
                <div class="number-value">{value_str}</div>
            </div>
        }
    }

    fn description(&self) -> String {
        if self.erasure == ErasureLevel::Value {
            "erased number".to_string()
        } else if self.is_integer() {
            format!("number {}", self.numerator)
        } else {
            format!("number {}/{}", self.numerator, self.denominator)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_creation() {
        let n = Number::new(42);
        assert_eq!(n.numerator(), 42);
        assert_eq!(n.denominator(), 1);
        assert!(n.is_integer());
    }

    #[test]
    fn test_rational_reduction() {
        let n = Number::rational(4, 8);
        assert_eq!(n.numerator(), 1);
        assert_eq!(n.denominator(), 2);
    }

    #[test]
    fn test_addition() {
        let a = Number::new(1);
        let b = Number::new(2);
        let result = a.apply_to(&b).unwrap();
        assert_eq!(result.numerator(), 3);
        assert_eq!(result.denominator(), 1);
    }

    #[test]
    fn test_rational_addition() {
        let a = Number::rational(1, 2);
        let b = Number::rational(1, 3);
        let result = a.apply_to(&b).unwrap();
        assert_eq!(result.numerator(), 5);
        assert_eq!(result.denominator(), 6);
    }

    #[test]
    fn test_pattern_matching() {
        let pattern = Number::erased();
        let value = Number::new(42);
        assert_eq!(pattern.matches(&value), MatchResult::Match);
    }

    #[test]
    fn test_exact_matching() {
        let a = Number::new(42);
        let b = Number::new(42);
        let c = Number::new(43);
        assert_eq!(a.matches(&b), MatchResult::Match);
        assert_eq!(a.matches(&c), MatchResult::NoMatch);
    }
}
