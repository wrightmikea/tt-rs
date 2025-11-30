//! DropZone struct and constructors.

use tt_rs_core::WidgetId;

/// A drop zone widget for puzzle verification.
///
/// DropZones display a label/instruction and verify that dropped widgets
/// match an expected pattern.
#[derive(Debug, Clone)]
pub struct DropZone {
    pub(crate) id: WidgetId,
    /// The instruction/label displayed to the user.
    pub(crate) label: String,
    /// URL to navigate to on success (optional).
    pub(crate) on_success_url: Option<String>,
    /// Message to show on success (optional).
    pub(crate) on_success_message: Option<String>,
    /// Whether the drop zone has been satisfied (puzzle solved).
    pub(crate) satisfied: bool,
}

impl DropZone {
    /// Creates a new drop zone with the given label.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            id: WidgetId::new(),
            label: label.into(),
            on_success_url: None,
            on_success_message: None,
            satisfied: false,
        }
    }

    /// Sets the URL to navigate to on success.
    pub fn with_success_url(mut self, url: impl Into<String>) -> Self {
        self.on_success_url = Some(url.into());
        self
    }

    /// Sets the message to show on success.
    pub fn with_success_message(mut self, message: impl Into<String>) -> Self {
        self.on_success_message = Some(message.into());
        self
    }

    /// Creates a copy of this drop zone with a new ID.
    pub fn copy_dropzone(&self) -> DropZone {
        DropZone {
            id: WidgetId::new(),
            label: self.label.clone(),
            on_success_url: self.on_success_url.clone(),
            on_success_message: self.on_success_message.clone(),
            satisfied: false, // Reset satisfied state on copy
        }
    }
}
