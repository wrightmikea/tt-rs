//! Demo scene initialization.

use std::collections::HashMap;
use tt_rs_core::WidgetId;
use tt_rs_drag::Position;
use tt_rs_number::{ArithOperator, Number};
use tt_rs_robot::Robot;
use tt_rs_scales::Scales;
use tt_rs_vacuum::Vacuum;
use tt_rs_wand::Wand;

use crate::box_state::BoxState;
use crate::widget_item::WidgetItem;

/// Initialize demo widgets and positions.
pub fn init_widgets() -> (HashMap<WidgetId, WidgetItem>, HashMap<WidgetId, Position>) {
    let mut widgets = HashMap::new();
    let mut positions = HashMap::new();

    for (i, widget) in demo_widgets().into_iter().enumerate() {
        let pos = widget_position(&widget, i);
        positions.insert(widget.id(), pos);
        widgets.insert(widget.id(), widget);
    }
    (widgets, positions)
}

/// Initialize demo boxes and positions.
pub fn init_boxes() -> (HashMap<WidgetId, BoxState>, HashMap<WidgetId, Position>) {
    let mut boxes = HashMap::new();
    let mut positions = HashMap::new();

    for (i, b) in demo_boxes().into_iter().enumerate() {
        positions.insert(b.id(), Position::new(20.0 + (i as f64) * 200.0, 290.0));
        boxes.insert(b.id(), b);
    }
    (boxes, positions)
}

fn widget_position(widget: &WidgetItem, index: usize) -> Position {
    let (start_x, start_y, spacing_x, spacing_y, cols) = (20.0, 50.0, 120.0, 80.0, 5);
    if matches!(widget, WidgetItem::Robot(_)) {
        Position::new(start_x + 5.0 * spacing_x, start_y)
    } else {
        Position::new(
            start_x + (index % cols) as f64 * spacing_x,
            start_y + (index / cols) as f64 * spacing_y,
        )
    }
}

fn demo_widgets() -> Vec<WidgetItem> {
    vec![
        WidgetItem::Number(Number::new(1).as_copy_source()),
        WidgetItem::Number(Number::new(5).as_copy_source()),
        WidgetItem::Number(arith_tool(1, ArithOperator::Subtract)),
        WidgetItem::Number(arith_tool(2, ArithOperator::Multiply)),
        WidgetItem::Number(arith_tool(2, ArithOperator::Divide)),
        WidgetItem::Number(Number::new(0)),
        WidgetItem::Scales(Scales::new()),
        WidgetItem::Vacuum(Vacuum::new()),
        WidgetItem::Wand(Wand::new()),
        WidgetItem::Robot(Robot::new()),
    ]
}

fn arith_tool(v: i64, op: ArithOperator) -> Number {
    Number::new(v).with_operator(op).as_copy_source()
}
fn demo_boxes() -> Vec<BoxState> {
    vec![BoxState::new(2), BoxState::new(3)]
}
