// SPDX-License-Identifier: MIT
// SigmaOS - Hyprland Tiling Window Manager
// Inspired by Hyprland (https://github.com/hyprwm/Hyprland)
// Provides tiling window manager with gaming optimizations

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// Window layout rule
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutRule {
    Dwindle,
    MasterStack,
    Split,
}

/// Window state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowState {
    Tiled,
    Floating,
    Fullscreen,
    PseudoFullscreen,
}

/// Window
#[derive(Debug)]
pub struct HyprlandWindow {
    pub id: u32,
    pub title: String,
    pub app_id: String,
    pub state: WindowState,
    pub workspace: u32,
    pub floating: AtomicBool,
    pub fullscreen: AtomicBool,
    pub visible: AtomicBool,
}

impl HyprlandWindow {
    pub fn new(id: u32, title: String, app_id: String, workspace: u32) -> Self {
        Self {
            id,
            title,
            app_id,
            state: WindowState::Tiled,
            workspace,
            floating: AtomicBool::new(false),
            fullscreen: AtomicBool::new(false),
            visible: AtomicBool::new(true),
        }
    }

    /// Set state
    pub fn set_state(&mut self, state: WindowState) {
        self.state = state;
    }

    /// Set floating
    pub fn set_floating(&self, floating: bool) {
        self.floating.store(floating, Ordering::Relaxed);
    }

    /// Set fullscreen
    pub fn set_fullscreen(&self, fullscreen: bool) {
        self.fullscreen.store(fullscreen, Ordering::Relaxed);
    }

    /// Is floating
    pub fn is_floating(&self) -> bool {
        self.floating.load(Ordering::Relaxed)
    }

    /// Is fullscreen
    pub fn is_fullscreen(&self) -> bool {
        self.fullscreen.load(Ordering::Relaxed)
    }

    /// Set visible
    pub fn set_visible(&self, visible: bool) {
        self.visible.store(visible, Ordering::Relaxed);
    }

    /// Is visible
    pub fn is_visible(&self) -> bool {
        self.visible.load(Ordering::Relaxed)
    }
}

/// Workspace
#[derive(Debug, Clone)]
pub struct HyprlandWorkspace {
    pub id: u32,
    pub name: String,
    pub windows: Vec<u32>,
    pub layout_rule: LayoutRule,
}

impl HyprlandWorkspace {
    pub fn new(id: u32, name: String) -> Self {
        Self {
            id,
            name,
            windows: Vec::new(),
            layout_rule: LayoutRule::Dwindle,
        }
    }

    /// Add window
    pub fn add_window(&mut self, window_id: u32) {
        self.windows.push(window_id);
    }

    /// Remove window
    pub fn remove_window(&mut self, window_id: u32) {
        self.windows.retain(|&w| w != window_id);
    }

    /// Set layout rule
    pub fn set_layout_rule(&mut self, rule: LayoutRule) {
        self.layout_rule = rule;
    }
}

/// Hyprland config
#[derive(Debug, Clone, Copy)]
pub struct HyprlandConfig {
    pub gaps_in: u32,
    pub gaps_out: u32,
    pub border_size: u32,
    pub rounding: u32,
    pub shadow_enabled: bool,
    pub floating_enabled: bool,
}

impl Default for HyprlandConfig {
    fn default() -> Self {
        Self {
            gaps_in: 8,
            gaps_out: 8,
            border_size: 2,
            rounding: 8,
            shadow_enabled: true,
            floating_enabled: true,
        }
    }
}

/// Hyprland instance
pub struct HyprlandInstance {
    pub windows: BTreeMap<u32, HyprlandWindow>,
    pub workspaces: BTreeMap<u32, HyprlandWorkspace>,
    pub current_workspace: AtomicU32,
    pub config: HyprlandConfig,
    pub active: AtomicBool,
    pub next_window_id: AtomicU32,
    pub next_workspace_id: AtomicU32,
}

impl HyprlandInstance {
    pub fn new() -> Self {
        Self {
            windows: BTreeMap::new(),
            workspaces: BTreeMap::new(),
            current_workspace: AtomicU32::new(1),
            config: HyprlandConfig::default(),
            active: AtomicBool::new(false),
            next_window_id: AtomicU32::new(1),
            next_workspace_id: AtomicU32::new(1),
        }
    }

    /// Initialize Hyprland
    pub fn init(&mut self) {
        self.active.store(true, Ordering::Relaxed);

        // Create default workspace
        let workspace = HyprlandWorkspace::new(1, "1".to_string());
        self.workspaces.insert(1, workspace);
    }

    /// Create window
    pub fn create_window(&mut self, title: String, app_id: String) -> u32 {
        let id = self.next_window_id.fetch_add(1, Ordering::Relaxed);
        let workspace = self.current_workspace.load(Ordering::Relaxed);
        let window = HyprlandWindow::new(id, title, app_id, workspace);

        self.windows.insert(id, window);

        // Add to workspace
        if let Some(ws) = self.workspaces.get_mut(&workspace) {
            ws.add_window(id);
        }

        id
    }

