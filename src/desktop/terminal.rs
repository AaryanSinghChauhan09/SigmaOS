// OOP-based Desktop Terminal for SigmaOS
// Implements terminal emulator, tab multiplexing (tmux-inspiration), and command history/autocomplete.

extern crate alloc;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type TerminalID = usize;
pub type TabID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalError {
    Success = 0,
    NotFound = 1,
    CommandFailed = 2,
    TabLimitReached = 3,
}

/// A multiplexed terminal tab (tmux / tabbed emulator inspiration)
#[derive(Debug, Clone)]
pub struct TerminalTab {
    pub id: TabID,
    pub title: String,
    pub working_directory: String,
}

impl TerminalTab {
    pub fn new(id: TabID, title: &str, working_directory: &str) -> Self {
        Self {
            id,
            title: String::from(title),
            working_directory: String::from(working_directory),
        }
    }
}

pub trait Terminal {
    fn id(&self) -> TerminalID;
    fn title(&self) -> &[u8];
    fn working_directory(&self) -> &[u8];
    fn set_working_directory(&mut self, path: &[u8]);
}

/// Simple terminal with support for tab multiplexing, command history, and suggestions
pub struct SimpleTerminal {
    pub id: TerminalID,
    pub title: [u8; 128],
    pub working_directory: [u8; 256],
    pub tabs: Vec<TerminalTab>,
    pub active_tab_index: usize,
    pub command_history: Vec<String>,
}

impl SimpleTerminal {
    pub fn new(id: TerminalID, title: &[u8]) -> Self {
        let mut title_array = [0u8; 128];
        let mut dir_array = [0u8; 256];
        let title_len = title.len().min(127);
        let dir_len = b"/home/user".len().min(255);
        title_array[..title_len].copy_from_slice(&title[..title_len]);
        dir_array[..dir_len].copy_from_slice(&b"/home/user"[..dir_len]);

        let default_tab = TerminalTab::new(1, "Shell 1", "/home/user");

        SimpleTerminal {
            id,
            title: title_array,
            working_directory: dir_array,
            tabs: vec![default_tab],
            active_tab_index: 0,
            command_history: Vec::new(),
        }
    }

    /// Add a new tab to the multiplexed terminal
    pub fn create_tab(&mut self, title: &str) -> Result<TabID, TerminalError> {
        if self.tabs.len() >= 10 {
            return Err(TerminalError::TabLimitReached);
        }
        let tab_id = self.tabs.len() + 1;
        let tab = TerminalTab::new(tab_id, title, "/home/user");
        self.tabs.push(tab);
        Ok(tab_id)
    }

    /// Switch the active terminal tab
    pub fn switch_tab(&mut self, tab_index: usize) -> Result<(), TerminalError> {
        if tab_index >= self.tabs.len() {
            return Err(TerminalError::NotFound);
        }
        self.active_tab_index = tab_index;
        // Sync working directory (clone bytes first to release immutable borrow on self)
        let path = self.tabs[tab_index].working_directory.as_bytes().to_vec();
        self.set_working_directory(&path);
        Ok(())
    }

    /// Log executed command in history
    pub fn history_push(&mut self, cmd: &str) {
        self.command_history.push(String::from(cmd));
    }

    /// Suggest autocompletes based on typed prefix and history (fish / zsh inspiration)
    pub fn get_suggestions(&self, prefix: &str) -> Vec<String> {
        let mut suggestions = Vec::new();
        for cmd in &self.command_history {
            if cmd.starts_with(prefix) && !suggestions.contains(cmd) {
                suggestions.push(cmd.clone());
            }
        }
        suggestions
    }
}

impl Terminal for SimpleTerminal {
    fn id(&self) -> TerminalID {
        self.id
    }

    fn title(&self) -> &[u8] {
        let len = self.title.iter().position(|&b| b == 0).unwrap_or(128);
        &self.title[..len]
    }

    fn working_directory(&self) -> &[u8] {
        let len = self
            .working_directory
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(256);
        &self.working_directory[..len]
    }

    fn set_working_directory(&mut self, path: &[u8]) {
        let path_len = path.len().min(255);
        self.working_directory = [0; 256];
        self.working_directory[..path_len].copy_from_slice(&path[..path_len]);
        // Also update active tab state
        self.tabs[self.active_tab_index].working_directory =
            String::from_utf8_lossy(&path[..path_len]).into_owned();
    }
}

pub trait TerminalManager {
    fn create_terminal(&mut self, title: &[u8]) -> Result<TerminalID, TerminalError>;
    fn close_terminal(&mut self, id: TerminalID) -> Result<(), TerminalError>;
    fn get_terminal(&self, id: TerminalID) -> Option<&dyn Terminal>;
    fn execute_command(
        &mut self,
        terminal_id: TerminalID,
        command: &[u8],
    ) -> Result<Vec<u8>, TerminalError>;
}

pub struct SimpleTerminalManager {
    pub terminals: Vec<Option<Box<dyn Terminal>>>,
    pub next_id: AtomicUsize,
}

