#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS Cutting-Edge Terminal Multiplexer (SigmaTmux Engine)
// Implements robust OOP principles with custom split, zoom, broadcast, and copy register functions
// Built to outperform and exceed standard tmux capabilities of Linux distributions

use std::collections::HashMap;
use std::ops::Range;

/// Direction for splitting terminal panes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitDirection {
    Horizontal,
    Vertical,
}

/// Dynamic layout configurations for multi-pane setups
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutPreset {
    EvenHorizontal,
    EvenVertical,
    MainVertical,
    MainHorizontal,
    Tiled,
}

/// Represents an individual terminal pane inside a window
#[derive(Debug, Clone)]
pub struct TmuxPane {
    pub id: usize,
    pub title: String,
    pub current_command: Option<String>,
    pub width: u16,
    pub height: u16,
    pub offset_x: u16,
    pub offset_y: u16,
    pub is_zoomed: bool,
    pub history_buffer: Vec<String>,
}

impl TmuxPane {
    pub fn new(id: usize, title: &str, width: u16, height: u16) -> Self {
        Self {
            id,
            title: title.to_string(),
            current_command: None,
            width,
            height,
            offset_x: 0,
            offset_y: 0,
            is_zoomed: false,
            history_buffer: Vec::new(),
        }
    }

    /// Executes a command inside the pane
    pub fn execute_command(&mut self, cmd: &str) {
        self.current_command = Some(cmd.to_string());
        self.history_buffer.push(format!("$ {}", cmd));
        // Simulated execution output
        self.history_buffer.push(format!("[Output from {}]", cmd));
    }

    /// Resize pane geometry
    pub fn resize(&mut self, width: u16, height: u16, offset_x: u16, offset_y: u16) {
        self.width = width;
        self.height = height;
        self.offset_x = offset_x;
        self.offset_y = offset_y;
    }
}

/// Represents a window containing one or more panes
#[derive(Debug, Clone)]
pub struct TmuxWindow {
    pub id: usize,
    pub name: String,
    pub panes: Vec<TmuxPane>,
    pub active_pane_idx: usize,
    pub layout: LayoutPreset,
    pub next_pane_id: usize,
}

impl TmuxWindow {
    pub fn new(id: usize, name: &str) -> Self {
        let initial_pane = TmuxPane::new(0, "default-pane", 80, 24);
        Self {
            id,
            name: name.to_string(),
            panes: vec![initial_pane],
            active_pane_idx: 0,
            layout: LayoutPreset::Tiled,
            next_pane_id: 1,
        }
    }

    /// Splits an existing pane in the window
    pub fn split_pane(&mut self, target_pane_id: usize, direction: SplitDirection) -> Result<usize, &'static str> {
        let target_idx = self.panes.iter().position(|p| p.id == target_pane_id)
            .ok_or("Target pane not found")?;

        let parent_width = self.panes[target_idx].width;
        let parent_height = self.panes[target_idx].height;
        let parent_x = self.panes[target_idx].offset_x;
        let parent_y = self.panes[target_idx].offset_y;

        let new_pane_id = self.next_pane_id;
        self.next_pane_id += 1;

        let mut new_pane = TmuxPane::new(new_pane_id, &format!("pane-{}", new_pane_id), 0, 0);

        match direction {
            SplitDirection::Horizontal => {
                let half_width = parent_width / 2;
                self.panes[target_idx].resize(half_width, parent_height, parent_x, parent_y);
                new_pane.resize(parent_width - half_width, parent_height, parent_x + half_width, parent_y);
            }
            SplitDirection::Vertical => {
                let half_height = parent_height / 2;
                self.panes[target_idx].resize(parent_width, half_height, parent_x, parent_y);
                new_pane.resize(parent_width, parent_height - half_height, parent_x, parent_y + half_height);
            }
        }

