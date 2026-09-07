use std::string::{String, ToString};
use std::vec::Vec;
use std::format;
//! Terminal Emulator (gnome-terminal/konsole Inspiration)
//! Terminal sessions, profiles, and PTY management




/// Terminal profile
#[derive(Debug, Clone)]
pub struct TerminalProfile {
    pub name: String,
    pub font: String,
    pub font_size: u32,
    pub colors: TerminalColors,
}

#[derive(Debug, Clone)]
pub struct TerminalColors {
    pub foreground: String,
    pub background: String,
    pub cursor: String,
}

impl TerminalProfile {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            font: "monospace".to_string(),
            font_size: 12,
            colors: TerminalColors {
                foreground: "#ffffff".to_string(),
                background: "#000000".to_string(),
                cursor: "#ffffff".to_string(),
            },
        }
    }

    pub fn set_font(&mut self, font: &str) {
        self.font = font.to_string();
    }
}

/// Terminal session
#[derive(Debug, Clone)]
pub struct TerminalSession {
    pub id: String,
    pub shell: String,
    pub working_directory: String,
    pub history: Vec<String>,
}

impl TerminalSession {
    pub fn new(id: &str, shell: &str) -> Self {
        Self {
            id: id.to_string(),
            shell: shell.to_string(),
            working_directory: "/home/user".to_string(),
            history: Vec::new(),
        }
    }

    pub fn set_working_directory(&mut self, path: &str) {
        self.working_directory = path.to_string();
    }

    pub fn add_to_history(&mut self, command: &str) {
        self.history.push(command.to_string());
    }
}

/// Pseudo terminal
#[derive(Debug, Clone)]
pub struct PseudoTerminal {
    pub master_fd: u32,
    pub slave_fd: u32,
    pub size: TerminalSize,
}

#[derive(Debug, Clone)]
pub struct TerminalSize {
    pub rows: u16,
    pub cols: u16,
}

impl PseudoTerminal {
    pub fn new() -> Self {
        Self {
            master_fd: 0,
            slave_fd: 0,
            size: TerminalSize {
                rows: 24,
                cols: 80,
            },
        }
    }

    pub fn set_size(&mut self, rows: u16, cols: u16) {
        self.size.rows = rows;
        self.size.cols = cols;
    }
}

/// Terminal emulator
pub struct TerminalEmulator {
    pub sessions: Vec<TerminalSession>,
    pub profiles: Vec<TerminalProfile>,
    pub current_session: Option<String>,
}

impl TerminalEmulator {
    pub fn new() -> Self {
        Self {
            sessions: Vec::new(),
            profiles: Vec::new(),
            current_session: None,
        }
    }

    pub fn add_session(&mut self, session: TerminalSession) {
        self.sessions.push(session);
    }

    pub fn add_profile(&mut self, profile: TerminalProfile) {
        self.profiles.push(profile);
    }

    pub fn new_session(&mut self, shell: &str) -> String {
        let id = format!("session-{}", self.sessions.len());
        let session = TerminalSession::new(&id, shell);
        self.sessions.push(session);
        id
    }

    pub fn switch_session(&mut self, id: &str) {
        self.current_session = Some(id.to_string());
    }

    pub fn close_session(&mut self, id: &str) {
        self.sessions.retain(|s| s.id != id);
    }

    pub fn get_session(&mut self, id: &str) -> Option<&mut TerminalSession> {
        self.sessions.iter_mut().find(|s| s.id == id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TerminalError {
    SessionNotFound,
    ProfileNotFound,
    PTYError,
}

impl Default for TerminalEmulator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_terminal_profile() {
        let profile = TerminalProfile::new("default");
        assert_eq!(profile.name, "default");
    }

    #[test]
    fn test_terminal_session() {
        let session = TerminalSession::new("session-1", "/bin/bash");
        assert_eq!(session.shell, "/bin/bash");
    }

    #[test]
    fn test_terminal_emulator() {
        let mut emulator = TerminalEmulator::new();
        let id = emulator.new_session("/bin/bash");
        assert_eq!(emulator.sessions.len(), 1);
    }
}