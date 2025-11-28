//! Pattern matching tests for ToonBox.

use tt_rs_box::ToonBox;
use tt_rs_core::{MatchResult, Widget};

#[test]
fn test_erased_matches_any_box() {
    let pattern = ToonBox::erased();
    let box1 = ToonBox::new(3);
    let box2 = ToonBox::new(5);
    let box3 = ToonBox::new(0);

    assert_eq!(pattern.matches(&box1), MatchResult::Match);
    assert_eq!(pattern.matches(&box2), MatchResult::Match);
    assert_eq!(pattern.matches(&box3), MatchResult::Match);
}

#[test]
fn test_erased_with_size_matches_same_size() {
    let pattern = ToonBox::erased_with_size(3);
    let box_same = ToonBox::new(3);
    let box_different = ToonBox::new(4);

    assert_eq!(pattern.matches(&box_same), MatchResult::Match);
    assert_eq!(pattern.matches(&box_different), MatchResult::NoMatch);
}

#[test]
fn test_box_matches_same_size() {
    let box1 = ToonBox::new(3);
    let box2 = ToonBox::new(3);
    let box3 = ToonBox::new(4);

    assert_eq!(box1.matches(&box2), MatchResult::Match);
    assert_eq!(box1.matches(&box3), MatchResult::NoMatch);
}

#[test]
fn test_description() {
    let empty = ToonBox::new(3);
    assert_eq!(empty.description(), "box[0/3]");

    let erased = ToonBox::erased();
    assert_eq!(erased.description(), "erased box");

    let erased_sized = ToonBox::erased_with_size(4);
    assert_eq!(erased_sized.description(), "erased box[4]");
}

#[test]
fn test_copy() {
    let original = ToonBox::new(3);
    let copy = original.copy();

    // Copy should have different ID but same structure
    assert_ne!(original.id(), copy.id());
    assert_eq!(original.description(), copy.description());
}
