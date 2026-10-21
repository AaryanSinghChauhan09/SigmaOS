// SPDX-License-Identifier: MIT
// SigmaOS - Command Manager
// Command management inspired by Omarchy command structure and Linux shell systems

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Command type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandType {
    System,
    User,
    Application,
    Utility,
}

/// Command permission
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandPermission {
    User,
    Admin,
    Root,
}

/// Command
#[derive(Debug, Clone)]
pub struct Command {
    pub id: String,
    pub name: String,
    pub command_type: CommandType,
    pub permission: CommandPermission,
    pub path: String,
    pub description: String,
    pub hidden: bool,
}

/// Command manager
pub struct SigmaCommandManager {
    commands: Vec<Command>,
}

impl SigmaCommandManager {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn register_command(&mut self, command: Command) {
        self.commands.push(command);
    }

    pub fn get_commands(&self) -> &[Command] {
        &self.commands
    }

    pub fn get_command(&self, id: &str) -> Option<&Command> {
        self.commands.iter().find(|c| c.id == id)
    }

    pub fn get_command_by_name(&self, name: &str) -> Option<&Command> {
        self.commands.iter().find(|c| c.name == name)
    }

    pub fn get_commands_by_type(&self, command_type: CommandType) -> Vec<&Command> {
        self.commands
            .iter()
            .filter(|c| c.command_type == command_type)
            .collect()
    }

    pub fn get_visible_commands(&self) -> Vec<&Command> {
        self.commands.iter().filter(|c| !c.hidden).collect()
    }

    pub fn unregister_command(&mut self, id: String) -> Result<(), String> {
        if let Some(idx) = self.commands.iter().position(|c| c.id == id) {
            self.commands.remove(idx);
            Ok(())
        } else {
            Err(format!("Command {} not found", id))
        }
    }

    pub fn check_permission(&self, id: &str, required_permission: CommandPermission) -> bool {
        if let Some(cmd) = self.get_command(id) {
            match required_permission {
                CommandPermission::User => true,
                CommandPermission::Admin => {
                    cmd.permission == CommandPermission::Admin
                        || cmd.permission == CommandPermission::Root
                }
                CommandPermission::Root => cmd.permission == CommandPermission::Root,
            }
        } else {
            false
        }
    }
}

impl Default for SigmaCommandManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_manager_creation() {
        let manager = SigmaCommandManager::new();
        assert_eq!(manager.get_commands().len(), 0);
    }

    #[test]
    fn test_register_command() {
        let mut manager = SigmaCommandManager::new();
        let cmd = Command {
            id: "test-cmd".to_string(),
            name: "test".to_string(),
            command_type: CommandType::Utility,
            permission: CommandPermission::User,
            path: "/usr/bin/test".to_string(),
            description: "Test command".to_string(),
            hidden: false,
        };
        manager.register_command(cmd);
        assert_eq!(manager.get_commands().len(), 1);
    }

    #[test]
    fn test_get_command_by_name() {
        let mut manager = SigmaCommandManager::new();
        let cmd = Command {
            id: "test-cmd".to_string(),
            name: "test".to_string(),
            command_type: CommandType::Utility,
            permission: CommandPermission::User,
            path: "/usr/bin/test".to_string(),
            description: "Test command".to_string(),
            hidden: false,
        };
        manager.register_command(cmd);
        assert!(manager.get_command_by_name("test").is_some());
    }

    #[test]
    fn test_visible_commands() {
        let mut manager = SigmaCommandManager::new();
        let cmd1 = Command {
            id: "visible".to_string(),
            name: "visible".to_string(),
            command_type: CommandType::Utility,
            permission: CommandPermission::User,
            path: "/usr/bin/visible".to_string(),
            description: "Visible command".to_string(),
            hidden: false,
        };
        let cmd2 = Command {
            id: "hidden".to_string(),
            name: "hidden".to_string(),
            command_type: CommandType::Utility,
            permission: CommandPermission::User,
            path: "/usr/bin/hidden".to_string(),
            description: "Hidden command".to_string(),
            hidden: true,
        };
        manager.register_command(cmd1);
        manager.register_command(cmd2);
        assert_eq!(manager.get_visible_commands().len(), 1);
    }

    #[test]
    fn test_check_permission() {
        let mut manager = SigmaCommandManager::new();
        let cmd = Command {
            id: "admin-cmd".to_string(),
            name: "admin".to_string(),
            command_type: CommandType::System,
            permission: CommandPermission::Admin,
            path: "/usr/bin/admin".to_string(),
            description: "Admin command".to_string(),
            hidden: false,
        };
        manager.register_command(cmd);
        assert!(manager.check_permission("admin-cmd", CommandPermission::Admin));
        assert!(!manager.check_permission("admin-cmd", CommandPermission::Root));
    }
}
