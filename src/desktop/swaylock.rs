// SPDX-License-Identifier: MIT
// SigmaOS - Swaylock Screen Locker
// Inspired by Swaylock (https://github.com/swaywm/swaylock)
// Provides screen locker for Wayland

#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};
use core::sync::atomic::{AtomicBool, Ordering};

/// Lock state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockState {
    Unlocked,
    Locking,
    Locked,
    Unlocking,
}

/// Swaylock instance
pub struct SwaylockInstance {
    pub locked: AtomicBool,
    pub state: LockState,
    pub password: String,
    pub attempts: u32,
    pub max_attempts: u32,
}

impl SwaylockInstance {
    pub fn new() -> Self {
        Self {
            locked: AtomicBool::new(false),
            state: LockState::Unlocked,
            password: String::new(),
            attempts: 0,
            max_attempts: 3,
        }
    }

    /// Lock screen
    pub fn lock(&mut self) {
        self.state = LockState::Locking;
        self.locked.store(true, Ordering::Relaxed);
        self.state = LockState::Locked;
    }

    /// Unlock screen
    pub fn unlock(&mut self, password: &str) -> Result<(), &'static str> {
        if !self.locked.load(Ordering::Relaxed) {
            return Err("Screen not locked");
        }

        self.state = LockState::Unlocking;

        if password == self.password {
            self.locked.store(false, Ordering::Relaxed);
            self.state = LockState::Unlocked;
            self.attempts = 0;
            Ok(())
        } else {
            self.attempts += 1;
            self.state = LockState::Locked;

            if self.attempts >= self.max_attempts {
                // In real implementation, would trigger security measures
                return Err("Too many failed attempts");
            }

            Err("Incorrect password")
        }
    }

    /// Set password
    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }

    /// Is locked
    pub fn is_locked(&self) -> bool {
        self.locked.load(Ordering::Relaxed)
    }

    /// Get state
    pub fn get_state(&self) -> LockState {
        self.state
    }

    /// Get remaining attempts
    pub fn get_remaining_attempts(&self) -> u32 {
        self.max_attempts - self.attempts
    }
}

impl Default for SwaylockInstance {
    fn default() -> Self {
        Self::new()
    }
}

/// Global swaylock instance
static mut SWAYLOCK_INSTANCE: Option<SwaylockInstance> = None;

/// Initialize swaylock
pub fn init_swaylock() {
    unsafe {
        SWAYLOCK_INSTANCE = Some(SwaylockInstance::new());
    }
}

/// Get global swaylock instance
pub fn get_swaylock() -> Option<&'static mut SwaylockInstance> {
    unsafe { SWAYLOCK_INSTANCE.as_mut() }
}

/// Lock screen
pub fn lock_screen() {
    if let Some(swaylock) = get_swaylock() {
        swaylock.lock();
    }
}

/// Unlock screen
pub fn unlock_screen(password: &str) -> Result<(), &'static str> {
    if let Some(swaylock) = get_swaylock() {
        swaylock.unlock(password)
    } else {
        Err("Swaylock not initialized")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swaylock_init() {
        let swaylock = SwaylockInstance::new();
        assert!(!swaylock.is_locked());
    }

    #[test]
    fn test_lock() {
        let mut swaylock = SwaylockInstance::new();
        swaylock.lock();
        assert!(swaylock.is_locked());
        assert_eq!(swaylock.get_state(), LockState::Locked);
    }

    #[test]
    fn test_unlock() {
        let mut swaylock = SwaylockInstance::new();
        swaylock.set_password("password".to_string());
        swaylock.lock();
        swaylock.unlock("password").unwrap();
        assert!(!swaylock.is_locked());
    }

    #[test]
    fn test_unlock_wrong_password() {
        let mut swaylock = SwaylockInstance::new();
        swaylock.set_password("password".to_string());
        swaylock.lock();
        assert!(swaylock.unlock("wrong").is_err());
        assert!(swaylock.is_locked());
    }

    #[test]
    fn test_max_attempts() {
        let mut swaylock = SwaylockInstance::new();
        swaylock.set_password("password".to_string());
        swaylock.lock();
        swaylock.unlock("wrong").unwrap_err();
        swaylock.unlock("wrong").unwrap_err();
        swaylock.unlock("wrong").unwrap_err();
        assert!(swaylock.unlock("wrong").is_err()); // Too many attempts
    }
}