impl SimpleTerminalManager {
    pub fn new() -> Self {
        SimpleTerminalManager {
            terminals: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl Default for SimpleTerminalManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TerminalManager for SimpleTerminalManager {
    fn create_terminal(&mut self, title: &[u8]) -> Result<TerminalID, TerminalError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let terminal = SimpleTerminal::new(id, title);
        self.terminals.push(Some(Box::new(terminal)));
        Ok(id)
    }

    fn close_terminal(&mut self, id: TerminalID) -> Result<(), TerminalError> {
        for i in 0..self.terminals.len() {
            if let Some(ref terminal) = self.terminals[i] {
                if terminal.id() == id {
                    self.terminals[i] = None;
                    return Ok(());
                }
            }
        }
        Err(TerminalError::NotFound)
    }

    fn get_terminal(&self, id: TerminalID) -> Option<&dyn Terminal> {
        for terminal_option in &self.terminals {
            if let Some(ref terminal) = *terminal_option {
                if terminal.id() == id {
                    return Some(terminal.as_ref());
                }
            }
        }
        None
    }

    fn execute_command(
        &mut self,
        terminal_id: TerminalID,
        command: &[u8],
    ) -> Result<Vec<u8>, TerminalError> {
        if self.get_terminal(terminal_id).is_some() {
            let mut output = Vec::new();
            output.extend_from_slice(command);
            output.push(b'\n');
            Ok(output)
        } else {
            Err(TerminalError::NotFound)
        }
    }
}

pub trait ShellIntegration {
    fn get_shell(&self) -> &[u8];
    fn set_shell(&mut self, shell: &[u8]);
    fn get_env_var(&self, key: &[u8]) -> Option<&[u8]>;
    fn set_env_var(&mut self, key: &[u8], value: &[u8]);
}

pub struct SimpleShellIntegration {
    pub shell: [u8; 64],
    pub env_vars: Vec<([u8; 64], [u8; 256])>,
}

impl SimpleShellIntegration {
    pub fn new() -> Self {
        let mut shell_array = [0u8; 64];
        let shell_len = b"/bin/bash".len().min(63);
        shell_array[..shell_len].copy_from_slice(&b"/bin/bash"[..shell_len]);

        SimpleShellIntegration {
            shell: shell_array,
            env_vars: Vec::new(),
        }
    }
}

impl Default for SimpleShellIntegration {
    fn default() -> Self {
        Self::new()
    }
}

impl ShellIntegration for SimpleShellIntegration {
    fn get_shell(&self) -> &[u8] {
        let len = self.shell.iter().position(|&b| b == 0).unwrap_or(64);
        &self.shell[..len]
    }

    fn set_shell(&mut self, shell: &[u8]) {
        let shell_len = shell.len().min(63);
        self.shell = [0; 64];
        self.shell[..shell_len].copy_from_slice(&shell[..shell_len]);
    }

    fn get_env_var(&self, key: &[u8]) -> Option<&[u8]> {
        for &(ref k, ref v) in &self.env_vars {
            let k_len = k.iter().position(|&b| b == 0).unwrap_or(64);
            if &k[..k_len] == key {
                let v_len = v.iter().position(|&b| b == 0).unwrap_or(256);
                return Some(&v[..v_len]);
            }
        }
        None
    }

    fn set_env_var(&mut self, key: &[u8], value: &[u8]) {
        let mut key_array = [0u8; 64];
        let mut value_array = [0u8; 256];
        let key_len = key.len().min(63);
        let value_len = value.len().min(255);

        key_array[..key_len].copy_from_slice(&key[..key_len]);
        value_array[..value_len].copy_from_slice(&value[..value_len]);

        self.env_vars.push((key_array, value_array));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terminal_manager_and_execution() {
        let mut manager = SimpleTerminalManager::new();
        let term_id = manager.create_terminal(b"SigmaTerminal").unwrap();

        let terminal = manager.get_terminal(term_id).unwrap();
        assert_eq!(terminal.title(), b"SigmaTerminal");
        assert_eq!(terminal.working_directory(), b"/home/user");

        // Executing command returning echo outputs
        let output = manager.execute_command(term_id, b"ls -la").unwrap();
        assert_eq!(output, b"ls -la\n");
    }

    #[test]
    fn test_terminal_multiplexing_and_autocomplete() {
        let mut term = SimpleTerminal::new(1, b"Shell");
        assert_eq!(term.tabs.len(), 1);

        // Create new tab
        let tab_id = term.create_tab("Developer Shell").unwrap();
        assert_eq!(tab_id, 2);
        assert_eq!(term.tabs.len(), 2);

        // Switch active tab and modify working directory
        term.switch_tab(1).unwrap();
        assert_eq!(term.active_tab_index, 1);
        term.set_working_directory(b"/home/user/workspace");
        assert_eq!(term.working_directory(), b"/home/user/workspace");

        // Push some history and retrieve autocompletes
        term.history_push("cargo build --release");
        term.history_push("cargo test");
        term.history_push("make build");

        let suggs = term.get_suggestions("cargo");
        assert_eq!(suggs.len(), 2);
        assert_eq!(suggs[0], "cargo build --release");
        assert_eq!(suggs[1], "cargo test");
    }
}
