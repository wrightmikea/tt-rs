//! Workspace list rendering for the workspace menu.

use super::{WorkspaceMenuProps, WorkspaceMetadata};
use yew::prelude::*;

/// Render the workspace list view.
pub fn render_workspace_list(props: &WorkspaceMenuProps, on_save_click: Callback<()>) -> Html {
    let on_import_change = {
        let on_import = props.on_import.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                    on_import.emit(file);
                }
            }
        })
    };

    // Separate user workspaces from bundled examples
    let user_workspaces: Vec<_> = props.workspaces.iter().filter(|w| !w.is_bundled).collect();

    let bundled_examples: Vec<_> = props.workspaces.iter().filter(|w| w.is_bundled).collect();

    html! {
        <div class="workspace-list">
            <div class="workspace-actions">
                <button class="workspace-action-btn save-btn" onclick={on_save_click.reform(|_| ())}>
                    { "Save Current Workspace" }
                </button>
                <label class="workspace-action-btn import-btn">
                    { "Import from File" }
                    <input
                        type="file"
                        accept=".json,.tt-rs.json"
                        onchange={on_import_change}
                        style="display: none;"
                    />
                </label>
            </div>

            if !user_workspaces.is_empty() {
                <div class="workspace-section">
                    <h3 class="workspace-section-title">{ "My Workspaces" }</h3>
                    { for user_workspaces.iter().map(|w| render_workspace_item(w, props, false)) }
                </div>
            }

            if !bundled_examples.is_empty() {
                <div class="workspace-section">
                    <h3 class="workspace-section-title">{ "Examples & Tutorials" }</h3>
                    { for bundled_examples.iter().map(|w| render_workspace_item(w, props, true)) }
                </div>
            }

            if user_workspaces.is_empty() && bundled_examples.is_empty() {
                <div class="workspace-empty">
                    <p>{ "No saved workspaces yet." }</p>
                    <p>{ "Create your first workspace by clicking \"Save Current Workspace\"." }</p>
                </div>
            }
        </div>
    }
}

fn render_workspace_item(
    workspace: &WorkspaceMetadata,
    props: &WorkspaceMenuProps,
    is_bundled: bool,
) -> Html {
    let id = workspace.id.clone();

    let on_load = {
        let on_load = props.on_load.clone();
        let id = id.clone();
        Callback::from(move |_| on_load.emit(id.clone()))
    };

    let on_export = {
        let on_export = props.on_export.clone();
        let id = id.clone();
        Callback::from(move |_| on_export.emit(id.clone()))
    };

    let on_delete = {
        let on_delete = props.on_delete.clone();
        let id = id.clone();
        Callback::from(move |_| on_delete.emit(id.clone()))
    };

    let icon = if is_bundled { "\u{1F4DA}" } else { "\u{2B50}" }; // book vs star

    html! {
        <div class="workspace-item">
            <div class="workspace-item-header">
                <span class="workspace-icon">{ icon }</span>
                <span class="workspace-name">{ &workspace.name }</span>
                <span class="workspace-level">{ format!("({})", &workspace.user_level) }</span>
            </div>
            <div class="workspace-description">
                { &workspace.description }
            </div>
            <div class="workspace-meta">
                { format!("Modified: {}", format_date(&workspace.modified_at)) }
            </div>
            <div class="workspace-item-actions">
                <button class="workspace-btn load-btn" onclick={on_load}>
                    { "Load" }
                </button>
                <button class="workspace-btn export-btn" onclick={on_export}>
                    { "Export" }
                </button>
                if !is_bundled {
                    <button class="workspace-btn delete-btn" onclick={on_delete}>
                        { "Delete" }
                    </button>
                }
            </div>
        </div>
    }
}

/// Format an ISO date string for display.
fn format_date(iso_date: &str) -> String {
    // Simple formatting - just take the date part
    if let Some(date_part) = iso_date.split('T').next() {
        date_part.to_string()
    } else {
        iso_date.to_string()
    }
}
