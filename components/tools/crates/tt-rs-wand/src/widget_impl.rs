//! Widget trait implementation for Wand.

use crate::wand::Wand;
use tt_rs_core::{MatchResult, Widget, WidgetId};
use yew::prelude::*;

impl Widget for Wand {
    fn type_name(&self) -> &'static str {
        "wand"
    }

    fn id(&self) -> WidgetId {
        self.id
    }

    fn copy(&self) -> Box<dyn Widget> {
        Box::new(self.copy_wand())
    }

    fn matches(&self, other: &dyn Widget) -> MatchResult {
        if other.type_name() == "wand" {
            MatchResult::Match
        } else {
            MatchResult::NoMatch
        }
    }

    fn render(&self) -> Html {
        html! {
            <div class="widget wand tool"
                 data-widget-id={self.id.to_string()}>
                <img src="images/tt-wand.svg" alt="wand" class="wand-image" />
            </div>
        }
    }

    fn description(&self) -> String {
        "magic wand tool".to_string()
    }
}
