//! Application state management.

use std::collections::HashMap;
use tt_rs_core::WidgetId;
use tt_rs_drag::Position;
use tt_rs_robot::Action;
use tt_rs_scales::Scales;

use crate::box_state::BoxState;
use crate::demo;
use crate::widget_item::WidgetItem;

/// Application state.
#[derive(Clone)]
pub struct AppState {
    pub widgets: HashMap<WidgetId, WidgetItem>,
    pub boxes: HashMap<WidgetId, BoxState>,
    pub positions: HashMap<WidgetId, Position>,
    pub widget_in_box: HashMap<WidgetId, (WidgetId, usize)>,
    pub training_robot_id: Option<WidgetId>,
}

impl AppState {
    pub fn new() -> Self {
        let (widgets, mut positions) = demo::init_widgets();
        let (boxes, box_positions) = demo::init_boxes();
        positions.extend(box_positions);

        Self {
            widgets,
            boxes,
            positions,
            widget_in_box: HashMap::new(),
            training_robot_id: None,
        }
    }

    pub fn record_action(&mut self, action: Action) {
        if let Some(rid) = self.training_robot_id {
            if let Some(WidgetItem::Robot(r)) = self.widgets.get_mut(&rid) {
                r.record_action(action);
            }
        }
    }

    pub fn update_scales_in_box(&mut self, box_id: WidgetId) {
        let contents = match self.boxes.get(&box_id) {
            Some(b) => b.contents.clone(),
            None => return,
        };

        for (&hole, &wid) in &contents {
            if let Some(WidgetItem::Scales(s)) = self.widgets.get(&wid) {
                let updated = update_scales(s, &contents, hole, &self.widgets);
                self.widgets.insert(wid, WidgetItem::Scales(updated));
            }
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

fn update_scales(
    scales: &Scales,
    contents: &HashMap<usize, WidgetId>,
    hole: usize,
    widgets: &HashMap<WidgetId, WidgetItem>,
) -> Scales {
    let mut s = scales.clone();
    if hole > 0 {
        if let Some(&lid) = contents.get(&(hole - 1)) {
            if let Some(WidgetItem::Number(n)) = widgets.get(&lid) {
                s.set_left(n.numerator());
            }
        }
    }
    if let Some(&rid) = contents.get(&(hole + 1)) {
        if let Some(WidgetItem::Number(n)) = widgets.get(&rid) {
            s.set_right(n.numerator());
        }
    }
    s
}
