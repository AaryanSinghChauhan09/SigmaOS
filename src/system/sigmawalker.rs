//! SigmaWalker - Compositor Management
//!
//! Inspired by Walker (Omarchy Linux), this module provides Wayland compositor
//! management including window rules, workspace management, and keybindings.

#![no_std]

use alloc::collections::BTreeMap;
use alloc::string::String;
use core::sync::atomic::{AtomicBool, Ordering};

/// Window type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowType {
    Normal,
    Dialog,
    Splash,
    Utility,
    Toolbar,
    Menu,
    Popup,
    Tooltip,
    Notification,
    Unknown,
;

/// Window state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowState {
    Tiled,
    Floating,
    Fullscreen,
    Maximized,
    Minimized,
    Unknown,
;

/// Window data structure
#[derive(Debug, Clone)]
pub struct Window {
    pub window_id: String,
    pub app_id: String,
    pub title: String,
    pub window_type: WindowType,
    pub state: WindowState,
    pub workspace: u32,
    pub is_focused: bool,
;

impl Window {
    /// Create a new window
    pub fn new(window_id: String, app_id: String, title: String, window_type: WindowType) -> Self {
        Self {
            window_id,
            app_id,
            title,
            window_type,
            state: WindowState::Tiled,
            workspace: 1,
            is_focused: false,
        }
    }

    /// Set state
    pub fn set_state(&mut self, state: WindowState) {
        self.state = state;
    }

    /// Set workspace
    pub fn set_workspace(&mut self, workspace: u32) {
        self.workspace = workspace;
    }

    /// Set focused state
    pub fn set_focused(&mut self, focused: bool) {
        self.is_focused = focused;
    }

    /// Check if window is focused
    pub fn is_window_focused(&self) -> bool {
        self.is_focused
    }
;

/// Keybinding data structure
#[derive(Debug, Clone)]
pub struct Keybinding {
    pub binding_id: String,
    pub key: String,
    pub modifiers: String,
    pub command: String,
    pub enabled: bool,
;

impl Keybinding {
    /// Create a new keybinding
    pub fn new(binding_id: String, key: String, modifiers: String, command: String) -> Self {
        Self {
            binding_id,
            key,
            modifiers,
            command,
            enabled: true,
        }
    }

    /// Enable keybinding
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable keybinding
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Check if keybinding is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
;

/// Workspace data structure
#[derive(Debug, Clone)]
pub struct Workspace {
    pub workspace_id: u32,
    pub name: String,
    pub is_active: bool,
;

impl Workspace {
    /// Create a new workspace
    pub fn new(workspace_id: u32, name: String) -> Self {
        Self {
            workspace_id,
            name,
            is_active: false,
        }
    }

    /// Set active state
    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    /// Check if workspace is active
    pub fn is_workspace_active(&self) -> bool {
        self.is_active
    }
;

/// Compositor manager
pub struct SigmaWalker {
    windows: BTreeMap<String, Window>,
    keybindings: BTreeMap<String, Keybinding>,
    workspaces: BTreeMap<u32, Workspace>,
    active_workspace: u32,
    running: AtomicBool,
;

impl SigmaWalker {
    /// Create a new compositor manager
    pub fn new() -> Self {
        Self {
            windows: BTreeMap::new(),
            keybindings: BTreeMap::new(),
            workspaces: BTreeMap::new(),
            active_workspace: 1,
            running: AtomicBool::new(false),
        }
    }

    /// Start the compositor manager
    pub fn start(&mut self) {
        self.running.store(true, Ordering::SeqCst);
    }

    /// Stop the compositor manager
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Add a window
    pub fn add_window(&mut self, window: Window) -> bool {
        let window_id = window.window_id.clone();
        self.windows.insert(window_id, window);
        true
    }

    /// Remove a window
    pub fn remove_window(&mut self, window_id: &str) -> Option<Window> {
        self.windows.remove(window_id)
    }

    /// Get a window by ID
    pub fn get_window(&self, window_id: &str) -> Option<&Window> {
        self.windows.get(window_id)
    }

    /// List all windows
    pub fn list_windows(&self) -> Vec<&Window> {
        self.windows.values().collect()
    }

    /// List windows by workspace
    pub fn list_windows_by_workspace(&self, workspace: u32) -> Vec<&Window> {
        self.windows
            .values()
            .filter(|w| w.workspace == workspace)
            .collect()
    }

    /// Focus a window
    pub fn focus_window(&mut self, window_id: &str) -> bool {
        // Unfocus all windows
        for window in self.windows.values_mut() {
            window.set_focused(false);
        }

        // Focus the target window
        if let Some(window) = self.windows.get_mut(window_id) {
            window.set_focused(true);
            true
        } else {
            false
        }
    }

