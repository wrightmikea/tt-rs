//! Widget trait implementation for Vacuum.

use crate::vacuum::Vacuum;
use tt_rs_core::{MatchResult, Widget, WidgetId};
use yew::prelude::*;

impl Widget for Vacuum {
    fn type_name(&self) -> &'static str {
        "vacuum"
    }

    fn id(&self) -> WidgetId {
        self.id
    }

    fn copy(&self) -> Box<dyn Widget> {
        Box::new(self.copy_vacuum())
    }

    fn matches(&self, other: &dyn Widget) -> MatchResult {
        if other.type_name() == "vacuum" {
            MatchResult::Match
        } else {
            MatchResult::NoMatch
        }
    }

    fn render(&self) -> Html {
        html! {
            <div class="widget vacuum tool"
                 data-widget-id={self.id.to_string()}>
                <img src="images/tt-vacuum.svg" alt="vacuum" class="vacuum-image" />
            </div>
        }
    }

    fn description(&self) -> String {
        "vacuum tool".to_string()
    }
}
