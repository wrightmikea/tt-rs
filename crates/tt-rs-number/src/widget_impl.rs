//! Widget trait implementation for Number.

use crate::number::{ErasureLevel, Number};
use tt_rs_core::{MatchResult, Widget, WidgetId};
use yew::prelude::*;

impl Widget for Number {
    fn type_name(&self) -> &'static str {
        "number"
    }

    fn id(&self) -> WidgetId {
        self.id
    }

    fn copy(&self) -> Box<dyn Widget> {
        Box::new(Number {
            id: WidgetId::new(),
            numerator: self.numerator,
            denominator: self.denominator,
            operator: self.operator,
            erasure: self.erasure,
        })
    }

    fn matches(&self, other: &dyn Widget) -> MatchResult {
        if other.type_name() != "number" {
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
        let value_str = format_value(self);
        let op = self.operator.symbol();
        html! {
            <div class="widget number">
                <div class="number-operator">{op}</div>
                <div class="number-value">{value_str}</div>
            </div>
        }
    }

    fn description(&self) -> String {
        if self.erasure == ErasureLevel::Value {
            "erased number".to_string()
        } else if self.is_integer() {
            format!("number {}", self.numerator)
        } else {
            format!("number {}/{}", self.numerator, self.denominator)
        }
    }
}

fn format_value(n: &Number) -> String {
    if n.erasure == ErasureLevel::Value {
        "?".to_string()
    } else if n.is_integer() {
        n.numerator.to_string()
    } else {
        format!("{}/{}", n.numerator, n.denominator)
    }
}
