//! Widget trait implementation for Nest.

use crate::nest::Nest;
use crate::rendering;
use tt_rs_core::{MatchResult, Widget, WidgetId};
use yew::Html;

impl Widget for Nest {
    fn type_name(&self) -> &'static str {
        "nest"
    }

    fn id(&self) -> WidgetId {
        self.id
    }

    fn copy(&self) -> Box<dyn Widget> {
        Box::new(self.copy_nest())
    }

    fn matches(&self, other: &dyn Widget) -> MatchResult {
        if other.type_name() != "nest" {
            return MatchResult::NoMatch;
        }
        // Nests match if they have the same color
        MatchResult::Match
    }

    fn render(&self) -> Html {
        rendering::render(self)
    }

    fn description(&self) -> String {
        let color_name = match self.color {
            crate::NestColor::Blue => "blue",
            crate::NestColor::Red => "red",
            crate::NestColor::Green => "green",
            crate::NestColor::Yellow => "yellow",
        };
        if self.messages.is_empty() {
            format!("{color_name} nest (empty)")
        } else {
            format!("{color_name} nest ({} messages)", self.messages.len())
        }
    }
}