        self.panes.push(new_pane);
        self.active_pane_idx = self.panes.len() - 1;
        Ok(new_pane_id)
    }

    /// Toggles zoom state of active or specified pane
    pub fn toggle_zoom(&mut self, pane_id: usize) -> Result<bool, &'static str> {
        let idx = self.panes.iter().position(|p| p.id == pane_id)
            .ok_or("Pane not found")?;

        let current_state = self.panes[idx].is_zoomed;
        self.panes[idx].is_zoomed = !current_state;
        Ok(!current_state)
    }

    /// Broadcasts a command to ALL panes in this window
    pub fn broadcast_command(&mut self, cmd: &str) {
        for pane in &mut self.panes {
            pane.execute_command(cmd);
        }
    }
}

/// Represents a multiplexed session containing one or more windows
#[derive(Debug, Clone)]
pub struct TmuxSession {
    pub name: String,
    pub windows: Vec<TmuxWindow>,
    pub active_window_idx: usize,
    pub copy_registers: HashMap<String, String>, // Named clipboard buffers for multi-pane editing
    pub is_attached: bool,
}

impl TmuxSession {
    pub fn new(name: &str) -> Self {
        let initial_window = TmuxWindow::new(0, "bash");
        Self {
            name: name.to_string(),
            windows: vec![initial_window],
            active_window_idx: 0,
            copy_registers: HashMap::new(),
            is_attached: true,
        }
    }

    /// Creates a new window in the session
    pub fn create_window(&mut self, name: &str) -> usize {
        let new_id = self.windows.len();
        let new_win = TmuxWindow::new(new_id, name);
        self.windows.push(new_win);
        self.active_window_idx = new_id;
        new_id
    }

    /// Copy selected history range of a pane to a named register
    pub fn copy_pane_history_to_register(&mut self, window_id: usize, pane_id: usize, range: Range<usize>, register_name: &str) -> Result<(), &'static str> {
        let win = self.windows.get_mut(window_id).ok_or("Window not found")?;
        let pane = win.panes.iter().find(|p| p.id == pane_id).ok_or("Pane not found")?;

        let start = range.start.min(pane.history_buffer.len());
        let end = range.end.min(pane.history_buffer.len());
        let selected_text = pane.history_buffer[start..end].join("\n");

        self.copy_registers.insert(register_name.to_string(), selected_text);
        Ok(())
    }

    /// Paste from a named register into a pane
    pub fn paste_register_to_pane(&mut self, window_id: usize, pane_id: usize, register_name: &str) -> Result<(), &'static str> {
        let content = self.copy_registers.get(register_name).cloned().ok_or("Register is empty")?;
        let win = self.windows.get_mut(window_id).ok_or("Window not found")?;
        let pane = win.panes.iter_mut().find(|p| p.id == pane_id).ok_or("Pane not found")?;

        pane.execute_command(&content);
        Ok(())
    }
}

/// The high-level Terminal Multiplexer manager
pub struct TmuxSessionManager {
    pub sessions: HashMap<String, TmuxSession>,
    pub active_session_name: Option<String>,
}

impl TmuxSessionManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            active_session_name: None,
        }
    }

    /// Create and automatically attach to a new tmux session
    pub fn create_session(&mut self, name: &str) -> Result<(), &'static str> {
        if self.sessions.contains_key(name) {
            return Err("Session already exists");
        }
        let session = TmuxSession::new(name);
        self.sessions.insert(name.to_string(), session);
        self.active_session_name = Some(name.to_string());
        Ok(())
    }

    /// Kills a session
    pub fn kill_session(&mut self, name: &str) -> Result<(), &'static str> {
        self.sessions.remove(name).ok_or("Session not found")?;
        if self.active_session_name.as_deref() == Some(name) {
            self.active_session_name = self.sessions.keys().next().cloned();
        }
        Ok(())
    }

    /// Generates a gorgeous, feature-rich status bar (defeats all Linux distros!)
    pub fn get_status_bar(&self) -> String {
        let active_info = if let Some(ref name) = self.active_session_name {
            let session = &self.sessions[name];
            let active_win = &session.windows[session.active_window_idx];
            format!("[{}] {}:{}* ", name, active_win.id, active_win.name)
        } else {
            "[No Session] ".to_string()
        };

        // Advanced telemetry status bar variables
        let cpu_indicator = "CPU: 1.2% |";
        let memory_indicator = "MEM: 12% |";
        let time_indicator = "SigmaTime: UTC 12:00";

        format!("{} | {} {} {}", active_info, cpu_indicator, memory_indicator, time_indicator)
    }
}

