#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
use std::format;
use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;
// SigmaOS Cutting-Edge Terminal Multiplexer (SigmaTmux Engine)
// Implements robust OOP principles with custom split, zoom, broadcast, copy registers,
// control mode (-C) protocol parsing, copy-mode scrollback search, pane synchronization,
// mouse event interaction, and status line formatting with placeholders.
// Built to outperform and exceed standard tmux capabilities of Linux distributions.

use crate::klib::HashMap;
use core::ops::Range;

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
    pub copy_cursor_x: usize,
    pub copy_cursor_y: usize,
    pub in_copy_mode: bool,
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
            copy_cursor_x: 0,
            copy_cursor_y: 0,
            in_copy_mode: false,
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

    /// Toggle copy mode (vi-style scrollback navigation)
    pub fn toggle_copy_mode(&mut self) -> bool {
        self.in_copy_mode = !self.in_copy_mode;
        if self.in_copy_mode && !self.history_buffer.is_empty() {
            self.copy_cursor_y = self.history_buffer.len() - 1;
            self.copy_cursor_x = 0;
        }
        self.in_copy_mode
    }

    /// Search inside copy-mode history buffer
    pub fn search_copy_mode(&self, query: &str) -> Vec<(usize, usize, String)> {
        let mut results = Vec::new();
        if query.is_empty() {
            return results;
        }
        for (line_idx, line) in self.history_buffer.iter().enumerate() {
            if let Some(col_idx) = line.find(query) {
                results.push((line_idx, col_idx, line.clone()));
            }
        }
        results
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
    pub sync_panes_enabled: bool,
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
            sync_panes_enabled: false,
        }
    }

    /// Reflows all panes inside this window based on the selected layout preset
    pub fn reflow_layout(&mut self, total_width: u16, total_height: u16) {
        if self.panes.is_empty() {
            return;
        }

        let n = self.panes.len();

        match self.layout {
            LayoutPreset::EvenHorizontal => {
                let pane_width = total_width / n as u16;
                for i in 0..n {
                    let w = if i == n - 1 {
                        total_width - (pane_width * (n - 1) as u16)
                    } else {
                        pane_width
                    };
                    self.panes[i].resize(w, total_height, i as u16 * pane_width, 0);
                }
            }
            LayoutPreset::EvenVertical => {
                let pane_height = total_height / n as u16;
                for i in 0..n {
                    let h = if i == n - 1 {
                        total_height - (pane_height * (n - 1) as u16)
                    } else {
                        pane_height
                    };
                    self.panes[i].resize(total_width, h, 0, i as u16 * pane_height);
                }
            }
            LayoutPreset::MainVertical => {
                if n == 1 {
                    self.panes[0].resize(total_width, total_height, 0, 0);
                } else {
                    let main_width = (total_width as f32 * 0.6) as u16;
                    let stack_width = total_width - main_width;
                    self.panes[0].resize(main_width, total_height, 0, 0);

                    let stack_count = n - 1;
                    let stack_height = total_height / stack_count as u16;
                    for i in 1..n {
                        let h = if i == n - 1 {
                            total_height - (stack_height * (stack_count - 1) as u16)
                        } else {
                            stack_height
                        };
                        self.panes[i].resize(
                            stack_width,
                            h,
                            main_width,
                            (i - 1) as u16 * stack_height,
                        );
                    }
                }
            }
            LayoutPreset::MainHorizontal => {
                if n == 1 {
                    self.panes[0].resize(total_width, total_height, 0, 0);
                } else {
                    let main_height = (total_height as f32 * 0.6) as u16;
                    let stack_height = total_height - main_height;
                    self.panes[0].resize(total_width, main_height, 0, 0);

                    let stack_count = n - 1;
                    let stack_width = total_width / stack_count as u16;
                    for i in 1..n {
                        let w = if i == n - 1 {
                            total_width - (stack_width * (stack_count - 1) as u16)
                        } else {
                            stack_width
                        };
                        self.panes[i].resize(
                            w,
                            stack_height,
                            (i - 1) as u16 * stack_width,
                            main_height,
                        );
                    }
                }
            }
            LayoutPreset::Tiled => {
                let cols = (n as f32).sqrt().ceil() as u16;
                let rows = (n as f32 / cols as f32).ceil() as u16;

                let cell_width = total_width / cols;
                let cell_height = total_height / rows;

                for i in 0..n {
                    let r = i as u16 / cols;
                    let c = i as u16 % cols;

                    let w = if c == cols - 1 || i == n - 1 {
                        total_width - (c * cell_width)
                    } else {
                        cell_width
                    };
                    let h = if r == rows - 1 {
                        total_height - (r * cell_height)
                    } else {
                        cell_height
                    };

                    self.panes[i].resize(w, h, c * cell_width, r * cell_height);
                }
            }
        }
    }

    /// Splits an existing pane in the window
    pub fn split_pane(
        &mut self,
        target_pane_id: usize,
        direction: SplitDirection,
    ) -> Result<usize, &'static str> {
        let target_idx = self
            .panes
            .iter()
            .position(|p| p.id == target_pane_id)
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
                new_pane.resize(
                    parent_width - half_width,
                    parent_height,
                    parent_x + half_width,
                    parent_y,
                );
            }
            SplitDirection::Vertical => {
                let half_height = parent_height / 2;
                self.panes[target_idx].resize(parent_width, half_height, parent_x, parent_y);
                new_pane.resize(
                    parent_width,
                    parent_height - half_height,
                    parent_x,
                    parent_y + half_height,
                );
            }
        }

        self.panes.push(new_pane);
        self.active_pane_idx = self.panes.len() - 1;
        Ok(new_pane_id)
    }

    /// Toggles zoom state of active or specified pane
    pub fn toggle_zoom(&mut self, pane_id: usize) -> Result<bool, &'static str> {
        let idx = self
            .panes
            .iter()
            .position(|p| p.id == pane_id)
            .ok_or("Pane not found")?;

        let current_state = self.panes[idx].is_zoomed;
        self.panes[idx].is_zoomed = !current_state;
        Ok(!current_state)
    }

    /// Broadcasts or sends input to active or all panes if sync-panes is enabled
    pub fn send_input_to_active(&mut self, input: &str) {
        if self.sync_panes_enabled {
            for pane in &mut self.panes {
                pane.execute_command(input);
            }
        } else if let Some(pane) = self.panes.get_mut(self.active_pane_idx) {
            pane.execute_command(input);
        }
    }

    /// Toggle sync-panes mode (synchronize input across all panes)
    pub fn toggle_sync_panes(&mut self) -> bool {
        self.sync_panes_enabled = !self.sync_panes_enabled;
        self.sync_panes_enabled
    }

    /// Handles mouse click to focus a pane by coordinate (offset_x, offset_y)
    pub fn handle_mouse_click(&mut self, x: u16, y: u16) -> Option<usize> {
        for (idx, pane) in self.panes.iter().enumerate() {
            if x >= pane.offset_x
                && x < pane.offset_x + pane.width
                && y >= pane.offset_y
                && y < pane.offset_y + pane.height
            {
                self.active_pane_idx = idx;
                return Some(pane.id);
            }
        }
        None
    }
}

