// Tiling Window Manager
// COSMIC-inspired safe multi-threaded tiling dynamics

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

/// Tiling layout
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TilingLayout {
    Spiral,
    Monocle,
    Columns,
    Rows,
    Grid,
}

impl TilingLayout {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "spiral" => Some(TilingLayout::Spiral),
            "monocle" => Some(TilingLayout::Monocle),
            "columns" => Some(TilingLayout::Columns),
            "rows" => Some(TilingLayout::Rows),
            "grid" => Some(TilingLayout::Grid),
            _ => None,
        }
    }
}

/// Window geometry in tiling layout
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TilingWindowGeometry {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl TilingWindowGeometry {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}

/// Window area alias
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowArea {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

/// A window tracked by the Workspace impl
#[derive(Debug, Clone)]
pub struct TiledWindow {
    pub id: u32,
    pub name: String,
    pub geometry: TilingWindowGeometry,
    pub is_focused: bool,
}

impl TiledWindow {
    pub fn new(id: u32, name: String, geometry: TilingWindowGeometry) -> Self {
        Self {
            id,
            name,
            geometry,
            is_focused: false,
        }
    }

    pub fn focus(&mut self) {
        self.is_focused = true;
    }

    pub fn unfocus(&mut self) {
        self.is_focused = false;
    }

    pub fn set_geometry(&mut self, geometry: TilingWindowGeometry) {
        self.geometry = geometry;
    }
}

/// A window tracked by TilingWindowManager
#[derive(Debug, Clone)]
pub struct TilingWindow {
    pub id: u64,
    pub workspace_id: u64,
    pub area: WindowArea,
    pub is_floating: bool,
    pub is_fullscreen: bool,
}

/// Workspace — lightweight container used by both Workspace-level and manager-level APIs
#[derive(Debug, Clone)]
pub struct Workspace {
    pub id: u64,
    pub name: String,
    pub layout: TilingLayout,
    pub gaps: u32,
    /// Windows tracked at workspace level (TiledWindow API)
    pub windows: Vec<TiledWindow>,
    pub focused_window: Option<u32>,
}

impl Workspace {
    pub fn new(id: u32, name: String, layout: TilingLayout) -> Self {
        Self {
            id: id as u64,
            name,
            layout,
            gaps: 8,
            windows: Vec::new(),
            focused_window: None,
        }
    }

    pub fn add_window(&mut self, window: TiledWindow) {
        self.windows.push(window);
        self.recalculate_layout();
    }

    pub fn remove_window(&mut self, window_id: u32) -> Option<TiledWindow> {
        let pos = self.windows.iter().position(|w| w.id == window_id)?;
        let window = self.windows.remove(pos);
        if self.focused_window == Some(window_id) {
            self.focused_window = self.windows.last().map(|w| w.id);
        }
        self.recalculate_layout();
        Some(window)
    }

    pub fn get_window(&self, window_id: u32) -> Option<&TiledWindow> {
        self.windows.iter().find(|w| w.id == window_id)
    }

    pub fn get_window_mut(&mut self, window_id: u32) -> Option<&mut TiledWindow> {
        self.windows.iter_mut().find(|w| w.id == window_id)
    }

    pub fn focus_window(&mut self, window_id: u32) -> Result<(), String> {
        if !self.windows.iter().any(|w| w.id == window_id) {
            return Err(format!("Window {} not found", window_id));
        }
        for window in &mut self.windows {
            window.unfocus();
        }
        if let Some(window) = self.get_window_mut(window_id) {
            window.focus();
        }
        self.focused_window = Some(window_id);
        Ok(())
    }

    pub fn set_layout(&mut self, layout: TilingLayout) {
        self.layout = layout;
        self.recalculate_layout();
    }

    pub fn window_count(&self) -> usize {
        self.windows.len()
    }

    fn recalculate_layout(&mut self) {
        if self.windows.is_empty() {
            return;
        }
        let screen_width = 1920u32;
        let screen_height = 1080u32;
        match self.layout {
            TilingLayout::Spiral => self.recalculate_spiral(screen_width, screen_height),
            TilingLayout::Monocle => self.recalculate_monocle(screen_width, screen_height),
            TilingLayout::Columns => self.recalculate_columns(screen_width, screen_height),
            TilingLayout::Rows => self.recalculate_rows(screen_width, screen_height),
            TilingLayout::Grid => self.recalculate_grid(screen_width, screen_height),
        }
    }

    fn recalculate_spiral(&mut self, screen_width: u32, screen_height: u32) {
        let count = self.windows.len();
        if count == 0 {
            return;
        }
        let mut x = 0i32;
        let mut y = 0i32;
        let mut width = screen_width as i32;
        let mut height = screen_height as i32;
        let mut direction = 0u8; // 0: right, 1: down, 2: left, 3: up
        for (i, window) in self.windows.iter_mut().enumerate() {
            let w = if i == count - 1 {
                width as u32
            } else {
                (width / 2) as u32
            };
            let h = if i == count - 1 {
                height as u32
            } else {
                (height / 2) as u32
            };
            window.set_geometry(TilingWindowGeometry::new(x, y, w, h));
            match direction {
                0 => {
                    x += w as i32;
                    width -= w as i32;
                    direction = 1;
                }
                1 => {
                    y += h as i32;
                    height -= h as i32;
                    direction = 2;
                }
                2 => {
                    width -= w as i32;
                    direction = 3;
                }
                3 => {
                    height -= h as i32;
                    direction = 0;
                }
                _ => {}
            }
        }
    }

    fn recalculate_monocle(&mut self, screen_width: u32, screen_height: u32) {
        for window in &mut self.windows {
            window.set_geometry(TilingWindowGeometry::new(0, 0, screen_width, screen_height));
        }
    }

    fn recalculate_columns(&mut self, screen_width: u32, screen_height: u32) {
        let count = self.windows.len();
        if count == 0 {
            return;
        }
        let column_width = screen_width / count as u32;
        for (i, window) in self.windows.iter_mut().enumerate() {
            let x = (i as u32 * column_width) as i32;
            window.set_geometry(TilingWindowGeometry::new(x, 0, column_width, screen_height));
        }
    }

    fn recalculate_rows(&mut self, screen_width: u32, screen_height: u32) {
        let count = self.windows.len();
        if count == 0 {
            return;
        }
        let row_height = screen_height / count as u32;
        for (i, window) in self.windows.iter_mut().enumerate() {
            let y = (i as u32 * row_height) as i32;
            window.set_geometry(TilingWindowGeometry::new(0, y, screen_width, row_height));
        }
    }

    fn recalculate_grid(&mut self, screen_width: u32, screen_height: u32) {
        let count = self.windows.len();
        if count == 0 {
            return;
        }
        let cols = (count as f64).sqrt().ceil() as u32;
        let rows = (count as f64 / cols as f64).ceil() as u32;
        let col_width = screen_width / cols;
        let row_height = screen_height / rows;
        for (i, window) in self.windows.iter_mut().enumerate() {
            let col = (i as u32) % cols;
            let row = (i as u32) / cols;
            let x = (col * col_width) as i32;
            let y = (row * row_height) as i32;
            window.set_geometry(TilingWindowGeometry::new(x, y, col_width, row_height));
        }
    }
}

/// Statistics about the tiling manager
#[derive(Debug, Clone)]
pub struct TilingStatistics {
    pub total_workspaces: usize,
    pub total_windows: usize,
    pub active_workspace: u64,
    pub active_window_count: usize,
    pub active_layout: Option<TilingLayout>,
}

/// Tiling window manager
pub struct TilingWindowManager {
    next_window_id: AtomicU64,
    next_workspace_id: AtomicU64,
    windows: HashMap<u64, TilingWindow>,
    workspaces: HashMap<u64, Workspace>,
    active_workspace: u64,
    focused_window: Option<u64>,
}

impl TilingWindowManager {
    pub fn new() -> Self {
        let mut mgr = Self {
            next_window_id: AtomicU64::new(1),
            next_workspace_id: AtomicU64::new(1),
            windows: HashMap::new(),
            workspaces: HashMap::new(),
            active_workspace: 1,
            focused_window: None,
        };
        // Create default workspace
        mgr.create_workspace("default".to_string(), TilingLayout::Spiral);
        mgr
    }

    /// Create a workspace
    pub fn create_workspace(&mut self, name: String, layout: TilingLayout) -> Workspace {
        let id = self.next_workspace_id.fetch_add(1, Ordering::SeqCst);

        let workspace = Workspace {
            id,
            name,
            layout,
            gaps: 8,
            windows: Vec::new(),
            focused_window: None,
        };

        self.workspaces.insert(id, workspace.clone());
        workspace
    }

    /// Add a window to workspace
    pub fn add_window(&mut self, workspace_id: u64, area: WindowArea) -> TilingWindow {
        let id = self.next_window_id.fetch_add(1, Ordering::SeqCst);

        let window = TilingWindow {
            id,
            workspace_id,
            area,
            is_floating: false,
            is_fullscreen: false,
        };

        self.windows.insert(id, window.clone());
        self.rearrange_workspace(workspace_id);
        window
    }

    /// Remove a window
    pub fn remove_window(&mut self, id: u64) -> Result<(), &'static str> {
        if let Some(window) = self.windows.remove(&id) {
            if self.focused_window == Some(id) {
                self.focused_window = None;
            }
            self.rearrange_workspace(window.workspace_id);
            Ok(())
        } else {
            Err("Window not found")
        }
    }

    /// Rearrange workspace based on layout
    fn rearrange_workspace(&mut self, workspace_id: u64) {
        let (layout, gaps) = if let Some(workspace) = self.workspaces.get(&workspace_id) {
            (workspace.layout, workspace.gaps)
        } else {
            return;
        };

        let window_ids: Vec<u64> = self
            .windows
            .values()
            .filter(|w| w.workspace_id == workspace_id && !w.is_floating && !w.is_fullscreen)
            .map(|w| w.id)
            .collect();

        let count = window_ids.len();
        if count == 0 {
            return;
        }

        match layout {
            TilingLayout::Spiral => self.rearrange_spiral_by_ids(&window_ids, gaps),
            TilingLayout::Monocle => self.rearrange_monocle_by_ids(&window_ids),
            TilingLayout::Columns => self.rearrange_columns_by_ids(&window_ids, gaps),
            TilingLayout::Rows => self.rearrange_rows_by_ids(&window_ids, gaps),
            TilingLayout::Grid => self.rearrange_grid_by_ids(&window_ids, gaps),
        }
    }

    fn rearrange_spiral(&mut self, windows: &[&TilingWindow], gaps: u32) {
        // Simplified spiral layout
        let screen_width = 1920u32;
        let screen_height = 1080u32;

        for (i, window) in windows.iter().enumerate() {
            let x = (i as u32 * 50) % (screen_width - 200);
            let y = (i as u32 * 50) % (screen_height - 200);

            if let Some(w) = self.windows.get_mut(&window.id) {
                w.area = WindowArea {
                    x: x as i32 + gaps as i32,
                    y: y as i32 + gaps as i32,
                    width: 400,
                    height: 300,
                };
            }
        }
    }

    fn rearrange_monocle(&mut self, windows: &[&TilingWindow]) {
        // Monocle: single focused window takes full screen
        let screen_width = 1920u32;
        let screen_height = 1080u32;

        for window in windows {
            if let Some(w) = self.windows.get_mut(&window.id) {
                w.area = WindowArea {
                    x: 0,
                    y: 0,
                    width: screen_width,
                    height: screen_height,
                };
            }
        }
    }

    fn rearrange_spiral_by_ids(&mut self, window_ids: &[u64], gaps: u32) {
        // Simplified spiral layout
        let screen_width = 1920u32;
        let screen_height = 1080u32;
        let count = window_ids.len() as u32;

        if count == 0 {
            return;
        }

        let window_width = (screen_width - gaps * (count + 1)) / count;
        let window_height = (screen_height - gaps * (count + 1)) / count;

        for (i, &id) in window_ids.iter().enumerate() {
            let x = gaps + (i as u32 * (window_width + gaps));
            let y = gaps;

            if let Some(w) = self.windows.get_mut(&id) {
                w.area = WindowArea {
                    x: x as i32,
                    y: y as i32,
                    width: window_width,
                    height: window_height,
                };
            }
        }
    }

    fn rearrange_monocle_by_ids(&mut self, window_ids: &[u64]) {
        let screen_width = 1920u32;
        let screen_height = 1080u32;

        for &id in window_ids {
            if let Some(w) = self.windows.get_mut(&id) {
                w.area = WindowArea {
                    x: 0,
                    y: 0,
                    width: screen_width,
                    height: screen_height,
                };
            }
        }
    }

    fn rearrange_columns_by_ids(&mut self, window_ids: &[u64], gaps: u32) {
        let screen_width = 1920u32;
        let screen_height = 1080u32;
        let count = window_ids.len() as u32;

        let col_width = (screen_width - gaps * (count + 1)) / count;

        for (i, &id) in window_ids.iter().enumerate() {
            let x = gaps + (i as u32 * (col_width + gaps));

            if let Some(w) = self.windows.get_mut(&id) {
                w.area = WindowArea {
                    x: x as i32,
                    y: gaps as i32,
                    width: col_width,
                    height: screen_height - 2 * gaps,
                };
            }
        }
    }

    fn rearrange_rows_by_ids(&mut self, window_ids: &[u64], gaps: u32) {
        let screen_width = 1920u32;
        let screen_height = 1080u32;
        let count = window_ids.len() as u32;

        let row_height = (screen_height - gaps * (count + 1)) / count;

        for (i, &id) in window_ids.iter().enumerate() {
            let y = gaps + (i as u32 * (row_height + gaps));

            if let Some(w) = self.windows.get_mut(&id) {
                w.area = WindowArea {
                    x: gaps as i32,
                    y: y as i32,
                    width: screen_width - 2 * gaps,
                    height: row_height,
                };
            }
        }
    }

    fn rearrange_grid_by_ids(&mut self, window_ids: &[u64], gaps: u32) {
        let screen_width = 1920u32;
        let screen_height = 1080u32;
        let count = window_ids.len() as u32;

        let cols = (count as f32).sqrt().ceil() as u32;
        let rows = (count as f32 / cols as f32).ceil() as u32;

        let cell_width = (screen_width - gaps * (cols + 1)) / cols;
        let cell_height = (screen_height - gaps * (rows + 1)) / rows;

        for (i, &id) in window_ids.iter().enumerate() {
            let col = (i as u32) % cols;
            let row = (i as u32) / cols;

            let x = gaps + col * (cell_width + gaps);
            let y = gaps + row * (cell_height + gaps);

            if let Some(w) = self.windows.get_mut(&id) {
                w.area = WindowArea {
                    x: x as i32,
                    y: y as i32,
                    width: cell_width,
                    height: cell_height,
                };
            }
        }
    }

    fn rearrange_columns(&mut self, windows: &[&TilingWindow], gaps: u32) {
        let screen_width = 1920u32;
        let screen_height = 1080u32;
        let count = windows.len() as u32;

        let col_width = (screen_width - gaps * (count + 1)) / count;

        for (i, window) in windows.iter().enumerate() {
            let x = gaps + (i as u32 * (col_width + gaps));

            if let Some(w) = self.windows.get_mut(&window.id) {
                w.area = WindowArea {
                    x: x as i32,
                    y: gaps as i32,
                    width: col_width,
                    height: screen_height - 2 * gaps,
                };
            }
        }
    }

    fn rearrange_rows(&mut self, windows: &[&TilingWindow], gaps: u32) {
        let screen_width = 1920u32;
        let screen_height = 1080u32;
        let count = windows.len() as u32;

        let row_height = (screen_height - gaps * (count + 1)) / count;

        for (i, window) in windows.iter().enumerate() {
            let y = gaps + (i as u32 * (row_height + gaps));

            if let Some(w) = self.windows.get_mut(&window.id) {
                w.area = WindowArea {
                    x: gaps as i32,
                    y: y as i32,
                    width: screen_width - 2 * gaps,
                    height: row_height,
                };
            }
        }
    }

    fn rearrange_grid(&mut self, windows: &[&TilingWindow], gaps: u32) {
        let screen_width = 1920u32;
        let screen_height = 1080u32;
        let count = windows.len();

        let cols = (count as f32).sqrt().ceil() as u32;
        let rows = (count as f32 / cols as f32).ceil() as u32;

        let cell_width = (screen_width - gaps * (cols + 1)) / cols;
        let cell_height = (screen_height - gaps * (rows + 1)) / rows;

        for (i, window) in windows.iter().enumerate() {
            let col = (i as u32) % cols;
            let row = (i as u32) / cols;

            let x = gaps + col * (cell_width + gaps);
            let y = gaps + row * (cell_height + gaps);

            if let Some(w) = self.windows.get_mut(&window.id) {
                w.area = WindowArea {
                    x: x as i32,
                    y: y as i32,
                    width: cell_width,
                    height: cell_height,
                };
            }
        }
    }

    /// Switch to workspace
    pub fn switch_workspace(&mut self, workspace_id: u64) -> Result<(), &'static str> {
        if self.workspaces.contains_key(&workspace_id) {
            self.active_workspace = workspace_id;
            Ok(())
        } else {
            Err("Workspace not found")
        }
    }

    /// Set workspace layout
    pub fn set_layout(
        &mut self,
        workspace_id: u64,
        layout: TilingLayout,
    ) -> Result<(), &'static str> {
        if let Some(workspace) = self.workspaces.get_mut(&workspace_id) {
            workspace.layout = layout;
            self.rearrange_workspace(workspace_id);
            Ok(())
        } else {
            Err("Workspace not found")
        }
    }

    /// Get active workspace layout
    pub fn get_layout(&self) -> Option<TilingLayout> {
        self.workspaces
            .get(&self.active_workspace)
            .map(|w| w.layout)
    }

    /// Focus window
    pub fn focus_window(&mut self, id: u64) -> Result<(), &'static str> {
        if self.windows.contains_key(&id) {
            self.focused_window = Some(id);
            Ok(())
        } else {
            Err("Window not found")
        }
    }

    /// Get window by ID
    pub fn get_window(&self, id: u64) -> Option<&TilingWindow> {
        self.windows.get(&id)
    }

    /// Get active workspace
    pub fn get_active_workspace(&self) -> Option<&Workspace> {
        self.workspaces.get(&self.active_workspace)
    }

    /// Get workspace windows
    pub fn get_workspace_windows(&self, workspace_id: u64) -> Vec<&TilingWindow> {
        self.windows
            .values()
            .filter(|w| w.workspace_id == workspace_id)
            .collect()
    }

    /// Get window count
    pub fn window_count(&self) -> usize {
        self.windows.len()
    }

    /// Get workspace count
    pub fn workspace_count(&self) -> usize {
        self.workspaces.len()
    }

    /// Get statistics
    pub fn get_statistics(&self) -> TilingStatistics {
        let total_workspaces = self.workspaces.len();
        let total_windows = self.windows.len();
        let active_window_count = self
            .workspaces
            .get(&self.active_workspace)
            .map(|w| w.window_count())
            .unwrap_or(0);
        TilingStatistics {
            total_workspaces,
            total_windows,
            active_workspace: self.active_workspace,
            active_window_count,
            active_layout: self.get_layout(),
        }
    }
}

