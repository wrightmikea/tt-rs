//! Widget trait implementation for Scales.

use crate::scales::{CompareResult, Scales};
use tt_rs_core::{MatchResult, Widget, WidgetId};
use yew::prelude::*;

impl Widget for Scales {
    fn type_name(&self) -> &'static str {
        "scales"
    }

    fn id(&self) -> WidgetId {
        self.id
    }

    fn copy(&self) -> Box<dyn Widget> {
        Box::new(Scales::new())
    }

    fn matches(&self, other: &dyn Widget) -> MatchResult {
        if other.type_name() == "scales" {
            MatchResult::Match
        } else {
            MatchResult::NoMatch
        }
    }

    fn render(&self) -> Html {
        let (tilt_class, image_src) = match self.result {
            CompareResult::Indeterminate => ("wobbling", "images/tt-scales.svg"),
            CompareResult::Balanced => ("balanced", "images/tt-scales.svg"),
            CompareResult::LeftHeavier => ("left-heavy", "images/tt-scales-left.svg"),
            CompareResult::RightHeavier => ("right-heavy", "images/tt-scales-right.svg"),
        };

        let left_display = self.left_value.map(|v| v.to_string()).unwrap_or_default();
        let right_display = self.right_value.map(|v| v.to_string()).unwrap_or_default();

        html! {
            <div class={classes!("widget", "scales", tilt_class)}
                 data-widget-id={self.id.to_string()}>
                <img src={image_src} alt="scales" class="scales-image" />
                <div class="scales-pans">
                    <div class="scales-pan left"
                         data-scales-id={self.id.to_string()}
                         data-pan="left">
                        <span class="pan-value">{left_display}</span>
                    </div>
                    <div class="scales-pan right"
                         data-scales-id={self.id.to_string()}
                         data-pan="right">
                        <span class="pan-value">{right_display}</span>
                    </div>
                </div>
            </div>
        }
    }

    fn description(&self) -> String {
        match self.result {
            CompareResult::Indeterminate => "wobbling scales".to_string(),
            CompareResult::Balanced => "balanced scales".to_string(),
            CompareResult::LeftHeavier => "scales tipping left".to_string(),
            CompareResult::RightHeavier => "scales tipping right".to_string(),
        }
    }
}
