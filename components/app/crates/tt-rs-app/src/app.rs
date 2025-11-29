//! Main application component.

use std::collections::HashMap;
use tt_rs_core::{Widget, WidgetId};
use tt_rs_drag::{
    CopySource, CopySourceClickEvent, DragEndEvent, DragStartEvent, Draggable, DropEvent, Position,
};
use tt_rs_number::{ArithOperator, Number};
use tt_rs_robot::{Action, Robot, RobotState};
use tt_rs_scales::{CompareResult, Scales};
use tt_rs_text::Text;
use tt_rs_ui::{Footer, HelpButton, HelpPanel, Tooltip, TooltipPosition};
use tt_rs_vacuum::Vacuum;
use tt_rs_wand::Wand;
use web_sys::Element;
use yew::prelude::*;

/// Check if debug logging is enabled.
/// In WASM, we check for a global JS variable `window.TT_DEBUG`.
fn is_debug_enabled() -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(val) = js_sys::Reflect::get(&window, &"TT_DEBUG".into()) {
                return val.is_truthy();
            }
        }
        false
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::env::var("TT_DEBUG")
            .map(|v| v == "true")
            .unwrap_or(false)
    }
}

/// Log a debug message only if TT_DEBUG is enabled.
macro_rules! debug_log {
    ($($arg:tt)*) => {
        if is_debug_enabled() {
            log::info!($($arg)*);
        }
    };
}

/// Tooltip information for a widget.
struct TooltipInfo {
    title: &'static str,
    description: &'static str,
    hint: &'static str,
}

/// A widget item with its type for rendering.
#[derive(Clone)]
#[allow(dead_code)]
enum WidgetItem {
    Number(Number),
    Text(Text),
    Scales(Scales),
    Vacuum(Vacuum),
    Wand(Wand),
    Robot(Robot),
}

impl WidgetItem {
    fn id(&self) -> WidgetId {
        match self {
            WidgetItem::Number(n) => n.id(),
            WidgetItem::Text(t) => t.id(),
            WidgetItem::Scales(s) => s.id(),
            WidgetItem::Vacuum(v) => v.id(),
            WidgetItem::Wand(w) => w.id(),
            WidgetItem::Robot(r) => r.id(),
        }
    }

    fn render(&self) -> Html {
        match self {
            WidgetItem::Number(n) => n.render(),
            WidgetItem::Text(t) => t.render(),
            WidgetItem::Scales(s) => s.render(),
            WidgetItem::Vacuum(v) => v.render(),
            WidgetItem::Wand(w) => w.render(),
            WidgetItem::Robot(r) => r.render(),
        }
    }

    fn render_small(&self) -> Html {
        // Render a smaller version for inside box holes
        match self {
            WidgetItem::Number(n) => {
                html! {
                    <div class="widget number in-hole">
                        <span class="number-operator">{ n.operator().symbol() }</span>
                        <span class="number-value">{ n.display_value() }</span>
                    </div>
                }
            }
            WidgetItem::Text(t) => {
                html! {
                    <div class="widget text in-hole">
                        <span class="text-value">{ format!("\"{}\"", t.value()) }</span>
                    </div>
                }
            }
            WidgetItem::Scales(s) => {
                let (class_modifier, image_src) = match s.result() {
                    CompareResult::Indeterminate => ("wobbling", "images/tt-scales.svg"),
                    CompareResult::Balanced => ("balanced", "images/tt-scales.svg"),
                    CompareResult::LeftHeavier => ("left-heavy", "images/tt-scales-left.svg"),
                    CompareResult::RightHeavier => ("right-heavy", "images/tt-scales-right.svg"),
                };
                html! {
                    <div class={format!("widget scales in-hole {class_modifier}")}>
                        <img src={image_src} alt="scales" class="scales-image-small" />
                    </div>
                }
            }
            WidgetItem::Vacuum(_) => {
                html! { <div class="widget vacuum in-hole">{"[vacuum]"}</div> }
            }
            WidgetItem::Wand(_) => {
                html! { <div class="widget wand in-hole">{"[wand]"}</div> }
            }
            WidgetItem::Robot(_) => {
                html! { <div class="widget robot in-hole">{"[robot]"}</div> }
            }
        }
    }

    /// Returns true if this widget is a vacuum tool.
    fn is_vacuum(&self) -> bool {
        matches!(self, WidgetItem::Vacuum(_))
    }

    /// Returns true if this widget is a wand tool.
    fn is_wand(&self) -> bool {
        matches!(self, WidgetItem::Wand(_))
    }

    /// Returns true if this widget is a robot.
    fn is_robot(&self) -> bool {
        matches!(self, WidgetItem::Robot(_))
    }

    /// Get tooltip information for this widget.
    fn tooltip_info(&self) -> TooltipInfo {
        match self {
            WidgetItem::Number(n) => {
                if n.is_copy_source() {
                    match n.operator() {
                        ArithOperator::Add => TooltipInfo {
                            title: "Number Source",
                            description: "Click to create a copy of this number.",
                            hint: "Drag copies onto other numbers to add them.",
                        },
                        ArithOperator::Subtract => TooltipInfo {
                            title: "Subtraction Tool",
                            description: "Click to create a subtraction operation.",
                            hint: "Drag onto a number to subtract this value.",
                        },
                        ArithOperator::Multiply => TooltipInfo {
                            title: "Multiplication Tool",
                            description: "Click to create a multiplication operation.",
                            hint: "Drag onto a number to multiply by this value.",
                        },
                        ArithOperator::Divide => TooltipInfo {
                            title: "Division Tool",
                            description: "Click to create a division operation.",
                            hint: "Drag onto a number to divide by this value.",
                        },
                    }
                } else {
                    TooltipInfo {
                        title: "Number",
                        description: "A numeric value you can manipulate.",
                        hint: "Drop arithmetic tools on this to change its value.",
                    }
                }
            }
            WidgetItem::Text(_) => TooltipInfo {
                title: "Text",
                description: "A text string.",
                hint: "Drag into box holes to store.",
            },
            WidgetItem::Scales(_) => TooltipInfo {
                title: "Scales",
                description: "Compare two numbers by dropping them on the pans.",
                hint: "The scales tip toward the larger number.",
            },
            WidgetItem::Vacuum(_) => TooltipInfo {
                title: "Vacuum",
                description: "Erases items it touches.",
                hint: "Drop on box holes to erase contents, or on numbers to delete them.",
            },
            WidgetItem::Wand(_) => TooltipInfo {
                title: "Magic Wand",
                description: "Creates copies of items it touches.",
                hint: "Drop on any widget to create a duplicate.",
            },
            WidgetItem::Robot(_) => TooltipInfo {
                title: "Robot",
                description: "Learns by watching your actions and can repeat them.",
                hint: "Click to start/stop training, click again to run.",
            },
        }
    }

    /// Returns a mutable reference to the robot if this is a robot widget.
    fn as_robot_mut(&mut self) -> Option<&mut Robot> {
        match self {
            WidgetItem::Robot(r) => Some(r),
            _ => None,
        }
    }
}

