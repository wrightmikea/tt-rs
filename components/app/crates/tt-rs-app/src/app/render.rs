//! Rendering functions for the app.
//!
//! Uses a z-plane architecture for guaranteed widget stacking order.
//! Each plane is a transparent container with pointer-events: none,
//! allowing clicks to pass through to elements on lower planes.
//! Widgets within each plane have pointer-events: auto to receive events.

use tt_rs_core::WidgetId;
use tt_rs_drag::{CopySource, Draggable, DropEvent, Position};
use tt_rs_ui::{
    Footer, HelpButton, HelpPanel, Tooltip, TooltipLayer, TooltipPosition, UserLevel,
    UserLevelSelector, WorkspaceButton, WorkspaceMenu, WorkspaceMetadata,
};
use yew::prelude::*;

use super::callbacks::Callbacks;
use super::ZPlanes;
use crate::box_state::render_box;
use crate::state::AppState;
use crate::widget_item::WidgetItem;

/// Z-plane indices for guaranteed stacking order.
/// Higher numbers appear on top.
/// Help panel uses CSS z-index: 600-650 (see main.css).
const Z_PLANE_STACKS: i32 = 0;
const Z_PLANE_BOXES: i32 = 100;
const Z_PLANE_VALUES: i32 = 200;
const Z_PLANE_AGENTS: i32 = 300;
const Z_PLANE_TOOLS: i32 = 400;
const Z_PLANE_TOOLTIPS: i32 = 500;

pub fn render_app(
    state: &AppState,
    help_open: bool,
    workspace_open: bool,
    user_level: UserLevel,
    cbs: &Callbacks,
    planes: &ZPlanes<'_>,
    workspaces: &[WorkspaceMetadata],
) -> Html {
    html! {
        <div class="workspace">
            <div class="workspace-header">
                <span class="header-title">{"tt-rs - Visual Programming Environment"}</span>
                <WorkspaceButton on_click={cbs.on_workspace_open.clone()} />
                <UserLevelSelector level={user_level} on_change={cbs.on_level_change.clone()} />
            </div>
            <HelpButton on_click={cbs.on_help_open.clone()} />
            <HelpPanel is_open={help_open} on_close={cbs.on_help_close.clone()} level={user_level} />
            <WorkspaceMenu
                is_open={workspace_open}
                on_close={cbs.on_workspace_close.clone()}
                on_save={cbs.on_workspace_save.clone()}
                on_load={cbs.on_workspace_load.clone()}
                on_delete={cbs.on_workspace_delete.clone()}
                on_export={cbs.on_workspace_export.clone()}
                on_import={cbs.on_workspace_import.clone()}
                current_level={user_level}
                workspaces={workspaces.to_vec()}
            />
            <div class="workspace-content">
                // Z-plane 0: Copy source stacks (lowest)
                { render_z_plane(Z_PLANE_STACKS, render_copy_sources(&planes.copy_sources, state, &cbs.on_copy_source_click, &cbs.on_move)) }
                // Z-plane 100: Boxes
                { render_z_plane(Z_PLANE_BOXES, render_boxes(state, cbs)) }
                // Z-plane 200: Values (numbers, text)
                { render_z_plane(Z_PLANE_VALUES, render_widgets(&planes.values, state, &cbs.on_move, &cbs.on_drop)) }
                // Z-plane 300: Agents (robot, bird, nest, scales)
                { render_z_plane(Z_PLANE_AGENTS, render_widgets(&planes.agents, state, &cbs.on_move, &cbs.on_drop)) }
                // Z-plane 400: Tools (vacuum, wand)
                { render_z_plane(Z_PLANE_TOOLS, render_widgets(&planes.tools, state, &cbs.on_move, &cbs.on_drop)) }
                // Z-plane 500: Tooltips (highest)
                { render_z_plane(Z_PLANE_TOOLTIPS, html! { <TooltipLayer /> }) }
            </div>
            <Footer />
        </div>
    }
}

/// Renders a z-plane container with the given z-index.
/// The plane itself is transparent and doesn't capture mouse events,
/// but its children (the widgets) do.
fn render_z_plane(z_index: i32, children: Html) -> Html {
    let style = format!(
        "position: absolute; top: 0; left: 0; width: 100%; height: 100%; z-index: {}; pointer-events: none;",
        z_index
    );
    html! {
        <div class="z-plane" style={style}>
            { children }
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
    on_move: &Callback<(WidgetId, Position)>,
) -> Html {
    srcs.iter().map(|(id, w)| {
        let pos = state.positions.get(id).copied().unwrap_or_default();
        let tip = w.tooltip_info();
        html! {
            <CopySource widget_id={**id} position={pos} on_click={on_click.clone()} on_move={on_move.clone()}>
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
