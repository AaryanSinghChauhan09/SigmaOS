// SPDX-License-Identifier: MIT
// SigmaOS - System Tools (inspired by Linux Mint Tools)
// Inspired by Linux Mint Tools (mintupdate, mintinstall, mintstick, etc.)
// Provides system utility tools

#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};
use core::sync::atomic::{AtomicBool, Ordering};

/// System tool
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemTool {
    SoftwareManager,
    UpdateManager,
    BackupTool,
    DriverManager,
    SystemMonitor,
    DiskUtility,
    Terminal,
    FileBrowser,
}

/// System tools manager
pub struct SystemToolsManager {
    pub active: AtomicBool,
}

impl SystemToolsManager {
    pub fn new() -> Self {
        Self {
            active: AtomicBool::new(false),
        }
    }

    /// Initialize system tools
    pub fn init(&mut self) {
        self.active.store(true, Ordering::Relaxed);
    }

    /// Launch tool
    pub fn launch_tool(&self, tool: SystemTool) -> Result<(), &'static str> {
        match tool {
            SystemTool::SoftwareManager => {
                // Launch software manager
                Ok(())
            }
            SystemTool::UpdateManager => {
                // Launch update manager
                Ok(())
            }
            SystemTool::BackupTool => {
                // Launch backup tool
                Ok(())
            }
            SystemTool::DriverManager => {
                // Launch driver manager
                Ok(())
            }
            SystemTool::SystemMonitor => {
                // Launch system monitor
                Ok(())
            }
            SystemTool::DiskUtility => {
                // Launch disk utility
                Ok(())
            }
            SystemTool::Terminal => {
                // Launch terminal
                Ok(())
            }
            SystemTool::FileBrowser => {
                // Launch file browser
                Ok(())
            }
        }
    }

    /// Is active
    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::Relaxed)
    }
}

impl Default for SystemToolsManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global system tools manager
static mut SYSTEM_TOOLS_MANAGER: Option<SystemToolsManager> = None;

/// Initialize system tools manager
pub fn init_system_tools_manager() {
    unsafe {
        SYSTEM_TOOLS_MANAGER = Some(SystemToolsManager::new());
    }
}

/// Get global system tools manager
pub fn get_system_tools_manager() -> Option<&'static mut SystemToolsManager> {
    unsafe { SYSTEM_TOOLS_MANAGER.as_mut() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_tools_manager_init() {
        let mut manager = SystemToolsManager::new();
        manager.init();
        assert!(manager.is_active());
    }

    #[test]
    fn test_launch_tool() {
        let manager = SystemToolsManager::new();
        assert!(manager.launch_tool(SystemTool::SoftwareManager).is_ok());
    }
}
