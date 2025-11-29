//! Number widget hit testing.

use tt_rs_core::WidgetId;

use crate::dom_helpers::{element_at_point, find_ancestor_with_class, parse_widget_id_attr};

/// Find which number widget (if any) is under the given mouse position.
pub fn find_number_at(x: f64, y: f64) -> Option<WidgetId> {
    let element = element_at_point(x, y)?;
    let number_element = find_ancestor_with_class(&element, "number")?;

    if number_element.has_attribute("data-widget-id") {
        parse_widget_id_attr(&number_element, "data-widget-id")
    } else {
        None
    }
}
