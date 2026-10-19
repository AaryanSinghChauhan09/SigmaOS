//! SigmaPowerManager - Power Management System
//!
//! Provides comprehensive power management inspired by Linux Mint and Omarchy.
//! Supports power profiles, battery management, power settings, and power statistics.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Power mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerMode {
    Performance,
    Balanced,
    PowerSaver,
}

/// Battery status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatteryStatus {
    Charging,
    Discharging,
    Full,
    NotPresent,
}

/// Power profile
#[derive(Debug, Clone)]
pub struct PowerProfile {
    pub id: String,
    pub name: String,
    pub power_mode: PowerMode,
    pub cpu_governor: String,
    pub screen_brightness: u8, // 0-100
    pub disk_idle_timeout: u32, // seconds
    pub sleep_timeout: u32, // seconds
    pub hibernate_timeout: u32, // seconds
    pub wifi_power_save: bool,
    pub bluetooth_power_save: bool,
}

impl PowerProfile {
    pub fn new(id: String, name: String, power_mode: PowerMode) -> Self {
        Self {
            id,
            name,
            power_mode,
            cpu_governor: "schedutil".to_string(),
            screen_brightness: 100,
            disk_idle_timeout: 300,
            sleep_timeout: 600,
            hibernate_timeout: 1800,
            wifi_power_save: false,
            bluetooth_power_save: false,
        }
    }

    pub fn set_cpu_governor(&mut self, governor: String) {
        self.cpu_governor = governor;
    }

    pub fn set_screen_brightness(&mut self, brightness: u8) {
        self.screen_brightness = brightness.min(100);
    }

    pub fn set_sleep_timeout(&mut self, timeout: u32) {
        self.sleep_timeout = timeout;
    }

    pub fn set_hibernate_timeout(&mut self, timeout: u32) {
        self.hibernate_timeout = timeout;
    }
}

/// Battery information
#[derive(Debug, Clone)]
pub struct BatteryInfo {
    pub id: String,
    pub capacity: u32, // percentage
    pub status: BatteryStatus,
    pub voltage: f32, // volts
    pub energy: f32, // watt-hours
    pub energy_rate: f32, // watts
    pub time_to_empty: Option<u32>, // seconds
    pub time_to_full: Option<u32>, // seconds
}

impl BatteryInfo {
    pub fn new(id: String) -> Self {
        Self {
            id,
            capacity: 100,
            status: BatteryStatus::NotPresent,
            voltage: 0.0,
            energy: 0.0,
            energy_rate: 0.0,
            time_to_empty: None,
            time_to_full: None,
        }
    }

    pub fn set_capacity(&mut self, capacity: u32) {
        self.capacity = capacity.min(100);
    }

    pub fn set_status(&mut self, status: BatteryStatus) {
        self.status = status;
    }

    pub fn set_voltage(&mut self, voltage: f32) {
        self.voltage = voltage;
    }

    pub fn set_energy(&mut self, energy: f32) {
        self.energy = energy;
    }

    pub fn set_energy_rate(&mut self, rate: f32) {
        self.energy_rate = rate;
    }

    pub fn set_time_to_empty(&mut self, time: Option<u32>) {
        self.time_to_empty = time;
    }

    pub fn set_time_to_full(&mut self, time: Option<u32>) {
        self.time_to_full = time;
    }

    pub fn is_charging(&self) -> bool {
        self.status == BatteryStatus::Charging
    }

    pub fn is_low(&self) -> bool {
        self.capacity < 20
    }

    pub fn is_critical(&self) -> bool {
        self.capacity < 10
    }
}

/// SigmaPowerManager - Power management
#[derive(Debug)]
pub struct SigmaPowerManager {
    profiles: BTreeMap<String, PowerProfile>,
    batteries: BTreeMap<String, BatteryInfo>,
    active_profile: Option<String>,
}

impl SigmaPowerManager {
    pub fn new() -> Self {
        Self {
            profiles: BTreeMap::new(),
            batteries: BTreeMap::new(),
            active_profile: None,
        }
    }

    pub fn add_profile(&mut self, profile: PowerProfile) {
        self.profiles.insert(profile.id.clone(), profile);
    }

    pub fn remove_profile(&mut self, id: &str) -> bool {
        self.profiles.remove(id).is_some()
    }

    pub fn get_profile(&self, id: &str) -> Option<&PowerProfile> {
        self.profiles.get(id)
    }

    pub fn get_profile_mut(&mut self, id: &str) -> Option<&mut PowerProfile> {
        self.profiles.get_mut(id)
    }

    pub fn add_battery(&mut self, battery: BatteryInfo) {
        self.batteries.insert(battery.id.clone(), battery);
    }

    pub fn remove_battery(&mut self, id: &str) -> bool {
        self.batteries.remove(id).is_some()
    }

