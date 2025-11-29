//! Rendering functions for the app.

use tt_rs_core::WidgetId;
use tt_rs_drag::{CopySource, Draggable, DropEvent, Position};
use tt_rs_ui::{
    Footer, HelpButton, HelpPanel, Tooltip, TooltipPosition, UserLevel, UserLevelSelector,
};
use yew::prelude::*;

use super::callbacks::Callbacks;
use crate::box_state::render_box;
use crate::state::AppState;
use crate::widget_item::WidgetItem;

type WidgetRefs<'a> = Vec<(&'a WidgetId, &'a WidgetItem)>;

pub fn render_app(
    state: &AppState,
    help_open: bool,
    user_level: UserLevel,
    cbs: &Callbacks,
    copy_sources: &WidgetRefs<'_>,
    regular: &WidgetRefs<'_>,
) -> Html {
    html! {
        <div class="workspace">
            <div class="workspace-header">
                <span class="header-title">{"tt-rs - Visual Programming Environment"}</span>
                <UserLevelSelector level={user_level} on_change={cbs.on_level_change.clone()} />
            </div>
            <HelpButton on_click={cbs.on_help_open.clone()} />
            <HelpPanel is_open={help_open} on_close={cbs.on_help_close.clone()} level={user_level} />
            <div class="workspace-content">
                { render_boxes(state, cbs) }
                { render_copy_sources(copy_sources, state, &cbs.on_copy_source_click) }
                { render_widgets(regular, state, &cbs.on_move, &cbs.on_drop) }
            </div>
            <Footer />
        </div>
    }
}

fn render_boxes(state: &AppState, cbs: &Callbacks) -> Html {
    state.boxes.iter().map(|(id, b)| {
        let pos = state.positions.get(id).copied().unwrap_or_default();
        html! {
            <Draggable widget_id={*id} position={pos} on_move={cbs.on_move.clone()} on_drag_start={cbs.on_box_drag_start.clone()} on_drag_end={cbs.on_box_drag_end.clone()} on_drop={cbs.on_box_drop.clone()}>
                <Tooltip title="Box" description="A container with holes for storing items." hint="Drag items into holes. Drop on number to split. Press 0-9 while dragging to create copy with that many holes." position={TooltipPosition::Right}>
                    { render_box(b, &state.widgets) }
                </Tooltip>
            </Draggable>
        }
    }).collect()
}

fn render_copy_sources(
    srcs: &[(&WidgetId, &WidgetItem)],
    state: &AppState,
    on_click: &Callback<tt_rs_drag::CopySourceClickEvent>,
) -> Html {
    srcs.iter().map(|(id, w)| {
        let pos = state.positions.get(id).copied().unwrap_or_default();
        let tip = w.tooltip_info();
        html! {
            <CopySource widget_id={**id} position={pos} on_click={on_click.clone()}>
                <Tooltip title={tip.title} description={tip.description} hint={tip.hint} position={TooltipPosition::Right}>{ w.render() }</Tooltip>
            </CopySource>
        }
    }).collect()
}

fn render_widgets(
    ws: &[(&WidgetId, &WidgetItem)],
    state: &AppState,
    on_move: &Callback<(WidgetId, Position)>,
    on_drop: &Callback<DropEvent>,
) -> Html {
    ws.iter().map(|(id, w)| {
        let pos = state.positions.get(id).copied().unwrap_or_default();
        let tip = w.tooltip_info();
        html! {
            <Draggable widget_id={**id} position={pos} on_move={on_move.clone()} on_drop={on_drop.clone()}>
                <Tooltip title={tip.title} description={tip.description} hint={tip.hint} position={TooltipPosition::Right}>{ w.render() }</Tooltip>
            </Draggable>
        }
    }).collect()
}
