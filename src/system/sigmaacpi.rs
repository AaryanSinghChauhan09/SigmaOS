//! SigmaACPI - ACPI Power Management System
//!
//! Provides comprehensive ACPI power management inspired by Linux ACPI and BSD ACPI.
//! Supports power events, battery management, and thermal control.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Power event type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerEvent {
    PowerButton,
    SleepButton,
    LidOpen,
    LidClose,
    Unknown,
;

/// Battery state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatteryState {
    Charging,
    Discharging,
    Full,
    Unknown,
;

/// Thermal state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThermalState {
    Normal,
    Warning,
    Critical,
    Unknown,
;

/// Battery information
#[derive(Debug, Clone)]
pub struct BatteryInfo {
    pub id: String,
    pub capacity: u32,
    pub remaining: u32,
    pub state: BatteryState,
    pub voltage: Option<u32>,
    pub temperature: Option<i32>,
;

impl BatteryInfo {
    pub fn new(id: String, capacity: u32) -> Self {
        Self {
            id,
            capacity,
            remaining: 0,
            state: BatteryState::Unknown,
            voltage: None,
            temperature: None,
        }
    }

    pub fn with_remaining(mut self, remaining: u32) -> Self {
        self.remaining = remaining;
        self
    }

    pub fn with_state(mut self, state: BatteryState) -> Self {
        self.state = state;
        self
    }

    pub fn with_voltage(mut self, voltage: u32) -> Self {
        self.voltage = Some(voltage);
        self
    }

    pub fn with_temperature(mut self, temp: i32) -> Self {
        self.temperature = Some(temp);
        self
    }

    pub fn percentage(&self) -> u32 {
        if self.capacity == 0 {
            0
        } else {
            (self.remaining * 100) / self.capacity
        }
    }
;

/// Thermal zone
#[derive(Debug, Clone)]
pub struct ThermalZone {
    pub id: String,
    pub name: String,
    pub temperature: i32,
    pub trip_points: Vec<i32>,
    pub state: ThermalState,
;

impl ThermalZone {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            temperature: 0,
            trip_points: Vec::new(),
            state: ThermalState::Normal,
        }
    }

    pub fn with_temperature(mut self, temp: i32) -> Self {
        self.temperature = temp;
        self
    }

    pub fn with_trip_points(mut self, points: Vec<i32>) -> Self {
        self.trip_points = points;
        self
    }

    pub fn update_state(&mut self) {
        if self.trip_points.is_empty() {
            self.state = ThermalState::Normal;
        } else if self.temperature >= self.trip_points[1] {
            self.state = ThermalState::Critical;
        } else if self.temperature >= self.trip_points[0] {
            self.state = ThermalState::Warning;
        } else {
            self.state = ThermalState::Normal;
        }
    }
;

/// ACPI manager
pub struct SigmaACPI {
    batteries: BTreeMap<String, BatteryInfo>,
    thermal_zones: BTreeMap<String, ThermalZone>,
    next_battery_id: u64,
    next_thermal_id: u64,
    ac_power: bool,
    lid_state: bool,
;

impl SigmaACPI {
    pub fn new() -> Self {
        Self {
            batteries: BTreeMap::new(),
            thermal_zones: BTreeMap::new(),
            next_battery_id: 1,
            next_thermal_id: 1,
            ac_power: true,
            lid_state: true,
        }
    }

    pub fn add_battery(&mut self, capacity: u32) -> String {
        let id = format!("battery-{}", self.next_battery_id);
        self.next_battery_id += 1;
        let battery = BatteryInfo::new(id.clone(), capacity);
        self.batteries.insert(id.clone(), battery);
        id
    }

    pub fn get_battery(&self, id: &str) -> Option<&BatteryInfo> {
        self.batteries.get(id)
    }

    pub fn get_battery_mut(&mut self, id: &str) -> Option<&mut BatteryInfo> {
        self.batteries.get_mut(id)
    }

    pub fn list_batteries(&self) -> Vec<&BatteryInfo> {
        self.batteries.values().collect()
    }

    pub fn remove_battery(&mut self, id: &str) -> bool {
        self.batteries.remove(id).is_some()
    }

    pub fn add_thermal_zone(&mut self, name: String) -> String {
        let id = format!("thermal-{}", self.next_thermal_id);
        self.next_thermal_id += 1;
        let zone = ThermalZone::new(id.clone(), name);
        self.thermal_zones.insert(id.clone(), zone);
        id
    }

    pub fn get_thermal_zone(&self, id: &str) -> Option<&ThermalZone> {
        self.thermal_zones.get(id)
    }

    pub fn get_thermal_zone_mut(&mut self, id: &str) -> Option<&mut ThermalZone> {
        self.thermal_zones.get_mut(id)
    }

    pub fn list_thermal_zones(&self) -> Vec<&ThermalZone> {
        self.thermal_zones.values().collect()
    }

    pub fn remove_thermal_zone(&mut self, id: &str) -> bool {
        self.thermal_zones.remove(id).is_some()
    }

