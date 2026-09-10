//! Omarchy-inspired Command Palette System
//! 
//! This module implements a command palette system inspired by Omarchy Linux,
//! which provides a filterable, nested command palette defined in JSONC for
//! system control and configuration.

#![allow(dead_code)]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Command action type
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CommandActionType {
    /// Execute a shell command
    Shell,
    /// Open a file or application
    Open,
    /// Toggle a setting
    Toggle,
    /// Navigate to a location
    Navigate,
    /// Custom action
    Custom,
}

/// Command palette item
#[derive(Debug, Clone)]
pub struct CommandPaletteItem {
    /// Command ID
    pub id: String,
    /// Command label
    pub label: String,
    /// Command description
    pub description: String,
    /// Command action type
    pub action_type: CommandActionType,
    /// Command to execute
    pub command: String,
    /// Category for grouping
    pub category: String,
    /// Keyboard shortcut (optional)
    pub shortcut: Option<String>,
    /// Icon name (optional)
    pub icon: Option<String>,
    /// Enabled state
    pub enabled: bool,
}

/// Command palette - manages system commands
#[derive(Debug)]
pub struct OmarchyCommandPalette {
    /// Available commands
    pub commands: Vec<CommandPaletteItem>,
    /// Command categories
    pub categories: Vec<String>,
    /// Search index (label -> command indices)
    search_index: BTreeMap<String, Vec<usize>>,
    /// Currently selected command
    pub selected_index: Option<usize>,
}

impl OmarchyCommandPalette {
    /// Create a new Command Palette
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            categories: Vec::new(),
            search_index: BTreeMap::new(),
            selected_index: None,
        }
    }
    
    /// Add a command
    pub fn add_command(&mut self, command: CommandPaletteItem) {
        let idx = self.commands.len();
        self.commands.push(command.clone());
        
        // Update search index
        let label_lower = command.label.to_lowercase();
        self.search_index
            .entry(label_lower)
            .or_insert_with(Vec::new)
            .push(idx);
        
        // Update categories
        if !self.categories.contains(&command.category) {
            self.categories.push(command.category.clone());
        }
    }
    
    /// Remove a command by ID
    pub fn remove_command(&mut self, command_id: &str) {
        if let Some(idx) = self.commands.iter().position(|c| c.id == command_id) {
            self.commands.remove(idx);
            // Rebuild search index
            self.rebuild_search_index();
        }
    }
    
    /// Search commands by query
    pub fn search(&self, query: &str) -> Vec<&CommandPaletteItem> {
        let query_lower = query.to_lowercase();
        if query.is_empty() {
            return self.commands.iter().collect();
        }
        
        self.commands
            .iter()
            .filter(|cmd| {
                cmd.label.to_lowercase().contains(&query_lower)
                    || cmd.description.to_lowercase().contains(&query_lower)
                    || cmd.category.to_lowercase().contains(&query_lower)
            })
            .collect()
    }
    
    /// Get commands by category
    pub fn get_commands_by_category(&self, category: &str) -> Vec<&CommandPaletteItem> {
        self.commands
            .iter()
            .filter(|cmd| cmd.category == category)
            .collect()
    }
    
    /// Execute selected command
    pub fn execute_selected(&self) -> Result<String, String> {
        if let Some(idx) = self.selected_index {
            if let Some(cmd) = self.commands.get(idx) {
                if cmd.enabled {
                    Ok(cmd.command.clone())
                } else {
                    Err("Command is disabled".to_string())
                }
            } else {
                Err("Invalid command index".to_string())
            }
        } else {
            Err("No command selected".to_string())
        }
    }
    
    /// Select command by index
    pub fn select_command(&mut self, index: usize) -> Result<(), String> {
        if index < self.commands.len() {
            self.selected_index = Some(index);
            Ok(())
        } else {
            Err("Invalid command index".to_string())
        }
    }
    
    /// Select next command
    pub fn select_next(&mut self) {
        let total = self.commands.len();
        if total == 0 {
            return;
        }
        
        let current = self.selected_index.unwrap_or(0);
        self.selected_index = Some((current + 1) % total);
    }
    
    /// Select previous command
    pub fn select_previous(&mut self) {
        let total = self.commands.len();
        if total == 0 {
            return;
        }
        
        let current = self.selected_index.unwrap_or(0);
        self.selected_index = Some(if current == 0 { total - 1 } else { current - 1 });
    }
    
    /// Get selected command
    pub fn get_selected(&self) -> Option<&CommandPaletteItem> {
        self.selected_index.and_then(|idx| self.commands.get(idx))
    }
    
    /// Enable/disable a command
    pub fn set_command_enabled(&mut self, command_id: &str, enabled: bool) {
        if let Some(cmd) = self.commands.iter_mut().find(|c| c.id == command_id) {
            cmd.enabled = enabled;
        }
    }
    
    /// Rebuild search index
    fn rebuild_search_index(&mut self) {
        self.search_index.clear();
        for (idx, cmd) in self.commands.iter().enumerate() {
            let label_lower = cmd.label.to_lowercase();
            self.search_index
                .entry(label_lower)
                .or_insert_with(Vec::new)
                .push(idx);
        }
    }
    
    /// Create default commands
    pub fn create_default_commands(&mut self) {
        // System commands
        self.add_command(CommandPaletteItem {
            id: "system-update".to_string(),
            label: "System Update".to_string(),
            description: "Update system packages".to_string(),
            action_type: CommandActionType::Shell,
            command: "sigma-update".to_string(),
            category: "System".to_string(),
            shortcut: Some("Ctrl+U".to_string()),
            icon: Some("update".to_string()),
            enabled: true,
        });
        
        self.add_command(CommandPaletteItem {
            id: "system-shutdown".to_string(),
            label: "Shutdown".to_string(),
            description: "Shutdown the system".to_string(),
            action_type: CommandActionType::Shell,
            command: "shutdown -h now".to_string(),
            category: "System".to_string(),
            shortcut: None,
            icon: Some("power-off".to_string()),
            enabled: true,
        });
        
        // Application commands
        self.add_command(CommandPaletteItem {
            id: "app-terminal".to_string(),
            label: "Open Terminal".to_string(),
            description: "Open a new terminal window".to_string(),
            action_type: CommandActionType::Open,
            command: "sigma-terminal".to_string(),
            category: "Applications".to_string(),
            shortcut: Some("Ctrl+Alt+T".to_string()),
            icon: Some("terminal".to_string()),
            enabled: true,
        });
        
        // Settings commands
        self.add_command(CommandPaletteItem {
            id: "settings-theme".to_string(),
            label: "Change Theme".to_string(),
            description: "Open theme switcher".to_string(),
            action_type: CommandActionType::Open,
            command: "sigma-theme-switcher".to_string(),
            category: "Settings".to_string(),
            shortcut: Some("Super+Shift+Ctrl+Space".to_string()),
            icon: Some("palette".to_string()),
            enabled: true,
        });
        
        self.add_command(CommandPaletteItem {
            id: "settings-display".to_string(),
            label: "Display Settings".to_string(),
            description: "Configure display and monitors".to_string(),
            action_type: CommandActionType::Open,
            command: "sigma-display-settings".to_string(),
            category: "Settings".to_string(),
            shortcut: None,
            icon: Some("display".to_string()),
            enabled: true,
        });
    }
}

