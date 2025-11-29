//! Generic widget hit testing.

use tt_rs_core::WidgetId;
use wasm_bindgen::JsCast;
use web_sys::Element;

use crate::dom_helpers::{elements_at_point, parse_widget_id_attr};

/// Find any widget at the given position, excluding a specific widget.
/// Returns (widget_id, is_box).
pub fn find_widget_at_excluding(x: f64, y: f64, skip_id: WidgetId) -> Option<(WidgetId, bool)> {
    let elements = elements_at_point(x, y)?;

    for i in 0..elements.length() {
        if let Ok(element) = elements.get(i).dyn_into::<Element>() {
            if let Some(widget_element) = find_widget_element(&element) {
                if let Some(result) = try_extract_widget_id(&widget_element, skip_id) {
                    return Some(result);
                }
            }
        }
    }
    None
}

fn find_widget_element(element: &Element) -> Option<Element> {
    let mut current = Some(element.clone());
    while let Some(el) = current {
        if el.class_list().contains("widget") {
            let has_widget_id = el.has_attribute("data-widget-id");
            let has_box_id =
                el.has_attribute("data-box-id") && !el.class_list().contains("box-hole");
            if has_widget_id || has_box_id {
                return Some(el);
            }
        }
        current = el.parent_element();
    }
    None
}

fn try_extract_widget_id(element: &Element, skip_id: WidgetId) -> Option<(WidgetId, bool)> {
    if let Some(id) = parse_widget_id_attr(element, "data-widget-id") {
        if id != skip_id {
            return Some((id, false));
        }
    }

    if let Some(id) = parse_widget_id_attr(element, "data-box-id") {
        if id != skip_id {
            return Some((id, true));
        }
    }

    None
}
