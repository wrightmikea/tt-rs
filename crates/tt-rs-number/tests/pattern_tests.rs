//! Pattern matching tests for Number widget.

use tt_rs_core::{MatchResult, Widget};
use tt_rs_number::Number;

#[test]
fn test_erased_matches_any() {
    let pattern = Number::erased();
    let value = Number::new(42);
    assert_eq!(pattern.matches(&value), MatchResult::Match);
}

#[test]
fn test_exact_match() {
    let a = Number::new(42);
    let b = Number::new(42);
    assert_eq!(a.matches(&b), MatchResult::Match);
}

#[test]
fn test_no_match_different_values() {
    let a = Number::new(42);
    let c = Number::new(43);
    assert_eq!(a.matches(&c), MatchResult::NoMatch);
}

#[test]
fn test_rational_exact_match() {
    let a = Number::rational(1, 2);
    let b = Number::rational(2, 4);
    assert_eq!(a.matches(&b), MatchResult::Match);
}
