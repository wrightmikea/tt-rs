//! Widget trait implementation for Bird.

use crate::bird::Bird;
use crate::rendering;
use crate::BirdColor;
use tt_rs_core::{MatchResult, Widget, WidgetId};
use yew::Html;

impl Widget for Bird {
    fn type_name(&self) -> &'static str {
        "bird"
    }

    fn id(&self) -> WidgetId {
        self.id
    }

    fn copy(&self) -> Box<dyn Widget> {
        Box::new(self.copy_bird())
    }

    fn matches(&self, other: &dyn Widget) -> MatchResult {
        if other.type_name() != "bird" {
            return MatchResult::NoMatch;
        }
        // Birds match if they have the same color
        MatchResult::Match
    }

    fn render(&self) -> Html {
        rendering::render(self)
    }

    fn description(&self) -> String {
        let color_name = match self.color {
            BirdColor::Blue => "blue",
            BirdColor::Red => "red",
            BirdColor::Green => "green",
            BirdColor::Yellow => "yellow",
        };
        if self.nest_id.is_some() {
            format!("{color_name} bird (has nest)")
        } else {
            format!("{color_name} bird (no nest)")
        }
    }
}