    /// Set window state
    pub fn set_window_state(&mut self, window_id: &str, state: WindowState) -> bool {
        if let Some(window) = self.windows.get_mut(window_id) {
            window.set_state(state);
            true
        } else {
            false
        }
    }

    /// Move window to workspace
    pub fn move_window_to_workspace(&mut self, window_id: &str, workspace: u32) -> bool {
        if let Some(window) = self.windows.get_mut(window_id) {
            window.set_workspace(workspace);
            true
        } else {
            false
        }
    }

    /// Add a keybinding
    pub fn add_keybinding(&mut self, keybinding: Keybinding) -> bool {
        let binding_id = keybinding.binding_id.clone();
        self.keybindings.insert(binding_id, keybinding);
        true
    }

    /// Remove a keybinding
    pub fn remove_keybinding(&mut self, binding_id: &str) -> Option<Keybinding> {
        self.keybindings.remove(binding_id)
    }

    /// Get a keybinding by ID
    pub fn get_keybinding(&self, binding_id: &str) -> Option<&Keybinding> {
        self.keybindings.get(binding_id)
    }

    /// List all keybindings
    pub fn list_keybindings(&self) -> Vec<&Keybinding> {
        self.keybindings.values().collect()
    }

    /// List enabled keybindings
    pub fn list_enabled_keybindings(&self) -> Vec<&Keybinding> {
        self.keybindings
            .values()
            .filter(|k| k.is_enabled())
            .collect()
    }

    /// Add a workspace
    pub fn add_workspace(&mut self, workspace: Workspace) -> bool {
        let workspace_id = workspace.workspace_id;
        self.workspaces.insert(workspace_id, workspace);
        true
    }

    /// Remove a workspace
    pub fn remove_workspace(&mut self, workspace_id: u32) -> Option<Workspace> {
        self.workspaces.remove(&workspace_id)
    }

    /// Get a workspace by ID
    pub fn get_workspace(&self, workspace_id: u32) -> Option<&Workspace> {
        self.workspaces.get(&workspace_id)
    }

    /// List all workspaces
    pub fn list_workspaces(&self) -> Vec<&Workspace> {
        self.workspaces.values().collect()
    }

    /// Switch to workspace
    pub fn switch_workspace(&mut self, workspace_id: u32) -> bool {
        if self.workspaces.contains_key(&workspace_id) {
            // Deactivate current workspace
            if let Some(ws) = self.workspaces.get_mut(&self.active_workspace) {
                ws.set_active(false);
            }

            // Activate new workspace
            self.active_workspace = workspace_id;
            if let Some(ws) = self.workspaces.get_mut(&workspace_id) {
                ws.set_active(true);
            }

            true
        } else {
            false
        }
    }

    /// Get active workspace
    pub fn get_active_workspace(&self) -> u32 {
        self.active_workspace
    }

    /// Get window count
    pub fn window_count(&self) -> usize {
        self.windows.len()
    }

    /// Get keybinding count
    pub fn keybinding_count(&self) -> usize {
        self.keybindings.len()
    }

