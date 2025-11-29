//! Box hole hit testing.

use tt_rs_core::WidgetId;
use wasm_bindgen::JsCast;
use web_sys::Element;

use crate::dom_helpers::{
    elements_at_point, find_ancestor_with_class, is_inside_dragging, parse_widget_id_attr,
};

/// Find which box hole (if any) is under the given mouse position.
pub fn find_box_hole_at(x: f64, y: f64) -> Option<(WidgetId, usize)> {
    let elements = elements_at_point(x, y)?;

    for i in 0..elements.length() {
        if let Ok(element) = elements.get(i).dyn_into::<Element>() {
            if is_inside_dragging(&element) {
                continue;
            }

            if let Some(result) = try_find_box_hole(&element) {
                return Some(result);
            }
        }
    }

    None
}

fn try_find_box_hole(element: &Element) -> Option<(WidgetId, usize)> {
    let hole_element = find_ancestor_with_class(element, "box-hole")?;
    let box_id = parse_widget_id_attr(&hole_element, "data-box-id")?;
    let hole_index = hole_element
        .get_attribute("data-hole-index")?
        .parse()
        .ok()?;
    Some((box_id, hole_index))
}
