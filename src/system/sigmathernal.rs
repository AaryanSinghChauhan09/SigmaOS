// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaThermal - Thermal Management Daemon
//!
//! SigmaThermal provides thermal management for SigmaOS, inspired by thermald and systemd-thermal from Linux.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// Thermal zone type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThermalZoneType {
    /// CPU thermal zone
    CPU,
    /// GPU thermal zone
    GPU,
    /// Memory thermal zone
    Memory,
    /// Storage thermal zone
    Storage,
    /// Battery thermal zone
    Battery,
    /// Unknown thermal zone
    Unknown,
}

impl fmt::Display for ThermalZoneType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ThermalZoneType::CPU => write!(f, "cpu"),
            ThermalZoneType::GPU => write!(f, "gpu"),
            ThermalZoneType::Memory => write!(f, "memory"),
            ThermalZoneType::Storage => write!(f, "storage"),
            ThermalZoneType::Battery => write!(f, "battery"),
            ThermalZoneType::Unknown => write!(f, "unknown"),
        }
    }
}

/// Cooling device type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoolingDeviceType {
    /// Fan
    Fan,
    /// Processor
    Processor,
    /// LCD
    LCD,
    /// Battery
    Battery,
    /// Unknown
    Unknown,
}

impl fmt::Display for CoolingDeviceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CoolingDeviceType::Fan => write!(f, "fan"),
            CoolingDeviceType::Processor => write!(f, "processor"),
            CoolingDeviceType::LCD => write!(f, "lcd"),
            CoolingDeviceType::Battery => write!(f, "battery"),
            CoolingDeviceType::Unknown => write!(f, "unknown"),
        }
    }
}

/// Thermal trip point type
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TripPointType {
    /// Critical trip point
    Critical,
    /// Hot trip point
    Hot,
    /// Passive trip point
    Passive,
    /// Active trip point
    Active,
}

impl fmt::Display for TripPointType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TripPointType::Critical => write!(f, "critical"),
            TripPointType::Hot => write!(f, "hot"),
            TripPointType::Passive => write!(f, "passive"),
            TripPointType::Active => write!(f, "active"),
        }
    }
}

/// Thermal zone
#[derive(Debug, Clone)]
pub struct ThermalZone {
    /// Zone ID
    pub zone_id: String,
    /// Zone type
    pub zone_type: ThermalZoneType,
    /// Current temperature (millidegrees Celsius)
    pub temperature: i32,
    /// Trip points (temperature in millidegrees Celsius)
    pub trip_points: BTreeMap<TripPointType, i32>,
    /// Cooling devices
    pub cooling_devices: Vec<String>,
}

impl ThermalZone {
    /// Create a new thermal zone
    pub fn new(zone_id: String, zone_type: ThermalZoneType) -> Self {
        Self {
            zone_id,
            zone_type,
            temperature: 0,
            trip_points: BTreeMap::new(),
            cooling_devices: Vec::new(),
        }
    }

    /// Set temperature
    pub fn with_temperature(mut self, temp: i32) -> Self {
        self.temperature = temp;
        self
    }

    /// Add trip point
    pub fn with_trip_point(mut self, trip_type: TripPointType, temp: i32) -> Self {
        self.trip_points.insert(trip_type, temp);
        self
    }

    /// Add cooling device
    pub fn with_cooling_device(mut self, device: String) -> Self {
        self.cooling_devices.push(device);
        self
    }
}

/// Cooling device
#[derive(Debug, Clone)]
pub struct CoolingDevice {
    /// Device ID
    pub device_id: String,
    /// Device type
    pub device_type: CoolingDeviceType,
    /// Current state (0-100)
    pub state: u8,
    /// Max state
    pub max_state: u8,
}

impl CoolingDevice {
    /// Create a new cooling device
    pub fn new(device_id: String, device_type: CoolingDeviceType) -> Self {
        Self {
            device_id,
            device_type,
            state: 0,
            max_state: 100,
        }
    }

    /// Set state
    pub fn with_state(mut self, state: u8) -> Self {
        self.state = state;
        self
    }

    /// Set max state
    pub fn with_max_state(mut self, max: u8) -> Self {
        self.max_state = max;
        self
    }
}

