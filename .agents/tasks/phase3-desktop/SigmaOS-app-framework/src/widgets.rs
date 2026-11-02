//! Widget library for SigmaOS-app-framework.
//!
//! Concrete widget implementations beyond the core Button/Label/Container
//! defined in lib.rs: Entry, ListBox, Dialog.

use crate::{AccessibilityNode, Widget, WidgetId};

// ── Entry (text input) ────────────────────────────────────────────────────────

/// A single-line text entry widget.
#[derive(Debug)]
pub struct Entry {
    id: WidgetId,
    pub placeholder: String,
    pub value: String,
    pub password_mode: bool,
}

impl Entry {
    pub fn new(id: WidgetId, placeholder: impl Into<String>) -> Self {
        Self {
            id,
            placeholder: placeholder.into(),
            value: String::new(),
            password_mode: false,
        }
    }

    pub fn set_value(&mut self, v: impl Into<String>) {
        self.value = v.into();
    }
}

impl Widget for Entry {
    fn id(&self) -> WidgetId { self.id }

    fn render(&self) -> String {
        if self.value.is_empty() {
            format!("[Entry: {}]", self.placeholder)
        } else if self.password_mode {
            format!("[Entry: {}]", "*".repeat(self.value.len()))
        } else {
            format!("[Entry: {}]", self.value)
        }
    }

    fn accessibility_node(&self) -> AccessibilityNode {
        AccessibilityNode::new("textbox", &self.placeholder, true)
    }
}

// ── ListBox ───────────────────────────────────────────────────────────────────

/// A vertical list of selectable items.
#[derive(Debug)]
pub struct ListBox {
    id: WidgetId,
    pub items: Vec<String>,
    pub selected_index: Option<usize>,
}

impl ListBox {
    pub fn new(id: WidgetId) -> Self {
        Self { id, items: Vec::new(), selected_index: None }
    }

    pub fn add_item(&mut self, item: impl Into<String>) {
        self.items.push(item.into());
    }

    pub fn select(&mut self, index: usize) {
        if index < self.items.len() {
            self.selected_index = Some(index);
        }
    }

    pub fn selected_item(&self) -> Option<&str> {
        self.selected_index.map(|i| self.items[i].as_str())
    }
}

impl Widget for ListBox {
    fn id(&self) -> WidgetId { self.id }

    fn render(&self) -> String {
        format!("[ListBox: {} items, selected={:?}]", self.items.len(), self.selected_index)
    }

    fn accessibility_node(&self) -> AccessibilityNode {
        AccessibilityNode::new("listbox", "list", true)
    }
}

// ── Dialog ────────────────────────────────────────────────────────────────────

/// Modal dialog widget.
#[derive(Debug)]
pub struct Dialog {
    id: WidgetId,
    pub title: String,
    pub message: String,
    pub open: bool,
}

impl Dialog {
    pub fn new(id: WidgetId, title: impl Into<String>, message: impl Into<String>) -> Self {
        Self { id, title: title.into(), message: message.into(), open: false }
    }

    pub fn show(&mut self) { self.open = true; }
    pub fn close(&mut self) { self.open = false; }
}

impl Widget for Dialog {
    fn id(&self) -> WidgetId { self.id }

    fn render(&self) -> String {
        if self.open {
            format!("[Dialog(open): {} — {}]", self.title, self.message)
        } else {
            format!("[Dialog(closed): {}]", self.title)
        }
    }

    fn accessibility_node(&self) -> AccessibilityNode {
        AccessibilityNode::new("dialog", &self.title, self.open)
    }
}
