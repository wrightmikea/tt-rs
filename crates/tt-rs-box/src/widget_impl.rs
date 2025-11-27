//! Widget trait implementation for ToonBox.

use crate::ToonBox;
use tt_rs_core::{MatchResult, Widget, WidgetId};
use yew::{Html, html};

impl Widget for ToonBox {
    fn type_name(&self) -> &'static str {
        "box"
    }

    fn id(&self) -> WidgetId {
        self.id
    }

    fn render(&self) -> Html {
        if self.erased {
            // Render erased box pattern
            let size_indicator = if self.holes.is_empty() {
                "?".to_string()
            } else {
                format!("?[{}]", self.holes.len())
            };
            html! {
                <div class="widget box erased">
                    <span class="box-erased">{ size_indicator }</span>
                </div>
            }
        } else {
            // Render normal box with holes
            html! {
                <div class="widget box">
                    <div class="box-holes">
                        { for self.holes.iter().map(|hole| {
                            let content = if let Some(content) = hole.content() {
                                content.html.clone()
                            } else {
                                html! { <span class="hole-empty">{ "\u{00A0}" }</span> }
                            };
                            html! {
                                <div class="box-hole">
                                    { content }
                                </div>
                            }
                        })}
                    </div>
                </div>
            }
        }
    }

    fn copy(&self) -> Box<dyn Widget> {
        let mut new_box = ToonBox::new(self.holes.len());
        new_box.erased = self.erased;
        // Note: Contents are not deep-copied, only the structure
        Box::new(new_box)
    }

    fn matches(&self, other: &dyn Widget) -> MatchResult {
        // Must be a box
        if other.type_name() != "box" {
            return MatchResult::NoMatch;
        }

        // Erased box with no size constraint matches any box
        if self.erased && self.holes.is_empty() {
            return MatchResult::Match;
        }

        // Parse the other box's size from its description
        // Description format: "box[filled/total]" or "erased box" or "erased box[size]"
        let other_desc = other.description();
        let other_size = parse_box_size(&other_desc);

        // Erased box with size constraint matches boxes of same size
        if self.erased {
            return if self.holes.len() == other_size {
                MatchResult::Match
            } else {
                MatchResult::NoMatch
            };
        }

        // Non-erased boxes must have same size
        if self.holes.len() != other_size {
            return MatchResult::NoMatch;
        }

        // For simple matching, same size is enough
        // More detailed content matching would require access to the actual other box
        MatchResult::Match
    }

    fn description(&self) -> String {
        if self.erased {
            if self.holes.is_empty() {
                "erased box".to_string()
            } else {
                format!("erased box[{}]", self.holes.len())
            }
        } else {
            let filled = self.filled_count();
            let total = self.holes.len();
            format!("box[{}/{}]", filled, total)
        }
    }
}

/// Parse box size from description string.
fn parse_box_size(desc: &str) -> usize {
    if desc.starts_with("erased box[") {
        // "erased box[4]"
        desc.trim_start_matches("erased box[")
            .trim_end_matches(']')
            .parse()
            .unwrap_or(0)
    } else if desc.starts_with("box[") {
        // "box[2/4]" -> extract the total (after /)
        let inner = desc.trim_start_matches("box[").trim_end_matches(']');
        if let Some(idx) = inner.find('/') {
            inner[idx + 1..].parse().unwrap_or(0)
        } else {
            0
        }
    } else {
        // "erased box" with no size
        0
    }
}
