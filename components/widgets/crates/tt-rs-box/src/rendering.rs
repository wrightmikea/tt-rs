//! Rendering and description functions for ToonBox.

use crate::ToonBox;
use tt_rs_core::MatchResult;
use yew::{html, Html};

/// Renders a ToonBox as HTML.
pub fn render(b: &ToonBox) -> Html {
    if b.is_erased() {
        render_erased(b)
    } else {
        render_normal(b)
    }
}

fn render_erased(b: &ToonBox) -> Html {
    let size_indicator = if b.is_empty() {
        "?".to_string()
    } else {
        format!("?[{}]", b.len())
    };
    html! {
        <div class="widget box erased">
            <span class="box-erased">{ size_indicator }</span>
        </div>
    }
}

fn render_normal(b: &ToonBox) -> Html {
    html! {
        <div class="widget box">
            <div class="box-holes">
                { for b.holes().map(|hole| {
                    let content = if let Some(c) = hole.content() {
                        c.html.clone()
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

/// Returns a description of the box.
pub fn describe(b: &ToonBox) -> String {
    if b.is_erased() {
        if b.is_empty() {
            "erased box".to_string()
        } else {
            format!("erased box[{}]", b.len())
        }
    } else {
        format!("box[{}/{}]", b.filled_count(), b.len())
    }
}

/// Checks if a ToonBox pattern matches another widget.
pub fn matches(b: &ToonBox, other: &dyn tt_rs_core::Widget) -> MatchResult {
    if other.type_name() != "box" {
        return MatchResult::NoMatch;
    }
    if b.is_erased() && b.is_empty() {
        return MatchResult::Match;
    }
    let other_size = parse_box_size(&other.description());
    if b.is_erased() {
        return if b.len() == other_size {
            MatchResult::Match
        } else {
            MatchResult::NoMatch
        };
    }
    if b.len() != other_size {
        return MatchResult::NoMatch;
    }
    MatchResult::Match
}

fn parse_box_size(desc: &str) -> usize {
    if desc.starts_with("erased box[") {
        desc.trim_start_matches("erased box[")
            .trim_end_matches(']')
            .parse()
            .unwrap_or(0)
    } else if desc.starts_with("box[") {
        let inner = desc.trim_start_matches("box[").trim_end_matches(']');
        inner
            .find('/')
            .map_or(0, |idx| inner[idx + 1..].parse().unwrap_or(0))
    } else {
        0
    }
}