/// A box with its holes that can contain widgets.
#[derive(Clone)]
#[allow(dead_code)]
struct BoxState {
    id: WidgetId,
    num_holes: usize,
    /// Map from hole index to the widget ID in that hole.
    contents: HashMap<usize, WidgetId>,
    erased: bool,
}

#[allow(dead_code)]
impl BoxState {
    fn new(num_holes: usize) -> Self {
        Self {
            id: WidgetId::new(),
            num_holes,
            contents: HashMap::new(),
            erased: false,
        }
    }

    fn erased() -> Self {
        Self {
            id: WidgetId::new(),
            num_holes: 0,
            contents: HashMap::new(),
            erased: true,
        }
    }

    fn id(&self) -> WidgetId {
        self.id
    }

    /// Place a widget in a specific hole.
    fn place_in_hole(&mut self, hole_index: usize, widget_id: WidgetId) {
        self.contents.insert(hole_index, widget_id);
    }

    /// Get the widget in a specific hole.
    fn widget_in_hole(&self, hole_index: usize) -> Option<WidgetId> {
        self.contents.get(&hole_index).copied()
    }

    /// Remove widget from a hole (for vacuum erase).
    fn clear_hole(&mut self, hole_index: usize) -> Option<WidgetId> {
        self.contents.remove(&hole_index)
    }

    fn render(&self, widgets: &HashMap<WidgetId, WidgetItem>) -> Html {
        if self.erased {
            html! {
                <div class="widget box erased" data-box-id={self.id.to_string()}>
                    <span class="box-erased">{ "?" }</span>
                </div>
            }
        } else {
            html! {
                <div class="widget box" data-box-id={self.id.to_string()}>
                    <div class="box-holes">
                        { for (0..self.num_holes).map(|i| {
                            let content = if let Some(widget_id) = self.contents.get(&i) {
                                if let Some(widget) = widgets.get(widget_id) {
                                    widget.render_small()
                                } else {
                                    html! { <span class="hole-empty">{ "\u{00A0}" }</span> }
                                }
                            } else {
                                html! { <span class="hole-empty">{ "\u{00A0}" }</span> }
                            };
                            html! {
                                <div class="box-hole" data-box-id={self.id.to_string()} data-hole-index={i.to_string()}>
                                    { content }
                                </div>
                            }
                        })}
                    </div>
                </div>
            }
        }
    }
}

/// Creates sample widgets for the demo.
///
/// Tutorial: Start with 0, use the tools to make it equal 10!
/// New: Use scales to compare numbers, vacuum to erase values.
fn demo_widgets() -> Vec<WidgetItem> {
    vec![
        // Value sources (copy sources with Add operator - just show the number)
        WidgetItem::Number(Number::new(1).as_copy_source()),
        WidgetItem::Number(Number::new(5).as_copy_source()),
        // Operation tools (non-Add operators - show the operation)
        WidgetItem::Number(
            Number::new(2)
                .with_operator(ArithOperator::Multiply)
                .as_copy_source(),
        ),
        WidgetItem::Number(
            Number::new(2)
                .with_operator(ArithOperator::Divide)
                .as_copy_source(),
        ),
        // The target number to manipulate - starts at 0
        WidgetItem::Number(Number::new(0)),
        // Scales for comparing numbers
        WidgetItem::Scales(Scales::new()),
        // Vacuum tool for erasing values
        WidgetItem::Vacuum(Vacuum::new()),
        // Wand tool for copying widgets
        WidgetItem::Wand(Wand::new()),
        // Robot for trainable automation
        WidgetItem::Robot(Robot::new()),
    ]
}

/// Creates sample boxes.
fn demo_boxes() -> Vec<BoxState> {
    vec![
        BoxState::new(2), // A box with 2 holes
        BoxState::new(3), // A box with 3 holes
    ]
}

/// Application state.
#[derive(Clone)]
struct AppState {
    /// All widgets (numbers, text).
    widgets: HashMap<WidgetId, WidgetItem>,
    /// All boxes.
    boxes: HashMap<WidgetId, BoxState>,
    /// Positions of free-floating widgets (not in boxes).
    positions: HashMap<WidgetId, Position>,
    /// Which widgets are currently inside boxes (widget_id -> (box_id, hole_index)).
    widget_in_box: HashMap<WidgetId, (WidgetId, usize)>,
    /// ID of robot currently in training mode (if any).
    training_robot_id: Option<WidgetId>,
}

impl AppState {
    fn new() -> Self {
        let widgets_vec = demo_widgets();
        let boxes_vec = demo_boxes();

        let mut widgets = HashMap::new();
        let mut positions = HashMap::new();
        let mut boxes = HashMap::new();

        // Initialize widgets
        let cols = 4;
        let spacing_x = 120.0;
        let spacing_y = 80.0;
        let start_x = 20.0;
        let start_y = 50.0;

        for (i, widget) in widgets_vec.into_iter().enumerate() {
            // Robot gets special position: just right of the /2 stack
            let (x, y) = if matches!(widget, WidgetItem::Robot(_)) {
                (start_x + 4.0 * spacing_x, start_y) // col 4, row 0 (right of /2)
            } else {
                let col = i % cols;
                let row = i / cols;
                (
                    start_x + (col as f64) * spacing_x,
                    start_y + (row as f64) * spacing_y,
                )
            };
            positions.insert(widget.id(), Position::new(x, y));
            widgets.insert(widget.id(), widget);
        }

        // Initialize boxes (placed below the widgets)
        let box_start_y = start_y + 3.0 * spacing_y;
        for (i, box_state) in boxes_vec.into_iter().enumerate() {
            let x = start_x + (i as f64) * 200.0;
            positions.insert(box_state.id(), Position::new(x, box_start_y));
            boxes.insert(box_state.id(), box_state);
        }

        Self {
            widgets,
            boxes,
            positions,
            widget_in_box: HashMap::new(),
            training_robot_id: None,
        }
    }

    /// Record an action to the training robot (if one is active).
    fn record_action(&mut self, action: Action) {
        if let Some(robot_id) = self.training_robot_id {
            if let Some(widget) = self.widgets.get_mut(&robot_id) {
                if let Some(robot) = widget.as_robot_mut() {
                    robot.record_action(action.clone());
                    log::info!("Recorded action: {:?}", action);
                }
            }
        }
    }

    /// Update scales in a box based on adjacent numbers.
    /// For a 3-hole box with scales in the middle (index 1):
    /// - hole[0] = left pan value
    /// - hole[1] = scales
    /// - hole[2] = right pan value
    fn update_scales_in_box(&mut self, box_id: WidgetId) {
        // Get box contents
        let contents = if let Some(box_state) = self.boxes.get(&box_id) {
            box_state.contents.clone()
        } else {
            return;
        };

        // Find any scales in this box and update them
        for (&hole_idx, &widget_id) in &contents {
            if let Some(WidgetItem::Scales(scales)) = self.widgets.get(&widget_id) {
                let mut updated = scales.clone();

                // Get number from left neighbor (hole_idx - 1)
                if hole_idx > 0 {
                    if let Some(&left_id) = contents.get(&(hole_idx - 1)) {
                        if let Some(WidgetItem::Number(n)) = self.widgets.get(&left_id) {
                            updated.set_left(n.numerator());
                        }
                    }
                }

                // Get number from right neighbor (hole_idx + 1)
                if let Some(&right_id) = contents.get(&(hole_idx + 1)) {
                    if let Some(WidgetItem::Number(n)) = self.widgets.get(&right_id) {
                        updated.set_right(n.numerator());
                    }
                }

                // Update the scales
                self.widgets.insert(widget_id, WidgetItem::Scales(updated));
            }
        }
    }

