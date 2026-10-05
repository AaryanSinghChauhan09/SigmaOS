// SigmaOS Desktop Screen Lock Manager
// Inspired by Linux Mint's screen lock and Omarchy's lock screen utilities

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Lock screen type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockScreenType {
    Password,
    PIN,
    Pattern,
    Biometric,
}

impl LockScreenType {
    pub fn as_str(&self) -> &'static str {
        match self {
            LockScreenType::Password => "Password",
            LockScreenType::PIN => "PIN",
            LockScreenType::Pattern => "Pattern",
            LockScreenType::Biometric => "Biometric",
        }
    }
}

/// Lock status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockStatus {
    Unlocked,
    Locked,
    Unlocking,
}

impl LockStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            LockStatus::Unlocked => "Unlocked",
            LockStatus::Locked => "Locked",
            LockStatus::Unlocking => "Unlocking",
        }
    }
}

/// Screen lock configuration
#[derive(Debug, Clone)]
pub struct ScreenLockConfig {
    pub lock_type: LockScreenType,
    pub auto_lock_enabled: bool,
    pub auto_lock_timeout: u32, // seconds
    pub lock_on_suspend: bool,
    pub lock_on_lid_close: bool,
    pub show_clock: bool,
    pub show_notifications: bool,
}

impl ScreenLockConfig {
    pub fn new(lock_type: LockScreenType) -> Self {
        ScreenLockConfig {
            lock_type,
            auto_lock_enabled: true,
            auto_lock_timeout: 300, // 5 minutes
            lock_on_suspend: true,
            lock_on_lid_close: true,
            show_clock: true,
            show_notifications: false,
        }
    }

    pub fn set_auto_lock_enabled(&mut self, enabled: bool) {
        self.auto_lock_enabled = enabled;
    }

    pub fn set_auto_lock_timeout(&mut self, timeout: u32) {
        self.auto_lock_timeout = timeout.max(30); // Minimum 30 seconds
    }

    pub fn set_lock_on_suspend(&mut self, lock: bool) {
        self.lock_on_suspend = lock;
    }

    pub fn set_lock_on_lid_close(&mut self, lock: bool) {
        self.lock_on_lid_close = lock;
    }

    pub fn set_show_clock(&mut self, show: bool) {
        self.show_clock = show;
    }

    pub fn set_show_notifications(&mut self, show: bool) {
        self.show_notifications = show;
    }
}

/// Screen Lock Manager
pub struct ScreenLockManager {
    config: ScreenLockConfig,
    status: LockStatus,
    lock_time: Option<u64>,
    failed_attempts: u32,
    max_attempts: u32,
}

impl ScreenLockManager {
    pub fn new() -> Self {
        ScreenLockManager {
            config: ScreenLockConfig::new(LockScreenType::Password),
            status: LockStatus::Unlocked,
            lock_time: None,
            failed_attempts: 0,
            max_attempts: 5,
        }
    }

    pub fn get_config(&self) -> &ScreenLockConfig {
        &self.config
    }

    pub fn set_config(&mut self, config: ScreenLockConfig) {
        self.config = config;
    }

    pub fn get_status(&self) -> LockStatus {
        self.status
    }

    pub fn lock(&mut self) {
        self.status = LockStatus::Locked;
        self.lock_time = Some(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        );
        self.failed_attempts = 0;
    }

    pub fn unlock(&mut self, _credential: &str) -> bool {
        self.status = LockStatus::Unlocking;

        // In production, validate credential here
        let success = true; // Simulated success

        if success {
            self.status = LockStatus::Unlocked;
            self.lock_time = None;
            self.failed_attempts = 0;
            true
        } else {
            self.status = LockStatus::Locked;
            self.failed_attempts += 1;
            false
        }
    }

    pub fn is_locked(&self) -> bool {
        self.status == LockStatus::Locked
    }

    pub fn get_lock_time(&self) -> Option<u64> {
        self.lock_time
    }

    pub fn get_lock_duration(&self) -> Option<u64> {
        if let Some(lock_time) = self.lock_time {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            Some(now.saturating_sub(lock_time))
        } else {
            None
        }
    }

    pub fn get_failed_attempts(&self) -> u32 {
        self.failed_attempts
    }

    pub fn reset_failed_attempts(&mut self) {
        self.failed_attempts = 0;
    }

    pub fn is_max_attempts_reached(&self) -> bool {
        self.failed_attempts >= self.max_attempts
    }

    pub fn set_max_attempts(&mut self, max: u32) {
        self.max_attempts = max.max(1);
    }

    pub fn should_auto_lock(&self, idle_time: u32) -> bool {
        self.config.auto_lock_enabled && idle_time >= self.config.auto_lock_timeout
    }

    pub fn get_statistics(&self) -> ScreenLockStatistics {
        ScreenLockStatistics {
            status: self.status,
            lock_type: self.config.lock_type,
            auto_lock_enabled: self.config.auto_lock_enabled,
            auto_lock_timeout: self.config.auto_lock_timeout,
            failed_attempts: self.failed_attempts,
        }
    }
}

impl Default for ScreenLockManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Screen Lock statistics
#[derive(Debug, Clone, Copy)]
pub struct ScreenLockStatistics {
    pub status: LockStatus,
    pub lock_type: LockScreenType,
    pub auto_lock_enabled: bool,
    pub auto_lock_timeout: u32,
    pub failed_attempts: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screen_lock_manager_initialization() {
        let manager = ScreenLockManager::new();
        assert_eq!(manager.get_status(), LockStatus::Unlocked);
        assert!(manager.get_config().auto_lock_enabled);
    }

    #[test]
    fn test_lock() {
        let mut manager = ScreenLockManager::new();
        manager.lock();
        assert!(manager.is_locked());
        assert!(manager.get_lock_time().is_some());
    }

    #[test]
    fn test_unlock() {
        let mut manager = ScreenLockManager::new();
        manager.lock();
        assert!(manager.unlock("password"));
        assert!(!manager.is_locked());
    }

    #[test]
    fn test_lock_duration() {
        let mut manager = ScreenLockManager::new();
        manager.lock();
        std::thread::sleep(std::time::Duration::from_secs(1));
        let duration = manager.get_lock_duration();
        assert!(duration.is_some());
        assert!(duration.unwrap() >= 1);
    }

    #[test]
    fn test_failed_attempts() {
        let mut manager = ScreenLockManager::new();
        manager.lock();
        manager.unlock("wrong"); // Will fail in production
        assert!(manager.get_failed_attempts() > 0);
    }

    #[test]
    fn test_reset_failed_attempts() {
        let mut manager = ScreenLockManager::new();
        manager.lock();
        manager.unlock("wrong");
        manager.reset_failed_attempts();
        assert_eq!(manager.get_failed_attempts(), 0);
    }

    #[test]
    fn test_should_auto_lock() {
        let manager = ScreenLockManager::new();
        assert!(manager.should_auto_lock(300));
        assert!(!manager.should_auto_lock(299));
    }

    #[test]
    fn test_set_config() {
        let mut manager = ScreenLockManager::new();
        let config = ScreenLockConfig::new(LockScreenType::PIN);
        manager.set_config(config);
        assert_eq!(manager.get_config().lock_type, LockScreenType::PIN);
    }

    #[test]
    fn test_statistics() {
        let manager = ScreenLockManager::new();
        let stats = manager.get_statistics();
        assert_eq!(stats.status, LockStatus::Unlocked);
        assert_eq!(stats.lock_type, LockScreenType::Password);
    }
}
