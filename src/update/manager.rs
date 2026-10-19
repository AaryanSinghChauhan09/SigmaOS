//! SigmaOS Update System Manager
//!
//! Inspired by: Omarchy update (omancy-update, omancy-update-analyze-logs, omancy-update-aur-pkgs, omancy-update-available, omancy-update-confirm, omancy-update-dev, omancy-update-firmware, omancy-update-keyring, omancy-update-lock, omancy-update-mise, omancy-update-orphan-pkgs, omancy-update-pacman-guard, omancy-update-pkg-prune, omancy-update-requires-free-space, omancy-update-restart, omancy-update-stay-awake, omancy-update-system-pkgs, omancy-update-system-pkgs-when-conflicted, omancy-update-time, omancy-update-user-notify)
//! Following principles: SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY,
//! LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING,
//! SECURE_DEFAULTS, FAIL_FAST_SYSTEM, CONTINUOUS_IMPROVEMENT, LONG_TERM_SUSTAINABILITY,
/// VERSION_CONTROL_PRACTICES, RACE_FREE DESIGN, SAFE_CONCURRENCY, DEADLOCK_AVOIDANCE.

/// Update status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateStatus {
    Idle,
    Checking,
    Available,
    Downloading,
    Installing,
    Completed,
    Failed(String),
    Locked,
}

/// Update type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateType {
    SystemPackages,
    AurPackages,
    Firmware,
    Keyring,
    Mise,
    All,
}

/// Update manager
pub struct UpdateManager {
    pub status: UpdateStatus,
    pub available_updates: u32,
    pub last_check: Option<u64>,
    pub last_update: Option<u64>,
    pub update_lock: bool,
    pub stay_awake: bool,
    pub auto_confirm: bool,
}

impl UpdateManager {
    pub fn new() -> Self {
        Self {
            status: UpdateStatus::Idle,
            available_updates: 0,
            last_check: None,
            last_update: None,
            update_lock: false,
            stay_awake: false,
            auto_confirm: false,
        }
    }

    /// Check for updates
    pub fn check_updates(&mut self) -> Result<u32, String> {
        if self.update_lock {
            return Err("Update is locked".to_string());
        }
        self.status = UpdateStatus::Checking;
        let count = 15; // Simulated
        self.available_updates = count;
        self.last_check = Some(now_secs());
        self.status = UpdateStatus::Available;
        Ok(count)
    }

    /// Install updates
    pub fn install_updates(&mut self, update_type: UpdateType) -> Result<(), String> {
        if self.update_lock {
            return Err("Update is locked".to_string());
        }
        if self.available_updates == 0 && !self.auto_confirm {
            return Err("No updates available".to_string());
        }

        self.status = UpdateStatus::Installing;
        self.last_update = Some(now_secs());
        self.available_updates = 0;
        self.status = UpdateStatus::Completed;
        Ok(())
    }

    /// Lock updates
    pub fn lock(&mut self) {
        self.update_lock = true;
    }

    /// Unlock updates
    pub fn unlock(&mut self) {
        self.update_lock = false;
    }

    /// Is locked
    pub fn is_locked(&self) -> bool {
        self.update_lock
    }

    /// Set stay awake
    pub fn set_stay_awake(&mut self, enabled: bool) {
        self.stay_awake = enabled;
    }

    /// Set auto confirm
    pub fn set_auto_confirm(&mut self, enabled: bool) {
        self.auto_confirm = enabled;
    }

    /// Get status
    pub fn get_status(&self) -> UpdateStatus {
        self.status.clone()
    }

    /// Get available updates count
    pub fn get_available_count(&self) -> u32 {
        self.available_updates
    }
}

impl Default for UpdateManager {
    fn default() -> Self {
        Self::new()
    }
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_updates() {
        let mut mgr = UpdateManager::new();
        let count = mgr.check_updates().unwrap();
        assert_eq!(count, 15);
        assert_eq!(mgr.get_status(), UpdateStatus::Available);
        assert_eq!(mgr.get_available_count(), 15);
    }

    #[test]
    fn test_install_updates() {
        let mut mgr = UpdateManager::new();
        mgr.check_updates().unwrap();
        assert!(mgr.install_updates(UpdateType::SystemPackages).is_ok());
        assert_eq!(mgr.get_status(), UpdateStatus::Completed);
        assert_eq!(mgr.get_available_count(), 0);
    }

    #[test]
    fn test_install_locked_fails() {
        let mut mgr = UpdateManager::new();
        mgr.lock();
        assert!(mgr.check_updates().is_err());
    }

    #[test]
    fn test_lock_unlock() {
        let mut mgr = UpdateManager::new();
        assert!(!mgr.is_locked());
        mgr.lock();
        assert!(mgr.is_locked());
        mgr.unlock();
        assert!(!mgr.is_locked());
    }

    #[test]
    fn test_stay_awake() {
        let mut mgr = UpdateManager::new();
        assert!(!mgr.stay_awake);
        mgr.set_stay_awake(true);
        assert!(mgr.stay_awake);
    }

    #[test]
    fn test_auto_confirm() {
        let mut mgr = UpdateManager::new();
        assert!(!mgr.auto_confirm);
        mgr.set_auto_confirm(true);
        assert!(mgr.auto_confirm);
    }
}