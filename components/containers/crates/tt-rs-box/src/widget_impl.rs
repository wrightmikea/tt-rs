//! Widget trait implementation for ToonBox.

use crate::{rendering, ToonBox};
use tt_rs_core::{MatchResult, Widget, WidgetId};
use yew::Html;

impl Widget for ToonBox {
    fn type_name(&self) -> &'static str {
        "box"
    }

    fn id(&self) -> WidgetId {
        self.id
    }

    fn render(&self) -> Html {
        rendering::render(self)
    }

    fn copy(&self) -> Box<dyn Widget> {
        let mut new_box = ToonBox::new(self.len());
        new_box.erased = self.is_erased();
        Box::new(new_box)
    }

    fn matches(&self, other: &dyn Widget) -> MatchResult {
        rendering::matches(self, other)
    }

    fn description(&self) -> String {
        rendering::describe(self)
    }
}
