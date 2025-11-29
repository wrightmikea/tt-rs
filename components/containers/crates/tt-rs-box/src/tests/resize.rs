//! Tests for ToonBox resize operations.

use crate::ToonBox;

#[test]
fn test_resize_increases_holes() {
    let mut box2 = ToonBox::new(2);
    assert_eq!(box2.len(), 2);

    box2.resize(5);
    assert_eq!(box2.len(), 5);
}

#[test]
fn test_resize_decreases_holes() {
    let mut box5 = ToonBox::new(5);
    assert_eq!(box5.len(), 5);

    box5.resize(2);
    assert_eq!(box5.len(), 2);
}

#[test]
fn test_resize_to_zero() {
    let mut box3 = ToonBox::new(3);
    box3.resize(0);
    assert_eq!(box3.len(), 0);
}

#[test]
fn test_resize_same_size_is_noop() {
    let mut box3 = ToonBox::new(3);
    box3.resize(3);
    assert_eq!(box3.len(), 3);
}
