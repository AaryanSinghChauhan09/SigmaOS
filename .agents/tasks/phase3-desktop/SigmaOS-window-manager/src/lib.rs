//! SigmaOS Window Manager — tiling and stacking layout engine.
//!
//! Upstream inspiration:
//! - Hyprland: <https://github.com/hyprwm/Hyprland> (dwindle/master-stack layouts)
//! - sway:     <https://github.com/swaywm/sway>
//! - i3:       <https://github.com/i3/i3>

pub mod layout;
pub use layout::{LayoutConfig, LayoutKind};

use std::fmt;

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

/// Errors produced by window-manager operations.
#[derive(Debug)]
pub enum WmError {
    /// The requested workspace index is out of range.
    WorkspaceNotFound(u32),
    /// A window ID was not found in the current layout.
    WindowNotFound(u64),
}

impl fmt::Display for WmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WmError::WorkspaceNotFound(n) => write!(f, "workspace {n} not found"),
            WmError::WindowNotFound(id) => write!(f, "window {id} not found"),
        }
    }
}

impl std::error::Error for WmError {}

// ---------------------------------------------------------------------------
// Layout mode selector
// ---------------------------------------------------------------------------

/// Requested window-manager mode at construction time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WmMode {
    Tiling,
    Stacking,
}

// ---------------------------------------------------------------------------
// Tiling layout
// ---------------------------------------------------------------------------

/// A simple horizontal-split tiling layout.
#[derive(Debug, Default)]
pub struct TilingLayout {
    /// Ordered list of window IDs in the current tile strip.
    windows: Vec<u64>,
}

impl TilingLayout {
    /// Add a window to the layout.
    pub fn add_window(&mut self, id: u64) {
        self.windows.push(id);
    }

    /// Remove a window from the layout.
    pub fn remove_window(&mut self, id: u64) -> Result<(), WmError> {
        let len_before = self.windows.len();
        self.windows.retain(|&w| w != id);
        if self.windows.len() == len_before {
            Err(WmError::WindowNotFound(id))
        } else {
            Ok(())
        }
    }

    /// Returns an ordered slice of tiled window IDs.
    pub fn windows(&self) -> &[u64] {
        &self.windows
    }
}

// ---------------------------------------------------------------------------
// Stacking layout
// ---------------------------------------------------------------------------

/// A z-order stacking layout (last = topmost).
#[derive(Debug, Default)]
pub struct StackingLayout {
    stack: Vec<u64>,
}

impl StackingLayout {
    /// Push a window onto the top of the stack.
    pub fn push(&mut self, id: u64) {
        self.stack.retain(|&w| w != id); // deduplicate
        self.stack.push(id);
    }

    /// Pop the topmost window.
    pub fn pop(&mut self) -> Option<u64> {
        self.stack.pop()
    }

    /// Returns the z-ordered stack (bottom to top).
    pub fn stack(&self) -> &[u64] {
        &self.stack
    }
}

// ---------------------------------------------------------------------------
// Window manager enum
// ---------------------------------------------------------------------------

/// Top-level window manager — either tiling or stacking mode.
#[derive(Debug)]
pub enum WindowManager {
    Tiling(TilingLayout),
    Stacking(StackingLayout),
}

impl WindowManager {
    /// Add a window to whichever layout is active.
    pub fn add_window(&mut self, id: u64) {
        match self {
            WindowManager::Tiling(l) => l.add_window(id),
            WindowManager::Stacking(l) => l.push(id),
        }
    }

    /// Returns the number of managed windows.
    pub fn window_count(&self) -> usize {
        match self {
            WindowManager::Tiling(l) => l.windows().len(),
            WindowManager::Stacking(l) => l.stack().len(),
        }
    }
}

// ---------------------------------------------------------------------------
// Workspace manager
// ---------------------------------------------------------------------------

/// Manages multiple virtual workspaces, each holding a [`WindowManager`].
pub struct WorkspaceManager {
    workspaces: Vec<WindowManager>,
    active: usize,
}

impl WorkspaceManager {
    /// Create a workspace manager with `count` tiling workspaces.
    pub fn new_tiling(count: usize) -> Self {
        let workspaces = (0..count)
            .map(|_| WindowManager::Tiling(TilingLayout::default()))
            .collect();
        Self {
            workspaces,
            active: 0,
        }
    }

    /// Switch to workspace `index`.
    pub fn switch_to(&mut self, index: usize) -> Result<(), WmError> {
        if index >= self.workspaces.len() {
            return Err(WmError::WorkspaceNotFound(index as u32));
        }
        self.active = index;
        Ok(())
    }

    /// Returns a reference to the currently active workspace.
    pub fn active_workspace(&self) -> &WindowManager {
        &self.workspaces[self.active]
    }

    /// Returns a mutable reference to the currently active workspace.
    pub fn active_workspace_mut(&mut self) -> &mut WindowManager {
        &mut self.workspaces[self.active]
    }

    /// Returns the index of the active workspace.
    pub fn active_index(&self) -> usize {
        self.active
    }
}

// ---------------------------------------------------------------------------
// Public entry-point
// ---------------------------------------------------------------------------

/// Construct a [`WindowManager`] in the requested mode.
pub fn create_window_manager(mode: WmMode) -> WindowManager {
    match mode {
        WmMode::Tiling => WindowManager::Tiling(TilingLayout::default()),
        WmMode::Stacking => WindowManager::Stacking(StackingLayout::default()),
    }
}

/// Construct a Hyprland-style layout by kind (dwindle or master-stack).
pub fn create_hyprland_layout(kind: layout::LayoutKind) -> layout::Layout {
    layout::layout_of(kind)
}

// ---------------------------------------------------------------------------
// Inline tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tiling_add_remove() {
        let mut wm = create_window_manager(WmMode::Tiling);
        wm.add_window(1);
        wm.add_window(2);
        assert_eq!(wm.window_count(), 2);
    }

    #[test]
    fn stacking_push_pop() {
        let mut layout = StackingLayout::default();
        layout.push(10);
        layout.push(20);
        assert_eq!(layout.pop(), Some(20));
        assert_eq!(layout.stack().len(), 1);
    }
}