    /// Get window
    pub fn get_window(&self, id: u32) -> Option<&HyprlandWindow> {
        self.windows.get(&id)
    }

    /// Get mutable window
    pub fn get_window_mut(&mut self, id: u32) -> Option<&mut HyprlandWindow> {
        self.windows.get_mut(&id)
    }

    /// Close window
    pub fn close_window(&mut self, id: u32) -> Result<(), &'static str> {
        let window = self.windows.get(&id).ok_or("Window not found")?;
        let workspace = window.workspace;

        // Remove from workspace
        if let Some(ws) = self.workspaces.get_mut(&workspace) {
            ws.remove_window(id);
        }

        self.windows.remove(&id).ok_or("Window not found")?;
        Ok(())
    }

    /// Create workspace
    pub fn create_workspace(&mut self, name: String) -> u32 {
        let id = self.next_workspace_id.fetch_add(1, Ordering::Relaxed);
        let workspace = HyprlandWorkspace::new(id, name);
        self.workspaces.insert(id, workspace);
        id
    }

    /// Switch to workspace
    pub fn switch_workspace(&mut self, workspace_id: u32) -> Result<(), &'static str> {
        if !self.workspaces.contains_key(&workspace_id) {
            return Err("Workspace not found");
        }

        self.current_workspace
            .store(workspace_id, Ordering::Relaxed);

        // Update all windows to new workspace
        for window in self.windows.values_mut() {
            window.workspace = workspace_id;
        }

        Ok(())
    }

    /// Get current workspace
    pub fn get_current_workspace(&self) -> u32 {
        self.current_workspace.load(Ordering::Relaxed)
    }

    /// Toggle floating
    pub fn toggle_floating(&self, id: u32) -> Result<(), &'static str> {
        let window = self.windows.get(&id).ok_or("Window not found")?;
        let floating = !window.is_floating();
        window.set_floating(floating);
        Ok(())
    }

    /// Toggle fullscreen
    pub fn toggle_fullscreen(&self, id: u32) -> Result<(), &'static str> {
        let window = self.windows.get(&id).ok_or("Window not found")?;
        let fullscreen = !window.is_fullscreen();
        window.set_fullscreen(fullscreen);
        Ok(())
    }

    /// Set config
    pub fn set_config(&mut self, config: HyprlandConfig) {
        self.config = config;
    }

    /// Get config
    pub fn get_config(&self) -> HyprlandConfig {
        self.config
    }

    /// Get workspace windows
    pub fn get_workspace_windows(&self, workspace_id: u32) -> Vec<&HyprlandWindow> {
        self.windows
            .values()
            .filter(|w| w.workspace == workspace_id)
            .collect()
    }

    /// Is active
    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::Relaxed)
    }
}

impl Default for HyprlandInstance {
    fn default() -> Self {
        Self::new()
    }
}

/// Global Hyprland instance
static mut HYPRLAND_INSTANCE: Option<HyprlandInstance> = None;

/// Initialize Hyprland
pub fn init_hyprland() {
    unsafe {
        HYPRLAND_INSTANCE = Some(HyprlandInstance::new());
    }
}

/// Get global Hyprland instance
pub fn get_hyprland() -> Option<&'static mut HyprlandInstance> {
    unsafe { HYPRLAND_INSTANCE.as_mut() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hyprland_init() {
        let mut hyprland = HyprlandInstance::new();
        hyprland.init();
        assert!(hyprland.is_active());
    }

    #[test]
    fn test_create_window() {
        let mut hyprland = HyprlandInstance::new();
        hyprland.init();
        let id = hyprland.create_window("Test".to_string(), "test_app".to_string());
        assert!(hyprland.get_window(id).is_some());
    }

    #[test]
    fn test_close_window() {
        let mut hyprland = HyprlandInstance::new();
        hyprland.init();
        let id = hyprland.create_window("Test".to_string(), "test_app".to_string());
        hyprland.close_window(id).unwrap();
        assert!(hyprland.get_window(id).is_none());
    }

    #[test]
    fn test_create_workspace() {
        let mut hyprland = HyprlandInstance::new();
        hyprland.init();
        let id = hyprland.create_workspace("2".to_string());
        assert!(hyprland.workspaces.contains_key(&id));
    }

    #[test]
    fn test_switch_workspace() {
        let mut hyprland = HyprlandInstance::new();
        hyprland.init();
        hyprland.create_workspace("2".to_string());
        hyprland.switch_workspace(2).unwrap();
        assert_eq!(hyprland.current_workspace.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn test_toggle_floating() {
        let mut hyprland = HyprlandInstance::new();
        hyprland.init();
        let id = hyprland.create_window("Test".to_string(), "test_app".to_string());
        hyprland.toggle_floating(id).unwrap();
        assert!(hyprland.get_window(id).unwrap().is_floating());
    }

    #[test]
    fn test_toggle_fullscreen() {
        let mut hyprland = HyprlandInstance::new();
        hyprland.init();
        let id = hyprland.create_window("Test".to_string(), "test_app".to_string());
        hyprland.toggle_fullscreen(id).unwrap();
        assert!(hyprland.get_window(id).unwrap().is_fullscreen());
    }
}
