// SigmaOS Desktop Layout Manager
// Inspired by Linux Mint's workspace management and Omarchy's layout utilities

use std::collections::HashMap;

/// Layout type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopLayoutType {
    Tiling,
    Stacking,
    Tabbed,
    Floating,
    Grid,
    Columns,
    Rows,
    Spiral,
}

impl DesktopLayoutType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DesktopLayoutType::Tiling => "Tiling",
            DesktopLayoutType::Stacking => "Stacking",
            DesktopLayoutType::Tabbed => "Tabbed",
            DesktopLayoutType::Floating => "Floating",
            DesktopLayoutType::Grid => "Grid",
            DesktopLayoutType::Columns => "Columns",
            DesktopLayoutType::Rows => "Rows",
            DesktopLayoutType::Spiral => "Spiral",
        }
    }
}

/// Workspace layout
#[derive(Debug, Clone)]
pub struct WorkspaceLayout {
    pub id: String,
    pub name: String,
    pub layout_type: DesktopLayoutType,
    pub gaps: (u32, u32), // inner gap, outer gap
    pub border_size: u32,
    pub is_default: bool,
}

impl WorkspaceLayout {
    pub fn new(id: String, name: String, layout_type: DesktopLayoutType) -> Self {
        WorkspaceLayout {
            id,
            name,
            layout_type,
            gaps: (8, 8),
            border_size: 2,
            is_default: false,
        }
    }

    pub fn set_gaps(&mut self, inner: u32, outer: u32) {
        self.gaps = (inner, outer);
    }

    pub fn set_border_size(&mut self, size: u32) {
        self.border_size = size;
    }

    pub fn set_default(&mut self, default: bool) {
        self.is_default = default;
    }
}

/// Layout Manager
pub struct DesktopLayoutManager {
    layouts: HashMap<String, WorkspaceLayout>,
    default_layout: Option<String>,
    next_layout_id: u32,
}

impl DesktopLayoutManager {
    pub fn new() -> Self {
        let mut manager = DesktopLayoutManager {
            layouts: HashMap::new(),
            default_layout: None,
            next_layout_id: 1,
        };

        // Add default layouts
        manager.add_default_layouts();

        manager
    }

    fn add_default_layouts(&mut self) {
        // Tiling layout
        let mut tiling = WorkspaceLayout::new(
            format!("layout_{}", self.next_layout_id),
            "Tiling".to_string(),
            DesktopLayoutType::Tiling,
        );
        tiling.set_default(true);
        let tiling_id = tiling.id.clone();
        self.layouts.insert(tiling_id.clone(), tiling);
        self.default_layout = Some(tiling_id);
        self.next_layout_id += 1;

        // Floating layout
        let floating = WorkspaceLayout::new(
            format!("layout_{}", self.next_layout_id),
            "Floating".to_string(),
            DesktopLayoutType::Floating,
        );
        self.layouts.insert(floating.id.clone(), floating);
        self.next_layout_id += 1;

        // Tabbed layout
        let tabbed = WorkspaceLayout::new(
            format!("layout_{}", self.next_layout_id),
            "Tabbed".to_string(),
            DesktopLayoutType::Tabbed,
        );
        self.layouts.insert(tabbed.id.clone(), tabbed);
        self.next_layout_id += 1;

        // Grid layout
        let grid = WorkspaceLayout::new(
            format!("layout_{}", self.next_layout_id),
            "Grid".to_string(),
            DesktopLayoutType::Grid,
        );
        self.layouts.insert(grid.id.clone(), grid);
        self.next_layout_id += 1;
    }

    pub fn add_layout(&mut self, name: String, layout_type: DesktopLayoutType) -> String {
        let id = format!("layout_{}", self.next_layout_id);
        let layout = WorkspaceLayout::new(id.clone(), name, layout_type);
        self.layouts.insert(id.clone(), layout);
        self.next_layout_id += 1;
        id
    }

    pub fn remove_layout(&mut self, id: &str) -> bool {
        if let Some(layout) = self.layouts.get(id) {
            if layout.is_default {
                return false; // Cannot remove default layout
            }
        }

        let removed = self.layouts.remove(id).is_some();

        // If we removed the default layout, clear it
        if self.default_layout.as_ref() == Some(&id.to_string()) {
            self.default_layout = None;
        }

        removed
    }

    pub fn get_layout(&self, id: &str) -> Option<&WorkspaceLayout> {
        self.layouts.get(id)
    }

    pub fn get_layouts(&self) -> Vec<&WorkspaceLayout> {
        self.layouts.values().collect()
    }

