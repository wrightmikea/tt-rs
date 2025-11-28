//! Tests for ToonBox widget.

use tt_rs_box::{Hole, ToonBox};

#[test]
fn test_create_empty_box() {
    let b = ToonBox::new(3);
    assert_eq!(b.len(), 3);
    assert!(!b.is_empty());
    assert!(!b.is_erased());
}

#[test]
fn test_create_zero_hole_box() {
    let b = ToonBox::new(0);
    assert_eq!(b.len(), 0);
    assert!(b.is_empty());
}

#[test]
fn test_erased_box() {
    let b = ToonBox::erased();
    assert!(b.is_erased());
    assert!(b.is_empty());
}

#[test]
fn test_erased_box_with_size() {
    let b = ToonBox::erased_with_size(4);
    assert!(b.is_erased());
    assert_eq!(b.len(), 4);
}

#[test]
fn test_hole_operations() {
    let b = ToonBox::new(3);

    // All holes start empty
    assert!(b.hole(0).unwrap().is_empty());
    assert!(b.hole(1).unwrap().is_empty());
    assert!(b.hole(2).unwrap().is_empty());

    // Empty and filled counts
    assert_eq!(b.empty_count(), 3);
    assert_eq!(b.filled_count(), 0);
}

#[test]
fn test_clear_hole() {
    let mut b = ToonBox::new(2);
    assert!(b.clear_hole(0));
    assert!(b.hole(0).unwrap().is_empty());
}

#[test]
fn test_hole_index() {
    let hole = Hole::new(5);
    assert_eq!(hole.index(), 5);
    assert!(hole.is_empty());
}

#[test]
fn test_invalid_hole_index() {
    let b = ToonBox::new(2);
    assert!(b.hole(5).is_none());
}
