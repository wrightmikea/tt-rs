//! Scales widget hit testing.

use tt_rs_core::WidgetId;

use crate::dom_helpers::{element_at_point, find_ancestor_with_class, parse_widget_id_attr};

/// Which pan of a scales was targeted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScalesPan {
    Left,
    Right,
}

/// Find which scales pan (if any) is under the given mouse position.
pub fn find_scales_pan_at(x: f64, y: f64) -> Option<(WidgetId, ScalesPan)> {
    let element = element_at_point(x, y)?;
    let pan_element = find_ancestor_with_class(&element, "scales-pan")?;

    let scales_id = parse_widget_id_attr(&pan_element, "data-scales-id")?;
    let pan = match pan_element.get_attribute("data-pan")?.as_str() {
        "left" => ScalesPan::Left,
        "right" => ScalesPan::Right,
        _ => return None,
    };

    Some((scales_id, pan))
}
