#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(dead_code)]

// SigmaOS Hyprland Tiling Manager - Omarchy Hyprland-inspired Tiling Window Manager
// Dynamic tiling Wayland compositor with dwindle and scrolling layouts

use std::collections::HashMap;

/// Layout type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutType {
    Dwindle,     // All windows visible, splitting focused window
    Scrolling,   // Side-scrolling tape of columns
}

/// Window state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowState {
    Tiled,       // Tiled in layout
    Floating,    // Floating on top
    Pinned,      // Pinned as float
}

/// Window
#[derive(Debug, Clone)]
pub struct Window {
    pub id: u32,
    pub title: String,
    pub class: String,
    pub workspace: u32,
    pub state: WindowState,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub is_focused: bool,
}

/// Hyprland workspace
#[derive(Debug, Clone)]
pub struct HyprlandWorkspace {
    pub id: u32,
    pub name: String,
    pub layout: LayoutType,
    pub windows: Vec<u32>,
    pub is_active: bool,
}

/// Workspace (alias for compatibility)
pub type Workspace = HyprlandWorkspace;

/// Window group
#[derive(Debug, Clone)]
pub struct WindowGroup {
    pub id: u32,
    pub name: String,
    pub windows: Vec<u32>,
    pub is_active: bool,
}

/// Hyprland configuration
#[derive(Debug, Clone)]
pub struct HyprlandConfig {
    pub default_layout: LayoutType,
    pub gap_inner: u32,
    pub gap_outer: u32,
    pub border_size: u32,
    pub preserve_split: bool,
    pub force_split: u32,
    pub auto_tile: bool,
    pub dim_special: f64,
}

impl Default for HyprlandConfig {
    fn default() -> Self {
        Self {
            default_layout: LayoutType::Dwindle,
            gap_inner: 5,
            gap_outer: 10,
            border_size: 2,
            preserve_split: true,
            force_split: 2,
            auto_tile: true,
            dim_special: 0.6,
        }
    }
}

/// Hyprland tiling manager
#[derive(Debug, Clone)]
pub struct HyprlandTilingManager {
    config: HyprlandConfig,
    windows: HashMap<u32, Window>,
    workspaces: HashMap<u32, HyprlandWorkspace>,
    window_groups: HashMap<u32, WindowGroup>,
    current_workspace: u32,
    focused_window: Option<u32>,
    active_group: Option<u32>,
}

impl HyprlandTilingManager {
    pub fn new(config: HyprlandConfig) -> Self {
        let mut workspaces = HashMap::new();
        for i in 1..=10 {
            workspaces.insert(i, HyprlandWorkspace {
                id: i,
                name: i.to_string(),
                layout: config.default_layout,
                windows: vec![],
                is_active: i == 1,
            });
        }

        Self {
            config,
            windows: HashMap::new(),
            workspaces,
            window_groups: HashMap::new(),
            current_workspace: 1,
            focused_window: None,
            active_group: None,
        }
    }

    pub fn with_default_config() -> Self {
        Self::new(HyprlandConfig::default())
    }

    /// Add window
    pub fn add_window(&mut self, window: Window) -> Result<(), String> {
        let window_id = window.id;
        let workspace = self.workspaces.get_mut(&window.workspace);
        if let Some(ws) = workspace {
            ws.windows.push(window_id);
            self.windows.insert(window_id, window);
            self.focused_window = Some(window_id);
            Ok(())
        } else {
            Err(format!("Workspace {} not found", window.workspace))
        }
    }

    /// Remove window
    pub fn remove_window(&mut self, id: u32) -> Result<(), String> {
        if let Some(window) = self.windows.remove(&id) {
            if let Some(workspace) = self.workspaces.get_mut(&window.workspace) {
                workspace.windows.retain(|w| *w != id);
            }
            if self.focused_window == Some(id) {
                self.focused_window = None;
            }
            Ok(())
        } else {
            Err(format!("Window {} not found", id))
        }
    }

    /// Get window
    pub fn get_window(&self, id: u32) -> Option<&Window> {
        self.windows.get(&id)
    }

    /// Get all windows
    pub fn get_windows(&self) -> Vec<&Window> {
        self.windows.values().collect()
    }

    /// Focus window
    pub fn focus_window(&mut self, id: u32) -> Result<(), String> {
        if self.windows.contains_key(&id) {
            self.focused_window = Some(id);
            Ok(())
        } else {
            Err(format!("Window {} not found", id))
        }
    }