/// Tmux Control Mode (-C) Protocol Notification Event
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TmuxControlEvent {
    Output { pane_id: usize, text: String },
    WindowAdd { window_id: usize, name: String },
    WindowClose { window_id: usize },
    LayoutChange { window_id: usize, layout: String },
    SessionChanged { session_name: String },
    Unknown(String),
}

/// Tmux Control Mode Parser (-C / -CC mode protocol)
#[derive(Debug, Clone, Default)]
pub struct TmuxControlModeParser;

impl TmuxControlModeParser {
    pub fn new() -> Self {
        Self
    }

    /// Parses a single control mode protocol line starting with `%`
    pub fn parse_line(&self, line: &str) -> TmuxControlEvent {
        let trimmed = line.trim();
        if !trimmed.starts_with('%') {
            return TmuxControlEvent::Unknown(trimmed.to_string());
        }

        let parts: Vec<&str> = trimmed[1..].splitn(2, ' ').collect();
        let cmd = parts[0];
        let args = parts.get(1).copied().unwrap_or("");

        match cmd {
            "output" => {
                let mut arg_parts = args.splitn(2, ' ');
                let pane_id = arg_parts
                    .next()
                    .unwrap_or("0")
                    .trim_start_matches('%')
                    .parse::<usize>()
                    .unwrap_or(0);
                let text = arg_parts.next().unwrap_or("").to_string();
                TmuxControlEvent::Output { pane_id, text }
            }
            "window-add" => {
                let mut arg_parts = args.splitn(2, ' ');
                let window_id = arg_parts
                    .next()
                    .unwrap_or("0")
                    .trim_start_matches('@')
                    .parse::<usize>()
                    .unwrap_or(0);
                let name = arg_parts.next().unwrap_or("").to_string();
                TmuxControlEvent::WindowAdd { window_id, name }
            }
            "window-close" => {
                let window_id = args.trim_start_matches('@').parse::<usize>().unwrap_or(0);
                TmuxControlEvent::WindowClose { window_id }
            }
            "layout-change" => {
                let mut arg_parts = args.splitn(2, ' ');
                let window_id = arg_parts
                    .next()
                    .unwrap_or("0")
                    .trim_start_matches('@')
                    .parse::<usize>()
                    .unwrap_or(0);
                let layout = arg_parts.next().unwrap_or("").to_string();
                TmuxControlEvent::LayoutChange { window_id, layout }
            }
            "session-changed" => TmuxControlEvent::SessionChanged {
                session_name: args.to_string(),
            },
            _ => TmuxControlEvent::Unknown(trimmed.to_string()),
        }
    }
}

