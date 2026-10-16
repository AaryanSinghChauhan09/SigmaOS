// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaPower - Power Management Daemon
//!
//! SigmaPower provides power management for SigmaOS, inspired by systemd-power and power-profiles-daemon from Linux.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// Power profile
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerProfile {
    /// Power saver profile
    PowerSaver,
    /// Balanced profile
    Balanced,
    /// Performance profile
    Performance,
}

impl fmt::Display for PowerProfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PowerProfile::PowerSaver => write!(f, "power_saver"),
            PowerProfile::Balanced => write!(f, "balanced"),
            PowerProfile::Performance => write!(f, "performance"),
        }
    }
}

/// Power state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerState {
    /// Active
    Active,
    /// Suspended
    Suspended,
    /// Hibernated
    Hibernated,
    /// Shutting down
    ShuttingDown,
}

impl fmt::Display for PowerState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PowerState::Active => write!(f, "active"),
            PowerState::Suspended => write!(f, "suspended"),
            PowerState::Hibernated => write!(f, "hibernated"),
            PowerState::ShuttingDown => write!(f, "shutting_down"),
        }
    }
}

/// Battery status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatteryStatus {
    /// Discharging
    Discharging,
    /// Charging
    Charging,
    /// Full
    Full,
    /// Unknown
    Unknown,
}

impl fmt::Display for BatteryStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BatteryStatus::Discharging => write!(f, "discharging"),
            BatteryStatus::Charging => write!(f, "charging"),
            BatteryStatus::Full => write!(f, "full"),
            BatteryStatus::Unknown => write!(f, "unknown"),
        }
    }
}

/// Battery information
#[derive(Debug, Clone)]
pub struct BatteryInfo {
    /// Battery ID
    pub battery_id: String,
    /// Battery capacity (percentage)
    pub capacity: u8,
    /// Battery status
    pub status: BatteryStatus,
    /// Current voltage (mV)
    pub voltage: Option<u32>,
    /// Current current (mA)
    pub current: Option<u32>,
    /// Estimated time remaining (seconds)
    pub time_remaining: Option<u64>,
}

impl BatteryInfo {
    /// Create new battery info
    pub fn new(battery_id: String, capacity: u8, status: BatteryStatus) -> Self {
        Self {
            battery_id,
            capacity,
            status,
            voltage: None,
            current: None,
            time_remaining: None,
        }
    }

    /// Set voltage
    pub fn with_voltage(mut self, voltage: u32) -> Self {
        self.voltage = Some(voltage);
        self
    }

    /// Set current
    pub fn with_current(mut self, current: u32) -> Self {
        self.current = Some(current);
        self
    }

    /// Set time remaining
    pub fn with_time_remaining(mut self, time: u64) -> Self {
        self.time_remaining = Some(time);
        self
    }
}

/// Power statistics
#[derive(Debug, Clone)]
pub struct PowerStats {
    /// Uptime (seconds)
    pub uptime: u64,
    /// Total energy consumed (Wh)
    pub energy_consumed: f64,
    /// Average power draw (W)
    pub average_power: f64,
    /// Profile switches count
    pub profile_switches: u32,
    /// Suspend count
    pub suspend_count: u32,
    /// Hibernate count
    pub hibernate_count: u32,
}

/// SigmaPower - Power Management Daemon
pub struct SigmaPower {
    /// Current power profile
    current_profile: PowerProfile,
    /// Current power state
    current_state: PowerState,
    /// Running state
    running: bool,
    /// Battery information (battery_id -> info)
    batteries: BTreeMap<String, BatteryInfo>,
    /// Power statistics
    stats: PowerStats,
    /// Profile-specific settings
    profile_settings: BTreeMap<PowerProfile, BTreeMap<String, String>>,
}

