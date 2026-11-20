//! Layout primitives for SigmaOS-app-framework.

use crate::{AccessibilityNode, Widget, WidgetId};

// ── Orientation ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation { Horizontal, Vertical }

impl Default for Orientation { fn default() -> Self { Self::Vertical } }

// ── LayoutBox ─────────────────────────────────────────────────────────────────

/// A box layout container (horizontal or vertical).
pub struct LayoutBox {
    id: WidgetId,
    pub orientation: Orientation,
    pub spacing: u32,
    children: Vec<Box<dyn Widget>>,
}

impl std::fmt::Debug for LayoutBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LayoutBox")
            .field("id", &self.id)
            .field("orientation", &self.orientation)
            .field("children", &self.children.len())
            .finish()
    }
}

impl LayoutBox {
    pub fn new(id: WidgetId, orientation: Orientation) -> Self {
        Self { id, orientation, spacing: 8, children: Vec::new() }
    }

    pub fn add(&mut self, child: Box<dyn Widget>) {
        self.children.push(child);
    }

    pub fn child_count(&self) -> usize { self.children.len() }
}

impl Widget for LayoutBox {
    fn id(&self) -> WidgetId { self.id }

    fn render(&self) -> String {
        let dir = match self.orientation { Orientation::Horizontal => "H", Orientation::Vertical => "V" };
        let kids: Vec<String> = self.children.iter().map(|c| c.render()).collect();
        format!("[Box({}): {}]", dir, kids.join(", "))
    }

    fn accessibility_node(&self) -> AccessibilityNode {
        AccessibilityNode::new("group", "layout-box", false)
    }
}

// ── LayoutGrid ────────────────────────────────────────────────────────────────

/// A grid layout container.
pub struct LayoutGrid {
    id: WidgetId,
    pub columns: usize,
    pub row_spacing: u32,
    pub column_spacing: u32,
    cells: Vec<(usize, usize, Box<dyn Widget>)>, // (col, row, widget)
}

impl std::fmt::Debug for LayoutGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LayoutGrid")
            .field("id", &self.id)
            .field("columns", &self.columns)
            .field("cells", &self.cells.len())
            .finish()
    }
}

impl LayoutGrid {
    pub fn new(id: WidgetId, columns: usize) -> Self {
        Self { id, columns, row_spacing: 8, column_spacing: 8, cells: Vec::new() }
    }

    pub fn attach(&mut self, col: usize, row: usize, widget: Box<dyn Widget>) {
        self.cells.push((col, row, widget));
    }

    pub fn cell_count(&self) -> usize { self.cells.len() }
}

impl Widget for LayoutGrid {
    fn id(&self) -> WidgetId { self.id }

    fn render(&self) -> String {
        format!("[Grid({}cols, {} cells)]", self.columns, self.cells.len())
    }

    fn accessibility_node(&self) -> AccessibilityNode {
        AccessibilityNode::new("grid", "layout-grid", false)
    }
}
