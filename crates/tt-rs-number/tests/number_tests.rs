//! Integration tests for Number widget.

use tt_rs_number::{ArithOperator, Number};

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
fn test_subtraction() {
    let a = Number::new(3).with_operator(ArithOperator::Subtract);
    let b = Number::new(10);
    let result = a.apply_to(&b).unwrap();
    assert_eq!(result.numerator(), 7);
}

#[test]
fn test_multiplication() {
    let a = Number::new(4).with_operator(ArithOperator::Multiply);
    let b = Number::new(5);
    let result = a.apply_to(&b).unwrap();
    assert_eq!(result.numerator(), 20);
}

#[test]
fn test_division() {
    let a = Number::new(2).with_operator(ArithOperator::Divide);
    let b = Number::new(10);
    let result = a.apply_to(&b).unwrap();
    assert_eq!(result.numerator(), 5);
}

#[test]
fn test_division_by_zero() {
    let a = Number::new(0).with_operator(ArithOperator::Divide);
    let b = Number::new(10);
    assert!(a.apply_to(&b).is_none());
}
