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

/// Window area
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowArea {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

/// Tiling window
#[derive(Debug, Clone)]
pub struct TilingWindow {
    pub id: u64,
    pub workspace_id: u64,
    pub area: WindowArea,
    pub is_floating: bool,
    pub is_fullscreen: bool,
}

/// Workspace
#[derive(Debug, Clone)]
pub struct Workspace {
    pub id: u64,
    pub name: String,
    pub layout: TilingLayout,
    pub gaps: u32,
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
        Self {
            next_window_id: AtomicU64::new(1),
            next_workspace_id: AtomicU64::new(1),
            windows: HashMap::new(),
            workspaces: HashMap::new(),
            active_workspace: 1,
            focused_window: None,
        }
    }

    /// Create a workspace
    pub fn create_workspace(&mut self, name: String, layout: TilingLayout) -> Workspace {
        let id = self.next_workspace_id.fetch_add(1, Ordering::SeqCst);
        
        let workspace = Workspace {
            id,
            name,
            layout,
            gaps: 8,
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

        let window_ids: Vec<u64> = self.windows
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
    pub fn set_layout(&mut self, workspace_id: u64, layout: TilingLayout) -> Result<(), &'static str> {
        if let Some(workspace) = self.workspaces.get_mut(&workspace_id) {
            workspace.layout = layout;
            self.rearrange_workspace(workspace_id);
            Ok(())
        } else {
            Err("Workspace not found")
        }
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
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let area = WindowArea { x: 0, y: 0, width: 800, height: 600 };
        
        let window = manager.add_window(workspace.id, area);
        assert_eq!(window.id, 1);
        assert_eq!(manager.window_count(), 1);
    }

    #[test]
    fn test_remove_window() {
        let mut manager = TilingWindowManager::new();
        
        let workspace = manager.create_workspace("1".to_string(), TilingLayout::Spiral);
        let area = WindowArea { x: 0, y: 0, width: 800, height: 600 };
        
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
    }

    #[test]
    fn test_set_layout() {
        let mut manager = TilingWindowManager::new();
        
        let workspace = manager.create_workspace("1".to_string(), TilingLayout::Spiral);
        assert!(manager.set_layout(workspace.id, TilingLayout::Grid).is_ok());
    }
}
