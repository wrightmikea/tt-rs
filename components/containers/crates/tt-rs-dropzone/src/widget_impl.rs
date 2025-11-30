//! Widget trait implementation for DropZone.

use crate::dropzone::DropZone;
use crate::rendering;
use tt_rs_core::{MatchResult, Widget, WidgetId};
use yew::Html;

impl Widget for DropZone {
    fn type_name(&self) -> &'static str {
        "dropzone"
    }

    fn id(&self) -> WidgetId {
        self.id
    }

    fn copy(&self) -> Box<dyn Widget> {
        Box::new(self.copy_dropzone())
    }

    fn matches(&self, other: &dyn Widget) -> MatchResult {
        // DropZones match other dropzones with the same label
        if other.type_name() != "dropzone" {
            return MatchResult::NoMatch;
        }

        if other.description() == self.description() {
            MatchResult::Match
        } else {
            MatchResult::NoMatch
        }
    }

    fn render(&self) -> Html {
        rendering::render(self)
    }

    fn description(&self) -> String {
        format!("dropzone \"{}\"", self.label)
    }
}