    pub fn set_ac_power(&mut self, ac_power: bool) {
        self.ac_power = ac_power;
    }

    pub fn get_ac_power(&self) -> bool {
        self.ac_power
    }

    pub fn set_lid_state(&mut self, open: bool) {
        self.lid_state = open;
    }

    pub fn get_lid_state(&self) -> bool {
        self.lid_state
    }

    pub fn battery_count(&self) -> usize {
        self.batteries.len()
    }

    pub fn thermal_zone_count(&self) -> usize {
        self.thermal_zones.len()
    }
;

impl Default for SigmaACPI {
    fn default() -> Self {
        Self::new()
    }
;

static mut SIGMAACPI_INSTANCE: Option<SigmaACPI> = None;

pub fn init_sigmaacpi() {
    unsafe {
        SIGMAACPI_INSTANCE = Some(SigmaACPI::new());
    }
;

pub fn get_sigmaacpi() -> &'static SigmaACPI {
    unsafe {
        SIGMAACPI_INSTANCE
            .as_ref()
            .expect("SigmaACPI not initialized")
    }
;

pub fn get_sigmaacpi_mut() -> &'static mut SigmaACPI {
    unsafe {
        SIGMAACPI_INSTANCE
            .as_mut()
            .expect("SigmaACPI not initialized")
    }
;

pub fn add_battery(capacity: u32) -> String {
    let manager = get_sigmaacpi_mut();
    manager.add_battery(capacity)
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_battery_info_creation() {
        let battery = BatteryInfo::new("battery-1".to_string(), 5000);
        assert_eq!(battery.id, "battery-1");
        assert_eq!(battery.capacity, 5000);
        assert_eq!(battery.remaining, 0);
    }

    #[test]
    fn test_battery_builder() {
        let battery = BatteryInfo::new("battery-1".to_string(), 5000)
            .with_remaining(2500)
            .with_state(BatteryState::Charging)
            .with_voltage(12000)
            .with_temperature(30);

        assert_eq!(battery.remaining, 2500);
        assert_eq!(battery.state, BatteryState::Charging);
        assert_eq!(battery.voltage, Some(12000));
        assert_eq!(battery.temperature, Some(30));
    }

    #[test]
    fn test_battery_percentage() {
        let battery = BatteryInfo::new("battery-1".to_string(), 5000).with_remaining(2500);
        assert_eq!(battery.percentage(), 50);
    }

    #[test]
    fn test_thermal_zone_creation() {
        let zone = ThermalZone::new("thermal-1".to_string(), "CPU".to_string());
        assert_eq!(zone.id, "thermal-1");
        assert_eq!(zone.name, "CPU");
        assert_eq!(zone.temperature, 0);
    }

    #[test]
    fn test_thermal_zone_state() {
        let mut zone = ThermalZone::new("thermal-1".to_string(), "CPU".to_string())
            .with_temperature(50)
            .with_trip_points(vec![60, 80]);

        zone.update_state();
        assert_eq!(zone.state, ThermalState::Normal);

        zone.temperature = 65;
        zone.update_state();
        assert_eq!(zone.state, ThermalState::Warning);

        zone.temperature = 85;
        zone.update_state();
        assert_eq!(zone.state, ThermalState::Critical);
    }

    #[test]
    fn test_acpi_manager() {
        let manager = SigmaACPI::new();
        assert_eq!(manager.battery_count(), 0);
        assert_eq!(manager.thermal_zone_count(), 0);
        assert!(manager.get_ac_power());
    }

    #[test]
    fn test_add_battery() {
        let mut manager = SigmaACPI::new();
        let id = manager.add_battery(5000);
        assert_eq!(id, "battery-1");
        assert_eq!(manager.battery_count(), 1);
    }

    #[test]
    fn test_add_thermal_zone() {
        let mut manager = SigmaACPI::new();
        let id = manager.add_thermal_zone("CPU".to_string());
        assert_eq!(id, "thermal-1");
        assert_eq!(manager.thermal_zone_count(), 1);
    }

    #[test]
    fn test_ac_power_control() {
        let mut manager = SigmaACPI::new();
        assert!(manager.get_ac_power());

        manager.set_ac_power(false);
        assert!(!manager.get_ac_power());
    }

    #[test]
    fn test_lid_state_control() {
        let mut manager = SigmaACPI::new();
        assert!(manager.get_lid_state());

        manager.set_lid_state(false);
        assert!(!manager.get_lid_state());
    }

    #[test]
    fn test_global_instance() {
        init_sigmaacpi();
        let manager = get_sigmaacpi();
        assert_eq!(manager.battery_count(), 0);

        let manager_mut = get_sigmaacpi_mut();
        manager_mut.add_battery(5000);
        assert_eq!(manager.battery_count(), 1);
    }

    #[test]
    fn test_add_battery_convenience() {
        init_sigmaacpi();
        let id = add_battery(5000);
        assert_eq!(id, "battery-1");

        let manager = get_sigmaacpi();
        assert!(manager.get_battery(&id).is_some());
    }
;
