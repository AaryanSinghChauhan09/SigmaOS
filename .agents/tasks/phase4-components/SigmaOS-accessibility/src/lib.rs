//! SigmaOS accessibility layer.
//!
//! Provides AT-SPI2 interfaces so assistive technologies can interact
//! with all desktop UI components.

#![warn(missing_docs)]

/// Accessibility event kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessibilityEvent {
    /// Focus moved to a new widget.
    FocusChanged,
    /// State of a widget changed.
    StateChanged,
    /// Text caret moved.
    CaretMoved,
    /// A widget became visible.
    Shown,
}

/// Trait implemented by UI elements that expose themselves to assistive tech.
pub trait Accessible {
    /// Human-readable name of the element.
    fn name(&self) -> &str;
    /// Role of the element (button, label, etc.).
    fn role(&self) -> AccessibleRole;
    /// Emit an accessibility event.
    fn emit(&self, event: AccessibilityEvent);
}

/// Roles describing the function of an accessible element.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessibleRole {
    /// Push button.
    Button,
    /// Static text label.
    Label,
    /// Text entry field.
    Entry,
    /// Container grouping other elements.
    Container,
}

/// Entry point exposed to the session bus.
pub fn init() {
    // D-Bus registration happens here once the transport crate is wired in.
}
