// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Aaryan Singh Chauhan

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Power state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerState {
    On,
    Sleep,
    Hibernate,
    Off,
}

/// Battery status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatteryStatus {
    Charging,
    Discharging,
    Full,
    NotPresent,
}

/// Power plan
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerPlan {
    Performance,
    Balanced,
    PowerSaver,
}

/// Power manager for managing system power
/// Inspired by Linux power management and Omarchy power commands
#[derive(Debug, Clone)]
pub struct SigmaPowerManager2 {
    power_state: PowerState,
    battery_level: u8,
    battery_status: BatteryStatus,
    power_plan: PowerPlan,
}

impl SigmaPowerManager2 {
    pub fn new() -> Self {
        Self {
            power_state: PowerState::On,
            battery_level: 100,
            battery_status: BatteryStatus::Full,
            power_plan: PowerPlan::Balanced,
        }
    }

    /// Get current power state
    pub fn get_power_state(&self) -> PowerState {
        self.power_state
    }

    /// Set power state
    pub fn set_power_state(&mut self, state: PowerState) -> Result<String, String> {
        self.power_state = state;
        Ok(format!("Power state set to: {:?}", state))
    }

    /// Get battery level
    pub fn get_battery_level(&self) -> u8 {
        self.battery_level
    }

    /// Set battery level
    pub fn set_battery_level(&mut self, level: u8) -> Result<String, String> {
        if level <= 100 {
            self.battery_level = level;
            Ok(format!("Battery level set to: {}%", level))
        } else {
            Err("Invalid battery level".to_string())
        }
    }

    /// Get battery status
    pub fn get_battery_status(&self) -> BatteryStatus {
        self.battery_status
    }

    /// Set battery status
    pub fn set_battery_status(&mut self, status: BatteryStatus) -> Result<String, String> {
        self.battery_status = status;
        Ok(format!("Battery status set to: {:?}", status))
    }

    /// Get power plan
    pub fn get_power_plan(&self) -> PowerPlan {
        self.power_plan
    }

    /// Set power plan
    pub fn set_power_plan(&mut self, plan: PowerPlan) -> Result<String, String> {
        self.power_plan = plan;
        Ok(format!("Power plan set to: {:?}", plan))
    }

    /// Suspend system
    pub fn suspend(&mut self) -> Result<String, String> {
        self.power_state = PowerState::Sleep;
        Ok("System suspended".to_string())
    }

    /// Hibernate system
    pub fn hibernate(&mut self) -> Result<String, String> {
        self.power_state = PowerState::Hibernate;
        Ok("System hibernated".to_string())
    }

    /// Shutdown system
    pub fn shutdown(&mut self) -> Result<String, String> {
        self.power_state = PowerState::Off;
        Ok("System shutdown".to_string())
    }

    /// Reboot system
    pub fn reboot(&mut self) -> Result<String, String> {
        self.power_state = PowerState::On;
        Ok("System rebooted".to_string())
    }

    /// Check if on battery
    pub fn is_on_battery(&self) -> bool {
        self.battery_status == BatteryStatus::Discharging
    }

    /// Check if battery low
    pub fn is_battery_low(&self) -> bool {
        self.battery_level < 20
    }

    /// Get estimated time remaining (simplified)
    pub fn get_time_remaining(&self) -> Option<u32> {
        if self.battery_status == BatteryStatus::Discharging {
            Some((self.battery_level as u32) * 3)
        } else {
            None
        }
    }
}

impl Default for SigmaPowerManager2 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_power_state() {
        let mut manager = SigmaPowerManager2::new();
        manager.set_power_state(PowerState::Sleep).unwrap();
        assert_eq!(manager.get_power_state(), PowerState::Sleep);
    }

    #[test]
    fn test_set_battery_level() {
        let mut manager = SigmaPowerManager2::new();
        manager.set_battery_level(50).unwrap();
        assert_eq!(manager.get_battery_level(), 50);
    }

    #[test]
    fn test_set_battery_level_invalid() {
        let mut manager = SigmaPowerManager2::new();
        assert!(manager.set_battery_level(150).is_err());
    }

    #[test]
    fn test_set_power_plan() {
        let mut manager = SigmaPowerManager2::new();
        manager.set_power_plan(PowerPlan::Performance).unwrap();
        assert_eq!(manager.get_power_plan(), PowerPlan::Performance);
    }

    #[test]
    fn test_suspend() {
        let mut manager = SigmaPowerManager2::new();
        manager.suspend().unwrap();
        assert_eq!(manager.get_power_state(), PowerState::Sleep);
    }

    #[test]
    fn test_hibernate() {
        let mut manager = SigmaPowerManager2::new();
        manager.hibernate().unwrap();
        assert_eq!(manager.get_power_state(), PowerState::Hibernate);
    }

    #[test]
    fn test_shutdown() {
        let mut manager = SigmaPowerManager2::new();
        manager.shutdown().unwrap();
        assert_eq!(manager.get_power_state(), PowerState::Off);
    }

    #[test]
    fn test_is_on_battery() {
        let mut manager = SigmaPowerManager2::new();
        manager.set_battery_status(BatteryStatus::Discharging).unwrap();
        assert!(manager.is_on_battery());
    }

    #[test]
    fn test_is_battery_low() {
        let mut manager = SigmaPowerManager2::new();
        manager.set_battery_level(15).unwrap();
        assert!(manager.is_battery_low());
    }

    #[test]
    fn test_get_time_remaining() {
        let mut manager = SigmaPowerManager2::new();
        manager.set_battery_level(50).unwrap();
        manager.set_battery_status(BatteryStatus::Discharging).unwrap();
        assert_eq!(manager.get_time_remaining(), Some(150));
    }
}