    /// Get workspace count
    pub fn workspace_count(&self) -> usize {
        self.workspaces.len()
    }
;

impl Default for SigmaWalker {
    fn default() -> Self {
        Self::new()
    }
;

/// Global SigmaWalker instance (placeholder for actual global state)
static mut SIGMAWALKER_INSTANCE: Option<SigmaWalker> = None;

/// Initialize the global SigmaWalker instance
pub fn init_sigmawalker() {
    unsafe {
        SIGMAWALKER_INSTANCE = Some(SigmaWalker::new());
    }
;

/// Get the global SigmaWalker instance
pub fn get_sigmawalker() -> Option<&'static SigmaWalker> {
    unsafe { SIGMAWALKER_INSTANCE.as_ref() }
;

/// Get mutable global SigmaWalker instance
pub fn get_sigmawalker_mut() -> Option<&'static mut SigmaWalker> {
    unsafe { SIGMAWALKER_INSTANCE.as_mut() }
;

/// Convenience function to add a window
pub fn add_window(window: Window) -> bool {
    if let Some(walker) = get_sigmawalker_mut() {
        walker.add_window(window)
    } else {
        false
    }
;

/// Convenience function to switch workspace
pub fn switch_workspace(workspace_id: u32) -> bool {
    if let Some(walker) = get_sigmawalker_mut() {
        walker.switch_workspace(workspace_id)
    } else {
        false
    }
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_creation() {
        let window = Window::new(
            "win1".to_string(),
            "firefox".to_string(),
            "Firefox".to_string(),
            WindowType::Normal,
        );

        assert_eq!(window.window_id, "win1");
        assert_eq!(window.state, WindowState::Tiled);
        assert_eq!(window.workspace, 1);
    }

    #[test]
    fn test_window_state() {
        let mut window = Window::new(
            "win1".to_string(),
            "firefox".to_string(),
            "Firefox".to_string(),
            WindowType::Normal,
        );

        window.set_state(WindowState::Fullscreen);
        assert_eq!(window.state, WindowState::Fullscreen);

        window.set_focused(true);
        assert!(window.is_window_focused());
    }

    #[test]
    fn test_keybinding_creation() {
        let keybinding = Keybinding::new(
            "kb1".to_string(),
            "Return".to_string(),
            "Super".to_string(),
            "exec terminal".to_string(),
        );

        assert_eq!(keybinding.key, "Return");
        assert!(keybinding.is_enabled());
    }

    #[test]
    fn test_workspace_creation() {
        let workspace = Workspace::new(1, "1".to_string());
        assert_eq!(workspace.workspace_id, 1);
        assert!(!workspace.is_workspace_active());
    }

    #[test]
    fn test_sigmawalker_creation() {
        let walker = SigmaWalker::new();
        assert!(!walker.is_running());
        assert_eq!(walker.window_count(), 0);
        assert_eq!(walker.workspace_count(), 0);
    }

    #[test]
    fn test_sigmawalker_start_stop() {
        let mut walker = SigmaWalker::new();
        walker.start();
        assert!(walker.is_running());
        walker.stop();
        assert!(!walker.is_running());
    }

    #[test]
    fn test_sigmawalker_add_window() {
        let mut walker = SigmaWalker::new();
        let window = Window::new(
            "win1".to_string(),
            "firefox".to_string(),
            "Firefox".to_string(),
            WindowType::Normal,
        );

        let added = walker.add_window(window);
        assert!(added);
        assert_eq!(walker.window_count(), 1);
    }

    #[test]
    fn test_sigmawalker_focus_window() {
        let mut walker = SigmaWalker::new();

        walker.add_window(Window::new(
            "win1".to_string(),
            "firefox".to_string(),
            "Firefox".to_string(),
            WindowType::Normal,
        ));

        let focused = walker.focus_window("win1");
        assert!(focused);

        let window = walker.get_window("win1").unwrap();
        assert!(window.is_window_focused());
    }

    #[test]
    fn test_sigmawalker_add_workspace() {
        let mut walker = SigmaWalker::new();
        let workspace = Workspace::new(1, "1".to_string());

        let added = walker.add_workspace(workspace);
        assert!(added);
        assert_eq!(walker.workspace_count(), 1);
    }

    #[test]
    fn test_sigmawalker_switch_workspace() {
        let mut walker = SigmaWalker::new();

        walker.add_workspace(Workspace::new(1, "1".to_string()));
        walker.add_workspace(Workspace::new(2, "2".to_string()));

        let switched = walker.switch_workspace(2);
        assert!(switched);
        assert_eq!(walker.get_active_workspace(), 2);
    }

    #[test]
    fn test_sigmawalker_list_by_workspace() {
        let mut walker = SigmaWalker::new();

        let mut win1 = Window::new(
            "win1".to_string(),
            "firefox".to_string(),
            "Firefox".to_string(),
            WindowType::Normal,
        );
        win1.set_workspace(1);

        let mut win2 = Window::new(
            "win2".to_string(),
            "terminal".to_string(),
            "Terminal".to_string(),
            WindowType::Normal,
        );
        win2.set_workspace(2);

        walker.add_window(win1);
        walker.add_window(win2);

        let ws1 = walker.list_windows_by_workspace(1);
        assert_eq!(ws1.len(), 1);
    }

    #[test]
    fn test_global_instance() {
        init_sigmawalker();

        let walker = get_sigmawalker();
        assert!(walker.is_some());

        let mut walker_mut = get_sigmawalker_mut();
        assert!(walker_mut.is_some());

        if let Some(walker) = walker_mut {
            walker.start();
            assert!(walker.is_running());
        }
    }

    #[test]
    fn test_convenience_add_window() {
        init_sigmawalker();

        let window = Window::new(
            "win1".to_string(),
            "firefox".to_string(),
            "Firefox".to_string(),
            WindowType::Normal,
        );

        let added = add_window(window);
        assert!(added);

        let walker = get_sigmawalker().unwrap();
        assert_eq!(walker.window_count(), 1);
    }

    #[test]
    fn test_convenience_switch_workspace() {
        init_sigmawalker();

        if let Some(walker) = get_sigmawalker_mut() {
            walker.add_workspace(Workspace::new(1, "1".to_string()));
            walker.add_workspace(Workspace::new(2, "2".to_string()));
        }

        let switched = switch_workspace(2);
        assert!(switched);

        let walker = get_sigmawalker().unwrap();
        assert_eq!(walker.get_active_workspace(), 2);
    }
;
