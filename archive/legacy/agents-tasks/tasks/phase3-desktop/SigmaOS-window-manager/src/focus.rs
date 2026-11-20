//! Focus management for SigmaOS-window-manager.

use crate::WindowId;

/// Tracks which window currently has keyboard focus.
#[derive(Debug, Default)]
pub struct FocusTracker {
    focused: Option<WindowId>,
    history: Vec<WindowId>,
}

impl FocusTracker {
    pub fn new() -> Self { Self::default() }

    pub fn set_focus(&mut self, id: WindowId) {
        if let Some(prev) = self.focused {
            if prev != id {
                self.history.push(prev);
                if self.history.len() > 32 {
                    self.history.remove(0);
                }
            }
        }
        self.focused = Some(id);
    }

    pub fn clear_focus(&mut self) {
        self.focused = None;
    }

    pub fn focused(&self) -> Option<WindowId> {
        self.focused
    }

    pub fn prev_focused(&self) -> Option<WindowId> {
        self.history.last().copied()
    }

    pub fn is_focused(&self, id: WindowId) -> bool {
        self.focused == Some(id)
    }
}