    /// Execute all actions recorded by a robot.
    fn execute_robot(&mut self, robot_id: WidgetId) {
        // Get the actions from the robot
        let actions: Vec<Action> = self
            .widgets
            .get(&robot_id)
            .and_then(|w| match w {
                WidgetItem::Robot(r) => Some(r.actions().to_vec()),
                _ => None,
            })
            .unwrap_or_default();

        if actions.is_empty() {
            log::info!("Robot {} has no actions to execute", robot_id);
            return;
        }

        // Set robot to working state
        if let Some(widget) = self.widgets.get_mut(&robot_id) {
            if let Some(robot) = widget.as_robot_mut() {
                robot.start_working();
            }
        }

        log::info!("Robot {} executing {} actions", robot_id, actions.len());

        // Execute each action
        for (i, action) in actions.iter().enumerate() {
            log::info!("Executing action {}: {:?}", i, action);
            match action {
                Action::ApplyArithmetic {
                    operator,
                    numerator,
                    denominator,
                    target_path,
                } => {
                    self.execute_apply_arithmetic(*operator, *numerator, *denominator, target_path);
                }
                Action::Drop { path } => {
                    self.execute_drop(path);
                }
                Action::Copy { path } => {
                    self.execute_copy(path);
                }
                Action::Remove { path } => {
                    self.execute_remove(path);
                }
                Action::PickUp { path } => {
                    log::info!("PickUp action at {} (not yet implemented)", path);
                }
            }
        }

        // Set robot back to idle
        if let Some(widget) = self.widgets.get_mut(&robot_id) {
            if let Some(robot) = widget.as_robot_mut() {
                robot.stop_working();
            }
        }
    }

    /// Execute an arithmetic operation.
    fn execute_apply_arithmetic(
        &mut self,
        operator: char,
        numerator: i64,
        denominator: i64,
        target_path: &str,
    ) {
        // Parse path like "widget:123"
        let target_id = self.parse_widget_path(target_path);

        log::info!(
            "execute_apply_arithmetic: operator={}, value={}/{}, target_path={}, target_id={:?}",
            operator,
            numerator,
            denominator,
            target_path,
            target_id
        );

        if let Some(target_id) = target_id {
            // Create a temporary tool number with the stored values
            let op = match operator {
                '+' => ArithOperator::Add,
                '-' => ArithOperator::Subtract,
                '*' => ArithOperator::Multiply,
                '/' => ArithOperator::Divide,
                _ => ArithOperator::Add,
            };
            let tool = Number::rational(numerator, denominator as u64).with_operator(op);

            let target_num = match self.widgets.get(&target_id) {
                Some(WidgetItem::Number(n)) => Some(n.clone()),
                _ => {
                    log::warn!("Target widget {} not found", target_id);
                    None
                }
            };

            if let Some(mut target) = target_num {
                if target.apply(&tool).is_some() {
                    self.widgets
                        .insert(target_id, WidgetItem::Number(target.clone()));
                    log::info!(
                        "Robot applied {} {} to target, result: {}",
                        tool.operator().symbol(),
                        tool.display_value(),
                        target.display_value()
                    );
                }
            } else {
                log::warn!("Could not execute arithmetic: target widget not found");
            }
        }
    }

    /// Execute a drop action (into a box hole).
    fn execute_drop(&mut self, path: &str) {
        // Parse paths like "box:123:hole:0"
        if let Some((box_id, hole_index)) = self.parse_box_hole_path(path) {
            // For now, we need a widget to drop - this would be the "held" widget
            // In a full implementation, we'd track what the robot is holding
            log::info!(
                "Robot drop to box {} hole {} (needs held widget)",
                box_id,
                hole_index
            );
        }
    }

    /// Execute a copy action.
    fn execute_copy(&mut self, path: &str) {
        if let Some(target_id) = self.parse_widget_path(path) {
            if let Some(target_widget) = self.widgets.get(&target_id) {
                let copy_widget = match target_widget {
                    WidgetItem::Number(n) => Some(WidgetItem::Number(n.copy_number())),
                    WidgetItem::Text(t) => Some(WidgetItem::Text(t.copy_text())),
                    WidgetItem::Scales(s) => Some(WidgetItem::Scales(s.copy_scales())),
                    WidgetItem::Vacuum(v) => Some(WidgetItem::Vacuum(v.copy_vacuum())),
                    WidgetItem::Wand(w) => Some(WidgetItem::Wand(w.copy_wand())),
                    WidgetItem::Robot(r) => Some(WidgetItem::Robot(r.copy_robot())),
                };

                if let Some(copied) = copy_widget {
                    let copy_id = copied.id();
                    let target_pos = self.positions.get(&target_id).copied().unwrap_or_default();
                    let copy_pos = Position::new(target_pos.x + 30.0, target_pos.y + 30.0);
                    self.widgets.insert(copy_id, copied);
                    self.positions.insert(copy_id, copy_pos);
                    log::info!(
                        "Robot copied widget {} to new widget {}",
                        target_id,
                        copy_id
                    );
                }
            }
        }
    }

    /// Execute a remove action.
    fn execute_remove(&mut self, path: &str) {
        if let Some((box_id, hole_index)) = self.parse_box_hole_path(path) {
            if let Some(box_state) = self.boxes.get_mut(&box_id) {
                if let Some(erased_widget_id) = box_state.clear_hole(hole_index) {
                    self.widget_in_box.remove(&erased_widget_id);
                    self.widgets.remove(&erased_widget_id);
                    log::info!(
                        "Robot removed widget from box {} hole {}",
                        box_id,
                        hole_index
                    );
                }
            }
        }
    }

    /// Parse a widget path like "widget:123" and return the WidgetId.
    fn parse_widget_path(&self, path: &str) -> Option<WidgetId> {
        let parts: Vec<&str> = path.split(':').collect();
        if parts.len() == 2 && parts[0] == "widget" {
            parts[1].parse::<u64>().ok().map(WidgetId::from_u64)
        } else {
            None
        }
    }

    /// Parse a box hole path like "box:123:hole:0" and return (box_id, hole_index).
    fn parse_box_hole_path(&self, path: &str) -> Option<(WidgetId, usize)> {
        let parts: Vec<&str> = path.split(':').collect();
        if parts.len() == 4 && parts[0] == "box" && parts[2] == "hole" {
            let box_id = parts[1].parse::<u64>().ok().map(WidgetId::from_u64)?;
            let hole_index = parts[3].parse::<usize>().ok()?;
            Some((box_id, hole_index))
        } else {
            None
        }
    }
}

