//! WorkspaceMenu component for save/load operations.
//!
//! Provides a slide panel with workspace listing, save dialog, and import/export.

mod list;
mod save_form;

use crate::slide_panel::SlidePanel;
use crate::user_level::UserLevel;
use yew::prelude::*;

pub use save_form::SaveFormData;

// Re-export WorkspaceListItem when it becomes used
#[allow(unused_imports)]
use list::render_workspace_list;

/// Workspace metadata for display in the list.
#[derive(Clone, PartialEq, Debug)]
pub struct WorkspaceMetadata {
    /// Unique identifier.
    pub id: String,
    /// User-provided name.
    pub name: String,
    /// User-provided description.
    pub description: String,
    /// User level when saved.
    pub user_level: String,
    /// Last modified timestamp (ISO 8601).
    pub modified_at: String,
    /// Whether this is a bundled example (read-only).
    pub is_bundled: bool,
}

/// Dialog mode for the workspace menu.
#[derive(Clone, PartialEq, Debug, Default)]
pub enum WorkspaceMenuMode {
    #[default]
    List,
    Save,
}

/// Properties for the WorkspaceMenu component.
#[derive(Properties, Clone, PartialEq)]
pub struct WorkspaceMenuProps {
    /// Whether the menu is open.
    pub is_open: bool,
    /// Callback when close is requested.
    pub on_close: Callback<()>,
    /// Callback when save is requested with form data.
    pub on_save: Callback<SaveFormData>,
    /// Callback when load is requested with workspace ID.
    pub on_load: Callback<String>,
    /// Callback when delete is requested with workspace ID.
    pub on_delete: Callback<String>,
    /// Callback when export is requested with workspace ID.
    pub on_export: Callback<String>,
    /// Callback when import file is selected.
    pub on_import: Callback<web_sys::File>,
    /// Current user level.
    pub current_level: UserLevel,
    /// List of available workspaces.
    pub workspaces: Vec<WorkspaceMetadata>,
}

/// Workspace menu slide panel.
#[function_component(WorkspaceMenu)]
pub fn workspace_menu(props: &WorkspaceMenuProps) -> Html {
    let mode = use_state(WorkspaceMenuMode::default);

    let on_save_click = {
        let mode = mode.clone();
        Callback::from(move |_| mode.set(WorkspaceMenuMode::Save))
    };

    let on_cancel_save = {
        let mode = mode.clone();
        Callback::from(move |_| mode.set(WorkspaceMenuMode::List))
    };

    let on_save_submit = {
        let on_save = props.on_save.clone();
        let mode = mode.clone();
        Callback::from(move |data: SaveFormData| {
            on_save.emit(data);
            mode.set(WorkspaceMenuMode::List);
        })
    };

    // Reset mode when menu closes
    {
        let mode = mode.clone();
        let is_open = props.is_open;
        use_effect_with(is_open, move |&open| {
            if !open {
                mode.set(WorkspaceMenuMode::List);
            }
        });
    }

    let title = match *mode {
        WorkspaceMenuMode::List => "Workspaces",
        WorkspaceMenuMode::Save => "Save Workspace",
    };

    let content = match *mode {
        WorkspaceMenuMode::List => list::render_workspace_list(props, on_save_click),
        WorkspaceMenuMode::Save => {
            save_form::render_save_form(props.current_level, on_save_submit, on_cancel_save)
        }
    };

    html! {
        <SlidePanel
            is_open={props.is_open}
            on_close={props.on_close.clone()}
            title={title}
        >
            { content }
        </SlidePanel>
    }
}
