// SPDX-License-Identifier: MIT
// SigmaOS - Terminal Emulator
// Inspired by GNOME Terminal
// Provides terminal emulation and shell integration

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};

/// Color scheme
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorScheme {
    Default,
    SolarizedLight,
    SolarizedDark,
    Tango,
    Gruvbox,
    Dracula,
    Nord,
}

/// Cursor shape
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorShape {
    Block,
    Underline,
    Bar,
}

/// Scrollbar position
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollbarPosition {
    Left,
    Right,
    Hidden,
}

/// Bell mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BellMode {
    None,
    Visual,
    Audible,
    Both,
}

/// Terminal profile
#[derive(Debug, Clone)]
pub struct TerminalProfile {
    pub name: String,
    pub color_scheme: ColorScheme,
    pub font_name: String,
    pub font_size: u32,
    pub cursor_shape: CursorShape,
    pub cursor_blink: bool,
    pub scrollbar_position: ScrollbarPosition,
    pub bell_mode: BellMode,
    pub scrollback_lines: u32,
    pub transparency: f32,
    pub bold_is_bright: bool,
}

/// Terminal tab
#[derive(Debug, Clone)]
pub struct TerminalTab {
    pub id: u32,
    pub title: String,
    pub profile_name: String,
    pub current_directory: String,
    pub is_active: bool,
}

/// Terminal session
#[derive(Debug, Clone)]
pub struct TerminalSession {
    pub id: u32,
    pub shell_command: String,
    pub environment: BTreeMap<String, String>,
    pub is_running: bool,
}

/// Terminal emulator
pub struct TerminalEmulator {
    pub profiles: BTreeMap<String, TerminalProfile>,
    pub tabs: Vec<TerminalTab>,
    pub sessions: BTreeMap<u32, TerminalSession>,
    pub active_tab: Option<usize>,
    pub next_tab_id: AtomicU32,
    next_session_id: AtomicU32,
}

impl TerminalEmulator {
    pub fn new() -> Self {
        let mut profiles = BTreeMap::new();

        // Default profile
        profiles.insert(
            "Default".to_string(),
            TerminalProfile {
                name: "Default".to_string(),
                color_scheme: ColorScheme::Default,
                font_name: "Monospace".to_string(),
                font_size: 12,
                cursor_shape: CursorShape::Block,
                cursor_blink: true,
                scrollbar_position: ScrollbarPosition::Right,
                bell_mode: BellMode::Both,
                scrollback_lines: 10000,
                transparency: 0.0,
                bold_is_bright: false,
            },
        );

        Self {
            profiles,
            tabs: Vec::new(),
            sessions: BTreeMap::new(),
            active_tab: None,
            next_tab_id: AtomicU32::new(1),
            next_session_id: AtomicU32::new(1),
        }
    }

    /// Add profile
    pub fn add_profile(&mut self, profile: TerminalProfile) {
        self.profiles.insert(profile.name.clone(), profile);
    }