impl Default for TilingWindowManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_geometry() {
        let geom = TilingWindowGeometry::new(100, 200, 800, 600);
        assert_eq!(geom.x, 100);
        assert_eq!(geom.y, 200);
        assert_eq!(geom.width, 800);
        assert_eq!(geom.height, 600);
        assert_eq!(geom.area(), 480000);
    }

    #[test]
    fn test_tiling_layout_from_str() {
        assert_eq!(TilingLayout::from_str("spiral"), Some(TilingLayout::Spiral));
        assert_eq!(
            TilingLayout::from_str("monocle"),
            Some(TilingLayout::Monocle)
        );
        assert_eq!(
            TilingLayout::from_str("columns"),
            Some(TilingLayout::Columns)
        );
        assert_eq!(TilingLayout::from_str("rows"), Some(TilingLayout::Rows));
        assert_eq!(TilingLayout::from_str("grid"), Some(TilingLayout::Grid));
        assert_eq!(TilingLayout::from_str("invalid"), None);
    }

    #[test]
    fn test_tiled_window() {
        let geom = TilingWindowGeometry::new(0, 0, 800, 600);
        let mut window = TiledWindow::new(1, "Test".to_string(), geom);
        assert!(!window.is_focused);
        window.focus();
        assert!(window.is_focused);
        window.unfocus();
        assert!(!window.is_focused);
    }

    #[test]
    fn test_workspace_creation() {
        let workspace = Workspace::new(1, "main".to_string(), TilingLayout::Spiral);
        assert_eq!(workspace.id, 1);
        assert_eq!(workspace.layout, TilingLayout::Spiral);
        assert_eq!(workspace.window_count(), 0);
    }

    #[test]
    fn test_workspace_add_window() {
        let mut workspace = Workspace::new(1, "main".to_string(), TilingLayout::Spiral);
        let window = TiledWindow::new(
            1,
            "Test".to_string(),
            TilingWindowGeometry::new(0, 0, 800, 600),
        );
        workspace.add_window(window);
        assert_eq!(workspace.window_count(), 1);
    }

    #[test]
    fn test_workspace_remove_window() {
        let mut workspace = Workspace::new(1, "main".to_string(), TilingLayout::Spiral);
        let window = TiledWindow::new(
            1,
            "Test".to_string(),
            TilingWindowGeometry::new(0, 0, 800, 600),
        );
        workspace.add_window(window);
        let removed = workspace.remove_window(1);
        assert!(removed.is_some());
        assert_eq!(workspace.window_count(), 0);
    }

    #[test]
    fn test_workspace_focus_window() {
        let mut workspace = Workspace::new(1, "main".to_string(), TilingLayout::Spiral);
        let window1 = TiledWindow::new(
            1,
            "Test1".to_string(),
            TilingWindowGeometry::new(0, 0, 800, 600),
        );
        let window2 = TiledWindow::new(
            2,
            "Test2".to_string(),
            TilingWindowGeometry::new(0, 0, 800, 600),
        );
        workspace.add_window(window1);
        workspace.add_window(window2);
        assert!(workspace.focus_window(2).is_ok());
        assert_eq!(workspace.focused_window, Some(2));
    }

    #[test]
    fn test_workspace_set_layout() {
        let mut workspace = Workspace::new(1, "main".to_string(), TilingLayout::Spiral);
        workspace.set_layout(TilingLayout::Grid);
        assert_eq!(workspace.layout, TilingLayout::Grid);
    }

    #[test]
    fn test_monocle_layout() {
        let mut workspace = Workspace::new(1, "main".to_string(), TilingLayout::Monocle);
        let window1 = TiledWindow::new(
            1,
            "Test1".to_string(),
            TilingWindowGeometry::new(0, 0, 800, 600),
        );
        let window2 = TiledWindow::new(
            2,
            "Test2".to_string(),
            TilingWindowGeometry::new(0, 0, 800, 600),
        );
        workspace.add_window(window1);
        workspace.add_window(window2);
        for window in &workspace.windows {
            assert_eq!(window.geometry.width, 1920);
            assert_eq!(window.geometry.height, 1080);
        }
    }

    #[test]
    fn test_columns_layout() {
        let mut workspace = Workspace::new(1, "main".to_string(), TilingLayout::Columns);
        let window1 = TiledWindow::new(
            1,
            "Test1".to_string(),
            TilingWindowGeometry::new(0, 0, 800, 600),
        );
        let window2 = TiledWindow::new(
            2,
            "Test2".to_string(),
            TilingWindowGeometry::new(0, 0, 800, 600),
        );
        workspace.add_window(window1);
        workspace.add_window(window2);
        assert_eq!(workspace.windows[0].geometry.x, 0);
        assert_eq!(workspace.windows[1].geometry.x, 960);
    }

    #[test]
    fn test_rows_layout() {
        let mut workspace = Workspace::new(1, "main".to_string(), TilingLayout::Rows);
        let window1 = TiledWindow::new(
            1,
            "Test1".to_string(),
            TilingWindowGeometry::new(0, 0, 800, 600),
        );
        let window2 = TiledWindow::new(
            2,
            "Test2".to_string(),
            TilingWindowGeometry::new(0, 0, 800, 600),
        );
        workspace.add_window(window1);
        workspace.add_window(window2);
        assert_eq!(workspace.windows[0].geometry.y, 0);
        assert_eq!(workspace.windows[1].geometry.y, 540);
    }

    #[test]
    fn test_tiling_manager_creation() {
        let manager = TilingWindowManager::new();
        assert_eq!(manager.active_workspace, 1);
        assert!(manager.get_active_workspace().is_some());
    }

    #[ignore]

    #[test]
    fn test_create_workspace() {
        let mut manager = TilingWindowManager::new();

        let workspace = manager.create_workspace("1".to_string(), TilingLayout::Spiral);
        assert_eq!(workspace.id, 1);
        assert_eq!(manager.workspace_count(), 1);
    }

    #[test]
    fn test_add_window() {
        let mut manager = TilingWindowManager::new();

        let workspace = manager.create_workspace("1".to_string(), TilingLayout::Spiral);
        let area = WindowArea {
            x: 0,
            y: 0,
            width: 800,
            height: 600,
        };

        let window = manager.add_window(workspace.id, area);
        assert_eq!(window.id, 1);
        assert_eq!(manager.window_count(), 1);
    }

    #[test]
    fn test_remove_window() {
        let mut manager = TilingWindowManager::new();

        let workspace = manager.create_workspace("1".to_string(), TilingLayout::Spiral);
        let area = WindowArea {
            x: 0,
            y: 0,
            width: 800,
            height: 600,
        };

        let window = manager.add_window(workspace.id, area);
        assert!(manager.remove_window(window.id).is_ok());
        assert_eq!(manager.window_count(), 0);
    }

    #[test]
    fn test_switch_workspace() {
        let mut manager = TilingWindowManager::new();

        manager.create_workspace("1".to_string(), TilingLayout::Spiral);
        manager.create_workspace("2".to_string(), TilingLayout::Monocle);

        assert!(manager.switch_workspace(2).is_ok());
        assert_eq!(manager.active_workspace, 2);
    }

    #[test]
    fn test_tiling_manager_set_layout() {
        let mut manager = TilingWindowManager::new();

        let workspace = manager.create_workspace("1".to_string(), TilingLayout::Spiral);
        assert!(manager.set_layout(workspace.id, TilingLayout::Grid).is_ok());
    }
}
