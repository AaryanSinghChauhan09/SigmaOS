//! Workspace management for SigmaOS-window-manager.

use std::collections::HashMap;
use crate::{Rect, Result, WindowId, WmError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LayoutMode {
    Tiling,
    Stacking,
    Fullscreen,
}

impl Default for LayoutMode {
    fn default() -> Self { LayoutMode::Tiling }
}

/// A window entry within a workspace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowEntry {
    pub id: WindowId,
    pub rect: Rect,
    pub floating: bool,
    pub fullscreen: bool,
    pub title: String,
    pub app_id: String,
}

/// A named workspace that holds windows.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub name: String,
    pub layout: LayoutMode,
    pub windows: Vec<WindowEntry>,
    pub focused_index: Option<usize>,
    pub gap: u32,
}

impl Workspace {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            layout: LayoutMode::Tiling,
            windows: Vec::new(),
            focused_index: None,
            gap: 8,
        }
    }

    pub fn add_window(&mut self, id: WindowId, title: impl Into<String>, app_id: impl Into<String>) {
        self.windows.push(WindowEntry {
            id,
            rect: Rect { x: 0, y: 0, width: 800, height: 600 },
            floating: false,
            fullscreen: false,
            title: title.into(),
            app_id: app_id.into(),
        });
        self.focused_index = Some(self.windows.len() - 1);
        self.retile();
    }

    pub fn remove_window(&mut self, id: WindowId) -> Result<()> {
        let pos = self.windows.iter().position(|w| w.id == id)
            .ok_or(WmError::WindowNotFound(id.0))?;
        self.windows.remove(pos);
        self.focused_index = if self.windows.is_empty() {
            None
        } else {
            Some(pos.saturating_sub(1))
        };
        self.retile();
        Ok(())
    }

    pub fn toggle_float(&mut self, id: WindowId) -> Result<()> {
        let win = self.windows.iter_mut().find(|w| w.id == id)
            .ok_or(WmError::WindowNotFound(id.0))?;
        win.floating = !win.floating;
        self.retile();
        Ok(())
    }

    pub fn toggle_fullscreen(&mut self, id: WindowId) -> Result<()> {
        let win = self.windows.iter_mut().find(|w| w.id == id)
            .ok_or(WmError::WindowNotFound(id.0))?;
        win.fullscreen = !win.fullscreen;
        Ok(())
    }

    /// Lay out tiled windows horizontally (simple algorithm).
    pub fn retile(&mut self) {
        let tiled: Vec<usize> = self.windows.iter().enumerate()
            .filter(|(_, w)| !w.floating)
            .map(|(i, _)| i)
            .collect();
        let count = tiled.len();
        if count == 0 { return; }
        let total_width = 1920u32;
        let total_height = 1080u32;
        let width = (total_width - self.gap * (count as u32 + 1)) / count as u32;
        for (slot, idx) in tiled.into_iter().enumerate() {
            self.windows[idx].rect = Rect {
                x: (self.gap + slot as u32 * (width + self.gap)) as i32,
                y: self.gap as i32,
                width,
                height: total_height - 2 * self.gap,
            };
        }
    }
}

/// Manages all workspaces and routing.
#[derive(Debug, Default)]
pub struct WorkspaceManager {
    workspaces: HashMap<String, Workspace>,
    active: Option<String>,
}

impl WorkspaceManager {
    pub fn new() -> Self { Self::default() }

    pub fn create_workspace(&mut self, name: &str) -> &str {
        self.workspaces.entry(name.to_string()).or_insert_with(|| Workspace::new(name));
        if self.active.is_none() {
            self.active = Some(name.to_string());
        }
        name
    }

    pub fn focus_workspace(&mut self, name: &str) -> Result<()> {
        if !self.workspaces.contains_key(name) {
            return Err(WmError::WorkspaceNotFound(name.to_string()));
        }
        self.active = Some(name.to_string());
        Ok(())
    }

    pub fn active_workspace(&self) -> Option<&Workspace> {
        self.active.as_ref().and_then(|n| self.workspaces.get(n))
    }

    pub fn active_workspace_mut(&mut self) -> Option<&mut Workspace> {
        self.active.as_ref().map(|n| n.clone())
            .and_then(move |n| self.workspaces.get_mut(&n))
    }

    pub fn workspace(&self, name: &str) -> Option<&Workspace> {
        self.workspaces.get(name)
    }

    pub fn workspace_mut(&mut self, name: &str) -> Option<&mut Workspace> {
        self.workspaces.get_mut(name)
    }

    pub fn workspace_count(&self) -> usize {
        self.workspaces.len()
    }

    /// Move a window from the active workspace to another.
    pub fn move_window_to(&mut self, id: WindowId, target: &str) -> Result<()> {
        let (entry, source) = {
            let active_name = self.active.clone().ok_or(WmError::WorkspaceNotFound("none".into()))?;
            let src = self.workspaces.get_mut(&active_name)
                .ok_or_else(|| WmError::WorkspaceNotFound(active_name.clone()))?;
            let pos = src.windows.iter().position(|w| w.id == id)
                .ok_or(WmError::WindowNotFound(id.0))?;
            let entry = src.windows.remove(pos);
            src.retile();
            (entry, active_name)
        };
        let dst = self.workspaces.get_mut(target)
            .ok_or_else(|| WmError::WorkspaceNotFound(target.to_string()))?;
        dst.windows.push(entry);
        dst.retile();
        let _ = source;
        Ok(())
    }

    /// Serialize all workspaces to JSON.
    pub fn serialize(&self) -> serde_json::Value {
        let names: Vec<&str> = self.workspaces.keys().map(|s| s.as_str()).collect();
        serde_json::json!({ "workspaces": names, "active": self.active })
    }
}