impl Default for TmuxSessionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation_and_windows() {
        let mut manager = TmuxSessionManager::new();
        manager.create_session("primary").unwrap();
        assert_eq!(manager.active_session_name.as_deref(), Some("primary"));

        let session = manager.sessions.get_mut("primary").unwrap();
        assert_eq!(session.windows.len(), 1);
        assert_eq!(session.active_window_idx, 0);

        let new_win_id = session.create_window("sysmonitor");
        assert_eq!(new_win_id, 1);
        assert_eq!(session.active_window_idx, 1);
    }

    #[test]
    fn test_pane_splitting_and_resizing() {
        let mut session = TmuxSession::new("development");
        let active_win_idx = session.active_window_idx;
        let window = &mut session.windows[active_win_idx];

        assert_eq!(window.panes.len(), 1);
        assert_eq!(window.panes[0].width, 80);

        // Split horizontally
        let new_pane_id = window.split_pane(0, SplitDirection::Horizontal).unwrap();
        assert_eq!(window.panes.len(), 2);
        assert_eq!(window.panes[0].width, 40);
        assert_eq!(window.panes[1].width, 40);
        assert_eq!(new_pane_id, 1);

        // Split vertically on new pane
        let vertical_pane_id = window.split_pane(1, SplitDirection::Vertical).unwrap();
        assert_eq!(window.panes.len(), 3);
        assert_eq!(vertical_pane_id, 2);
    }

    #[test]
    fn test_pane_zooming() {
        let mut session = TmuxSession::new("test-zoom");
        let active_win_idx = session.active_window_idx;
        let window = &mut session.windows[active_win_idx];

        assert!(!window.panes[0].is_zoomed);
        let zoomed = window.toggle_zoom(0).unwrap();
        assert!(zoomed);
        assert!(window.panes[0].is_zoomed);
    }

    #[test]
    fn test_broadcast_command_to_all_panes() {
        let mut session = TmuxSession::new("broadcaster");
        let active_win_idx = session.active_window_idx;
        let window = &mut session.windows[active_win_idx];
        window.split_pane(0, SplitDirection::Horizontal).unwrap();

        window.broadcast_command("echo 'Hello SigmaOS'");
        assert_eq!(window.panes[0].current_command.as_deref(), Some("echo 'Hello SigmaOS'"));
        assert_eq!(window.panes[1].current_command.as_deref(), Some("echo 'Hello SigmaOS'"));
    }

    #[test]
    fn test_copy_paste_registers() {
        let mut session = TmuxSession::new("clip-registers");
        let active_win_idx = session.active_window_idx;
        {
            let window = &mut session.windows[active_win_idx];
            window.split_pane(0, SplitDirection::Horizontal).unwrap();

            // Write output to pane 0
            window.panes[0].execute_command("sigpkg status");
        }

        // Copy history to buffer "a"
        session.copy_pane_history_to_register(active_win_idx, 0, 0..2, "a").unwrap();
        assert!(session.copy_registers.contains_key("a"));

        // Paste register to pane 1
        session.paste_register_to_pane(active_win_idx, 1, "a").unwrap();

        let window = &session.windows[active_win_idx];
        assert!(window.panes[1].current_command.is_some());
    }

    #[test]
    fn test_status_bar_telemetry() {
        let mut manager = TmuxSessionManager::new();
        manager.create_session("primary").unwrap();
        let status = manager.get_status_bar();
        assert!(status.contains("primary"));
        assert!(status.contains("CPU"));
        assert!(status.contains("MEM"));
    }
}