    pub fn get_layouts_by_type(&self, layout_type: DesktopLayoutType) -> Vec<&WorkspaceLayout> {
        self.layouts
            .values()
            .filter(|l| l.layout_type == layout_type)
            .collect()
    }

    pub fn get_default_layout(&self) -> Option<&WorkspaceLayout> {
        if let Some(default_id) = &self.default_layout {
            self.layouts.get(default_id)
        } else {
            None
        }
    }

    pub fn set_default_layout(&mut self, id: &str) -> bool {
        if !self.layouts.contains_key(id) {
            return false;
        }

        // Clear default from all layouts
        for layout in self.layouts.values_mut() {
            layout.set_default(false);
        }

        // Set new default
        if let Some(layout) = self.layouts.get_mut(id) {
            layout.set_default(true);
            self.default_layout = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn set_layout_gaps(&mut self, id: &str, inner: u32, outer: u32) -> bool {
        if let Some(layout) = self.layouts.get_mut(id) {
            layout.set_gaps(inner, outer);
            true
        } else {
            false
        }
    }

    pub fn set_layout_border_size(&mut self, id: &str, size: u32) -> bool {
        if let Some(layout) = self.layouts.get_mut(id) {
            layout.set_border_size(size);
            true
        } else {
            false
        }
    }

    pub fn get_statistics(&self) -> DesktopLayoutStatistics {
        DesktopLayoutStatistics {
            total_layouts: self.layouts.len(),
            tiling_layouts: self.get_layouts_by_type(DesktopLayoutType::Tiling).len(),
            floating_layouts: self.get_layouts_by_type(DesktopLayoutType::Floating).len(),
        }
    }
}

impl Default for DesktopLayoutManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Layout statistics
#[derive(Debug, Clone, Copy)]
pub struct DesktopLayoutStatistics {
    pub total_layouts: usize,
    pub tiling_layouts: usize,
    pub floating_layouts: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_manager_initialization() {
        let manager = DesktopLayoutManager::new();
        assert_eq!(manager.get_layouts().len(), 4);
        assert!(manager.get_default_layout().is_some());
    }

    #[test]
    fn test_add_layout() {
        let mut manager = DesktopLayoutManager::new();
        let id = manager.add_layout("Spiral".to_string(), DesktopLayoutType::Spiral);
        assert!(manager.get_layout(&id).is_some());
        assert_eq!(manager.get_layouts().len(), 5);
    }

    #[test]
    fn test_remove_layout() {
        let mut manager = DesktopLayoutManager::new();
        let id = manager.add_layout("Spiral".to_string(), DesktopLayoutType::Spiral);
        assert!(manager.remove_layout(&id));
        assert!(!manager.get_layout(&id).is_some());
        assert_eq!(manager.get_layouts().len(), 4);
    }

    #[test]
    fn test_remove_default_layout() {
        let mut manager = DesktopLayoutManager::new();
        let default_id = manager.get_default_layout().unwrap().id.clone();
        assert!(!manager.remove_layout(&default_id));
    }

    #[test]
    fn test_set_default_layout() {
        let mut manager = DesktopLayoutManager::new();
        let id = manager.add_layout("Spiral".to_string(), DesktopLayoutType::Spiral);
        assert!(manager.set_default_layout(&id));
        assert_eq!(
            manager.get_default_layout().unwrap().layout_type,
            DesktopLayoutType::Spiral
        );
    }

    #[test]
    fn test_set_layout_gaps() {
        let mut manager = DesktopLayoutManager::new();
        let id = manager.add_layout("Spiral".to_string(), DesktopLayoutType::Spiral);
        assert!(manager.set_layout_gaps(&id, 16, 16));
        assert_eq!(manager.get_layout(&id).unwrap().gaps, (16, 16));
    }

    #[test]
    fn test_set_layout_border_size() {
        let mut manager = DesktopLayoutManager::new();
        let id = manager.add_layout("Spiral".to_string(), DesktopLayoutType::Spiral);
        assert!(manager.set_layout_border_size(&id, 4));
        assert_eq!(manager.get_layout(&id).unwrap().border_size, 4);
    }

    #[test]
    fn test_get_layouts_by_type() {
        let manager = DesktopLayoutManager::new();
        let tiling = manager.get_layouts_by_type(DesktopLayoutType::Tiling);
        assert_eq!(tiling.len(), 1);

        let floating = manager.get_layouts_by_type(DesktopLayoutType::Floating);
        assert_eq!(floating.len(), 1);
    }

    #[test]
    fn test_statistics() {
        let manager = DesktopLayoutManager::new();
        let stats = manager.get_statistics();
        assert_eq!(stats.total_layouts, 4);
        assert_eq!(stats.tiling_layouts, 1);
        assert_eq!(stats.floating_layouts, 1);
    }
}