impl SigmaPower {
    /// Create a new SigmaPower instance
    pub fn new() -> Self {
        let mut profile_settings = BTreeMap::new();

        // Power saver settings
        let mut power_saver = BTreeMap::new();
        power_saver.insert("cpu_governor".to_string(), "powersave".to_string());
        power_saver.insert("screen_brightness".to_string(), "50".to_string());
        power_saver.insert("disk_apm".to_string(), "127".to_string());
        profile_settings.insert(PowerProfile::PowerSaver, power_saver);

        // Balanced settings
        let mut balanced = BTreeMap::new();
        balanced.insert("cpu_governor".to_string(), "ondemand".to_string());
        balanced.insert("screen_brightness".to_string(), "75".to_string());
        balanced.insert("disk_apm".to_string(), "255".to_string());
        profile_settings.insert(PowerProfile::Balanced, balanced);

        // Performance settings
        let mut performance = BTreeMap::new();
        performance.insert("cpu_governor".to_string(), "performance".to_string());
        performance.insert("screen_brightness".to_string(), "100".to_string());
        performance.insert("disk_apm".to_string(), "255".to_string());
        profile_settings.insert(PowerProfile::Performance, performance);

        Self {
            current_profile: PowerProfile::Balanced,
            current_state: PowerState::Active,
            running: false,
            batteries: BTreeMap::new(),
            stats: PowerStats {
                uptime: 0,
                energy_consumed: 0.0,
                average_power: 0.0,
                profile_switches: 0,
                suspend_count: 0,
                hibernate_count: 0,
            },
            profile_settings,
        }
    }

    /// Start the power daemon
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the power daemon
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the daemon is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Set power profile
    pub fn set_profile(&mut self, profile: PowerProfile) -> Result<(), String> {
        if !self.running {
            return Err("SigmaPower not running".to_string());
        }

        if self.current_profile != profile {
            self.current_profile = profile;
            self.stats.profile_switches += 1;
        }

        Ok(())
    }

    /// Get current power profile
    pub fn get_profile(&self) -> PowerProfile {
        self.current_profile
    }

    /// Set power state
    pub fn set_state(&mut self, state: PowerState) -> Result<(), String> {
        if !self.running {
            return Err("SigmaPower not running".to_string());
        }

        self.current_state = state;

        match state {
            PowerState::Suspended => self.stats.suspend_count += 1,
            PowerState::Hibernated => self.stats.hibernate_count += 1,
            _ => {}
        }

        Ok(())
    }

    /// Get current power state
    pub fn get_state(&self) -> PowerState {
        self.current_state
    }

    /// Add battery information
    pub fn add_battery(&mut self, battery: BatteryInfo) -> Result<(), String> {
        if self.batteries.contains_key(&battery.battery_id) {
            return Err(format!("Battery '{}' already exists", battery.battery_id));
        }
        self.batteries.insert(battery.battery_id.clone(), battery);
        Ok(())
    }

    /// Update battery information
    pub fn update_battery(&mut self, battery: BatteryInfo) -> Result<(), String> {
        if !self.batteries.contains_key(&battery.battery_id) {
            return Err(format!("Battery '{}' not found", battery.battery_id));
        }
        self.batteries.insert(battery.battery_id.clone(), battery);
        Ok(())
    }

    /// Get battery information
    pub fn get_battery(&self, battery_id: &str) -> Option<&BatteryInfo> {
        self.batteries.get(battery_id)
    }

    /// List all batteries
    pub fn list_batteries(&self) -> Vec<&BatteryInfo> {
        self.batteries.values().collect()
    }

    /// Get total battery capacity
    pub fn total_capacity(&self) -> u8 {
        if self.batteries.is_empty() {
            return 0;
        }

        let total: u32 = self.batteries.values().map(|b| b.capacity as u32).sum();
        (total / self.batteries.len() as u32) as u8
    }

    /// Suspend system
    pub fn suspend(&mut self) -> Result<(), String> {
        if !self.running {
            return Err("SigmaPower not running".to_string());
        }

        self.set_state(PowerState::Suspended)?;
        Ok(())
    }