/// Find which box hole (if any) is under the given mouse position.
/// Uses elementsFromPoint to look through all elements at the position,
/// skipping elements that belong to the dragged widget (inside .draggable.dragging).
fn find_box_hole_at(x: f64, y: f64) -> Option<(WidgetId, usize)> {
    use wasm_bindgen::JsCast;

    let window = web_sys::window()?;
    let document = window.document()?;

    // Get ALL elements at the mouse position (returns array from top to bottom)
    let elements = document.elements_from_point(x as f32, y as f32);

    debug_log!(
        "find_box_hole_at({}, {}): checking {} elements",
        x,
        y,
        elements.length()
    );

    // Iterate through elements looking for a box-hole
    for i in 0..elements.length() {
        // elements_from_point returns a js_sys::Array of Elements
        let js_val = elements.get(i);
        let element: web_sys::Element = match js_val.dyn_into() {
            Ok(el) => el,
            Err(_) => continue,
        };

        let tag = element.tag_name();
        let class = element.class_name();
        debug_log!("  [{}] tag={}, class={}", i, tag, class);

        // Skip elements that are inside a .draggable.dragging (the widget being dragged)
        if is_inside_dragging(&element) {
            debug_log!("    -> skipping (inside dragging element)");
            continue;
        }

        // Check if this element or an ancestor is a box-hole
        if let Some(hole_element) = find_box_hole_element(&element) {
            let box_id_str = hole_element.get_attribute("data-box-id")?;
            let hole_index_str = hole_element.get_attribute("data-hole-index")?;

            debug_log!(
                "    -> Found box-hole: box_id={}, hole_index={}",
                box_id_str,
                hole_index_str
            );

            let box_id = box_id_str.parse::<u64>().ok().map(WidgetId::from_u64)?;
            let hole_index = hole_index_str.parse::<usize>().ok()?;

            return Some((box_id, hole_index));
        }
    }

    debug_log!("  -> No box-hole found");
    None
}

/// Check if an element is inside a .draggable.dragging element (the widget being dragged).
fn is_inside_dragging(element: &web_sys::Element) -> bool {
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

/// Find which number widget (if any) is under the given mouse position.
fn find_number_at(x: f64, y: f64) -> Option<WidgetId> {
    let window = web_sys::window()?;
    let document = window.document()?;

    // Get the element at the mouse position
    let element = document.element_from_point(x as f32, y as f32)?;

    // Check if it's a number widget or inside one
    let number_element = find_number_element(&element)?;

    // Get the widget ID from data attribute
    let widget_id_str = number_element.get_attribute("data-widget-id")?;
    let widget_id = widget_id_str.parse::<u64>().ok().map(WidgetId::from_u64)?;

    Some(widget_id)
}

/// Walk up the DOM tree to find a number widget element.
fn find_number_element(element: &Element) -> Option<Element> {
    let mut current = Some(element.clone());
    while let Some(el) = current {
        if el.class_list().contains("number") && el.has_attribute("data-widget-id") {
            return Some(el);
        }
        current = el.parent_element();
    }
    None
}

/// Walk up the DOM tree to find a box-hole element.
fn find_box_hole_element(element: &Element) -> Option<Element> {
    let mut current = Some(element.clone());
    while let Some(el) = current {
        if el.class_list().contains("box-hole") {
            return Some(el);
        }
        current = el.parent_element();
    }
    None
}

/// Which pan of a scales was targeted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ScalesPan {
    Left,
    Right,
}

/// Find which scales pan (if any) is under the given mouse position.
fn find_scales_pan_at(x: f64, y: f64) -> Option<(WidgetId, ScalesPan)> {
    let window = web_sys::window()?;
    let document = window.document()?;

    let element = document.element_from_point(x as f32, y as f32)?;
    let pan_element = find_scales_pan_element(&element)?;

    let scales_id_str = pan_element.get_attribute("data-scales-id")?;
    let pan_str = pan_element.get_attribute("data-pan")?;

    let scales_id = scales_id_str.parse::<u64>().ok().map(WidgetId::from_u64)?;
    let pan = match pan_str.as_str() {
        "left" => ScalesPan::Left,
        "right" => ScalesPan::Right,
        _ => return None,
    };

    Some((scales_id, pan))
}

/// Walk up the DOM tree to find a scales-pan element.
fn find_scales_pan_element(element: &Element) -> Option<Element> {
    let mut current = Some(element.clone());
    while let Some(el) = current {
        if el.class_list().contains("scales-pan") {
            return Some(el);
        }
        current = el.parent_element();
    }
    None
}

/// Find any widget at the given position (for wand copying).
/// Returns widget ID and whether it's a box.
/// Skips the widget with skip_id (typically the wand being dropped).
fn find_widget_at_excluding(x: f64, y: f64, skip_id: WidgetId) -> Option<(WidgetId, bool)> {
    use wasm_bindgen::JsCast;

    let window = web_sys::window()?;
    let document = window.document()?;

    // Use elementsFromPoint to get all elements at this position
    let elements = document.elements_from_point(x as f32, y as f32);

    for i in 0..elements.length() {
        let js_val = elements.get(i);
        if let Ok(element) = js_val.dyn_into::<Element>() {
            if let Some(widget_element) = find_widget_element(&element) {
                // Try data-widget-id first (regular widgets)
                if let Some(widget_id_str) = widget_element.get_attribute("data-widget-id") {
                    if let Some(widget_id) =
                        widget_id_str.parse::<u64>().ok().map(WidgetId::from_u64)
                    {
                        if widget_id != skip_id {
                            return Some((widget_id, false));
                        }
                    }
                }

                // Try data-box-id (boxes)
                if let Some(box_id_str) = widget_element.get_attribute("data-box-id") {
                    if let Some(box_id) = box_id_str.parse::<u64>().ok().map(WidgetId::from_u64) {
                        if box_id != skip_id {
                            return Some((box_id, true));
                        }
                    }
                }
            }
        }
    }

    None
}

/// Walk up the DOM tree to find any widget element (has data-widget-id).
/// Also handles boxes which use data-box-id.
fn find_widget_element(element: &Element) -> Option<Element> {
    let mut current = Some(element.clone());
    while let Some(el) = current {
        if el.class_list().contains("widget") {
            // Check for widget-id (numbers, scales, vacuum, wand, text)
            if el.has_attribute("data-widget-id") {
                return Some(el);
            }
            // Check for box-id (boxes use different attribute)
            if el.has_attribute("data-box-id") && !el.class_list().contains("box-hole") {
                return Some(el);
            }
        }
        current = el.parent_element();
    }
    None
}

