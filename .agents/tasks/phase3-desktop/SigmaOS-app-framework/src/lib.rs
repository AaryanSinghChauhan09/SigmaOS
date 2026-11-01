//! SigmaOS App Framework — shared library for desktop applications.
//!
//! Provides widgets, theming integration, and accessibility support.
//!
//! Upstream inspiration:
//! - GTK:              <https://github.com/linuxmint/gtk>
//! - cinnamon-desktop: <https://github.com/linuxmint/cinnamon-desktop>

pub mod ipc;
pub mod layout;
pub mod widgets;

use std::fmt;

// ---------------------------------------------------------------------------
// Widget ID type alias
// ---------------------------------------------------------------------------

/// Opaque widget identifier.
pub type WidgetId = u64;

// ---------------------------------------------------------------------------
// Widget trait (composition over inheritance)
// ---------------------------------------------------------------------------

/// Core interface every UI widget must satisfy.
///
/// Composition over inheritance: complex widgets compose simpler widgets
/// rather than subclassing them.
pub trait Widget: fmt::Debug {
    /// Unique identifier for this widget instance.
    fn id(&self) -> WidgetId;
    /// Render a text representation (placeholder for real rendering).
    fn render(&self) -> String;
    /// Returns the accessibility node associated with this widget.
    fn accessibility_node(&self) -> AccessibilityNode;
}

// ---------------------------------------------------------------------------
// Accessibility node
// ---------------------------------------------------------------------------

/// ARIA-like accessibility node providing a role and label for assistive tech.
#[derive(Debug, Clone)]
pub struct AccessibilityNode {
    /// Accessible role, e.g. `"button"`, `"label"`, `"container"`.
    pub role: String,
    /// Accessible label read by screen readers.
    pub label: String,
    /// Whether this node is currently focusable.
    pub focusable: bool,
}

impl AccessibilityNode {
    pub fn new(role: impl Into<String>, label: impl Into<String>, focusable: bool) -> Self {
        Self {
            role: role.into(),
            label: label.into(),
            focusable,
        }
    }
}

// ---------------------------------------------------------------------------
// Button widget
// ---------------------------------------------------------------------------

/// A clickable button widget.
#[derive(Debug)]
pub struct Button {
    id: WidgetId,
    pub label: String,
    pub enabled: bool,
}

impl Button {
    pub fn new(id: WidgetId, label: impl Into<String>) -> Self {
        Self {
            id,
            label: label.into(),
            enabled: true,
        }
    }
}

impl Widget for Button {
    fn id(&self) -> WidgetId {
        self.id
    }

    fn render(&self) -> String {
        if self.enabled {
            format!("[Button: {}]", self.label)
        } else {
            format!("[Button: {} (disabled)]", self.label)
        }
    }

    fn accessibility_node(&self) -> AccessibilityNode {
        AccessibilityNode::new("button", &self.label, self.enabled)
    }
}

// ---------------------------------------------------------------------------
// Label widget
// ---------------------------------------------------------------------------

/// A non-interactive text label.
#[derive(Debug)]
pub struct Label {
    id: WidgetId,
    pub text: String,
}

impl Label {
    pub fn new(id: WidgetId, text: impl Into<String>) -> Self {
        Self {
            id,
            text: text.into(),
        }
    }
}

impl Widget for Label {
    fn id(&self) -> WidgetId {
        self.id
    }

    fn render(&self) -> String {
        format!("[Label: {}]", self.text)
    }

    fn accessibility_node(&self) -> AccessibilityNode {
        AccessibilityNode::new("label", &self.text, false)
    }
}

// ---------------------------------------------------------------------------
// Container widget
// ---------------------------------------------------------------------------

/// A container that holds child widgets (composition over inheritance).
pub struct Container {
    id: WidgetId,
    pub label: String,
    children: Vec<Box<dyn Widget>>,
}

impl fmt::Debug for Container {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Container")
            .field("id", &self.id)
            .field("label", &self.label)
            .field("child_count", &self.children.len())
            .finish()
    }
}

impl Container {
    pub fn new(id: WidgetId, label: impl Into<String>) -> Self {
        Self {
            id,
            label: label.into(),
            children: Vec::new(),
        }
    }

    /// Add a child widget.
    pub fn add_child(&mut self, widget: Box<dyn Widget>) {
        self.children.push(widget);
    }

    /// Returns the number of direct children.
    pub fn child_count(&self) -> usize {
        self.children.len()
    }

    /// Collect accessibility nodes from all direct children.
    pub fn child_accessibility_nodes(&self) -> Vec<AccessibilityNode> {
        self.children.iter().map(|w| w.accessibility_node()).collect()
    }
}

impl Widget for Container {
    fn id(&self) -> WidgetId {
        self.id
    }

    fn render(&self) -> String {
        let children: Vec<String> = self.children.iter().map(|w| w.render()).collect();
        format!("[Container({}): {}]", self.label, children.join(", "))
    }

    fn accessibility_node(&self) -> AccessibilityNode {
        AccessibilityNode::new("container", &self.label, false)
    }
}

// ---------------------------------------------------------------------------
// App context
// ---------------------------------------------------------------------------

/// Top-level application context returned by [`init_app_framework`].
#[derive(Debug)]
pub struct AppContext {
    /// Counter used to generate unique widget IDs.
    next_id: WidgetId,
}

impl AppContext {
    fn new() -> Self {
        Self { next_id: 1 }
    }

    /// Allocate a unique widget ID.
    pub fn alloc_id(&mut self) -> WidgetId {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

// ---------------------------------------------------------------------------
// Public entry-point
// ---------------------------------------------------------------------------

/// Initialise the app framework and return an [`AppContext`].
pub fn init_app_framework() -> AppContext {
    AppContext::new()
}

// ---------------------------------------------------------------------------
// Inline tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alloc_id_is_unique() {
        let mut ctx = init_app_framework();
        let a = ctx.alloc_id();
        let b = ctx.alloc_id();
        assert_ne!(a, b);
    }

    #[test]
    fn button_renders_label() {
        let btn = Button::new(1, "OK");
        assert!(btn.render().contains("OK"));
    }

    #[test]
    fn container_collects_children() {
        let mut c = Container::new(10, "root");
        c.add_child(Box::new(Button::new(11, "Cancel")));
        c.add_child(Box::new(Label::new(12, "Status")));
        assert_eq!(c.child_count(), 2);
    }
}
