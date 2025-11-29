//! Tests for erased ToonBox.

use crate::ToonBox;

#[test]
fn test_erased_box_matches_any_size() {
    let erased = ToonBox::erased();
    assert!(erased.is_erased());
    assert_eq!(erased.len(), 0);
}

#[test]
fn test_erased_with_size() {
    let erased = ToonBox::erased_with_size(3);
    assert!(erased.is_erased());
    assert_eq!(erased.len(), 3);
}