    /// Remove profile
    pub fn remove_profile(&mut self, name: &str) -> Result<(), &'static str> {
        if name == "Default" {
            return Err("Cannot remove default profile");
        }
        if !self.profiles.contains_key(name) {
            return Err("Profile not found");
        }
        self.profiles.remove(name);
        Ok(())
    }

    /// Get profile
    pub fn get_profile(&self, name: &str) -> Option<&TerminalProfile> {
        self.profiles.get(name)
    }

    /// Create tab
    pub fn create_tab(&mut self, profile_name: String, title: String) -> usize {
        let id = self.next_tab_id.fetch_add(1, Ordering::Relaxed);
        let tab = TerminalTab {
            id,
            title,
            profile_name,
            current_directory: "/".to_string(),
            is_active: false,
        };
        self.tabs.push(tab);
        if self.tabs.len() == 1 {
            self.active_tab = Some(0);
            self.tabs[0].is_active = true;
        }
        self.tabs.len() - 1
    }

    /// Close tab
    pub fn close_tab(&mut self, index: usize) -> Result<(), &'static str> {
        if index >= self.tabs.len() {
            return Err("Tab index out of bounds");
        }
        if self.tabs.len() == 1 {
            return Err("Cannot close last tab");
        }
        self.tabs.remove(index);
        if self.active_tab == Some(index) {
            self.active_tab = Some(index.saturating_sub(1));
        } else if self.active_tab > Some(index) {
            self.active_tab = self.active_tab.map(|a| a - 1);
        }
        if let Some(active) = self.active_tab {
            self.tabs[active].is_active = true;
        }
        Ok(())
    }

    /// Switch tab
    pub fn switch_tab(&mut self, index: usize) -> Result<(), &'static str> {
        if index >= self.tabs.len() {
            return Err("Tab index out of bounds");
        }
        if let Some(active) = self.active_tab {
            self.tabs[active].is_active = false;
        }
        self.active_tab = Some(index);
        self.tabs[index].is_active = true;
        Ok(())
    }

    /// Set tab title
    pub fn set_tab_title(&mut self, index: usize, title: String) -> Result<(), &'static str> {
        if index >= self.tabs.len() {
            return Err("Tab index out of bounds");
        }
        self.tabs[index].title = title;
        Ok(())
    }

    /// Set current directory
    pub fn set_current_directory(
        &mut self,
        index: usize,
        path: String,
    ) -> Result<(), &'static str> {
        if index >= self.tabs.len() {
            return Err("Tab index out of bounds");
        }
        self.tabs[index].current_directory = path;
        Ok(())
    }

    /// Create session
    pub fn create_session(&mut self, shell_command: String) -> u32 {
        let id = self.next_session_id.fetch_add(1, Ordering::Relaxed);
        let mut environment = BTreeMap::new();
        environment.insert("TERM".to_string(), "xterm-256color".to_string());
        environment.insert("LANG".to_string(), "en_US.UTF-8".to_string());

        let session = TerminalSession {
            id,
            shell_command,
            environment,
            is_running: true,
        };
        self.sessions.insert(id, session);
        id
    }

    /// Close session
    pub fn close_session(&mut self, id: u32) -> Result<(), &'static str> {
        if !self.sessions.contains_key(&id) {
            return Err("Session not found");
        }
        self.sessions.remove(&id);
        Ok(())
    }

    /// Send input to session
    pub fn send_input(&mut self, session_id: u32, input: String) -> Result<(), &'static str> {
        let session = self
            .sessions
            .get_mut(&session_id)
            .ok_or("Session not found")?;
        if !session.is_running {
            return Err("Session not running");
        }
        // Simplified - in real implementation would send to shell
        Ok(())
    }

    /// Get active tab
    pub fn get_active_tab(&self) -> Option<&TerminalTab> {
        self.active_tab.and_then(|index| self.tabs.get(index))
    }

    /// Get tab count
    pub fn tab_count(&self) -> usize {
        self.tabs.len()
    }

    /// Get session count
    pub fn session_count(&self) -> usize {
        self.sessions.len()
    }

    /// List profiles
    pub fn list_profiles(&self) -> Vec<&String> {
        self.profiles.keys().collect()
    }
}

/// Global terminal emulator
static mut TERMINAL_EMULATOR: Option<TerminalEmulator> = None;

/// Initialize terminal emulator
pub fn init_terminal() {
    unsafe {
        TERMINAL_EMULATOR = Some(TerminalEmulator::new());
    }
}

/// Get terminal emulator
pub fn get_terminal() -> &'static mut TerminalEmulator {
    unsafe {
        TERMINAL_EMULATOR
            .as_mut()
            .expect("Terminal emulator not initialized")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terminal_creation() {
        let term = TerminalEmulator::new();
        assert!(term.profiles.contains_key("Default"));
    }

    #[test]
    fn test_add_profile() {
        let mut term = TerminalEmulator::new();
        let profile = TerminalProfile {
            name: "Custom".to_string(),
            color_scheme: ColorScheme::Dracula,
            font_name: "Monospace".to_string(),
            font_size: 14,
            cursor_shape: CursorShape::Underline,
            cursor_blink: false,
            scrollbar_position: ScrollbarPosition::Left,
            bell_mode: BellMode::Visual,
            scrollback_lines: 5000,
            transparency: 0.1,
            bold_is_bright: true,
        };
        term.add_profile(profile);
        assert!(term.profiles.contains_key("Custom"));
    }

    #[test]
    fn test_create_tab() {
        let mut term = TerminalEmulator::new();
        let index = term.create_tab("Default".to_string(), "Terminal".to_string());
        assert_eq!(term.tab_count(), 1);
        assert_eq!(term.active_tab, Some(0));
    }

    #[test]
    fn test_close_tab() {
        let mut term = TerminalEmulator::new();
        term.create_tab("Default".to_string(), "Tab1".to_string());
        term.create_tab("Default".to_string(), "Tab2".to_string());
        term.close_tab(0).unwrap();
        assert_eq!(term.tab_count(), 1);
    }

    #[test]
    fn test_switch_tab() {
        let mut term = TerminalEmulator::new();
        term.create_tab("Default".to_string(), "Tab1".to_string());
        term.create_tab("Default".to_string(), "Tab2".to_string());
        term.switch_tab(1).unwrap();
        assert_eq!(term.active_tab, Some(1));
    }

    #[test]
    fn test_create_session() {
        let mut term = TerminalEmulator::new();
        let id = term.create_session("/bin/bash".to_string());
        assert!(term.sessions.contains_key(&id));
    }
}