/// Thermal policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThermalPolicy {
    /// Performance policy
    Performance,
    /// Balanced policy
    Balanced,
    /// Power saver policy
    PowerSaver,
}

impl fmt::Display for ThermalPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ThermalPolicy::Performance => write!(f, "performance"),
            ThermalPolicy::Balanced => write!(f, "balanced"),
            ThermalPolicy::PowerSaver => write!(f, "power_saver"),
        }
    }
}

/// SigmaThermal - Thermal Management Daemon
pub struct SigmaThermal {
    /// Thermal zones (zone_id -> zone)
    thermal_zones: BTreeMap<String, ThermalZone>,
    /// Cooling devices (device_id -> device)
    cooling_devices: BTreeMap<String, CoolingDevice>,
    /// Current thermal policy
    current_policy: ThermalPolicy,
    /// Running state
    running: bool,
    /// Trip point trigger count
    trip_triggers: u32,
}

impl SigmaThermal {
    /// Create a new SigmaThermal instance
    pub fn new() -> Self {
        Self {
            thermal_zones: BTreeMap::new(),
            cooling_devices: BTreeMap::new(),
            current_policy: ThermalPolicy::Balanced,
            running: false,
            trip_triggers: 0,
        }
    }

    /// Start the thermal daemon
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the thermal daemon
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the daemon is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Set thermal policy
    pub fn set_policy(&mut self, policy: ThermalPolicy) -> Result<(), String> {
        if !self.running {
            return Err("SigmaThermal not running".to_string());
        }

        self.current_policy = policy;
        Ok(())
    }

    /// Get current thermal policy
    pub fn get_policy(&self) -> ThermalPolicy {
        self.current_policy
    }

    /// Add thermal zone
    pub fn add_thermal_zone(&mut self, zone: ThermalZone) -> Result<(), String> {
        if self.thermal_zones.contains_key(&zone.zone_id) {
            return Err(format!("Thermal zone '{}' already exists", zone.zone_id));
        }
        self.thermal_zones.insert(zone.zone_id.clone(), zone);
        Ok(())
    }

    /// Update thermal zone temperature
    pub fn update_temperature(&mut self, zone_id: &str, temperature: i32) -> Result<(), String> {
        match self.thermal_zones.get_mut(zone_id) {
            Some(zone) => {
                zone.temperature = temperature;
                Ok(())
            }
            None => Err(format!("Thermal zone '{}' not found", zone_id)),
        }
    }

    /// Get thermal zone
    pub fn get_thermal_zone(&self, zone_id: &str) -> Option<&ThermalZone> {
        self.thermal_zones.get(zone_id)
    }

    /// List all thermal zones
    pub fn list_thermal_zones(&self) -> Vec<&ThermalZone> {
        self.thermal_zones.values().collect()
    }

    /// Add cooling device
    pub fn add_cooling_device(&mut self, device: CoolingDevice) -> Result<(), String> {
        if self.cooling_devices.contains_key(&device.device_id) {
            return Err(format!(
                "Cooling device '{}' already exists",
                device.device_id
            ));
        }
        self.cooling_devices
            .insert(device.device_id.clone(), device);
        Ok(())
    }

    /// Update cooling device state
    pub fn update_cooling_state(&mut self, device_id: &str, state: u8) -> Result<(), String> {
        match self.cooling_devices.get_mut(device_id) {
            Some(device) => {
                if state > device.max_state {
                    return Err(format!(
                        "State {} exceeds max state {}",
                        state, device.max_state
                    ));
                }
                device.state = state;
                Ok(())
            }
            None => Err(format!("Cooling device '{}' not found", device_id)),
        }
    }

    /// Get cooling device
    pub fn get_cooling_device(&self, device_id: &str) -> Option<&CoolingDevice> {
        self.cooling_devices.get(device_id)
    }

    /// List all cooling devices
    pub fn list_cooling_devices(&self) -> Vec<&CoolingDevice> {
        self.cooling_devices.values().collect()
    }

