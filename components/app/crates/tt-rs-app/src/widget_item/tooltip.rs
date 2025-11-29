//! Tooltip information for widget items.

use tt_rs_number::ArithOperator;

use super::WidgetItem;

/// Tooltip information for a widget.
pub struct TooltipInfo {
    pub title: &'static str,
    pub description: &'static str,
    pub hint: &'static str,
}

const TOOLTIP_NUMBER_ADD: TooltipInfo = TooltipInfo {
    title: "Number Source",
    description: "Click to create a copy of this number.",
    hint: "Drag copies onto other numbers to add them.",
};

const TOOLTIP_NUMBER_SUB: TooltipInfo = TooltipInfo {
    title: "Subtraction Tool",
    description: "Click to create a subtraction operation.",
    hint: "Drag onto a number to subtract this value.",
};

const TOOLTIP_NUMBER_MUL: TooltipInfo = TooltipInfo {
    title: "Multiplication Tool",
    description: "Click to create a multiplication operation.",
    hint: "Drag onto a number to multiply by this value.",
};

const TOOLTIP_NUMBER_DIV: TooltipInfo = TooltipInfo {
    title: "Division Tool",
    description: "Click to create a division operation.",
    hint: "Drag onto a number to divide by this value.",
};

const TOOLTIP_NUMBER: TooltipInfo = TooltipInfo {
    title: "Number",
    description: "A numeric value you can manipulate.",
    hint: "Drop arithmetic tools on this to change its value.",
};

const TOOLTIP_TEXT: TooltipInfo = TooltipInfo {
    title: "Text",
    description: "A text string.",
    hint: "Drag into box holes to store.",
};

const TOOLTIP_SCALES: TooltipInfo = TooltipInfo {
    title: "Scales",
    description: "Compare two numbers by dropping them on the pans.",
    hint: "The scales tip toward the larger number.",
};

const TOOLTIP_VACUUM: TooltipInfo = TooltipInfo {
    title: "Vacuum",
    description: "Erases items it touches.",
    hint: "Drop on box holes to erase contents, or on numbers to delete them.",
};

const TOOLTIP_WAND: TooltipInfo = TooltipInfo {
    title: "Magic Wand",
    description: "Creates copies of items it touches.",
    hint: "Drop on any widget to create a duplicate.",
};

const TOOLTIP_ROBOT: TooltipInfo = TooltipInfo {
    title: "Robot",
    description: "Learns by watching your actions and can repeat them.",
    hint: "Click to start/stop training, click again to run.",
};

const TOOLTIP_NEST: TooltipInfo = TooltipInfo {
    title: "Nest",
    description: "Receives messages from birds.",
    hint: "Birds deliver items here. Click to take the oldest message.",
};

const TOOLTIP_BIRD: TooltipInfo = TooltipInfo {
    title: "Bird",
    description: "Delivers messages to its home nest.",
    hint: "Drop an item on a bird to send it to the nest.",
};

/// Get tooltip information for a widget item.
pub fn tooltip_info(item: &WidgetItem) -> &'static TooltipInfo {
    match item {
        WidgetItem::Number(n) if n.is_copy_source() => match n.operator() {
            ArithOperator::Add => &TOOLTIP_NUMBER_ADD,
            ArithOperator::Subtract => &TOOLTIP_NUMBER_SUB,
            ArithOperator::Multiply => &TOOLTIP_NUMBER_MUL,
            ArithOperator::Divide => &TOOLTIP_NUMBER_DIV,
        },
        WidgetItem::Number(_) => &TOOLTIP_NUMBER,
        WidgetItem::Text(_) => &TOOLTIP_TEXT,
        WidgetItem::Scales(_) => &TOOLTIP_SCALES,
        WidgetItem::Vacuum(_) => &TOOLTIP_VACUUM,
        WidgetItem::Wand(_) => &TOOLTIP_WAND,
        WidgetItem::Robot(_) => &TOOLTIP_ROBOT,
        WidgetItem::Nest(_) => &TOOLTIP_NEST,
        WidgetItem::Bird(_) => &TOOLTIP_BIRD,
    }
}
