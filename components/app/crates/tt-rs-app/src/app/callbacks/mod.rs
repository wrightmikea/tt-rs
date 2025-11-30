//! Callback handler creation.

mod box_handlers;
mod widget_handlers;

use std::cell::RefCell;
use std::rc::Rc;
use tt_rs_core::WidgetId;
use tt_rs_drag::{CopySourceClickEvent, DragEndEvent, DragStartEvent, DropEvent, Position};
use tt_rs_ui::{SaveFormData, UserLevel};
use yew::prelude::*;

use crate::state::{default_notes_for_level, AppState};

pub struct Callbacks {
    pub on_help_open: Callback<()>,
    pub on_help_close: Callback<()>,
    pub on_level_change: Callback<UserLevel>,
    pub on_box_drag_start: Callback<DragStartEvent>,
    pub on_box_drag_end: Callback<DragEndEvent>,
    pub on_box_drop: Callback<DropEvent>,
    pub on_copy_source_click: Callback<CopySourceClickEvent>,
    pub on_move: Callback<(WidgetId, Position)>,
    pub on_drop: Callback<DropEvent>,
    // Workspace callbacks
    pub on_workspace_open: Callback<()>,
    pub on_workspace_close: Callback<()>,
    pub on_workspace_save: Callback<SaveFormData>,
    pub on_workspace_load: Callback<String>,
    pub on_workspace_delete: Callback<String>,
    pub on_workspace_export: Callback<String>,
    pub on_workspace_import: Callback<web_sys::File>,
    // TextPane callbacks
    pub on_text_pane_change: Callback<String>,
    pub on_text_pane_resize: Callback<(f64, f64)>,
    pub on_text_pane_move: Callback<Position>,
}

pub fn create_callbacks(
    state: UseStateHandle<AppState>,
    help_open: UseStateHandle<bool>,
    user_level: UseStateHandle<UserLevel>,
    workspace_open: UseStateHandle<bool>,
    dragged_box_id: Rc<RefCell<Option<WidgetId>>>,
    pending_new_box: Rc<RefCell<Option<usize>>>,
) -> Callbacks {
    Callbacks {
        on_help_open: {
            let h = help_open.clone();
            Callback::from(move |_| h.set(true))
        },
        on_help_close: {
            let h = help_open;
            Callback::from(move |_| h.set(false))
        },
        on_level_change: {
            let s = state.clone();
            Callback::from(move |level: UserLevel| {
                user_level.set(level);
                // Update text pane content when level changes
                let mut new_state = (*s).clone();
                new_state.text_pane_content = default_notes_for_level(level).to_string();
                s.set(new_state);
            })
        },
        on_box_drag_start: box_handlers::create_box_drag_start(
            dragged_box_id.clone(),
            pending_new_box.clone(),
        ),
        on_box_drag_end: box_handlers::create_box_drag_end(dragged_box_id, pending_new_box.clone()),
        on_box_drop: box_handlers::create_box_drop(state.clone(), pending_new_box),
        on_copy_source_click: widget_handlers::create_copy_source(state.clone()),
        on_move: widget_handlers::create_move(state.clone()),
        on_drop: widget_handlers::create_widget_drop(state.clone()),
        // Workspace callbacks - stubs for now, will be implemented with storage
        on_workspace_open: {
            let w = workspace_open.clone();
            Callback::from(move |_| w.set(true))
        },
        on_workspace_close: {
            let w = workspace_open;
            Callback::from(move |_| w.set(false))
        },
        on_workspace_save: Callback::from(|data: SaveFormData| {
            log::info!("Save workspace: {} - {}", data.name, data.description);
            // TODO: Implement actual save with storage backend
        }),
        on_workspace_load: Callback::from(|id: String| {
            log::info!("Load workspace: {}", id);
            // TODO: Implement actual load with storage backend
        }),
        on_workspace_delete: Callback::from(|id: String| {
            log::info!("Delete workspace: {}", id);
            // TODO: Implement actual delete with storage backend
        }),
        on_workspace_export: Callback::from(|id: String| {
            log::info!("Export workspace: {}", id);
            // TODO: Implement actual export to file
        }),
        on_workspace_import: Callback::from(|_file: web_sys::File| {
            log::info!("Import workspace from file");
            // TODO: Implement actual import from file
        }),
        on_text_pane_change: {
            let s = state.clone();
            Callback::from(move |content: String| {
                let mut new_state = (*s).clone();
                new_state.text_pane_content = content;
                s.set(new_state);
            })
        },
        on_text_pane_resize: {
            let s = state.clone();
            Callback::from(move |size: (f64, f64)| {
                let mut new_state = (*s).clone();
                new_state.text_pane_size = size;
                s.set(new_state);
            })
        },
        on_text_pane_move: {
            let s = state;
            Callback::from(move |pos: Position| {
                let mut new_state = (*s).clone();
                new_state.text_pane_position = pos;
                s.set(new_state);
            })
        },
    }
}
