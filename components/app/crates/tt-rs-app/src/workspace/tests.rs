//! Tests for workspace serialization.

#[cfg(test)]
mod tests {
    use super::super::data::*;

    const PUZZLE_FILL_BOX: &str = include_str!("../../assets/puzzles/puzzle-fill-box.json");
    const PUZZLE_MAKE_FOUR: &str = include_str!("../../assets/puzzles/puzzle-make-four.json");
    const PUZZLE_MAKE_NINE: &str = include_str!("../../assets/puzzles/puzzle-make-nine.json");

    #[test]
    fn test_parse_puzzle_fill_box() {
        let workspace: Workspace =
            serde_json::from_str(PUZZLE_FILL_BOX).expect("Failed to parse puzzle-fill-box.json");

        assert_eq!(workspace.metadata.id, "puzzle-fill-box");
        assert_eq!(workspace.metadata.name, "Fill a Box");
        assert_eq!(workspace.metadata.user_level, "tt1");
        assert!(workspace.metadata.is_bundled);

        // Should have 3 widgets: two numbers and a dropzone
        assert_eq!(workspace.widgets.len(), 3);

        // Should have 1 box
        assert_eq!(workspace.boxes.len(), 1);
        assert_eq!(workspace.boxes[0].num_holes, 2);
    }

    #[test]
    fn test_parse_puzzle_make_four() {
        let workspace: Workspace =
            serde_json::from_str(PUZZLE_MAKE_FOUR).expect("Failed to parse puzzle-make-four.json");

        assert_eq!(workspace.metadata.id, "puzzle-make-four");
        assert_eq!(workspace.metadata.name, "Make a 4");

        // Should have 3 widgets: two 2s and a dropzone
        assert_eq!(workspace.widgets.len(), 3);

        // No boxes
        assert_eq!(workspace.boxes.len(), 0);
    }

    #[test]
    fn test_parse_puzzle_make_nine() {
        let workspace: Workspace =
            serde_json::from_str(PUZZLE_MAKE_NINE).expect("Failed to parse puzzle-make-nine.json");

        assert_eq!(workspace.metadata.id, "puzzle-make-nine");
        assert_eq!(workspace.metadata.name, "Make a 9");

        // Should have 2 widgets: one 3 (copy source) and a dropzone
        assert_eq!(workspace.widgets.len(), 2);

        // Check the 3 is a copy source
        if let WidgetData::Number(n) = &workspace.widgets[0] {
            assert_eq!(n.numerator, 3);
            assert!(n.is_copy_source);
        } else {
            panic!("Expected first widget to be a number");
        }
    }

    #[test]
    fn test_dropzone_expected_number() {
        let workspace: Workspace =
            serde_json::from_str(PUZZLE_MAKE_FOUR).expect("Failed to parse puzzle-make-four.json");

        // Find the dropzone
        let dropzone = workspace
            .widgets
            .iter()
            .find(|w| matches!(w, WidgetData::DropZone(_)));
        assert!(dropzone.is_some(), "Should have a dropzone");

        if let WidgetData::DropZone(dz) = dropzone.unwrap() {
            assert_eq!(dz.label, "I need a 4. Please drop it here.");

            // Expected should be a number 4
            let expected = dz.expected.as_ref().expect("Expected pattern should exist");
            if let WidgetData::Number(n) = expected.as_ref() {
                assert_eq!(n.numerator, 4);
            } else {
                panic!("Expected pattern should be a number");
            }
        }
    }

    #[test]
    fn test_dropzone_expected_box() {
        let workspace: Workspace =
            serde_json::from_str(PUZZLE_FILL_BOX).expect("Failed to parse puzzle-fill-box.json");

        // Find the dropzone
        let dropzone = workspace
            .widgets
            .iter()
            .find(|w| matches!(w, WidgetData::DropZone(_)));
        assert!(dropzone.is_some(), "Should have a dropzone");

        if let WidgetData::DropZone(dz) = dropzone.unwrap() {
            // Expected should be a box with [1, 2]
            let expected = dz.expected.as_ref().expect("Expected pattern should exist");
            if let WidgetData::Box(b) = expected.as_ref() {
                assert_eq!(b.num_holes, 2);
                assert_eq!(b.contents.len(), 2);
            } else {
                panic!("Expected pattern should be a box");
            }
        }
    }
}
