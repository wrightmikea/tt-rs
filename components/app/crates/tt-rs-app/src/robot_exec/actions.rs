//! Individual robot action handlers.

use tt_rs_drag::Position;
use tt_rs_number::{ArithOperator, Number};

use super::path_parse::{parse_box_hole_path, parse_widget_path};
use crate::state::AppState;
use crate::widget_item::WidgetItem;

pub fn execute_arithmetic(state: &mut AppState, op: char, num: i64, den: i64, path: &str) {
    let target_id = match parse_widget_path(path) {
        Some(id) => id,
        None => return,
    };
    let operator = char_to_op(op);
    let tool = Number::rational(num, den as u64).with_operator(operator);

    if let Some(WidgetItem::Number(n)) = state.widgets.get(&target_id) {
        let mut target = n.clone();
        if target.apply(&tool).is_some() {
            state.widgets.insert(target_id, WidgetItem::Number(target));
        }
    }
}

pub fn execute_copy(state: &mut AppState, path: &str) {
    let target_id = match parse_widget_path(path) {
        Some(id) => id,
        None => return,
    };
    let widget = match state.widgets.get(&target_id) {
        Some(w) => w.clone(),
        None => return,
    };
    let copy = copy_item(&widget);
    let pos = state.positions.get(&target_id).copied().unwrap_or_default();
    state
        .positions
        .insert(copy.id(), Position::new(pos.x + 30.0, pos.y + 30.0));
    state.widgets.insert(copy.id(), copy);
}

pub fn execute_remove(state: &mut AppState, path: &str) {
    let (box_id, hole) = match parse_box_hole_path(path) {
        Some(p) => p,
        None => return,
    };
    if let Some(b) = state.boxes.get_mut(&box_id) {
        if let Some(wid) = b.clear_hole(hole) {
            state.widget_in_box.remove(&wid);
            state.widgets.remove(&wid);
        }
    }
}

pub fn execute_drop(path: &str) {
    if let Some((box_id, hole)) = parse_box_hole_path(path) {
        log::info!(
            "Robot drop to box {} hole {} (needs held widget)",
            box_id,
            hole
        );
    }
}

fn char_to_op(c: char) -> ArithOperator {
    match c {
        '+' => ArithOperator::Add,
        '-' => ArithOperator::Subtract,
        '*' => ArithOperator::Multiply,
        '/' => ArithOperator::Divide,
        _ => ArithOperator::Add,
    }
}

fn copy_item(w: &WidgetItem) -> WidgetItem {
    match w {
        WidgetItem::Number(n) => WidgetItem::Number(n.copy_number()),
        WidgetItem::Text(t) => WidgetItem::Text(t.copy_text()),
        WidgetItem::Scales(s) => WidgetItem::Scales(s.copy_scales()),
        WidgetItem::Vacuum(v) => WidgetItem::Vacuum(v.copy_vacuum()),
        WidgetItem::Wand(w) => WidgetItem::Wand(w.copy_wand()),
        WidgetItem::Robot(r) => WidgetItem::Robot(r.copy_robot()),
        WidgetItem::Nest(n) => WidgetItem::Nest(n.copy_nest()),
        WidgetItem::Bird(b) => WidgetItem::Bird(b.copy_bird()),
    }
}