impl Default for OmarchyCommandPalette {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_command_palette_creation() {
        let palette = OmarchyCommandPalette::new();
        assert_eq!(palette.commands.len(), 0);
    }
    
    #[test]
    fn test_add_command() {
        let mut palette = OmarchyCommandPalette::new();
        let command = CommandPaletteItem {
            id: "test".to_string(),
            label: "Test".to_string(),
            description: "Test command".to_string(),
            action_type: CommandActionType::Shell,
            command: "echo test".to_string(),
            category: "Test".to_string(),
            shortcut: None,
            icon: None,
            enabled: true,
        };
        
        palette.add_command(command);
        assert_eq!(palette.commands.len(), 1);
    }
    
    #[test]
    fn test_search() {
        let mut palette = OmarchyCommandPalette::new();
        palette.create_default_commands();
        
        let results = palette.search("system");
        assert!(results.len() > 0);
    }
    
    #[test]
    fn test_select_navigation() {
        let mut palette = OmarchyCommandPalette::new();
        palette.create_default_commands();
        
        palette.select_next();
        assert!(palette.selected_index.is_some());
        
        let current = palette.selected_index.unwrap();
        palette.select_next();
        assert_ne!(palette.selected_index.unwrap(), current);
    }
    
    #[test]
    fn test_execute_selected() {
        let mut palette = OmarchyCommandPalette::new();
        palette.create_default_commands();
        
        palette.select_command(0).unwrap();
        let result = palette.execute_selected();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_category_filtering() {
        let mut palette = OmarchyCommandPalette::new();
        palette.create_default_commands();
        
        let system_commands = palette.get_commands_by_category("System");
        assert!(system_commands.len() > 0);
    }
}
