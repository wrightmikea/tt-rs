//! DOM helper functions for hit testing.

use tt_rs_core::WidgetId;
use web_sys::Element;

/// Check if an element is inside a dragging element.
pub fn is_inside_dragging(element: &Element) -> bool {
    let mut current = Some(element.clone());
    while let Some(el) = current {
        let class_list = el.class_list();
        if class_list.contains("draggable") && class_list.contains("dragging") {
            return true;
        }
        current = el.parent_element();
    }
    false
}

/// Walk up the DOM tree to find an ancestor with the given class.
pub fn find_ancestor_with_class(element: &Element, class_name: &str) -> Option<Element> {
    let mut current = Some(element.clone());
    while let Some(el) = current {
        if el.class_list().contains(class_name) {
            return Some(el);
        }
        current = el.parent_element();
    }
    None
}

/// Parse a widget ID from an element attribute.
pub fn parse_widget_id_attr(element: &Element, attr_name: &str) -> Option<WidgetId> {
    element
        .get_attribute(attr_name)?
        .parse::<u64>()
        .ok()
        .map(WidgetId::from_u64)
}

/// Get elements at a point from the document.
pub fn elements_at_point(x: f64, y: f64) -> Option<js_sys::Array> {
    let document = web_sys::window()?.document()?;
    Some(document.elements_from_point(x as f32, y as f32))
}

/// Get the topmost element at a point.
pub fn element_at_point(x: f64, y: f64) -> Option<Element> {
    let document = web_sys::window()?.document()?;
    document.element_from_point(x as f32, y as f32)
}
