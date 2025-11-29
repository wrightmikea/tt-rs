//! Box rendering functions.

use std::collections::HashMap;
use tt_rs_core::WidgetId;
use yew::prelude::*;

use super::BoxState;
use crate::widget_item::{render_small, WidgetItem};

/// Render a box with its contents.
pub fn render_box(box_state: &BoxState, widgets: &HashMap<WidgetId, WidgetItem>) -> Html {
    if box_state.erased {
        render_erased(box_state)
    } else {
        render_normal(box_state, widgets)
    }
}

fn render_erased(b: &BoxState) -> Html {
    html! { <div class="widget box erased" data-box-id={b.id().to_string()}><span class="box-erased">{"?"}</span></div> }
}

fn render_normal(b: &BoxState, widgets: &HashMap<WidgetId, WidgetItem>) -> Html {
    html! {
        <div class="widget box" data-box-id={b.id().to_string()}>
            <div class="box-holes">{ for (0..b.num_holes).map(|i| render_hole(b, widgets, i)) }</div>
        </div>
    }
}

fn render_hole(b: &BoxState, widgets: &HashMap<WidgetId, WidgetItem>, idx: usize) -> Html {
    let content = b
        .contents
        .get(&idx)
        .and_then(|wid| widgets.get(wid))
        .map(render_small)
        .unwrap_or_else(|| html! { <span class="hole-empty">{"\u{00A0}"}</span> });
    html! { <div class="box-hole" data-box-id={b.id().to_string()} data-hole-index={idx.to_string()}>{ content }</div> }
}