    /// Hibernate system
    pub fn hibernate(&mut self) -> Result<(), String> {
        if !self.running {
            return Err("SigmaPower not running".to_string());
        }

        self.set_state(PowerState::Hibernated)?;
        Ok(())
    }

    /// Shutdown system
    pub fn shutdown(&mut self) -> Result<(), String> {
        if !self.running {
            return Err("SigmaPower not running".to_string());
        }

        self.set_state(PowerState::ShuttingDown)?;
        Ok(())
    }

    /// Update statistics
    pub fn update_stats(&mut self, uptime: u64, energy: f64, power: f64) {
        self.stats.uptime = uptime;
        self.stats.energy_consumed = energy;
        self.stats.average_power = power;
    }

    /// Get power statistics
    pub fn get_stats(&self) -> &PowerStats {
        &self.stats
    }

    /// Get profile settings
    pub fn get_profile_settings(&self, profile: PowerProfile) -> Option<&BTreeMap<String, String>> {
        self.profile_settings.get(&profile)
    }

    /// Auto-switch profile based on battery capacity
    pub fn auto_switch_profile(&mut self) -> Option<PowerProfile> {
        if !self.running {
            return None;
        }

        let capacity = self.total_capacity();

        let new_profile = if capacity < 20 {
            PowerProfile::PowerSaver
        } else if capacity > 80 {
            PowerProfile::Performance
        } else {
            PowerProfile::Balanced
        };

        if new_profile != self.current_profile {
            let _ = self.set_profile(new_profile);
            Some(new_profile)
        } else {
            None
        }
    }
}

impl Default for SigmaPower {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaPower instance (placeholder for actual global state)
static mut SIGMAPOWER_INSTANCE: Option<SigmaPower> = None;

/// Initialize the global SigmaPower instance
pub fn init_sigmapower() {
    unsafe {
        SIGMAPOWER_INSTANCE = Some(SigmaPower::new());
    }
}

/// Get the global SigmaPower instance
pub fn get_sigmapower() -> Option<&'static mut SigmaPower> {
    unsafe { SIGMAPOWER_INSTANCE.as_mut() }
}

/// Convenience function to set power profile
pub fn set_power_profile(profile: PowerProfile) -> Result<(), String> {
    if let Some(power) = get_sigmapower() {
        power.set_profile(profile)
    } else {
        Err("SigmaPower not initialized".to_string())
    }
}

/// Convenience function to suspend system
pub fn suspend_system() -> Result<(), String> {
    if let Some(power) = get_sigmapower() {
        power.suspend()
    } else {
        Err("SigmaPower not initialized".to_string())
    }
}