    pub fn get_battery(&self, id: &str) -> Option<&BatteryInfo> {
        self.batteries.get(id)
    }

    pub fn get_battery_mut(&mut self, id: &str) -> Option<&mut BatteryInfo> {
        self.batteries.get_mut(id)
    }

    pub fn set_active_profile(&mut self, id: &str) -> bool {
        if self.profiles.contains_key(id) {
            self.active_profile = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn get_active_profile(&self) -> Option<&PowerProfile> {
        self.active_profile.as_ref()
            .and_then(|id| self.profiles.get(id))
    }

    pub fn list_low_batteries(&self) -> Vec<&BatteryInfo> {
        self.batteries.values()
            .filter(|battery| battery.is_low())
            .collect()
    }

    pub fn list_critical_batteries(&self) -> Vec<&BatteryInfo> {
        self.batteries.values()
            .filter(|battery| battery.is_critical())
            .collect()
    }

    pub fn profile_count(&self) -> usize {
        self.profiles.len()
    }

    pub fn battery_count(&self) -> usize {
        self.batteries.len()
    }
}

impl Default for SigmaPowerManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaPowerManager instance
static mut SIGMAPOWERMANAGER_INSTANCE: Option<SigmaPowerManager> = None;

/// Initialize the global SigmaPowerManager instance
pub fn init_sigmapowermanager() {
    unsafe {
        SIGMAPOWERMANAGER_INSTANCE = Some(SigmaPowerManager::new());
    }
}

/// Get the global SigmaPowerManager instance
pub fn get_sigmapowermanager() -> Option<&'static SigmaPowerManager> {
    unsafe {
        SIGMAPOWERMANAGER_INSTANCE.as_ref()
    }
}

/// Get the global SigmaPowerManager instance (mutable)
pub fn get_sigmapowermanager_mut() -> Option<&'static mut SigmaPowerManager> {
    unsafe {
        SIGMAPOWERMANAGER_INSTANCE.as_mut()
    }
}

/// Convenience function to add a profile
pub fn add_profile(profile: PowerProfile) -> bool {
    if let Some(manager) = get_sigmapowermanager_mut() {
        manager.add_profile(profile);
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_profile_creation() {
        let profile = PowerProfile::new(
            "perf".to_string(),
            "Performance".to_string(),
            PowerMode::Performance,
        );
        assert_eq!(profile.id, "perf");
        assert_eq!(profile.power_mode, PowerMode::Performance);
    }

    #[test]
    fn test_power_profile_modification() {
        let mut profile = PowerProfile::new(
            "balanced".to_string(),
            "Balanced".to_string(),
            PowerMode::Balanced,
        );
        profile.set_cpu_governor("powersave".to_string());
        profile.set_screen_brightness(75);
        profile.set_sleep_timeout(300);
        assert_eq!(profile.cpu_governor, "powersave");
        assert_eq!(profile.screen_brightness, 75);
    }

    #[test]
    fn test_battery_info_creation() {
        let battery = BatteryInfo::new("BAT0".to_string());
        assert_eq!(battery.id, "BAT0");
        assert_eq!(battery.status, BatteryStatus::NotPresent);
    }

    #[test]
    fn test_battery_info_modification() {
        let mut battery = BatteryInfo::new("BAT0".to_string());
        battery.set_capacity(85);
        battery.set_status(BatteryStatus::Discharging);
        battery.set_energy(50.0);
        battery.set_energy_rate(15.0);
        assert_eq!(battery.capacity, 85);
        assert!(battery.is_charging() == false);
        assert!(!battery.is_low());
    }

    #[test]
    fn test_battery_low_detection() {
        let mut battery = BatteryInfo::new("BAT0".to_string());
        battery.set_capacity(15);
        assert!(battery.is_low());
        assert!(battery.is_critical());
    }

    #[test]
    fn test_sigmapowermanager_creation() {
        let manager = SigmaPowerManager::new();
        assert_eq!(manager.profile_count(), 0);
        assert_eq!(manager.battery_count(), 0);
    }

    #[test]
    fn test_sigmapowermanager_management() {
        let mut manager = SigmaPowerManager::new();
        let profile = PowerProfile::new(
            "balanced".to_string(),
            "Balanced".to_string(),
            PowerMode::Balanced,
        );
        manager.add_profile(profile);
        assert_eq!(manager.profile_count(), 1);
    }

    #[test]
    fn test_active_profile() {
        let mut manager = SigmaPowerManager::new();
        let profile = PowerProfile::new(
            "perf".to_string(),
            "Performance".to_string(),
            PowerMode::Performance,
        );
        manager.add_profile(profile);
        manager.set_active_profile("perf");
        let active = manager.get_active_profile();
        assert_eq!(active.unwrap().id, "perf");
    }
}