    /// Check trip points and trigger cooling
    pub fn check_trip_points(&mut self) -> Vec<String> {
        let mut actions = Vec::new();

        if !self.running {
            return actions;
        }

        for (zone_id, zone) in self.thermal_zones.iter_mut() {
            for (trip_type, trip_temp) in &zone.trip_points {
                if zone.temperature >= *trip_temp {
                    self.trip_triggers += 1;

                    match trip_type {
                        TripPointType::Critical => {
                            actions.push(format!(
                                "CRITICAL: Zone {} at {}°C",
                                zone_id,
                                zone.temperature / 1000
                            ));
                        }
                        TripPointType::Hot => {
                            actions.push(format!(
                                "HOT: Zone {} at {}°C",
                                zone_id,
                                zone.temperature / 1000
                            ));
                        }
                        TripPointType::Passive => {
                            actions.push(format!(
                                "PASSIVE: Zone {} at {}°C",
                                zone_id,
                                zone.temperature / 1000
                            ));
                        }
                        TripPointType::Active => {
                            actions.push(format!(
                                "ACTIVE: Zone {} at {}°C",
                                zone_id,
                                zone.temperature / 1000
                            ));
                        }
                    }
                }
            }
        }

        actions
    }

    /// Get maximum temperature across all zones
    pub fn max_temperature(&self) -> i32 {
        self.thermal_zones
            .values()
            .map(|z| z.temperature)
            .max()
            .unwrap_or(0)
    }

    /// Get thermal zone count
    pub fn thermal_zone_count(&self) -> usize {
        self.thermal_zones.len()
    }

    /// Get cooling device count
    pub fn cooling_device_count(&self) -> usize {
        self.cooling_devices.len()
    }

    /// Get thermal statistics
    pub fn thermal_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert(
            "thermal_zones".to_string(),
            self.thermal_zone_count().to_string(),
        );
        stats.insert(
            "cooling_devices".to_string(),
            self.cooling_device_count().to_string(),
        );
        stats.insert(
            "max_temperature".to_string(),
            (self.max_temperature() / 1000).to_string(),
        );
        stats.insert("trip_triggers".to_string(), self.trip_triggers.to_string());
        stats.insert(
            "current_policy".to_string(),
            self.current_policy.to_string(),
        );
        stats.insert("running".to_string(), self.running.to_string());
        stats
    }
}

impl Default for SigmaThermal {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaThermal instance (placeholder for actual global state)
static mut SIGMATHERMAL_INSTANCE: Option<SigmaThermal> = None;

/// Initialize the global SigmaThermal instance
pub fn init_sigmathernal() {
    unsafe {
        SIGMATHERMAL_INSTANCE = Some(SigmaThermal::new());
    }
}

/// Get the global SigmaThermal instance
pub fn get_sigmathernal() -> Option<&'static mut SigmaThermal> {
    unsafe { SIGMATHERMAL_INSTANCE.as_mut() }
}

/// Convenience function to set thermal policy
pub fn set_thermal_policy(policy: ThermalPolicy) -> Result<(), String> {
    if let Some(thermal) = get_sigmathernal() {
        thermal.set_policy(policy)
    } else {
        Err("SigmaThermal not initialized".to_string())
    }
}