    /// Get focused window
    pub fn get_focused_window(&self) -> Option<&Window> {
        self.focused_window.and_then(|id| self.windows.get(&id))
    }

    /// Switch workspace
    pub fn switch_workspace(&mut self, id: u32) -> Result<(), String> {
        if let Some(workspace) = self.workspaces.get(&id) {
            self.current_workspace = id;
            for ws in self.workspaces.values_mut() {
                ws.is_active = ws.id == id;
            }
            Ok(())
        } else {
            Err(format!("Workspace {} not found", id))
        }
    }

    /// Get current workspace
    pub fn get_current_workspace(&self) -> Option<&HyprlandWorkspace> {
        self.workspaces.get(&self.current_workspace)
    }

    /// Move window to workspace
    pub fn move_to_workspace(&mut self, window_id: u32, workspace_id: u32) -> Result<(), String> {
        if !self.windows.contains_key(&window_id) {
            return Err(format!("Window {} not found", window_id));
        }
        if !self.workspaces.contains_key(&workspace_id) {
            return Err(format!("Workspace {} not found", workspace_id));
        }

        let window = self.windows.get(&window_id).unwrap();
        let old_workspace = window.workspace;

        if let Some(ws) = self.workspaces.get_mut(&old_workspace) {
            ws.windows.retain(|w| *w != window_id);
        }

        if let Some(ws) = self.workspaces.get_mut(&workspace_id) {
            ws.windows.push(window_id);
        }

        if let Some(w) = self.windows.get_mut(&window_id) {
            w.workspace = workspace_id;
        }

        Ok(())
    }

    /// Toggle layout
    pub fn toggle_layout(&mut self) -> Result<(), String> {
        if let Some(workspace) = self.workspaces.get_mut(&self.current_workspace) {
            workspace.layout = match workspace.layout {
                LayoutType::Dwindle => LayoutType::Scrolling,
                LayoutType::Scrolling => LayoutType::Dwindle,
            };
            Ok(())
        } else {
            Err("Current workspace not found".to_string())
        }
    }

    /// Set layout
    pub fn set_layout(&mut self, layout: LayoutType) -> Result<(), String> {
        if let Some(workspace) = self.workspaces.get_mut(&self.current_workspace) {
            workspace.layout = layout;
            Ok(())
        } else {
            Err("Current workspace not found".to_string())
        }
    }

    /// Toggle floating
    pub fn toggle_floating(&mut self, id: u32) -> Result<(), String> {
        if let Some(window) = self.windows.get_mut(&id) {
            window.state = match window.state {
                WindowState::Tiled => WindowState::Floating,
                WindowState::Floating => WindowState::Tiled,
                WindowState::Pinned => WindowState::Pinned,
            };
            Ok(())
        } else {
            Err(format!("Window {} not found", id))
        }
    }

    /// Pin window
    pub fn pin_window(&mut self, id: u32) -> Result<(), String> {
        if let Some(window) = self.windows.get_mut(&id) {
            window.state = WindowState::Pinned;
            Ok(())
        } else {
            Err(format!("Window {} not found", id))
        }
    }

    /// Create window group
    pub fn create_group(&mut self, name: String) -> Result<(), String> {
        let id = self.window_groups.len() as u32 + 1;
        self.window_groups.insert(id, WindowGroup {
            id,
            name,
            windows: vec![],
            is_active: false,
        });
        Ok(())
    }

    /// Add window to group
    pub fn add_to_group(&mut self, group_id: u32, window_id: u32) -> Result<(), String> {
        if let Some(group) = self.window_groups.get_mut(&group_id) {
            if self.windows.contains_key(&window_id) {
                group.windows.push(window_id);
                Ok(())
            } else {
                Err(format!("Window {} not found", window_id))
            }
        } else {
            Err(format!("Group {} not found", group_id))
        }
    }

    /// Activate group
    pub fn activate_group(&mut self, group_id: u32) -> Result<(), String> {
        if self.window_groups.contains_key(&group_id) {
            self.active_group = Some(group_id);
            for group in self.window_groups.values_mut() {
                group.is_active = group.id == group_id;
            }
            Ok(())
        } else {
            Err(format!("Group {} not found", group_id))
        }
    }

