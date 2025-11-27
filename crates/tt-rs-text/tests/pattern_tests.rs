//! Pattern matching tests for Text widget.

use tt_rs_core::{MatchResult, Widget};
use tt_rs_text::Text;

#[test]
fn test_exact_match() {
    let pattern = Text::new("hello");
    let target = Text::new("hello");
    assert_eq!(pattern.matches(&target), MatchResult::Match);
}

#[test]
fn test_no_match_different_values() {
    let pattern = Text::new("hello");
    let target = Text::new("world");
    assert_eq!(pattern.matches(&target), MatchResult::NoMatch);
}

#[test]
fn test_erased_matches_any() {
    let pattern = Text::erased();
    let target1 = Text::new("hello");
    let target2 = Text::new("world");
    let target3 = Text::new("");

    assert_eq!(pattern.matches(&target1), MatchResult::Match);
    assert_eq!(pattern.matches(&target2), MatchResult::Match);
    assert_eq!(pattern.matches(&target3), MatchResult::Match);
}

#[test]
fn test_case_sensitive() {
    let pattern = Text::new("Hello");
    let target = Text::new("hello");
    assert_eq!(pattern.matches(&target), MatchResult::NoMatch);
}
