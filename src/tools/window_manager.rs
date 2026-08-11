//! Window Manager (i3/sway Inspiration)
//! Tiling window manager with workspaces and containers

#![no_std]

extern crate alloc;

use crate::klib::{Vec, String};

/// Window
#[derive(Debug, Clone)]
pub struct Window {
    pub id: String,
    pub title: String,
    pub pid: u32,
    pub workspace: u32,
    pub floating: bool,
    pub fullscreen: bool,
}

impl Window {
    pub fn new(id: &str, title: &str, pid: u32) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            pid,
            workspace: 1,
            floating: false,
            fullscreen: false,
        }
    }

    pub fn set_workspace(&mut self, workspace: u32) {
        self.workspace = workspace;
    }

    pub fn set_floating(&mut self, floating: bool) {
        self.floating = floating;
    }

    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        self.fullscreen = fullscreen;
    }
}

/// Container
#[derive(Debug, Clone)]
pub struct Container {
    pub id: String,
    pub layout: LayoutType,
    pub windows: Vec<String>,
    pub children: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutType {
    SplitH,
    SplitV,
    Stacked,
    Tabbed,
}

impl Container {
    pub fn new(id: &str, layout: LayoutType) -> Self {
        Self {
            id: id.to_string(),
            layout,
            windows: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn add_window(&mut self, window_id: &str) {
        self.windows.push(window_id.to_string());
    }

    pub fn add_child(&mut self, container_id: &str) {
        self.children.push(container_id.to_string());
    }
}

/// Workspace
#[derive(Debug, Clone)]
pub struct Workspace {
    pub number: u32,
    pub name: String,
    pub windows: Vec<String>,
    pub output: String,
}

impl Workspace {
    pub fn new(number: u32, name: &str) -> Self {
        Self {
            number,
            name: name.to_string(),
            windows: Vec::new(),
            output: "HDMI-1".to_string(),
        }
    }

    pub fn add_window(&mut self, window_id: &str) {
        self.windows.push(window_id.to_string());
    }

    pub fn remove_window(&mut self, window_id: &str) {
        self.windows.retain(|w| w != window_id);
    }
}

/// Key binding
#[derive(Debug, Clone)]
pub struct KeyBinding {
    pub key: String,
    pub modifiers: Vec<String>,
    pub command: String,
}

impl KeyBinding {
    pub fn new(key: &str, command: &str) -> Self {
        Self {
            key: key.to_string(),
            modifiers: Vec::new(),
            command: command.to_string(),
        }
    }

    pub fn add_modifier(&mut self, modifier: &str) {
        self.modifiers.push(modifier.to_string());
    }
}

/// Tiling window manager
pub struct TilingWindowManager {
    pub workspaces: Vec<Workspace>,
    pub containers: Vec<Container>,
    pub windows: Vec<Window>,
    pub bindings: Vec<KeyBinding>,
    pub focus: Option<String>,
}

impl TilingWindowManager {
    pub fn new() -> Self {
        Self {
            workspaces: Vec::new(),
            containers: Vec::new(),
            windows: Vec::new(),
            bindings: Vec::new(),
            focus: None,
        }
    }

    pub fn add_workspace(&mut self, workspace: Workspace) {
        self.workspaces.push(workspace);
    }

    pub fn add_container(&mut self, container: Container) {
        self.containers.push(container);
    }

    pub fn add_window(&mut self, window: Window) {
        self.windows.push(window);
    }

    pub fn add_binding(&mut self, binding: KeyBinding) {
        self.bindings.push(binding);
    }

    pub fn get_window(&mut self, id: &str) -> Option<&mut Window> {
        self.windows.iter_mut().find(|w| w.id == id)
    }

    pub fn focus_window(&mut self, id: &str) {
        self.focus = Some(id.to_string());
    }

    pub fn switch_workspace(&mut self, number: u32) {
        if let Some(workspace) = self.workspaces.iter_mut().find(|w| w.number == number) {
            // Switch to workspace
        }
    }

    pub fn move_window_to_workspace(&mut self, window_id: &str, workspace_num: u32) -> Result<(), WMError> {
        if let Some(window) = self.get_window(window_id) {
            window.set_workspace(workspace_num);
            Ok(())
        } else {
            Err(WMError::WindowNotFound)
        }
    }

    pub fn split_container(&mut self, container_id: &str, layout: LayoutType) -> Result<(), WMError> {
        if let Some(container) = self.containers.iter_mut().find(|c| c.id == container_id) {
            container.layout = layout;
            Ok(())
        } else {
            Err(WMError::ContainerNotFound)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WMError {
    WindowNotFound,
    ContainerNotFound,
    WorkspaceNotFound,
}

impl Default for TilingWindowManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window() {
        let window = Window::new("1", "Terminal", 1234);
        assert_eq!(window.title, "Terminal");
    }

    #[test]
    fn test_workspace() {
        let workspace = Workspace::new(1, "1");
        assert_eq!(workspace.number, 1);
    }

    #[test]
    fn test_tiling_window_manager() {
        let mut wm = TilingWindowManager::new();
        let workspace = Workspace::new(1, "1");
        wm.add_workspace(workspace);
        assert_eq!(wm.workspaces.len(), 1);
    }
}