    /// Deactivate group
    pub fn deactivate_group(&mut self) {
        self.active_group = None;
        for group in self.window_groups.values_mut() {
            group.is_active = false;
        }
    }

    /// Get active group
    pub fn get_active_group(&self) -> Option<&WindowGroup> {
        self.active_group.and_then(|id| self.window_groups.get(&id))
    }

    /// Get statistics
    pub fn get_statistics(&self) -> (usize, usize, usize) {
        let total_windows = self.windows.len();
        let total_workspaces = self.workspaces.len();
        let total_groups = self.window_groups.len();
        (total_windows, total_workspaces, total_groups)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hyprland_manager_creation() {
        let manager = HyprlandTilingManager::with_default_config();
        assert_eq!(manager.workspaces.len(), 10);
        assert_eq!(manager.current_workspace, 1);
    }

    #[test]
    fn test_config_default() {
        let config = HyprlandConfig::default();
        assert_eq!(config.default_layout, LayoutType::Dwindle);
        assert_eq!(config.gap_inner, 5);
        assert_eq!(config.gap_outer, 10);
        assert!(config.preserve_split);
        assert_eq!(config.force_split, 2);
    }

    #[test]
    fn test_add_window() {
        let mut manager = HyprlandTilingManager::with_default_config();
        let window = Window {
            id: 1,
            title: "Test Window".to_string(),
            class: "Test".to_string(),
            workspace: 1,
            state: WindowState::Tiled,
            x: 0,
            y: 0,
            width: 800,
            height: 600,
            is_focused: false,
        };

        manager.add_window(window).unwrap();
        assert_eq!(manager.windows.len(), 1);
        assert_eq!(manager.focused_window, Some(1));
    }

    #[test]
    fn test_switch_workspace() {
        let mut manager = HyprlandTilingManager::with_default_config();
        manager.switch_workspace(2).unwrap();
        assert_eq!(manager.current_workspace, 2);
        assert!(manager.get_current_workspace().unwrap().is_active);
    }

    #[test]
    fn test_toggle_layout() {
        let mut manager = HyprlandTilingManager::with_default_config();
        manager.toggle_layout().unwrap();
        assert_eq!(manager.get_current_workspace().unwrap().layout, LayoutType::Scrolling);
        manager.toggle_layout().unwrap();
        assert_eq!(manager.get_current_workspace().unwrap().layout, LayoutType::Dwindle);
    }

    #[test]
    fn test_toggle_floating() {
        let mut manager = HyprlandTilingManager::with_default_config();
        let window = Window {
            id: 1,
            title: "Test Window".to_string(),
            class: "Test".to_string(),
            workspace: 1,
            state: WindowState::Tiled,
            x: 0,
            y: 0,
            width: 800,
            height: 600,
            is_focused: false,
        };

        manager.add_window(window).unwrap();
        manager.toggle_floating(1).unwrap();
        assert_eq!(manager.get_window(1).unwrap().state, WindowState::Floating);
    }

    #[test]
    fn test_move_to_workspace() {
        let mut manager = HyprlandTilingManager::with_default_config();
        let window = Window {
            id: 1,
            title: "Test Window".to_string(),
            class: "Test".to_string(),
            workspace: 1,
            state: WindowState::Tiled,
            x: 0,
            y: 0,
            width: 800,
            height: 600,
            is_focused: false,
        };

        manager.add_window(window).unwrap();
        manager.move_to_workspace(1, 2).unwrap();
        assert_eq!(manager.get_window(1).unwrap().workspace, 2);
    }

    #[test]
    fn test_window_group() {
        let mut manager = HyprlandTilingManager::with_default_config();
        manager.create_group("Test Group".to_string()).unwrap();
        assert_eq!(manager.window_groups.len(), 1);
    }

    #[test]
    fn test_get_statistics() {
        let mut manager = HyprlandTilingManager::with_default_config();
        let window = Window {
            id: 1,
            title: "Test Window".to_string(),
            class: "Test".to_string(),
            workspace: 1,
            state: WindowState::Tiled,
            x: 0,
            y: 0,
            width: 800,
            height: 600,
            is_focused: false,
        };

        manager.add_window(window).unwrap();
        let (windows, workspaces, groups) = manager.get_statistics();

        assert_eq!(windows, 1);
        assert_eq!(workspaces, 10);
        assert_eq!(groups, 0);
    }
}