/// Convenience function to shutdown system
pub fn shutdown_system() -> Result<(), String> {
    if let Some(power) = get_sigmapower() {
        power.shutdown()
    } else {
        Err("SigmaPower not initialized".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_profile_display() {
        assert_eq!(PowerProfile::PowerSaver.to_string(), "power_saver");
        assert_eq!(PowerProfile::Balanced.to_string(), "balanced");
    }

    #[test]
    fn test_power_state_display() {
        assert_eq!(PowerState::Active.to_string(), "active");
        assert_eq!(PowerState::Suspended.to_string(), "suspended");
    }

    #[test]
    fn test_battery_status_display() {
        assert_eq!(BatteryStatus::Charging.to_string(), "charging");
        assert_eq!(BatteryStatus::Discharging.to_string(), "discharging");
    }

    #[test]
    fn test_battery_info() {
        let battery = BatteryInfo::new("BAT0".to_string(), 85, BatteryStatus::Charging)
            .with_voltage(12000)
            .with_current(2000)
            .with_time_remaining(3600);

        assert_eq!(battery.battery_id, "BAT0");
        assert_eq!(battery.capacity, 85);
        assert_eq!(battery.voltage, Some(12000));
    }

    #[test]
    fn test_sigmapower() {
        let mut power = SigmaPower::new();
        assert!(!power.is_running());
        power.start();
        assert!(power.is_running());
        power.stop();
        assert!(!power.is_running());
    }

    #[test]
    fn test_sigmapower_set_profile() {
        let mut power = SigmaPower::new();
        power.start();

        assert!(power.set_profile(PowerProfile::Performance).is_ok());
        assert_eq!(power.get_profile(), PowerProfile::Performance);
    }

    #[test]
    fn test_sigmapower_profile_switches() {
        let mut power = SigmaPower::new();
        power.start();

        power.set_profile(PowerProfile::Performance).unwrap();
        power.set_profile(PowerProfile::PowerSaver).unwrap();

        let stats = power.get_stats();
        assert_eq!(stats.profile_switches, 2);
    }

    #[test]
    fn test_sigmapower_add_battery() {
        let mut power = SigmaPower::new();

        let battery = BatteryInfo::new("BAT0".to_string(), 85, BatteryStatus::Charging);
        assert!(power.add_battery(battery).is_ok());
        assert_eq!(power.list_batteries().len(), 1);
    }

    #[test]
    fn test_sigmapower_update_battery() {
        let mut power = SigmaPower::new();

        let battery1 = BatteryInfo::new("BAT0".to_string(), 85, BatteryStatus::Charging);
        power.add_battery(battery1).unwrap();

        let battery2 = BatteryInfo::new("BAT0".to_string(), 90, BatteryStatus::Charging);
        assert!(power.update_battery(battery2).is_ok());

        let battery = power.get_battery("BAT0").unwrap();
        assert_eq!(battery.capacity, 90);
    }

    #[test]
    fn test_sigmapower_total_capacity() {
        let mut power = SigmaPower::new();

        let battery1 = BatteryInfo::new("BAT0".to_string(), 80, BatteryStatus::Charging);
        let battery2 = BatteryInfo::new("BAT1".to_string(), 90, BatteryStatus::Charging);

        power.add_battery(battery1).unwrap();
        power.add_battery(battery2).unwrap();

        assert_eq!(power.total_capacity(), 85);
    }

    #[test]
    fn test_sigmapower_suspend() {
        let mut power = SigmaPower::new();
        power.start();

        assert!(power.suspend().is_ok());
        assert_eq!(power.get_state(), PowerState::Suspended);
    }

    #[test]
    fn test_sigmapower_hibernate() {
        let mut power = SigmaPower::new();
        power.start();

        assert!(power.hibernate().is_ok());
        assert_eq!(power.get_state(), PowerState::Hibernated);
    }

    #[test]
    fn test_sigmapower_shutdown() {
        let mut power = SigmaPower::new();
        power.start();

        assert!(power.shutdown().is_ok());
        assert_eq!(power.get_state(), PowerState::ShuttingDown);
    }

    #[test]
    fn test_sigmapower_auto_switch() {
        let mut power = SigmaPower::new();
        power.start();

        let battery = BatteryInfo::new("BAT0".to_string(), 15, BatteryStatus::Discharging);
        power.add_battery(battery).unwrap();

        let new_profile = power.auto_switch_profile();
        assert_eq!(new_profile, Some(PowerProfile::PowerSaver));
    }

    #[test]
    fn test_set_power_profile_convenience() {
        init_sigmapower();

        let mut power = get_sigmapower().unwrap();
        power.start();

        let result = set_power_profile(PowerProfile::Performance);
        assert!(result.is_ok());
    }

    #[test]
    fn test_suspend_system_convenience() {
        init_sigmapower();

        let mut power = get_sigmapower().unwrap();
        power.start();

        let result = suspend_system();
        assert!(result.is_ok());
    }

    #[test]
    fn test_shutdown_system_convenience() {
        init_sigmapower();

        let mut power = get_sigmapower().unwrap();
        power.start();

        let result = shutdown_system();
        assert!(result.is_ok());
    }
}
