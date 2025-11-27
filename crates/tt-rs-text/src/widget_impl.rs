//! Widget trait implementation for Text.

use crate::text::{ErasureLevel, Text};
use tt_rs_core::{MatchResult, Widget, WidgetId};
use yew::prelude::*;

impl Widget for Text {
    fn type_name(&self) -> &'static str {
        "text"
    }

    fn id(&self) -> WidgetId {
        self.id
    }

    fn copy(&self) -> Box<dyn Widget> {
        Box::new(Text {
            id: WidgetId::new(),
            value: self.value.clone(),
            erasure: self.erasure,
        })
    }

    fn matches(&self, other: &dyn Widget) -> MatchResult {
        if other.type_name() != "text" {
            return MatchResult::NoMatch;
        }
        let matches =
            self.erasure == ErasureLevel::Value || other.description() == self.description();
        if matches {
            MatchResult::Match
        } else {
            MatchResult::NoMatch
        }
    }

    fn render(&self) -> Html {
        let display = if self.erasure == ErasureLevel::Value {
            "?".to_string()
        } else {
            format!("\"{}\"", self.value)
        };
        html! {
            <div class="widget text">
                <div class="text-value">{display}</div>
            </div>
        }
    }

    fn description(&self) -> String {
        if self.erasure == ErasureLevel::Value {
            "erased text".to_string()
        } else {
            format!("text \"{}\"", self.value)
        }
    }
}
