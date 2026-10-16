// SPDX-License-Identifier: MIT
// SigmaOS - Wlogout Logout Screen
// Inspired by Wlogout (https://github.com/ArtsyMecha/wlogout)
// Provides logout screen for Wayland

#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};
use core::sync::atomic::{AtomicBool, Ordering};

/// Logout action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogoutAction {
    Lock,
    Logout,
    Suspend,
    Hibernate,
    Reboot,
    Shutdown,
    Cancel,
}

/// Wlogout instance
pub struct WlogoutInstance {
    pub visible: AtomicBool,
    pub current_action: LogoutAction,
}

impl WlogoutInstance {
    pub fn new() -> Self {
        Self {
            visible: AtomicBool::new(false),
            current_action: LogoutAction::Cancel,
        }
    }

    /// Show logout screen
    pub fn show(&self) {
        self.visible.store(true, Ordering::Relaxed);
    }

    /// Hide logout screen
    pub fn hide(&self) {
        self.visible.store(false, Ordering::Relaxed);
    }

    /// Is visible
    pub fn is_visible(&self) -> bool {
        self.visible.load(Ordering::Relaxed)
    }

    /// Select action
    pub fn select_action(&mut self, action: LogoutAction) {
        self.current_action = action;
    }

    /// Confirm action
    pub fn confirm_action(&self) -> Result<(), &'static str> {
        match self.current_action {
            LogoutAction::Lock => {
                // Lock screen
                Ok(())
            }
            LogoutAction::Logout => {
                // Logout user
                Ok(())
            }
            LogoutAction::Suspend => {
                // Suspend system
                Ok(())
            }
            LogoutAction::Hibernate => {
                // Hibernate system
                Ok(())
            }
            LogoutAction::Reboot => {
                // Reboot system
                Ok(())
            }
            LogoutAction::Shutdown => {
                // Shutdown system
                Ok(())
            }
            LogoutAction::Cancel => Ok(()),
        }
    }

    /// Get current action
    pub fn get_current_action(&self) -> LogoutAction {
        self.current_action
    }
}

impl Default for WlogoutInstance {
    fn default() -> Self {
        Self::new()
    }
}

/// Global Wlogout instance
static mut WLOGOUT_INSTANCE: Option<WlogoutInstance> = None;

/// Initialize Wlogout
pub fn init_wlogout() {
    unsafe {
        WLOGOUT_INSTANCE = Some(WlogoutInstance::new());
    }
}

/// Get global Wlogout instance
pub fn get_wlogout() -> Option<&'static mut WlogoutInstance> {
    unsafe { WLOGOUT_INSTANCE.as_mut() }
}

/// Show logout screen
pub fn show_wlogout() {
    if let Some(wlogout) = get_wlogout() {
        wlogout.show();
    }
}

/// Hide logout screen
pub fn hide_wlogout() {
    if let Some(wlogout) = get_wlogout() {
        wlogout.hide();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wlogout_init() {
        let wlogout = WlogoutInstance::new();
        assert!(!wlogout.is_visible());
    }

    #[test]
    fn test_show_hide() {
        let wlogout = WlogoutInstance::new();
        wlogout.show();
        assert!(wlogout.is_visible());
        wlogout.hide();
        assert!(!wlogout.is_visible());
    }

    #[test]
    fn test_select_action() {
        let mut wlogout = WlogoutInstance::new();
        wlogout.select_action(LogoutAction::Shutdown);
        assert_eq!(wlogout.get_current_action(), LogoutAction::Shutdown);
    }

    #[test]
    fn test_confirm_action() {
        let mut wlogout = WlogoutInstance::new();
        wlogout.select_action(LogoutAction::Lock);
        assert!(wlogout.confirm_action().is_ok());
    }
}
