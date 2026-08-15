#![no_std]
#![no_main]

/// OOP-based Desktop Terminal for SigmaOS
/// Implements terminal emulator, ANSI escape interpretation, and shell integration.
/// Inspired by Alacritty, GNOME-Terminal, xterm, and tmux from Linux & BSD distributions.
/// Enhanced with SerenityOS-style tab support for better productivity.

extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;
use alloc::string::String;

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type TerminalID = usize;
pub type TabID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalError { Success = 0, NotFound = 1, CommandFailed = 2, TabNotFound = 3, TabLimitReached = 4 }

pub trait Terminal {
    fn id(&self) -> TerminalID;
    fn title(&self) -> &[u8];
    fn working_directory(&self) -> &[u8];
    fn set_working_directory(&mut self, path: &[u8]);
}

/// SerenityOS-style Tab for terminal emulator
#[repr(C)]
pub struct TerminalTab {
    pub id: TabID,
    pub title: String,
    pub terminal_id: TerminalID,
    pub is_active: bool,
    pub is_pinned: bool,
    pub color_scheme: TabColorScheme,
    pub working_directory: String,
    pub shell_process: Option<usize>, // Process ID of shell
    pub scrollback_lines: Vec<String>,
    pub current_line: String,
    pub cursor_position: (usize, usize), // (row, col)
    pub history: Vec<String>,           // Command history
    pub history_index: usize,
    pub split_config: Option<TabSplitConfig>, // For split panes
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabColorScheme {
    Default = 0,
    Dark = 1,
    Light = 2,
    Solarized = 3,
    Custom = 4,
}

/// Tab split configuration for terminal panes
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitDirection {
    Horizontal = 0,
    Vertical = 1,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct TabSplitConfig {
    pub direction: SplitDirection,
    pub split_ratio: f32, // 0.0 to 1.0, position of split
    pub child_tabs: Vec<TabID>, // IDs of child tabs in split
}

impl TerminalTab {
    pub fn new(id: TabID, title: &str, terminal_id: TerminalID) -> Self {
        TerminalTab {
            id,
            title: String::from(title),
            terminal_id,
            is_active: false,
            is_pinned: false,
            color_scheme: TabColorScheme::Default,
            working_directory: String::from("/home/user"),
            shell_process: None,
            scrollback_lines: Vec::new(),
            current_line: String::new(),
            cursor_position: (0, 0),
            history: Vec::new(),
            history_index: 0,
            split_config: None,
        }
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = String::from(title);
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    pub fn set_pinned(&mut self, pinned: bool) {
        self.is_pinned = pinned;
    }

    pub fn set_color_scheme(&mut self, scheme: TabColorScheme) {
        self.color_scheme = scheme;
    }

    pub fn search_command_history(&self, query: &str) -> Vec<String> {
        self.history
            .iter()
            .filter(|cmd| cmd.contains(query))
            .cloned()
            .collect()
    }

    pub fn set_working_directory(&mut self, path: &str) {
        self.working_directory = String::from(path);
    }

    pub fn get_working_directory(&self) -> &str {
        &self.working_directory
    }

    pub fn add_to_history(&mut self, command: &str) {
        if !command.trim().is_empty() {
            self.history.retain(|c| c != command); // Remove duplicates
            self.history.push(String::from(command));
            self.history_index = self.history.len();
        }
    }

    pub fn get_history_previous(&mut self) -> Option<&str> {
        if self.history.is_empty() {
            return None;
        }
        if self.history_index > 0 {
            self.history_index -= 1;
            self.history.get(self.history_index).map(|s| s.as_str())
        } else {
            self.history.get(0).map(|s| s.as_str())
        }
    }

    pub fn get_history_next(&mut self) -> Option<&str> {
        if self.history.is_empty() {
            return None;
        }
        if self.history_index < self.history.len() - 1 {
            self.history_index += 1;
            self.history.get(self.history_index).map(|s| s.as_str())
        } else {
            None
        }
    }

    pub fn split_tab(&mut self, direction: SplitDirection, ratio: f32, new_tab_id: TabID) {
        let mut child_tabs = vec![self.id, new_tab_id];
        self.split_config = Some(TabSplitConfig {
            direction,
            split_ratio: ratio,
            child_tabs,
        });
    }

    pub fn write_to_scrollback(&mut self, line: &str) {
        self.scrollback_lines.push(String::from(line));
        // Limit scrollback to prevent memory issues
        if self.scrollback_lines.len() > 10000 {
            self.scrollback_lines.remove(0);
        }
    }

    pub fn get_scrollback(&self) -> &[String] {
        &self.scrollback_lines
    }

    pub fn clear_scrollback(&mut self) {
        self.scrollback_lines.clear();
    }

    pub fn set_cursor_position(&mut self, row: usize, col: usize) {
        self.cursor_position = (row, col);
    }

    pub fn get_cursor_position(&self) -> (usize, usize) {
        self.cursor_position
    }
}

#[repr(C)]
pub struct SimpleTerminal {
    pub id: TerminalID,
    pub title: [u8; 128],
    pub working_directory: [u8; 256],
}

impl SimpleTerminal {
    pub fn new(id: TerminalID, title: &[u8]) -> Self {
        let mut title_array = [0u8; 128];
        let mut dir_array = [0u8; 256];
        let title_len = title.len().min(127);
        let dir_len = b"/home/user".len().min(255);
        unsafe {
            core::ptr::copy_nonoverlapping(title.as_ptr(), title_array.as_mut_ptr(), title_len);
            core::ptr::copy_nonoverlapping(b"/home/user".as_ptr(), dir_array.as_mut_ptr(), dir_len);
        }
        SimpleTerminal {
            id,
            title: title_array,
            working_directory: dir_array,
        }
    }
}

impl Terminal for SimpleTerminal {
    fn id(&self) -> TerminalID { self.id }
    fn title(&self) -> &[u8] {
        let len = self.title.iter().position(|&b| b == 0).unwrap_or(128);
        &self.title[..len]
    }
    fn working_directory(&self) -> &[u8] {
        let len = self.working_directory.iter().position(|&b| b == 0).unwrap_or(256);
        &self.working_directory[..len]
    }
    
    fn set_working_directory(&mut self, path: &[u8]) {
        let path_len = path.len().min(255);
        unsafe {
            core::ptr::copy_nonoverlapping(path.as_ptr(), self.working_directory.as_mut_ptr(), path_len);
        }
    }
}

pub trait TerminalManager {
    fn create_terminal(&mut self, title: &[u8]) -> Result<TerminalID, TerminalError>;
    fn close_terminal(&mut self, id: TerminalID) -> Result<(), TerminalError>;
    fn get_terminal(&self, id: TerminalID) -> Option<&dyn Terminal>;
    fn execute_command(&mut self, terminal_id: TerminalID, command: &[u8]) -> Result<Vec<u8>, TerminalError>;
}

/// Tab group for organizing related tabs
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TabGroup {
    pub id: usize,
    pub name: String,
    pub tab_ids: Vec<TabID>,
    pub color: Option<u32>, // RGB color for group indicator
}

impl TabGroup {
    pub fn new(id: usize, name: &str) -> Self {
        TabGroup {
            id,
            name: String::from(name),
            tab_ids: Vec::new(),
            color: None,
        }
    }

    pub fn add_tab(&mut self, tab_id: TabID) {
        if !self.tab_ids.contains(&tab_id) {
            self.tab_ids.push(tab_id);
        }
    }

    pub fn remove_tab(&mut self, tab_id: TabID) {
        self.tab_ids.retain(|&id| id != tab_id);
    }
}

/// SerenityOS-style Tab Manager for terminal emulator
#[repr(C)]
pub struct TabManager {
    pub tabs: Vec<TerminalTab>,
    pub active_tab_id: Option<TabID>,
    pub next_tab_id: AtomicUsize,
    pub max_tabs: usize,
    pub tab_groups: Vec<TabGroup>, // For organizing tabs into groups
    pub search_query: Option<String>, // For tab search/filtering
}

impl TabManager {
    pub fn new(max_tabs: usize) -> Self {
        TabManager {
            tabs: Vec::new(),
            active_tab_id: None,
            next_tab_id: AtomicUsize::new(1),
            max_tabs,
            tab_groups: Vec::new(),
            search_query: None,
        }
    }

    /// Create a new tab
    pub fn create_tab(&mut self, title: &str, terminal_id: TerminalID) -> Result<TabID, TerminalError> {
        if self.tabs.len() >= self.max_tabs {
            return Err(TerminalError::TabLimitReached);
        }

        let tab_id = self.next_tab_id.fetch_add(1, Ordering::SeqCst);
        let mut tab = TerminalTab::new(tab_id, title, terminal_id);
        
        // If this is the first tab, make it active
        if self.tabs.is_empty() {
            tab.set_active(true);
            self.active_tab_id = Some(tab_id);
        }

        self.tabs.push(tab);
        Ok(tab_id)
    }

    /// Close a tab
    pub fn close_tab(&mut self, tab_id: TabID) -> Result<(), TerminalError> {
        let tab_index = self.tabs.iter().position(|t| t.id == tab_id)
            .ok_or(TerminalError::TabNotFound)?;

        let was_active = self.tabs[tab_index].is_active;
        self.tabs.remove(tab_index);

        // If we closed the active tab, activate another one
        if was_active && !self.tabs.is_empty() {
            let new_active_index = tab_index.min(self.tabs.len() - 1);
            self.tabs[new_active_index].set_active(true);
            self.active_tab_id = Some(self.tabs[new_active_index].id);
        } else if self.tabs.is_empty() {
            self.active_tab_id = None;
        }

        Ok(())
    }

    /// Switch to a specific tab
    pub fn switch_to_tab(&mut self, tab_id: TabID) -> Result<(), TerminalError> {
        let tab = self.tabs.iter_mut().find(|t| t.id == tab_id)
            .ok_or(TerminalError::TabNotFound)?;

        // Deactivate current active tab
        if let Some(current_id) = self.active_tab_id {
            if let Some(current_tab) = self.tabs.iter_mut().find(|t| t.id == current_id) {
                current_tab.set_active(false);
            }
        }

        // Activate new tab
        tab.set_active(true);
        self.active_tab_id = Some(tab_id);
        Ok(())
    }

    /// Get the active tab
    pub fn get_active_tab(&self) -> Option<&TerminalTab> {
        self.active_tab_id.and_then(|id| self.tabs.iter().find(|t| t.id == id))
    }

    /// Get all tabs
    pub fn get_tabs(&self) -> &[TerminalTab] {
        &self.tabs
    }

    /// Move tab to new position
    pub fn move_tab(&mut self, tab_id: TabID, new_index: usize) -> Result<(), TerminalError> {
        let current_index = self.tabs.iter().position(|t| t.id == tab_id)
            .ok_or(TerminalError::TabNotFound)?;

        if new_index >= self.tabs.len() {
            return Err(TerminalError::NotFound);
        }

        let tab = self.tabs.remove(current_index);
        self.tabs.insert(new_index, tab);
        Ok(())
    }

    /// Pin/unpin a tab
    pub fn toggle_pin_tab(&mut self, tab_id: TabID) -> Result<(), TerminalError> {
        let tab = self.tabs.iter_mut().find(|t| t.id == tab_id)
            .ok_or(TerminalError::TabNotFound)?;
        tab.set_pinned(!tab.is_pinned);
        Ok(())
    }

    /// Set tab color scheme
    pub fn set_tab_color_scheme(&mut self, tab_id: TabID, scheme: TabColorScheme) -> Result<(), TerminalError> {
        let tab = self.tabs.iter_mut().find(|t| t.id == tab_id)
            .ok_or(TerminalError::TabNotFound)?;
        tab.set_color_scheme(scheme);
        Ok(())
    }

    /// Get next tab (for cycling)
    pub fn get_next_tab(&self) -> Option<&TerminalTab> {
        if let Some(current_id) = self.active_tab_id {
            let current_index = self.tabs.iter().position(|t| t.id == current_id)?;
            let next_index = (current_index + 1) % self.tabs.len();
            self.tabs.get(next_index)
        } else {
            self.tabs.first()
        }
    }

    /// Get previous tab (for cycling)
    pub fn get_previous_tab(&self) -> Option<&TerminalTab> {
        if let Some(current_id) = self.active_tab_id {
            let current_index = self.tabs.iter().position(|t| t.id == current_id)?;
            let prev_index = if current_index == 0 {
                self.tabs.len() - 1
            } else {
                current_index - 1
            };
            self.tabs.get(prev_index)
        } else {
            self.tabs.last()
        }
    }

    /// Create a new tab group
    pub fn create_tab_group(&mut self, name: &str) -> usize {
        let group_id = self.tab_groups.len();
        let group = TabGroup::new(group_id, name);
        self.tab_groups.push(group);
        group_id
    }

    /// Add tab to group
    pub fn add_tab_to_group(&mut self, tab_id: TabID, group_id: usize) -> Result<(), TerminalError> {
        if let Some(group) = self.tab_groups.get_mut(group_id) {
            group.add_tab(tab_id);
            Ok(())
        } else {
            Err(TerminalError::NotFound)
        }
    }

    /// Remove tab from group
    pub fn remove_tab_from_group(&mut self, tab_id: TabID, group_id: usize) -> Result<(), TerminalError> {
        if let Some(group) = self.tab_groups.get_mut(group_id) {
            group.remove_tab(tab_id);
            Ok(())
        } else {
            Err(TerminalError::NotFound)
        }
    }

    /// Get tabs in a group
    pub fn get_group_tabs(&self, group_id: usize) -> Vec<&TerminalTab> {
        if let Some(group) = self.tab_groups.get(group_id) {
            group.tab_ids.iter()
                .filter_map(|&tab_id| self.tabs.iter().find(|t| t.id == tab_id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Search tabs by title or content
    pub fn search_tabs(&mut self, query: &str) -> Vec<&TerminalTab> {
        self.search_query = Some(String::from(query));
        
        if query.is_empty() {
            return self.tabs.iter().collect();
        }

        let query_lower = query.to_lowercase();
        self.tabs.iter()
            .filter(|tab| {
                tab.title.to_lowercase().contains(&query_lower) ||
                tab.working_directory.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// Clear search query
    pub fn clear_search(&mut self) {
        self.search_query = None;
    }

    /// Duplicate a tab (create new tab with same working directory)
    pub fn duplicate_tab(&mut self, tab_id: TabID) -> Result<TabID, TerminalError> {
        let original_tab = self.tabs.iter().find(|t| t.id == tab_id)
            .ok_or(TerminalError::TabNotFound)?;

        let new_tab_id = self.create_tab(&format!("{} (copy)", original_tab.title), original_tab.terminal_id)?;
        
        if let Some(new_tab) = self.tabs.iter_mut().find(|t| t.id == new_tab_id) {
            new_tab.set_working_directory(&original_tab.working_directory);
            new_tab.color_scheme = original_tab.color_scheme;
        }

        Ok(new_tab_id)
    }

    /// Export tab history to file
    pub fn export_tab_history(&self, tab_id: TabID) -> Result<Vec<String>, TerminalError> {
        let tab = self.tabs.iter().find(|t| t.id == tab_id)
            .ok_or(TerminalError::TabNotFound)?;
        Ok(tab.history.clone())
    }

    /// Import tab history from file
    pub fn import_tab_history(&mut self, tab_id: TabID, history: Vec<String>) -> Result<(), TerminalError> {
        let tab = self.tabs.iter_mut().find(|t| t.id == tab_id)
            .ok_or(TerminalError::TabNotFound)?;
        tab.history = history;
        tab.history_index = tab.history.len();
        Ok(())
    }

    /// Get tab statistics
    pub fn get_tab_stats(&self, tab_id: TabID) -> Result<TabStats, TerminalError> {
        let tab = self.tabs.iter().find(|t| t.id == tab_id)
            .ok_or(TerminalError::TabNotFound)?;

        Ok(TabStats {
            id: tab.id,
            title: tab.title.clone(),
            working_directory: tab.working_directory.clone(),
            command_count: tab.history.len(),
            scrollback_lines: tab.scrollback_lines.len(),
            is_active: tab.is_active,
            is_pinned: tab.is_pinned,
            has_split: tab.split_config.is_some(),
        })
    }

    /// Get all tab statistics
    pub fn get_all_tab_stats(&self) -> Vec<TabStats> {
        self.tabs.iter()
            .filter_map(|tab| self.get_tab_stats(tab.id).ok())
            .collect()
    }

    pub fn helenos_async_ipc_dispatch(&mut self, tab_id: TabID, msg: &str) -> Result<(), TerminalError> {
        if let Some(tab) = self.tabs.iter_mut().find(|t| t.id == tab_id) {
            tab.scrollback_lines.push(format!("[HelenOS-IPC]: {}", msg));
            Ok(())
        } else {
            Err(TerminalError::TabNotFound)
        }
    }

    pub fn kuroko_script_control(&mut self, tab_id: TabID, script: &str) -> Result<String, TerminalError> {
        if let Some(tab) = self.tabs.iter_mut().find(|t| t.id == tab_id) {
            tab.history.push(script.to_string());
            Ok(format!("Kuroko Executed: '{}' on tab {}", script, tab_id))
        } else {
            Err(TerminalError::TabNotFound)
        }
    }
}

/// Tab statistics for monitoring and debugging
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TabStats {
    pub id: TabID,
    pub title: String,
    pub working_directory: String,
    pub command_count: usize,
    pub scrollback_lines: usize,
    pub is_active: bool,
    pub is_pinned: bool,
    pub has_split: bool,
}

#[repr(C)]
pub struct SimpleTerminalManager {
    pub terminals: Vec<Option<Box<dyn Terminal>>>,
    pub next_id: AtomicUsize,
    pub tab_manager: TabManager,
}

impl SimpleTerminalManager {
    pub fn new() -> Self {
        SimpleTerminalManager {
            terminals: Vec::new(),
            next_id: AtomicUsize::new(1),
            tab_manager: TabManager::new(32), // Max 32 tabs per terminal window
        }
    }
}

impl TerminalManager for SimpleTerminalManager {
    fn create_terminal(&mut self, title: &[u8]) -> Result<TerminalID, TerminalError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let terminal = SimpleTerminal::new(id, title);
        self.terminals.push(Some(Box::new(terminal)));
        
        // Automatically create a default tab for the new terminal
        let title_str = core::str::from_utf8(title).unwrap_or("Terminal");
        self.tab_manager.create_tab(title_str, id).ok();
        
        Ok(id)
    }
    
    fn close_terminal(&mut self, id: TerminalID) -> Result<(), TerminalError> {
        // Close all tabs associated with this terminal
        let tabs_to_close: Vec<TabID> = self.tab_manager.get_tabs()
            .iter()
            .filter(|t| t.terminal_id == id)
            .map(|t| t.id)
            .collect();
        
        for tab_id in tabs_to_close {
            self.tab_manager.close_tab(tab_id).ok();
        }

        for terminal_option in &mut self.terminals {
            if let Some(ref terminal) = *terminal_option {
                let term_ref: &dyn Terminal = terminal.as_ref();
                if term_ref.id() == id {
                    *terminal_option = None;
                    return Ok(());
                }
            }
        }
        Err(TerminalError::NotFound)
    }
    
    fn get_terminal(&self, id: TerminalID) -> Option<&dyn Terminal> {
        for terminal_option in &self.terminals {
            if let Some(ref terminal) = *terminal_option {
                let term_ref: &dyn Terminal = terminal.as_ref();
                if term_ref.id() == id { return Some(term_ref); }
            }
        }
        None
    }
    
    fn execute_command(&mut self, terminal_id: TerminalID, command: &[u8]) -> Result<Vec<u8>, TerminalError> {
        if self.get_terminal(terminal_id).is_some() {
            let mut output = Vec::new();
            for &byte in command {
                output.push(byte);
            }
            output.push(b'\n');
            Ok(output)
        } else {
            Err(TerminalError::NotFound)
        }
    }

    /// Create a split terminal pane vertically or horizontally (SerenityOS Terminal Parity)
    pub fn create_split_terminal(&mut self, parent_tab_id: TabID, is_vertical: bool) -> Result<TabID, TerminalError> {
        let title = b"Split Terminal";
        let new_term_id = self.create_terminal(title)?;

        let direction = if is_vertical {
            SplitDirection::Vertical
        } else {
            SplitDirection::Horizontal
        };

        if let Some(parent_tab) = self.tab_manager.tabs.iter_mut().find(|t| t.id == parent_tab_id) {
            let new_tab_id = self.tab_manager.tabs.last().map(|t| t.id).unwrap_or(0);
            parent_tab.split_tab(direction, 0.5, new_tab_id);
            Ok(new_tab_id)
        } else {
            Err(TerminalError::TabNotFound)
        }
    }
}

pub trait ShellIntegration {
    fn get_shell(&self) -> &[u8];
    fn set_shell(&mut self, shell: &[u8]);
    fn get_env_var(&self, key: &[u8]) -> Option<&[u8]>;
    fn set_env_var(&mut self, key: &[u8], value: &[u8]);
}

#[repr(C)]
pub struct SimpleShellIntegration {
    pub shell: [u8; 64],
    pub env_vars: Vec<([u8; 64], [u8; 256])>,
}

impl SimpleShellIntegration {
    pub fn new() -> Self {
        let mut shell_array = [0u8; 64];
        let shell_len = b"/bin/bash".len().min(63);
        for i in 0..shell_len {
            shell_array[i] = b"/bin/bash"[i];
        }
        SimpleShellIntegration {
            shell: shell_array,
            env_vars: Vec::new(),
        }
    }
}

impl ShellIntegration for SimpleShellIntegration {
    fn get_shell(&self) -> &[u8] {
        let len = self.shell.iter().position(|&b| b == 0).unwrap_or(64);
        &self.shell[..len]
    }
    
    fn set_shell(&mut self, shell: &[u8]) {
        let shell_len = shell.len().min(63);
        for i in 0..shell_len {
            self.shell[i] = shell[i];
        }
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
        for i in 0..key_len { key_array[i] = key[i]; }
        for i in 0..value_len { value_array[i] = value[i]; }
        self.env_vars.push((key_array, value_array));
    }
}

// ==============================================================================
// 1. ANSI Escape Code Interpreter (SGR color and attribute parser)
// ==============================================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextAttribute {
    pub fg_color: u8, // ANSI 8-color model (e.g. 31=Red, 32=Green)
    pub bg_color: u8, // ANSI 8-color model (e.g. 40=Black, 41=Red)
    pub is_bold: bool,
    pub is_blinking: bool,
}

pub struct AnsiEscapeInterpreter {
    pub active_attr: TextAttribute,
}

impl AnsiEscapeInterpreter {
    pub fn new() -> Self {
        Self {
            active_attr: TextAttribute {
                fg_color: 37, // White
                bg_color: 40, // Black
                is_bold: false,
                is_blinking: false,
            },
        }
    }

    pub fn parse_escape_sequence(&mut self, code: &[u8]) -> bool {
        // Parses SGR codes (Select Graphic Rendition) e.g., "\x1b[31;1m" (Bold Red)
        if code.len() >= 3 && code[0] == b'\x1b' && code[1] == b'[' {
            let last_byte = code[code.len() - 1];
            if last_byte == b'm' {
                // Simplistic parser for common ANSI colors
                if code.contains(&b'1') {
                    self.active_attr.is_bold = true;
                }
                if code.contains(&b'0') {
                    self.active_attr.is_bold = false;
                    self.active_attr.is_blinking = false;
                }
                if code.contains(&b'5') {
                    self.active_attr.is_blinking = true;
                }
                // Foregrounds
                if code.contains(&b'3') && code.contains(&b'1') { self.active_attr.fg_color = 31; } // Red
                if code.contains(&b'3') && code.contains(&b'2') { self.active_attr.fg_color = 32; } // Green
                return true;
            }
        }
        false
    }
}

impl Default for AnsiEscapeInterpreter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serenity_terminal_tab_management() {
        let mut manager = SimpleTerminalManager::new();
        let term_id = manager.create_terminal(b"Development").unwrap();
        let active_tab = manager.tab_manager.get_active_tab().unwrap();
        let tab_id = active_tab.id;

        // Split terminal vertically
        let split_tab_id = manager.create_split_terminal(tab_id, true).unwrap();
        assert!(split_tab_id > 0);

        // Tab group management
        let group_id = manager.tab_manager.create_group("DevGroup");
        assert!(manager.tab_manager.add_tab_to_group(tab_id, group_id).is_ok());

        // Tab statistics
        let stats = manager.tab_manager.get_tab_stats(tab_id).unwrap();
        assert_eq!(stats.id, tab_id);
    }
}

// ==============================================================================
// 2. Scrollback Buffer and History Grid
// ==============================================================================
#[derive(Clone, Copy)]
pub struct TerminalCell {
    pub glyph: char,
    pub attribute: TextAttribute,
}

pub struct ScrollbackGrid {
    pub lines: Vec<Vec<TerminalCell>>,
    pub cursor_row: usize,
    pub cursor_col: usize,
    pub max_scrollback_lines: usize,
}

impl ScrollbackGrid {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            cursor_row: 0,
            cursor_col: 0,
            max_scrollback_lines: 1000,
        }
    }

    pub fn write_character(&mut self, ch: char, attr: TextAttribute) {
        if self.lines.is_empty() {
            self.lines.push(Vec::new());
        }
        let row = self.lines.len() - 1;
        self.lines[row].push(TerminalCell { glyph: ch, attribute: attr });
        self.cursor_col += 1;

        if ch == '\n' {
            self.lines.push(Vec::new());
            self.cursor_row += 1;
            self.cursor_col = 0;
        }

        // Limit scrollback history
        if self.lines.len() > self.max_scrollback_lines {
            self.lines.remove(0);
            if self.cursor_row > 0 {
                self.cursor_row -= 1;
            }
        }
    }
}

impl Default for ScrollbackGrid {
    fn default() -> Self {
        Self::new()
    }
}

// ==============================================================================
// 3. PTY (Pseudo-Terminal) Session Pair
// ==============================================================================
pub struct PtySessionPair {
    pub master_fd: i32,
    pub slave_fd: i32,
    pub shell_path: [u8; 64],
}

impl PtySessionPair {
    pub fn new(master: i32, slave: i32) -> Self {
        let mut shell_arr = [0u8; 64];
        let path = b"/bin/sigma-shell";
        shell_arr[..path.len()].copy_from_slice(path);
        Self {
            master_fd: master,
            slave_fd: slave,
            shell_path: shell_arr,
        }
    }
}

// ==============================================================================
// 4. UTF-8 Multi-byte Character Decoder
// ==============================================================================
pub struct Utf8Decoder {
    pub expected_bytes: usize,
    pub bytes_collected: Vec<u8>,
}

impl Utf8Decoder {
    pub fn new() -> Self {
        Self {
            expected_bytes: 0,
            bytes_collected: Vec::new(),
        }
    }

    pub fn decode_byte(&mut self, b: u8) -> Option<char> {
        if self.expected_bytes == 0 {
            if b & 0x80 == 0 {
                return Some(b as char); // Standard 1-byte ASCII
            } else if b & 0xE0 == 0xC0 {
                self.expected_bytes = 2;
                self.bytes_collected.push(b);
            } else if b & 0xF0 == 0xE0 {
                self.expected_bytes = 3;
                self.bytes_collected.push(b);
            } else if b & 0xF8 == 0xF0 {
                self.expected_bytes = 4;
                self.bytes_collected.push(b);
            }
        } else {
            self.bytes_collected.push(b);
            if self.bytes_collected.len() == self.expected_bytes {
                // Decode multi-byte into char
                let ch = match self.expected_bytes {
                    2 => {
                        let c = (((self.bytes_collected[0] & 0x1F) as u32) << 6) | ((self.bytes_collected[1] & 0x3F) as u32);
                        core::char::from_u32(c)
                    }
                    3 => {
                        let c = (((self.bytes_collected[0] & 0x0F) as u32) << 12) | (((self.bytes_collected[1] & 0x3F) as u32) << 6) | ((self.bytes_collected[2] & 0x3F) as u32);
                        core::char::from_u32(c)
                    }
                    _ => Some('?'),
                };
                self.expected_bytes = 0;
                self.bytes_collected.clear();
                return ch;
            }
        }
        None
    }
}

impl Default for Utf8Decoder {
    fn default() -> Self {
        Self::new()
    }
}

