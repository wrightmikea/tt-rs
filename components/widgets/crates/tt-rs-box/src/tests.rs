//! Tests for ToonBox widget.

#[cfg(test)]
mod tests {
    use crate::ToonBox;

    #[test]
    fn test_new_creates_box_with_specified_holes() {
        let box3 = ToonBox::new(3);
        assert_eq!(box3.len(), 3);

        let box0 = ToonBox::new(0);
        assert_eq!(box0.len(), 0);

        let box1 = ToonBox::new(1);
        assert_eq!(box1.len(), 1);
    }

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

    #[test]
    fn test_holes_are_initially_empty() {
        let box3 = ToonBox::new(3);
        for i in 0..3 {
            assert!(box3.hole(i).is_some());
            assert!(box3.hole(i).unwrap().is_empty());
        }
    }

    #[test]
    fn test_hole_out_of_bounds_returns_none() {
        let box2 = ToonBox::new(2);
        assert!(box2.hole(0).is_some());
        assert!(box2.hole(1).is_some());
        assert!(box2.hole(2).is_none());
        assert!(box2.hole(100).is_none());
    }

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
}