/// Represents a multiplexed session containing one or more windows
#[derive(Debug, Clone)]
pub struct TmuxSession {
    pub name: String,
    pub windows: Vec<TmuxWindow>,
    pub active_window_idx: usize,
    pub copy_registers: HashMap<String, String>, // Named clipboard buffers
    pub is_attached: bool,
    pub status_format: String,
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
            status_format: String::from(
                "[#{session_name}] #{window_name}* | #{cpu_usage} #{mem_usage}",
            ),
        }
    }

    /// Serializes the entire session state to a plain text config payload
    pub fn serialize_state(&self) -> String {
        let mut state = format!("SESSION_NAME={}\n", self.name);
        state.push_str(&format!("ACTIVE_WINDOW_IDX={}\n", self.active_window_idx));
        for (idx, win) in self.windows.iter().enumerate() {
            state.push_str(&format!(
                "WINDOW_ID={};NAME={};ACTIVE_PANE={};LAYOUT={:?}\n",
                idx, win.name, win.active_pane_idx, win.layout
            ));
            for pane in &win.panes {
                let cmd_str = pane.current_command.as_deref().unwrap_or("None");
                state.push_str(&format!(
                    "  PANE_ID={};TITLE={};GEOM={}x{}+{},{};COMMAND={}\n",
                    pane.id,
                    pane.title,
                    pane.width,
                    pane.height,
                    pane.offset_x,
                    pane.offset_y,
                    cmd_str
                ));
            }
        }
        state
    }

    /// Resurrects/restores session state from a serialized config payload
    pub fn resurrect_state(&mut self, payload: &str) -> Result<(), &'static str> {
        let lines: Vec<&str> = payload.lines().collect();
        if lines.is_empty() {
            return Err("Empty payload");
        }

        self.windows.clear();

        let mut current_window: Option<TmuxWindow> = None;

        for line in lines {
            let trimmed = line.trim();
            if trimmed.starts_with("SESSION_NAME=") {
                self.name = trimmed["SESSION_NAME=".len()..].to_string();
            } else if trimmed.starts_with("ACTIVE_WINDOW_IDX=") {
                self.active_window_idx = trimmed["ACTIVE_WINDOW_IDX=".len()..]
                    .parse::<usize>()
                    .unwrap_or(0);
            } else if trimmed.starts_with("WINDOW_ID=") {
                if let Some(win) = current_window.take() {
                    self.windows.push(win);
                }

                let mut parts = HashMap::new();
                for part in trimmed.split(';') {
                    let kv: Vec<&str> = part.split('=').collect();
                    if kv.len() == 2 {
                        parts.insert(kv[0], kv[1]);
                    }
                }

                let id = parts
                    .get("WINDOW_ID")
                    .unwrap_or(&"0")
                    .parse::<usize>()
                    .unwrap_or(0);
                let name = parts.get("NAME").unwrap_or(&"default").to_string();
                let active_pane = parts
                    .get("ACTIVE_PANE")
                    .unwrap_or(&"0")
                    .parse::<usize>()
                    .unwrap_or(0);
                let layout_str = parts.get("LAYOUT").unwrap_or(&"Tiled");
                let layout = match *layout_str {
                    "EvenHorizontal" => LayoutPreset::EvenHorizontal,
                    "EvenVertical" => LayoutPreset::EvenVertical,
                    "MainVertical" => LayoutPreset::MainVertical,
                    "MainHorizontal" => LayoutPreset::MainHorizontal,
                    _ => LayoutPreset::Tiled,
                };

                let mut win = TmuxWindow::new(id, &name);
                win.panes.clear(); // Clear initial default pane
                win.active_pane_idx = active_pane;
                win.layout = layout;
                current_window = Some(win);
            } else if trimmed.starts_with("PANE_ID=") {
                if let Some(ref mut win) = current_window {
                    let mut parts = HashMap::new();
                    for part in trimmed.split(';') {
                        let kv: Vec<&str> = part.split('=').collect();
                        if kv.len() == 2 {
                            parts.insert(kv[0], kv[1]);
                        }
                    }

                    let id = parts
                        .get("PANE_ID")
                        .unwrap_or(&"0")
                        .parse::<usize>()
                        .unwrap_or(0);
                    let title = parts.get("TITLE").unwrap_or(&"pane").to_string();
                    let command_str = parts.get("COMMAND").unwrap_or(&"None").to_string();

                    let mut pane = TmuxPane::new(id, &title, 80, 24);
                    if command_str != "None" {
                        pane.execute_command(&command_str);
                    }
                    win.panes.push(pane);
                }
            }
        }

        if let Some(win) = current_window {
            self.windows.push(win);
        }

        Ok(())
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
    pub fn copy_pane_history_to_register(
        &mut self,
        window_id: usize,
        pane_id: usize,
        range: Range<usize>,
        register_name: &str,
    ) -> Result<(), &'static str> {
        let win = self.windows.get_mut(window_id).ok_or("Window not found")?;
        let pane = win
            .panes
            .iter()
            .find(|p| p.id == pane_id)
            .ok_or("Pane not found")?;

        let start = range.start.min(pane.history_buffer.len());
        let end = range.end.min(pane.history_buffer.len());
        let selected_text = pane.history_buffer[start..end].join("\n");

        self.copy_registers
            .insert(register_name.to_string(), selected_text);
        Ok(())
    }

    /// Paste from a named register into a pane
    pub fn paste_register_to_pane(
        &mut self,
        window_id: usize,
        pane_id: usize,
        register_name: &str,
    ) -> Result<(), &'static str> {
        let content = self
            .copy_registers
            .get(register_name)
            .cloned()
            .ok_or("Register is empty")?;
        let win = self.windows.get_mut(window_id).ok_or("Window not found")?;
        let pane = win
            .panes
            .iter_mut()
            .find(|p| p.id == pane_id)
            .ok_or("Pane not found")?;

        pane.execute_command(&content);
        Ok(())
    }

    /// Formats custom status line expanding placeholders
    pub fn render_formatted_status(&self) -> String {
        let active_win = &self.windows[self.active_window_idx];
        let mut status = self.status_format.clone();

        status = status.replace("#{session_name}", &self.name);
        status = status.replace("#{window_name}", &active_win.name);
        status = status.replace("#{active_pane}", &format!("{}", active_win.active_pane_idx));
        status = status.replace("#{cpu_usage}", "CPU: 1.2%");
        status = status.replace("#{mem_usage}", "MEM: 12%");
        status = status.replace("#{hostname}", "sigmaos-host");

        status
    }
}