/// Convenience function to update temperature
pub fn update_temperature(zone_id: &str, temperature: i32) -> Result<(), String> {
    if let Some(thermal) = get_sigmathernal() {
        thermal.update_temperature(zone_id, temperature)
    } else {
        Err("SigmaThermal not initialized".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thermal_zone_type_display() {
        assert_eq!(ThermalZoneType::CPU.to_string(), "cpu");
        assert_eq!(ThermalZoneType::GPU.to_string(), "gpu");
    }

    #[test]
    fn test_cooling_device_type_display() {
        assert_eq!(CoolingDeviceType::Fan.to_string(), "fan");
        assert_eq!(CoolingDeviceType::Processor.to_string(), "processor");
    }

    #[test]
    fn test_trip_point_type_display() {
        assert_eq!(TripPointType::Critical.to_string(), "critical");
        assert_eq!(TripPointType::Hot.to_string(), "hot");
    }

    #[test]
    fn test_thermal_zone() {
        let zone = ThermalZone::new("cpu0".to_string(), ThermalZoneType::CPU)
            .with_temperature(45000)
            .with_trip_point(TripPointType::Critical, 100000)
            .with_cooling_device("fan0".to_string());

        assert_eq!(zone.zone_id, "cpu0");
        assert_eq!(zone.temperature, 45000);
        assert_eq!(zone.trip_points.len(), 1);
    }

    #[test]
    fn test_cooling_device() {
        let device = CoolingDevice::new("fan0".to_string(), CoolingDeviceType::Fan)
            .with_state(50)
            .with_max_state(100);

        assert_eq!(device.device_id, "fan0");
        assert_eq!(device.state, 50);
        assert_eq!(device.max_state, 100);
    }

    #[test]
    fn test_sigmathernal() {
        let mut thermal = SigmaThermal::new();
        assert!(!thermal.is_running());
        thermal.start();
        assert!(thermal.is_running());
        thermal.stop();
        assert!(!thermal.is_running());
    }

    #[test]
    fn test_sigmathernal_set_policy() {
        let mut thermal = SigmaThermal::new();
        thermal.start();

        assert!(thermal.set_policy(ThermalPolicy::Performance).is_ok());
        assert_eq!(thermal.get_policy(), ThermalPolicy::Performance);
    }

    #[test]
    fn test_sigmathernal_add_thermal_zone() {
        let mut thermal = SigmaThermal::new();

        let zone = ThermalZone::new("cpu0".to_string(), ThermalZoneType::CPU);
        assert!(thermal.add_thermal_zone(zone).is_ok());
        assert_eq!(thermal.thermal_zone_count(), 1);
    }

    #[test]
    fn test_sigmathernal_update_temperature() {
        let mut thermal = SigmaThermal::new();

        let zone = ThermalZone::new("cpu0".to_string(), ThermalZoneType::CPU);
        thermal.add_thermal_zone(zone).unwrap();

        assert!(thermal.update_temperature("cpu0", 50000).is_ok());

        let zone = thermal.get_thermal_zone("cpu0").unwrap();
        assert_eq!(zone.temperature, 50000);
    }

    #[test]
    fn test_sigmathernal_add_cooling_device() {
        let mut thermal = SigmaThermal::new();

        let device = CoolingDevice::new("fan0".to_string(), CoolingDeviceType::Fan);
        assert!(thermal.add_cooling_device(device).is_ok());
        assert_eq!(thermal.cooling_device_count(), 1);
    }

    #[test]
    fn test_sigmathernal_update_cooling_state() {
        let mut thermal = SigmaThermal::new();

        let device = CoolingDevice::new("fan0".to_string(), CoolingDeviceType::Fan);
        thermal.add_cooling_device(device).unwrap();

        assert!(thermal.update_cooling_state("fan0", 75).is_ok());

        let device = thermal.get_cooling_device("fan0").unwrap();
        assert_eq!(device.state, 75);
    }

    #[test]
    fn test_sigmathernal_check_trip_points() {
        let mut thermal = SigmaThermal::new();
        thermal.start();

        let zone = ThermalZone::new("cpu0".to_string(), ThermalZoneType::CPU)
            .with_temperature(95000)
            .with_trip_point(TripPointType::Critical, 90000);

        thermal.add_thermal_zone(zone).unwrap();

        let actions = thermal.check_trip_points();
        assert!(!actions.is_empty());
    }

    #[test]
    fn test_sigmathernal_max_temperature() {
        let mut thermal = SigmaThermal::new();

        let zone1 =
            ThermalZone::new("cpu0".to_string(), ThermalZoneType::CPU).with_temperature(45000);
        let zone2 =
            ThermalZone::new("gpu0".to_string(), ThermalZoneType::GPU).with_temperature(60000);

        thermal.add_thermal_zone(zone1).unwrap();
        thermal.add_thermal_zone(zone2).unwrap();

        assert_eq!(thermal.max_temperature(), 60000);
    }

    #[test]
    fn test_set_thermal_policy_convenience() {
        init_sigmathernal();

        let mut thermal = get_sigmathernal().unwrap();
        thermal.start();

        let result = set_thermal_policy(ThermalPolicy::Performance);
        assert!(result.is_ok());
    }

    #[test]
    fn test_update_temperature_convenience() {
        init_sigmathernal();

        let mut thermal = get_sigmathernal().unwrap();

        let zone = ThermalZone::new("cpu0".to_string(), ThermalZoneType::CPU);
        thermal.add_thermal_zone(zone).unwrap();

        let result = update_temperature("cpu0", 50000);
        assert!(result.is_ok());
    }
}
