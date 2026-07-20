// Hardware Watchdog - Linux-style watchdog timer for system monitoring
// Supports hardware watchdog timers and system health monitoring

#![no_std]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatchdogState {
    Stopped,
    Running,
    Expired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatchdogAction {
    Reset,       // System reset
    PowerOff,    // Power off
    Panic,       // Kernel panic
    Interrupt,   // Generate interrupt
}

#[derive(Debug, Clone)]
pub struct WatchdogDevice {
    pub name: String,
    pub timeout: u32,        // seconds
    pub state: WatchdogState,
    pub action: WatchdogAction,
    pub last_keepalive: u64, // timestamp
}

#[derive(Debug, Clone)]
pub struct HardwareMonitor {
    pub cpu_temperature: f32,    // Celsius
    pub cpu_usage: f32,         // percentage
    pub memory_usage: f32,      // percentage
    pub disk_usage: f32,       // percentage
    pub uptime: u64,            // seconds
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonitorThreshold {
    CpuTempCritical,
    CpuTempWarning,
    MemoryCritical,
    MemoryWarning,
    DiskCritical,
    DiskWarning,
}

pub struct WatchdogManager {
    watchdogs: BTreeMap<String, WatchdogDevice>,
    active_watchdog: Option<String>,
    monitor: HardwareMonitor,
    thresholds: BTreeMap<MonitorThreshold, f32>,
}

impl WatchdogManager {
    pub fn new() -> Self {
        let mut thresholds = BTreeMap::new();
        thresholds.insert(MonitorThreshold::CpuTempCritical, 90.0);
        thresholds.insert(MonitorThreshold::CpuTempWarning, 80.0);
        thresholds.insert(MonitorThreshold::MemoryCritical, 95.0);
        thresholds.insert(MonitorThreshold::MemoryWarning, 85.0);
        thresholds.insert(MonitorThreshold::DiskCritical, 95.0);
        thresholds.insert(MonitorThreshold::DiskWarning, 85.0);

        Self {
            watchdogs: BTreeMap::new(),
            active_watchdog: None,
            monitor: HardwareMonitor {
                cpu_temperature: 45.0,
                cpu_usage: 10.0,
                memory_usage: 30.0,
                disk_usage: 40.0,
                uptime: 0,
            },
            thresholds,
        }
    }

    /// Register a watchdog device
    pub fn register_watchdog(&mut self, name: String, timeout: u32, action: WatchdogAction) -> Result<(), &'static str> {
        if self.watchdogs.contains_key(&name) {
            return Err("Watchdog already exists");
        }

        let watchdog = WatchdogDevice {
            name: name.clone(),
            timeout,
            state: WatchdogState::Stopped,
            action,
            last_keepalive: 0,
        };

        self.watchdogs.insert(name, watchdog);
        Ok(())
    }

    /// Start a watchdog
    pub fn start_watchdog(&mut self, name: &str) -> Result<(), &'static str> {
        let watchdog = self.watchdogs.get_mut(name)
            .ok_or("Watchdog not found")?;

        watchdog.state = WatchdogState::Running;
        watchdog.last_keepalive = self.get_timestamp();

        if self.active_watchdog.is_none() {
            self.active_watchdog = Some(name.to_string());
        }

        Ok(())
    }

    /// Stop a watchdog
    pub fn stop_watchdog(&mut self, name: &str) -> Result<(), &'static str> {
        let watchdog = self.watchdogs.get_mut(name)
            .ok_or("Watchdog not found")?;

        watchdog.state = WatchdogState::Stopped;

        if self.active_watchdog == Some(name.to_string()) {
            self.active_watchdog = None;
        }

        Ok(())
    }

    /// Send keepalive to a watchdog
    pub fn keepalive(&mut self, name: &str) -> Result<(), &'static str> {
        let watchdog = self.watchdogs.get_mut(name)
            .ok_or("Watchdog not found")?;

        if watchdog.state != WatchdogState::Running {
            return Err("Watchdog not running");
        }

        watchdog.last_keepalive = self.get_timestamp();
        Ok(())
    }

    /// Set watchdog timeout
    pub fn set_timeout(&mut self, name: &str, timeout: u32) -> Result<(), &'static str> {
        let watchdog = self.watchdogs.get_mut(name)
            .ok_or("Watchdog not found")?;

        watchdog.timeout = timeout;
        Ok(())
    }

    /// Set watchdog action
    pub fn set_action(&mut self, name: &str, action: WatchdogAction) -> Result<(), &'static str> {
        let watchdog = self.watchdogs.get_mut(name)
            .ok_or("Watchdog not found")?;

        watchdog.action = action;
        Ok(())
    }

    /// Check if watchdog has expired
    pub fn check_watchdog(&mut self, name: &str) -> Result<bool, &'static str> {
        let watchdog = self.watchdogs.get(name)
            .ok_or("Watchdog not found")?;

        if watchdog.state != WatchdogState::Running {
            return Ok(false);
        }

        let current_time = self.get_timestamp();
        let elapsed = current_time - watchdog.last_keepalive;
        let expired = elapsed > (watchdog.timeout as u64);

        if expired {
            // Mark as expired
            if let Some(w) = self.watchdogs.get_mut(name) {
                w.state = WatchdogState::Expired;
            }
        }

        Ok(expired)
    }

    /// Update hardware monitor values
    pub fn update_monitor(&mut self, cpu_temp: f32, cpu_usage: f32, memory_usage: f32, disk_usage: f32) {
        self.monitor.cpu_temperature = cpu_temp;
        self.monitor.cpu_usage = cpu_usage;
        self.monitor.memory_usage = memory_usage;
        self.monitor.disk_usage = disk_usage;
        self.monitor.uptime += 1;
    }

    /// Check if thresholds are exceeded
    pub fn check_thresholds(&self) -> Vec<MonitorThreshold> {
        let mut exceeded = Vec::new();

        if let Some(&threshold) = self.thresholds.get(&MonitorThreshold::CpuTempCritical) {
            if self.monitor.cpu_temperature >= threshold {
                exceeded.push(MonitorThreshold::CpuTempCritical);
            }
        }

        if let Some(&threshold) = self.thresholds.get(&MonitorThreshold::CpuTempWarning) {
            if self.monitor.cpu_temperature >= threshold {
                exceeded.push(MonitorThreshold::CpuTempWarning);
            }
        }

        if let Some(&threshold) = self.thresholds.get(&MonitorThreshold::MemoryCritical) {
            if self.monitor.memory_usage >= threshold {
                exceeded.push(MonitorThreshold::MemoryCritical);
            }
        }

        if let Some(&threshold) = self.thresholds.get(&MonitorThreshold::MemoryWarning) {
            if self.monitor.memory_usage >= threshold {
                exceeded.push(MonitorThreshold::MemoryWarning);
            }
        }

        if let Some(&threshold) = self.thresholds.get(&MonitorThreshold::DiskCritical) {
            if self.monitor.disk_usage >= threshold {
                exceeded.push(MonitorThreshold::DiskCritical);
            }
        }

        if let Some(&threshold) = self.thresholds.get(&MonitorThreshold::DiskWarning) {
            if self.monitor.disk_usage >= threshold {
                exceeded.push(MonitorThreshold::DiskWarning);
            }
        }

        exceeded
    }

    /// Get current monitor state
    pub fn get_monitor(&self) -> &HardwareMonitor {
        &self.monitor
    }

    /// Get active watchdog
    pub fn active_watchdog(&self) -> Option<&WatchdogDevice> {
        if let Some(ref name) = self.active_watchdog {
            self.watchdogs.get(name)
        } else {
            None
        }
    }

    /// Get watchdog count
    pub fn watchdog_count(&self) -> usize {
        self.watchdogs.len()
    }

    /// Get timestamp (simplified)
    fn get_timestamp(&self) -> u64 {
        self.monitor.uptime
    }
}

