//! Screen Saver Manager
//!
//! Screen saver management inspired by Linux Mint's screen saver and Omarchy's
//! screen utilities, supporting screen locking, idle detection, and customization.

use std::time::{SystemTime, UNIX_EPOCH};

/// Screen saver mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopScreenSaverMode {
    Disabled,
    Blank,
    Photos,
    Clock,
    Matrix,
}

impl DesktopScreenSaverMode {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "disabled" => Some(DesktopScreenSaverMode::Disabled),
            "blank" => Some(DesktopScreenSaverMode::Blank),
            "photos" => Some(DesktopScreenSaverMode::Photos),
            "clock" => Some(DesktopScreenSaverMode::Clock),
            "matrix" => Some(DesktopScreenSaverMode::Matrix),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            DesktopScreenSaverMode::Disabled => "Disabled",
            DesktopScreenSaverMode::Blank => "Blank",
            DesktopScreenSaverMode::Photos => "Photos",
            DesktopScreenSaverMode::Clock => "Clock",
            DesktopScreenSaverMode::Matrix => "Matrix",
        }
    }
}

/// Lock on sleep behavior
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopLockOnSleep {
    Never,
    WhenSuspended,
    WhenScreenSaver,
}

impl DesktopLockOnSleep {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "never" => Some(DesktopLockOnSleep::Never),
            "whensuspended" => Some(DesktopLockOnSleep::WhenSuspended),
            "whenscreensaver" => Some(DesktopLockOnSleep::WhenScreenSaver),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            DesktopLockOnSleep::Never => "Never",
            DesktopLockOnSleep::WhenSuspended => "When Suspended",
            DesktopLockOnSleep::WhenScreenSaver => "When Screen Saver",
        }
    }
}

/// Screen saver manager
#[derive(Debug)]
pub struct DesktopScreenSaverManager {
    mode: DesktopScreenSaverMode,
    idle_timeout_seconds: u32,
    lock_on_sleep: DesktopLockOnSleep,
    is_locked: bool,
    last_activity: u64,
}

impl DesktopScreenSaverManager {
    pub fn new() -> Self {
        Self {
            mode: DesktopScreenSaverMode::Blank,
            idle_timeout_seconds: 300, // 5 minutes
            lock_on_sleep: DesktopLockOnSleep::WhenScreenSaver,
            is_locked: false,
            last_activity: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    /// Get mode
    pub fn get_mode(&self) -> DesktopScreenSaverMode {
        self.mode
    }

    /// Set mode
    pub fn set_mode(&mut self, mode: DesktopScreenSaverMode) {
        self.mode = mode;
    }

    /// Get idle timeout
    pub fn get_idle_timeout(&self) -> u32 {
        self.idle_timeout_seconds
    }

    /// Set idle timeout
    pub fn set_idle_timeout(&mut self, timeout_seconds: u32) {
        self.idle_timeout_seconds = timeout_seconds;
    }

    /// Get lock on sleep setting
    pub fn get_lock_on_sleep(&self) -> DesktopLockOnSleep {
        self.lock_on_sleep
    }

    /// Set lock on sleep
    pub fn set_lock_on_sleep(&mut self, setting: DesktopLockOnSleep) {
        self.lock_on_sleep = setting;
    }

    /// Check if locked
    pub fn is_locked(&self) -> bool {
        self.is_locked
    }

    /// Lock screen
    pub fn lock(&mut self) {
        self.is_locked = true;
    }

    /// Unlock screen
    pub fn unlock(&mut self) {
        self.is_locked = false;
    }

    /// Update activity
    pub fn update_activity(&mut self) {
        self.last_activity = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    /// Check if screen saver should activate
    pub fn should_activate(&self) -> bool {
        if self.mode == DesktopScreenSaverMode::Disabled {
            return false;
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let idle_time = now - self.last_activity;
        idle_time >= self.idle_timeout_seconds as u64
    }

    /// Check if should lock
    pub fn should_lock(&self) -> bool {
        match self.lock_on_sleep {
            DesktopLockOnSleep::Never => false,
            DesktopLockOnSleep::WhenSuspended => false, // Would check suspend state
            DesktopLockOnSleep::WhenScreenSaver => self.should_activate(),
        }
    }

    /// Get idle time
    pub fn get_idle_time(&self) -> u64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        now - self.last_activity
    }

    /// Get statistics
    pub fn get_statistics(&self) -> DesktopScreenSaverStatistics {
        DesktopScreenSaverStatistics {
            mode: self.mode,
            idle_timeout: self.idle_timeout_seconds,
            lock_on_sleep: self.lock_on_sleep,
            is_locked: self.is_locked,
            idle_time: self.get_idle_time(),
        }
    }
}

impl Default for DesktopScreenSaverManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Screen saver statistics
#[derive(Debug, Clone)]
pub struct DesktopScreenSaverStatistics {
    pub mode: DesktopScreenSaverMode,
    pub idle_timeout: u32,
    pub lock_on_sleep: DesktopLockOnSleep,
    pub is_locked: bool,
    pub idle_time: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screen_saver_mode_from_str() {
        assert_eq!(DesktopScreenSaverMode::from_str("blank"), Some(DesktopScreenSaverMode::Blank));
        assert_eq!(DesktopScreenSaverMode::from_str("photos"), Some(DesktopScreenSaverMode::Photos));
    }

    #[test]
    fn test_lock_on_sleep_from_str() {
        assert_eq!(DesktopLockOnSleep::from_str("never"), Some(DesktopLockOnSleep::Never));
        assert_eq!(DesktopLockOnSleep::from_str("whenscreensaver"), Some(DesktopLockOnSleep::WhenScreenSaver));
    }

    #[test]
    fn test_screen_saver_manager_creation() {
        let manager = DesktopScreenSaverManager::new();
        assert_eq!(manager.get_mode(), DesktopScreenSaverMode::Blank);
    }

    #[test]
    fn test_set_mode() {
        let mut manager = DesktopScreenSaverManager::new();
        manager.set_mode(DesktopScreenSaverMode::Photos);
        assert_eq!(manager.get_mode(), DesktopScreenSaverMode::Photos);
    }

    #[test]
    fn test_lock_unlock() {
        let mut manager = DesktopScreenSaverManager::new();
        manager.lock();
        assert!(manager.is_locked());
        manager.unlock();
        assert!(!manager.is_locked());
    }

    #[test]
    fn test_update_activity() {
        let mut manager = DesktopScreenSaverManager::new();
        let before = manager.get_idle_time();
        manager.update_activity();
        let after = manager.get_idle_time();
        assert!(after <= before);
        assert_eq!(after, 0);
    }

    #[test]
    fn test_should_activate() {
        let mut manager = DesktopScreenSaverManager::new();
        manager.set_idle_timeout(1);
        manager.update_activity();
        assert!(!manager.should_activate());
        // Would need to wait to test actual activation
    }

    #[test]
    fn test_disabled_mode() {
        let mut manager = DesktopScreenSaverManager::new();
        manager.set_mode(DesktopScreenSaverMode::Disabled);
        assert!(!manager.should_activate());
    }

    #[test]
    fn test_statistics() {
        let manager = DesktopScreenSaverManager::new();
        let stats = manager.get_statistics();
        assert_eq!(stats.mode, DesktopScreenSaverMode::Blank);
    }
}