/// Main application component.
#[function_component(App)]
pub fn app() -> Html {
    // Application state
    let state = use_state(AppState::new);

    // Help panel visibility state
    let help_open = use_state(|| false);

    // Track which box is currently being dragged (for keyboard hole control)
    // Using use_mut_ref for shared mutable state that persists across closures
    let dragged_box_id = use_mut_ref(|| None::<WidgetId>);

    // Track pending new box creation (number of holes) - box created when dragged box is dropped
    let pending_new_box = use_mut_ref(|| None::<usize>);

    // Callbacks for help panel
    let on_help_open = {
        let help_open = help_open.clone();
        Callback::from(move |_| help_open.set(true))
    };

    let on_help_close = {
        let help_open = help_open.clone();
        Callback::from(move |_| help_open.set(false))
    };

    // Callback for when a box drag starts
    let on_box_drag_start = {
        let dragged_box_id = dragged_box_id.clone();
        let pending_new_box = pending_new_box.clone();
        Callback::from(move |event: DragStartEvent| {
            *dragged_box_id.borrow_mut() = Some(event.widget_id);
            *pending_new_box.borrow_mut() = None; // Clear any pending new box
            log::info!("Box drag started: {}", event.widget_id);
        })
    };

    // Callback for when a box drag ends - clear tracking state
    // Note: New box creation happens in on_box_drop which has position info
    let on_box_drag_end = {
        let dragged_box_id = dragged_box_id.clone();
        let pending_new_box = pending_new_box.clone();
        Callback::from(move |_event: DragEndEvent| {
            *pending_new_box.borrow_mut() = None;
            *dragged_box_id.borrow_mut() = None;
            log::info!("Box drag ended");
        })
    };

    // Callback for when a box is dropped (handles box creation, splitting, joining)
    let on_box_drop = {
        let state = state.clone();
        let pending_new_box = pending_new_box.clone();
        Callback::from(move |event: DropEvent| {
            let mut new_state = (*state).clone();
            let box_id = event.widget_id;
            let mouse_x = event.mouse_position.x;
            let mouse_y = event.mouse_position.y;

            // Check if user pressed a number key to create a new box
            // Note: We copy the value first to avoid holding the borrow across borrow_mut
            let pending_num_holes = *pending_new_box.borrow();
            if let Some(num_holes) = pending_num_holes {
                // Clear pending state first (before any other operations)
                *pending_new_box.borrow_mut() = None;

                // Create a new empty box with the specified number of holes
                let new_box = BoxState::new(num_holes);
                let new_box_id = new_box.id();
                // Position near the drop location (offset slightly)
                let new_pos = Position::new(event.position.x + 50.0, event.position.y + 50.0);
                new_state.positions.insert(new_box_id, new_pos);
                new_state.boxes.insert(new_box_id, new_box);

                // Update original box position
                new_state.positions.insert(box_id, event.position);

                log::info!(
                    "Created new box {} with {} holes at ({}, {})",
                    new_box_id,
                    num_holes,
                    new_pos.x,
                    new_pos.y
                );
                state.set(new_state);
                return;
            }

            // Check if we're dropping a box on a number (to split it)
            if let Some(target_number_id) = find_number_at(mouse_x, mouse_y) {
                if let Some(WidgetItem::Number(n)) = new_state.widgets.get(&target_number_id) {
                    // Don't split on copy sources or negative numbers
                    if !n.is_copy_source() {
                        let split_at = n.numerator() as usize;

                        if let Some(box_state) = new_state.boxes.get(&box_id) {
                            let total_holes = box_state.num_holes;

                            // Only split if the number is valid (1 to total_holes-1)
                            if split_at >= 1 && split_at < total_holes {
                                // Create left box (holes 0..split_at)
                                let mut left_box = BoxState::new(split_at);
                                for i in 0..split_at {
                                    if let Some(widget_id) = box_state.contents.get(&i).copied() {
                                        left_box.place_in_hole(i, widget_id);
                                        new_state
                                            .widget_in_box
                                            .insert(widget_id, (left_box.id(), i));
                                    }
                                }

                                // Create right box (holes split_at..total_holes)
                                let right_holes = total_holes - split_at;
                                let mut right_box = BoxState::new(right_holes);
                                for i in split_at..total_holes {
                                    if let Some(widget_id) = box_state.contents.get(&i).copied() {
                                        let new_hole = i - split_at;
                                        right_box.place_in_hole(new_hole, widget_id);
                                        new_state
                                            .widget_in_box
                                            .insert(widget_id, (right_box.id(), new_hole));
                                    }
                                }

                                // Position the two new boxes
                                let orig_pos = new_state
                                    .positions
                                    .get(&box_id)
                                    .copied()
                                    .unwrap_or(event.position);
                                let left_pos = orig_pos;
                                let right_pos = Position::new(orig_pos.x + 100.0, orig_pos.y);

                                // Add new boxes
                                let left_id = left_box.id();
                                let right_id = right_box.id();
                                new_state.positions.insert(left_id, left_pos);
                                new_state.positions.insert(right_id, right_pos);
                                new_state.boxes.insert(left_id, left_box);
                                new_state.boxes.insert(right_id, right_box);

                                // Remove original box
                                new_state.boxes.remove(&box_id);
                                new_state.positions.remove(&box_id);

                                // Remove the number used for splitting
                                new_state.widgets.remove(&target_number_id);
                                new_state.positions.remove(&target_number_id);

                                log::info!(
                                    "Split box {} ({} holes) at {} into {} ({}) and {} ({})",
                                    box_id,
                                    total_holes,
                                    split_at,
                                    left_id,
                                    split_at,
                                    right_id,
                                    right_holes
                                );
                                state.set(new_state);
                                return;
                            }
                        }
                    }
                }
            }

            // Check if we're dropping a box on another box (to join them)
            // Look for any other box under the mouse position
            if let Some((target_box_id, _is_box)) =
                find_widget_at_excluding(mouse_x, mouse_y, box_id)
            {
                // Check if the target is actually a box (not a widget)
                if new_state.boxes.contains_key(&target_box_id) {
                    if let (Some(source_box), Some(target_box)) = (
                        new_state.boxes.get(&box_id).cloned(),
                        new_state.boxes.get(&target_box_id).cloned(),
                    ) {
                        // Create combined box
                        let total_holes = source_box.num_holes + target_box.num_holes;
                        let mut joined_box = BoxState::new(total_holes);

                        // Copy contents from target box (first half)
                        for i in 0..target_box.num_holes {
                            if let Some(widget_id) = target_box.contents.get(&i).copied() {
                                joined_box.place_in_hole(i, widget_id);
                                new_state
                                    .widget_in_box
                                    .insert(widget_id, (joined_box.id(), i));
                            }
                        }

                        // Copy contents from source box (second half)
                        for i in 0..source_box.num_holes {
                            if let Some(widget_id) = source_box.contents.get(&i).copied() {
                                let new_hole = target_box.num_holes + i;
                                joined_box.place_in_hole(new_hole, widget_id);
                                new_state
                                    .widget_in_box
                                    .insert(widget_id, (joined_box.id(), new_hole));
                            }
                        }

                        // Position at target's location
                        let joined_pos = new_state
                            .positions
                            .get(&target_box_id)
                            .copied()
                            .unwrap_or(event.position);
                        let joined_id = joined_box.id();
                        new_state.positions.insert(joined_id, joined_pos);
                        new_state.boxes.insert(joined_id, joined_box);

                        // Remove both original boxes
                        new_state.boxes.remove(&box_id);
                        new_state.boxes.remove(&target_box_id);
                        new_state.positions.remove(&box_id);
                        new_state.positions.remove(&target_box_id);

                        log::info!(
                            "Joined box {} ({} holes) with {} ({} holes) into {} ({} holes)",
                            box_id,
                            source_box.num_holes,
                            target_box_id,
                            target_box.num_holes,
                            joined_id,
                            total_holes
                        );
                        state.set(new_state);
                        return;
                    }
                }
            }

            // Default: just update position
            new_state.positions.insert(box_id, event.position);
            state.set(new_state);
        })
    };

    // Keyboard event handler for creating new boxes
    // Pressing 0-9 while dragging a box queues creation of a new box with that many holes
    let on_keydown = {
        let dragged_box_id = dragged_box_id.clone();
        let pending_new_box = pending_new_box.clone();
        Callback::from(move |e: web_sys::KeyboardEvent| {
            let key = e.key();
            let current_dragged = *dragged_box_id.borrow();

            // Only handle if we're dragging a box
            if current_dragged.is_some() {
                // Check if it's a digit 0-9
                if let Some(digit) = key.chars().next() {
                    if digit.is_ascii_digit() {
                        let num_holes = digit.to_digit(10).unwrap() as usize;
                        // Store pending new box creation - will be created when drag ends
                        *pending_new_box.borrow_mut() = Some(num_holes);
                        log::info!(
                            "Queued creation of new box with {} holes (will create on drop)",
                            num_holes
                        );
                        e.prevent_default();
                    }
                }
            }
        })
    };

    // Set up global keydown listener using use_effect
    {
        let on_keydown = on_keydown.clone();
        use_effect_with((), move |_| {
            use wasm_bindgen::closure::Closure;
            use wasm_bindgen::JsCast;

            let window = web_sys::window().unwrap();
            let callback = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
                on_keydown.emit(e);
            }) as Box<dyn FnMut(_)>);

            window
                .add_event_listener_with_callback("keydown", callback.as_ref().unchecked_ref())
                .unwrap();

            // Return cleanup function
            let cleanup_window = window.clone();
            let cleanup_callback = callback;
            move || {
                let _ = cleanup_window.remove_event_listener_with_callback(
                    "keydown",
                    cleanup_callback.as_ref().unchecked_ref(),
                );
            }
        });
    }

    // Callback for when a copy source is clicked - create a copy as a new draggable widget
    let on_copy_source_click = {
        let state = state.clone();
        Callback::from(move |event: CopySourceClickEvent| {
            let mut new_state = (*state).clone();

            // Get the copy source and create a copy
            if let Some(WidgetItem::Number(n)) = new_state.widgets.get(&event.source_id) {
                let copy_number = n.copy_number();
                let copy_id = copy_number.id();

                // Add the copy to state at the click position
                new_state
                    .widgets
                    .insert(copy_id, WidgetItem::Number(copy_number));
                new_state.positions.insert(copy_id, event.position);

                log::info!(
                    "Created copy {} from copy source {}",
                    copy_id,
                    event.source_id
                );
                state.set(new_state);
            }
        })
    };

    // Callback for when a widget is moved
    let on_move = {
        let state = state.clone();
        Callback::from(move |(id, new_pos): (WidgetId, Position)| {
            let mut new_state = (*state).clone();
            new_state.positions.insert(id, new_pos);
            state.set(new_state);
        })
    };

    // Callback for when a widget is dropped
    let on_drop = {
        let state = state.clone();
        Callback::from(move |event: DropEvent| {
            let mut new_state = (*state).clone();
            let widget_id = event.widget_id;
            let mouse_x = event.mouse_position.x;
            let mouse_y = event.mouse_position.y;

            // Check if we're clicking a robot (double-click to toggle training)
            // Detect a "click" by checking if position hasn't changed much
            if new_state
                .widgets
                .get(&widget_id)
                .map(|w| w.is_robot())
                .unwrap_or(false)
            {
                let old_pos = new_state.positions.get(&widget_id).copied();
                let moved_distance = old_pos
                    .map(|p| {
                        let dx = p.x - event.position.x;
                        let dy = p.y - event.position.y;
                        (dx * dx + dy * dy).sqrt()
                    })
                    .unwrap_or(0.0);

                // If barely moved, treat as click - cycle robot state
                if moved_distance < 10.0 {
                    // Check current robot state and action count
                    let (robot_state, has_actions) = new_state
                        .widgets
                        .get(&widget_id)
                        .and_then(|w| match w {
                            WidgetItem::Robot(r) => Some((r.state(), !r.actions().is_empty())),
                            _ => None,
                        })
                        .unwrap_or((RobotState::Idle, false));

                    match robot_state {
                        RobotState::Training => {
                            // Stop training
                            if let Some(widget) = new_state.widgets.get_mut(&widget_id) {
                                if let Some(robot) = widget.as_robot_mut() {
                                    robot.stop_training();
                                }
                            }
                            new_state.training_robot_id = None;
                            log::info!("Robot {} stopped training", widget_id);
                        }
                        RobotState::Idle if has_actions => {
                            // Execute recorded actions
                            log::info!("Robot {} executing recorded actions", widget_id);
                            new_state.execute_robot(widget_id);
                        }
                        RobotState::Idle => {
                            // Start training (no actions yet)
                            // Stop any currently training robot first
                            if let Some(old_id) = new_state.training_robot_id {
                                if let Some(old_widget) = new_state.widgets.get_mut(&old_id) {
                                    if let Some(old_robot) = old_widget.as_robot_mut() {
                                        old_robot.stop_training();
                                    }
                                }
                            }
                            // Start training the clicked robot
                            if let Some(widget) = new_state.widgets.get_mut(&widget_id) {
                                if let Some(robot) = widget.as_robot_mut() {
                                    robot.start_training();
                                }
                            }
                            new_state.training_robot_id = Some(widget_id);
                            log::info!("Robot {} started training", widget_id);
                        }
                        RobotState::Working => {
                            // Robot is currently working, do nothing
                            log::info!("Robot {} is working, cannot interrupt", widget_id);
                        }
                    }
                    // Keep position (didn't move)
                    if let Some(old_pos) = old_pos {
                        new_state.positions.insert(widget_id, old_pos);
                    }
                    state.set(new_state);
                    return;
                }
            }

            // Check if we're dropping a vacuum on a box hole (to erase its contents)
            if new_state
                .widgets
                .get(&widget_id)
                .map(|w| w.is_vacuum())
                .unwrap_or(false)
            {
                // First check if we're over a box hole
                if let Some((box_id, hole_index)) = find_box_hole_at(mouse_x, mouse_y) {
                    if let Some(box_state) = new_state.boxes.get_mut(&box_id) {
                        if let Some(erased_widget_id) = box_state.clear_hole(hole_index) {
                            // Record action if training
                            new_state.record_action(Action::Remove {
                                path: format!("box:{}:hole:{}", box_id, hole_index),
                            });
                            // Remove the widget from widget_in_box tracking
                            new_state.widget_in_box.remove(&erased_widget_id);
                            // Remove the widget entirely
                            new_state.widgets.remove(&erased_widget_id);
                            log::info!(
                                "Vacuum erased widget {} from box {} hole {}",
                                erased_widget_id,
                                box_id,
                                hole_index
                            );
                            // Vacuum stays where it is (persistent tool)
                            new_state.positions.insert(widget_id, event.position);
                            state.set(new_state);
                            return;
                        }
                    }
                }
                // Check if vacuum is over a free-floating widget (not in a box)
                if let Some((target_id, is_box)) =
                    find_widget_at_excluding(mouse_x, mouse_y, widget_id)
                {
                    if !is_box {
                        // Check if target is deletable (not a tool or copy source)
                        let is_deletable = new_state
                            .widgets
                            .get(&target_id)
                            .map(|w| {
                                // Don't delete tools (vacuum, wand, robot) or copy sources
                                !matches!(
                                    w,
                                    WidgetItem::Vacuum(_)
                                        | WidgetItem::Wand(_)
                                        | WidgetItem::Robot(_)
                                ) && !matches!(w, WidgetItem::Number(n) if n.is_copy_source())
                            })
                            .unwrap_or(false);

                        if is_deletable {
                            // Delete the target widget
                            new_state.widgets.remove(&target_id);
                            new_state.positions.remove(&target_id);
                            log::info!("Vacuum deleted widget {}", target_id);
                            // Vacuum stays where it is
                            new_state.positions.insert(widget_id, event.position);
                            state.set(new_state);
                            return;
                        }
                    }
                    // Can't delete boxes with vacuum - just move vacuum
                }
                // Vacuum dropped elsewhere - just move it
                new_state.positions.insert(widget_id, event.position);
                state.set(new_state);
                return;
            }

            // Check if we're dropping a wand on a widget (to copy it)
            if new_state
                .widgets
                .get(&widget_id)
                .map(|w| w.is_wand())
                .unwrap_or(false)
            {
                // Find what widget is under the wand (excluding the wand itself)
                if let Some((target_id, is_box)) =
                    find_widget_at_excluding(mouse_x, mouse_y, widget_id)
                {
                    if is_box {
                        // Copy a box
                        if let Some(target_box) = new_state.boxes.get(&target_id) {
                            let copy_box = BoxState {
                                id: WidgetId::new(),
                                num_holes: target_box.num_holes,
                                contents: HashMap::new(), // Empty copy
                                erased: target_box.erased,
                            };
                            let copy_id = copy_box.id();
                            let target_pos = new_state
                                .positions
                                .get(&target_id)
                                .copied()
                                .unwrap_or_default();
                            let copy_pos = Position::new(target_pos.x + 30.0, target_pos.y + 30.0);
                            new_state.boxes.insert(copy_id, copy_box);
                            new_state.positions.insert(copy_id, copy_pos);
                            log::info!("Wand copied box {} to new box {}", target_id, copy_id);
                        }
                    } else if let Some(target_widget) = new_state.widgets.get(&target_id) {
                        // Check if target is a copy source (shouldn't duplicate copy sources)
                        let is_copy_source =
                            matches!(target_widget, WidgetItem::Number(n) if n.is_copy_source());
                        if !is_copy_source {
                            // Create a copy of the target widget
                            let copy_widget = match target_widget {
                                WidgetItem::Number(n) => Some(WidgetItem::Number(n.copy_number())),
                                WidgetItem::Text(t) => Some(WidgetItem::Text(t.copy_text())),
                                WidgetItem::Scales(s) => Some(WidgetItem::Scales(s.copy_scales())),
                                WidgetItem::Vacuum(v) => Some(WidgetItem::Vacuum(v.copy_vacuum())),
                                WidgetItem::Wand(w) => Some(WidgetItem::Wand(w.copy_wand())),
                                WidgetItem::Robot(r) => Some(WidgetItem::Robot(r.copy_robot())),
                            };

                            if let Some(copied) = copy_widget {
                                // Record action if training
                                new_state.record_action(Action::Copy {
                                    path: format!("widget:{}", target_id),
                                });
                                let copy_id = copied.id();
                                // Place copy slightly offset from original
                                let target_pos = new_state
                                    .positions
                                    .get(&target_id)
                                    .copied()
                                    .unwrap_or_default();
                                let copy_pos =
                                    Position::new(target_pos.x + 30.0, target_pos.y + 30.0);
                                new_state.widgets.insert(copy_id, copied);
                                new_state.positions.insert(copy_id, copy_pos);
                                log::info!(
                                    "Wand copied widget {} to new widget {}",
                                    target_id,
                                    copy_id
                                );
                            }
                        }
                    }
                }
                // Wand stays where it is (persistent tool)
                new_state.positions.insert(widget_id, event.position);
                state.set(new_state);
                return;
            }

            // Check if we're dropping a number onto a scales pan
            if let Some((scales_id, pan)) = find_scales_pan_at(mouse_x, mouse_y) {
                if let Some(WidgetItem::Number(n)) = new_state.widgets.get(&widget_id) {
                    if !n.is_copy_source() {
                        let value = n.numerator(); // Use numerator for comparison
                        if let Some(WidgetItem::Scales(scales)) =
                            new_state.widgets.get_mut(&scales_id)
                        {
                            match pan {
                                ScalesPan::Left => scales.set_left(value),
                                ScalesPan::Right => scales.set_right(value),
                            }
                            // Remove the dropped number
                            new_state.widgets.remove(&widget_id);
                            new_state.positions.remove(&widget_id);
                            log::info!("Placed {} on {:?} pan of scales", value, pan);
                            state.set(new_state);
                            return;
                        }
                    }
                }
            }

            // Check if we're dropping a number onto another number
            if let Some(target_id) = find_number_at(mouse_x, mouse_y) {
                if target_id != widget_id {
                    let dropped_num = match new_state.widgets.get(&widget_id) {
                        Some(WidgetItem::Number(n)) => Some(n.clone()),
                        _ => None,
                    };
                    let target_num = match new_state.widgets.get(&target_id) {
                        Some(WidgetItem::Number(n)) => Some(n.clone()),
                        _ => None,
                    };

                    if let (Some(dropped), Some(mut target)) = (dropped_num, target_num) {
                        if target.is_copy_source() {
                            state.set(new_state);
                            return;
                        }
                        if target.apply(&dropped).is_some() {
                            // Record action if training - store actual values
                            let op_char = dropped.operator().symbol().chars().next().unwrap_or('+');
                            new_state.record_action(Action::ApplyArithmetic {
                                operator: op_char,
                                numerator: dropped.numerator(),
                                denominator: dropped.denominator() as i64,
                                target_path: format!("widget:{}", target_id),
                            });
                            new_state
                                .widgets
                                .insert(target_id, WidgetItem::Number(target.clone()));
                            new_state.widgets.remove(&widget_id);
                            new_state.positions.remove(&widget_id);
                            log::info!(
                                "Applied {} {} to target, result: {}",
                                dropped.operator().symbol(),
                                dropped.display_value(),
                                target.display_value()
                            );
                        } else {
                            log::warn!("Division by zero attempted");
                        }
                        state.set(new_state);
                        return;
                    }
                }
            }

            // Check if we're dropping onto a box hole
            if let Some((box_id, hole_index)) = find_box_hole_at(mouse_x, mouse_y) {
                log::debug!(
                    "Dropping widget {} onto box {} hole {}",
                    widget_id,
                    box_id,
                    hole_index
                );
                if new_state.widgets.contains_key(&widget_id) {
                    // Check what's in the hole
                    let existing_widget_id = new_state
                        .boxes
                        .get(&box_id)
                        .and_then(|b| b.widget_in_hole(hole_index));
                    log::debug!(
                        "Dropping to box hole, existing widget: {:?}",
                        existing_widget_id
                    );

                    // If there's already a widget in the hole, remove it first
                    // (make it a free widget at the drop position)
                    if let Some(old_widget_id) = existing_widget_id {
                        // Remove old widget from box
                        if let Some(box_state) = new_state.boxes.get_mut(&box_id) {
                            box_state.clear_hole(hole_index);
                        }
                        new_state.widget_in_box.remove(&old_widget_id);
                        // Place old widget at drop position (so user can pick it up)
                        new_state.positions.insert(
                            old_widget_id,
                            Position::new(event.mouse_position.x + 50.0, event.mouse_position.y),
                        );
                        log::debug!(
                            "Replaced widget {} in hole, moved to free position",
                            old_widget_id
                        );
                    }

                    // Now the hole is empty, proceed with placing the new widget
                    // Record action if training
                    new_state.record_action(Action::Drop {
                        path: format!("box:{}:hole:{}", box_id, hole_index),
                    });

                    // Check if this is a "tool" widget that should be auto-copied
                    // Tools (Scales, Vacuum, Wand, Robot) keep the original, place a copy
                    let is_tool = matches!(
                        new_state.widgets.get(&widget_id),
                        Some(WidgetItem::Scales(_))
                            | Some(WidgetItem::Vacuum(_))
                            | Some(WidgetItem::Wand(_))
                            | Some(WidgetItem::Robot(_))
                    );

                    let id_to_place = if is_tool {
                        // Create a copy for the box, keep original
                        let copy = match new_state.widgets.get(&widget_id) {
                            Some(WidgetItem::Scales(s)) => {
                                Some(WidgetItem::Scales(s.copy_scales()))
                            }
                            Some(WidgetItem::Vacuum(v)) => {
                                Some(WidgetItem::Vacuum(v.copy_vacuum()))
                            }
                            Some(WidgetItem::Wand(w)) => Some(WidgetItem::Wand(w.copy_wand())),
                            Some(WidgetItem::Robot(r)) => Some(WidgetItem::Robot(r.copy_robot())),
                            _ => None,
                        };
                        if let Some(copied) = copy {
                            let copy_id = copied.id();
                            new_state.widgets.insert(copy_id, copied);
                            copy_id
                        } else {
                            widget_id
                        }
                    } else {
                        // Regular widget (Number, Text) - move it into the box
                        new_state.positions.remove(&widget_id);
                        widget_id
                    };

                    // Place the widget (or its copy) in the box hole
                    if let Some(box_state) = new_state.boxes.get_mut(&box_id) {
                        box_state.place_in_hole(hole_index, id_to_place);
                    }
                    new_state
                        .widget_in_box
                        .insert(id_to_place, (box_id, hole_index));

                    // Update scales if numbers are placed adjacent to them
                    new_state.update_scales_in_box(box_id);
                    state.set(new_state);
                    return;
                }
            }

            // Default: drop on empty background - keep widget at drop position
            new_state.positions.insert(widget_id, event.position);
            state.set(new_state);
        })
    };

    // Separate copy sources from regular widgets
    let copy_sources: Vec<_> = state
        .widgets
        .iter()
        .filter(|(id, widget)| {
            !state.widget_in_box.contains_key(id)
                && matches!(widget, WidgetItem::Number(n) if n.is_copy_source())
        })
        .collect();

    let regular_widgets: Vec<_> = state
        .widgets
        .iter()
        .filter(|(id, widget)| {
            !state.widget_in_box.contains_key(id)
                && !matches!(widget, WidgetItem::Number(n) if n.is_copy_source())
        })
        .collect();

    html! {
        <div class="workspace">
            <div class="workspace-header">
                {"tt-rs - Visual Programming Environment"}
            </div>
            <HelpButton on_click={on_help_open} />
            <HelpPanel is_open={*help_open} on_close={on_help_close} />
            <div class="workspace-content">
                // Render boxes first (so widgets can be dropped on them)
                {
                    state.boxes.iter().map(|(id, box_state)| {
                        let pos = state.positions.get(id).copied().unwrap_or_default();
                        html! {
                            <Draggable
                                widget_id={*id}
                                position={pos}
                                on_move={on_move.clone()}
                                on_drag_start={on_box_drag_start.clone()}
                                on_drag_end={on_box_drag_end.clone()}
                                on_drop={on_box_drop.clone()}
                            >
                                <Tooltip
                                    title="Box"
                                    description="A container with holes for storing items."
                                    hint="Drag items into holes. Drop on numbers to split."
                                    position={TooltipPosition::Right}
                                >
                                    { box_state.render(&state.widgets) }
                                </Tooltip>
                            </Draggable>
                        }
                    }).collect::<Html>()
                }
                // Render copy sources (static stacks that create copies on click)
                {
                    copy_sources.iter().map(|(id, widget)| {
                        let pos = state.positions.get(id).copied().unwrap_or_default();
                        let tooltip = widget.tooltip_info();
                        html! {
                            <CopySource
                                widget_id={**id}
                                position={pos}
                                on_click={on_copy_source_click.clone()}
                            >
                                <Tooltip
                                    title={tooltip.title}
                                    description={tooltip.description}
                                    hint={tooltip.hint}
                                    position={TooltipPosition::Right}
                                >
                                    { widget.render() }
                                </Tooltip>
                            </CopySource>
                        }
                    }).collect::<Html>()
                }
                // Render regular draggable widgets
                {
                    regular_widgets.iter().map(|(id, widget)| {
                        let pos = state.positions.get(id).copied().unwrap_or_default();
                        let tooltip = widget.tooltip_info();
                        html! {
                            <Draggable
                                widget_id={**id}
                                position={pos}
                                on_move={on_move.clone()}
                                on_drop={on_drop.clone()}
                            >
                                <Tooltip
                                    title={tooltip.title}
                                    description={tooltip.description}
                                    hint={tooltip.hint}
                                    position={TooltipPosition::Right}
                                >
                                    { widget.render() }
                                </Tooltip>
                            </Draggable>
                        }
                    }).collect::<Html>()
                }
            </div>
            <Footer />
        </div>
    }
}