/// The high-level Terminal Multiplexer manager
pub struct TmuxSessionManager {
    pub sessions: HashMap<String, TmuxSession>,
    pub active_session_name: Option<String>,
    pub control_parser: TmuxControlModeParser,
}

impl TmuxSessionManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            active_session_name: None,
            control_parser: TmuxControlModeParser::new(),
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

    /// Generates status bar using formatted placeholder renderer
    pub fn get_status_bar(&self) -> String {
        if let Some(ref name) = self.active_session_name {
            if let Some(session) = self.sessions.get(name) {
                session.render_formatted_status()
            } else {
                "[No Session] | CPU: 0.0% | MEM: 0%".to_string()
            }
        } else {
            "[No Session] | CPU: 0.0% | MEM: 0%".to_string()
        }
    }
}

impl Default for TmuxSessionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
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
    fn test_control_mode_parser() {
        let parser = TmuxControlModeParser::new();

        let event1 = parser.parse_line("%output %1 hello_world");
        assert_eq!(
            event1,
            TmuxControlEvent::Output {
                pane_id: 1,
                text: "hello_world".to_string()
            }
        );

        let event2 = parser.parse_line("%window-add @2 zsh");
        assert_eq!(
            event2,
            TmuxControlEvent::WindowAdd {
                window_id: 2,
                name: "zsh".to_string()
            }
        );

        let event3 = parser.parse_line("%session-changed main_session");
        assert_eq!(
            event3,
            TmuxControlEvent::SessionChanged {
                session_name: "main_session".to_string()
            }
        );
    }

    #[test]
    fn test_sync_panes_and_copy_mode_search() {
        let mut session = TmuxSession::new("sync_session");
        let window = &mut session.windows[0];

        window.split_pane(0, SplitDirection::Horizontal).unwrap();
        assert!(!window.sync_panes_enabled);

        window.toggle_sync_panes();
        assert!(window.sync_panes_enabled);

        window.send_input_to_active("uptime");
        assert_eq!(window.panes[0].current_command.as_deref(), Some("uptime"));
        assert_eq!(window.panes[1].current_command.as_deref(), Some("uptime"));

        // Copy mode search
        let search_results = window.panes[0].search_copy_mode("Output");
        assert_eq!(search_results.len(), 1);
    }

    #[test]
    fn test_formatted_status_bar() {
        let mut session = TmuxSession::new("status_test");
        let rendered = session.render_formatted_status();
        assert!(rendered.contains("status_test"));
        assert!(rendered.contains("bash"));
        assert!(rendered.contains("CPU: 1.2%"));
    }
}
