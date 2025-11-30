//! Drop zone hit testing.

use tt_rs_core::WidgetId;
use wasm_bindgen::JsCast;
use web_sys::Element;

use super::dom_helpers::{elements_at_point, is_inside_dragging, parse_widget_id_attr};

/// Find a drop zone at the given screen coordinates.
/// Returns the WidgetId of the dropzone if found.
pub fn find_dropzone_at(x: f64, y: f64) -> Option<WidgetId> {
    let elements = elements_at_point(x, y)?;

    for i in 0..elements.length() {
        let element: Element = elements.get(i).dyn_into().ok()?;

        // Skip elements that are being dragged
        if is_inside_dragging(&element) {
            continue;
        }

        // Check if this element or an ancestor is a dropzone
        if let Some(dropzone_id) = find_dropzone_ancestor(&element) {
            return Some(dropzone_id);
        }
    }

    None
}

/// Walk up the DOM tree to find a dropzone ancestor.
fn find_dropzone_ancestor(element: &Element) -> Option<WidgetId> {
    let mut current = Some(element.clone());
    while let Some(el) = current {
        if el.class_list().contains("dropzone") {
            return parse_widget_id_attr(&el, "data-widget-id");
        }
        current = el.parent_element();
    }
    None
}