impl Default for WatchdogManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_watchdog() {
        let mut manager = WatchdogManager::new();
        
        manager.register_watchdog("watchdog0".to_string(), 60, WatchdogAction::Reset).unwrap();
        assert_eq!(manager.watchdog_count(), 1);
    }

    #[test]
    fn test_start_watchdog() {
        let mut manager = WatchdogManager::new();
        
        manager.register_watchdog("watchdog0".to_string(), 60, WatchdogAction::Reset).unwrap();
        manager.start_watchdog("watchdog0").unwrap();
        
        let watchdog = manager.watchdogs.get("watchdog0").unwrap();
        assert_eq!(watchdog.state, WatchdogState::Running);
    }

    #[test]
    fn test_stop_watchdog() {
        let mut manager = WatchdogManager::new();
        
        manager.register_watchdog("watchdog0".to_string(), 60, WatchdogAction::Reset).unwrap();
        manager.start_watchdog("watchdog0").unwrap();
        manager.stop_watchdog("watchdog0").unwrap();
        
        let watchdog = manager.watchdogs.get("watchdog0").unwrap();
        assert_eq!(watchdog.state, WatchdogState::Stopped);
    }

    #[test]
    fn test_keepalive() {
        let mut manager = WatchdogManager::new();
        
        manager.register_watchdog("watchdog0".to_string(), 60, WatchdogAction::Reset).unwrap();
        manager.start_watchdog("watchdog0").unwrap();
        manager.keepalive("watchdog0").unwrap();
        
        let watchdog = manager.watchdogs.get("watchdog0").unwrap();
        assert!(watchdog.last_keepalive > 0);
    }

    #[test]
    fn test_set_timeout() {
        let mut manager = WatchdogManager::new();
        
        manager.register_watchdog("watchdog0".to_string(), 60, WatchdogAction::Reset).unwrap();
        manager.set_timeout("watchdog0", 120).unwrap();
        
        let watchdog = manager.watchdogs.get("watchdog0").unwrap();
        assert_eq!(watchdog.timeout, 120);
    }

    #[test]
    fn test_check_thresholds() {
        let mut manager = WatchdogManager::new();
        
        manager.update_monitor(95.0, 50.0, 96.0, 50.0);
        let exceeded = manager.check_thresholds();
        
        assert!(exceeded.contains(&MonitorThreshold::CpuTempCritical));
        assert!(exceeded.contains(&MonitorThreshold::MemoryCritical));
    }

    #[test]
    fn test_active_watchdog() {
        let mut manager = WatchdogManager::new();
        
        manager.register_watchdog("watchdog0".to_string(), 60, WatchdogAction::Reset).unwrap();
        manager.start_watchdog("watchdog0").unwrap();
        
        let active = manager.active_watchdog();
        assert!(active.is_some());
        assert_eq!(active.unwrap().name, "watchdog0");
    }

    #[test]
    fn test_watchdog_expiration() {
        let mut manager = WatchdogManager::new();
        
        manager.register_watchdog("watchdog0".to_string(), 5, WatchdogAction::Reset).unwrap();
        manager.start_watchdog("watchdog0").unwrap();
        
        // Simulate time passing
        manager.monitor.uptime = 10;
        
        let expired = manager.check_watchdog("watchdog0").unwrap();
        assert!(expired);
    }
}
