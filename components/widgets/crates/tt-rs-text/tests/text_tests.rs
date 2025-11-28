//! Tests for Text widget.

use tt_rs_core::Widget;
use tt_rs_text::Text;

#[test]
fn test_text_creation() {
    let text = Text::new("hello");
    assert_eq!(text.value(), "hello");
    assert_eq!(text.len(), 5);
    assert!(!text.is_empty());
}

#[test]
fn test_empty_text() {
    let text = Text::new("");
    assert!(text.is_empty());
    assert_eq!(text.len(), 0);
}

#[test]
fn test_first_char() {
    let text = Text::new("hello");
    assert_eq!(text.first_char(), Some('h'));

    let empty = Text::new("");
    assert_eq!(empty.first_char(), None);
}

#[test]
fn test_rest() {
    let text = Text::new("hello");
    let rest = text.rest();
    assert_eq!(rest.value(), "ello");

    let single = Text::new("x");
    let rest_single = single.rest();
    assert!(rest_single.is_empty());
}

#[test]
fn test_description() {
    let text = Text::new("hello");
    assert_eq!(text.description(), "text \"hello\"");

    let erased = Text::erased();
    assert_eq!(erased.description(), "erased text");
}

#[test]
fn test_type_name() {
    let text = Text::new("test");
    assert_eq!(text.type_name(), "text");
}
