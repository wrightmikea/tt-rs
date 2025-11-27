//! Widget trait definition.

use crate::WidgetId;
use yew::Html;

/// Result of pattern matching between widgets.
#[derive(Debug, Clone, PartialEq)]
pub enum MatchResult {
    /// Pattern matches the widget.
    Match,
    /// Pattern does not match.
    NoMatch,
}

/// The fundamental trait for all ToonTalk widgets.
pub trait Widget: std::fmt::Debug {
    /// Returns the type name (e.g., "number", "box").
    fn type_name(&self) -> &'static str;

    /// Returns the unique identifier.
    fn id(&self) -> WidgetId;

    /// Creates a deep copy with a new ID.
    fn copy(&self) -> Box<dyn Widget>;

    /// Checks if this widget matches another.
    fn matches(&self, other: &dyn Widget) -> MatchResult;

    /// Renders this widget as HTML.
    fn render(&self) -> Html;

    /// Returns a text description.
    fn description(&self) -> String;
}
