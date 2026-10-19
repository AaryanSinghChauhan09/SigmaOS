//! SigmaOS System Control Manager
//!
//! Inspired by: Omarchy system (omancy-system-factory-reset, omancy-system-lid-close, omancy-system-lock, omancy-system-logout, omancy-system-reboot, omancy-system-shutdown, omancy-system-sleep-lock, omancy-system-sleep-monitor, omancy-system-stats, omancy-system-wake)
//! Following principles: SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY,
//! LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING,
//! SECURE_DEFAULTS, FAIL_FAST_SYSTEM, LOW_LEVEL_CONTROL, DIRECT_DEVICE_CONTROL,
/// MINIMAL_KERNEL_PHILOSOPHY, POWER_MANAGEMENT, USER_CONTROL.

/// System action
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SystemAction {
    Reboot,
    Shutdown,
    Lock,
    Logout,
    Sleep,
    Wake,
    LidClose,
    FactoryReset,
    Stats,
}

/// System state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SystemState {
    Running,
    Locked,
    Sleeping,
    Rebooting,
    ShuttingDown,
}

/// System stats
#[derive(Debug, Clone)]
pub struct SystemStats {
    pub uptime_seconds: u64,
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub disk_usage: f32,
    pub temperature: Option<f32>,
    pub battery_percent: Option<u32>,
}

/// System control manager
pub struct SystemControlManager {
    pub state: SystemState,
    pub stats: SystemStats,
    pub lock_enabled: bool,
    pub factory_reset_confirmed: bool,
}

impl SystemControlManager {
    pub fn new() -> Self {
        Self {
            state: SystemState::Running,
            stats: SystemStats {
                uptime_seconds: 0,
                cpu_usage: 0.0,
                memory_usage: 0.0,
                disk_usage: 0.0,
                temperature: None,
                battery_percent: None,
            },
            lock_enabled: false,
            factory_reset_confirmed: false,
        }
    }

    /// Lock the system
    pub fn lock(&mut self) -> Result<(), String> {
        if self.state == SystemState::Locked {
            return Err("System already locked".to_string());
        }
        if self.state == SystemState::Sleeping || self.state == SystemState::ShuttingDown {
            return Err("Cannot lock system in current state".to_string());
        }
        self.state = SystemState::Locked;
        self.lock_enabled = true;
        Ok(())
    }

    /// Unlock the system
    pub fn unlock(&mut self) -> Result<(), String> {
        if self.state != SystemState::Locked {
            return Err("System is not locked".to_string());
        }
        self.state = SystemState::Running;
        self.lock_enabled = false;
        Ok(())
    }

    /// Reboot the system
    pub fn reboot(&mut self) -> Result<(), String> {
        if self.state == SystemState::ShuttingDown {
            return Err("System is shutting down".to_string());
        }
        self.state = SystemState::Rebooting;
        Ok(())
    }

    /// Shutdown the system
    pub fn shutdown(&mut self) -> Result<(), String> {
        if self.state == SystemState::Rebooting {
            return Err("System is rebooting".to_string());
        }
        self.state = SystemState::ShuttingDown;
        Ok(())
    }

    /// Sleep the system
    pub fn sleep(&mut self) -> Result<(), String> {
        if self.state == SystemState::Locked {
            return Err("Cannot sleep while locked".to_string());
        }
        self.state = SystemState::Sleeping;
        Ok(())
    }

    /// Wake the system
    pub fn wake(&mut self) -> Result<(), String> {
        if self.state != SystemState::Sleeping {
            return Err("System is not sleeping".to_string());
        }
        self.state = SystemState::Running;
        Ok(())
    }

    /// Close lid
    pub fn lid_close(&mut self) -> Result<(), String> {
        if self.state == SystemState::Sleeping {
            return Err("System already sleeping".to_string());
        }
        self.state = SystemState::Sleeping;
        Ok(())
    }

    /// Factory reset (requires confirmation)
    pub fn factory_reset(&mut self, confirmed: bool) -> Result<(), String> {
        if !confirmed {
            return Err("Factory reset requires confirmation".to_string());
        }
        self.factory_reset_confirmed = true;
        self.state = SystemState::Rebooting;
        Ok(())
    }

    /// Update stats
    pub fn update_stats(&mut self, stats: SystemStats) {
        self.stats = stats;
    }

    /// Get stats
    pub fn get_stats(&self) -> &SystemStats {
        &self.stats
    }

    /// Is locked
    pub fn is_locked(&self) -> bool {
        self.state == SystemState::Locked
    }

    /// Is sleeping
    pub fn is_sleeping(&self) -> bool {
        self.state == SystemState::Sleeping
    }

    /// Is running
    pub fn is_running(&self) -> bool {
        self.state == SystemState::Running
    }
}

impl Default for SystemControlManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lock_unlock() {
        let mut mgr = SystemControlManager::new();
        assert!(mgr.lock().is_ok());
        assert!(mgr.is_locked());
        assert!(mgr.unlock().is_ok());
        assert!(!mgr.is_locked());
    }

    #[test]
    fn test_lock_twice_fails() {
        let mut mgr = SystemControlManager::new();
        mgr.lock().unwrap();
        assert!(mgr.lock().is_err());
    }

    #[test]
    fn test_reboot_shutdown() {
        let mut mgr = SystemControlManager::new();
        assert!(mgr.reboot().is_ok());
        assert_eq!(mgr.state, SystemState::Rebooting);
        assert!(mgr.shutdown().is_err());
    }

    #[test]
    fn test_sleep_wake() {
        let mut mgr = SystemControlManager::new();
        assert!(mgr.sleep().is_ok());
        assert!(mgr.is_sleeping());
        assert!(mgr.wake().is_ok());
        assert!(mgr.is_running());
    }

    #[test]
    fn test_factory_reset_without_confirmation_fails() {
        let mut mgr = SystemControlManager::new();
        assert!(mgr.factory_reset(false).is_err());
    }

    #[test]
    fn test_factory_reset_with_confirmation() {
        let mut mgr = SystemControlManager::new();
        assert!(mgr.factory_reset(true).is_ok());
        assert!(mgr.factory_reset_confirmed);
    }

    #[test]
    fn test_update_stats() {
        let mut mgr = SystemControlManager::new();
        let stats = SystemStats {
            uptime_seconds: 3600,
            cpu_usage: 25.5,
            memory_usage: 45.0,
            disk_usage: 60.0,
            temperature: Some(55.0),
            battery_percent: Some(80),
        };
        mgr.update_stats(stats);
        assert_eq!(mgr.get_stats().uptime_seconds, 3600);
        assert_eq!(mgr.get_stats().cpu_usage, 25.5);
    }
}