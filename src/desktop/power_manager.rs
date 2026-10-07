// Desktop Power Manager
// Linux Mint & Omarchy inspiration for comprehensive power management

use std::collections::HashMap;

/// Power Action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerAction {
    Nothing,
    Suspend,
    Hibernate,
    Shutdown,
    Reboot,
    ScreenOff,
}

impl PowerAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            PowerAction::Nothing => "nothing",
            PowerAction::Suspend => "suspend",
            PowerAction::Hibernate => "hibernate",
            PowerAction::Shutdown => "shutdown",
            PowerAction::Reboot => "reboot",
            PowerAction::ScreenOff => "screen-off",
        }
    }
}

/// Power Profile
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerProfile {
    Performance,
    Balanced,
    PowerSaver,
}

impl PowerProfile {
    pub fn as_str(&self) -> &'static str {
        match self {
            PowerProfile::Performance => "performance",
            PowerProfile::Balanced => "balanced",
            PowerProfile::PowerSaver => "power-saver",
        }
    }
}

/// Battery Status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatteryStatus {
    Charging,
    Discharging,
    Full,
    Unknown,
}

impl BatteryStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            BatteryStatus::Charging => "charging",
            BatteryStatus::Discharging => "discharging",
            BatteryStatus::Full => "full",
            BatteryStatus::Unknown => "unknown",
        }
    }
}

/// Battery Device
#[derive(Debug, Clone)]
pub struct BatteryDevice {
    pub id: String,
    pub name: String,
    pub capacity: u32,      // 0-100 percentage
    pub status: BatteryStatus,
    pub health: u32,        // 0-100 percentage
    pub voltage: f32,       // Volts
    pub current: f32,       // Amperes
    pub time_remaining: Option<u32>, // minutes
}

impl BatteryDevice {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            capacity: 100,
            status: BatteryStatus::Unknown,
            health: 100,
            voltage: 0.0,
            current: 0.0,
            time_remaining: None,
        }
    }

    pub fn update_capacity(&mut self, capacity: u32) {
        self.capacity = capacity.min(100);
    }

    pub fn update_status(&mut self, status: BatteryStatus) {
        self.status = status;
    }

    pub fn update_time_remaining(&mut self, minutes: u32) {
        self.time_remaining = Some(minutes);
    }
}

/// Desktop Power Manager
pub struct DesktopPowerManager {
    batteries: HashMap<String, BatteryDevice>,
    current_profile: PowerProfile,
    lid_close_action: PowerAction,
    power_button_action: PowerAction,
    battery_critical_action: PowerAction,
    auto_suspend_timeout: u32, // minutes
    screen_off_timeout: u32,   // minutes
    counter: u32,
}

impl DesktopPowerManager {
    pub fn new() -> Self {
        let mut manager = Self {
            batteries: HashMap::new(),
            current_profile: PowerProfile::Balanced,
            lid_close_action: PowerAction::Suspend,
            power_button_action: PowerAction::ScreenOff,
            battery_critical_action: PowerAction::Suspend,
            auto_suspend_timeout: 30,
            screen_off_timeout: 10,
            counter: 1000,
        };

        // Add default battery
        manager.add_default_battery();

        manager
    }

    fn add_default_battery(&mut self) {
        let battery = BatteryDevice::new(
            "battery_0".to_string(),
            "Battery 0".to_string(),
        );

        self.batteries.insert(battery.id.clone(), battery);
    }

    pub fn add_battery(&mut self, battery: BatteryDevice) -> String {
        let id = format!("battery_{}", self.counter);
        self.counter += 1;

        let battery = BatteryDevice {
            id: id.clone(),
            ..battery
        };

        self.batteries.insert(id.clone(), battery);
        id
    }

    pub fn remove_battery(&mut self, id: &str) -> bool {
        self.batteries.remove(id).is_some()
    }

    pub fn get_battery(&self, id: &str) -> Option<&BatteryDevice> {
        self.batteries.get(id)
    }

    pub fn get_batteries(&self) -> Vec<&BatteryDevice> {
        self.batteries.values().collect()
    }

    pub fn update_battery_capacity(&mut self, id: &str, capacity: u32) -> bool {
        if let Some(battery) = self.batteries.get_mut(id) {
            battery.update_capacity(capacity);
            true
        } else {
            false
        }
    }

    pub fn update_battery_status(&mut self, id: &str, status: BatteryStatus) -> bool {
        if let Some(battery) = self.batteries.get_mut(id) {
            battery.update_status(status);
            true
        } else {
            false
        }
    }

    pub fn update_battery_time_remaining(&mut self, id: &str, minutes: u32) -> bool {
        if let Some(battery) = self.batteries.get_mut(id) {
            battery.update_time_remaining(minutes);
            true
        } else {
            false
        }
    }

    pub fn set_power_profile(&mut self, profile: PowerProfile) {
        self.current_profile = profile;
    }

    pub fn get_power_profile(&self) -> PowerProfile {
        self.current_profile
    }

    pub fn set_lid_close_action(&mut self, action: PowerAction) {
        self.lid_close_action = action;
    }

    pub fn get_lid_close_action(&self) -> PowerAction {
        self.lid_close_action
    }

    pub fn set_power_button_action(&mut self, action: PowerAction) {
        self.power_button_action = action;
    }

    pub fn get_power_button_action(&self) -> PowerAction {
        self.power_button_action
    }

    pub fn set_battery_critical_action(&mut self, action: PowerAction) {
        self.battery_critical_action = action;
    }

    pub fn get_battery_critical_action(&self) -> PowerAction {
        self.battery_critical_action
    }

    pub fn set_auto_suspend_timeout(&mut self, minutes: u32) {
        self.auto_suspend_timeout = minutes;
    }

    pub fn get_auto_suspend_timeout(&self) -> u32 {
        self.auto_suspend_timeout
    }

    pub fn set_screen_off_timeout(&mut self, minutes: u32) {
        self.screen_off_timeout = minutes;
    }

    pub fn get_screen_off_timeout(&self) -> u32 {
        self.screen_off_timeout
    }

    pub fn get_average_capacity(&self) -> f32 {
        if self.batteries.is_empty() {
            return 0.0;
        }

        let total: u32 = self.batteries.values().map(|b| b.capacity).sum();
        total as f32 / self.batteries.len() as f32
    }

    pub fn is_on_battery(&self) -> bool {
        self.batteries
            .values()
            .any(|b| b.status == BatteryStatus::Discharging)
    }

    pub fn is_charging(&self) -> bool {
        self.batteries
            .values()
            .any(|b| b.status == BatteryStatus::Charging)
    }

    pub fn get_battery_critical(&self, threshold: u32) -> bool {
        self.batteries
            .values()
            .any(|b| b.capacity < threshold && b.status == BatteryStatus::Discharging)
    }

    pub fn get_statistics(&self) -> PowerManagerStatistics {
        PowerManagerStatistics {
            total_batteries: self.batteries.len(),
            average_capacity: self.get_average_capacity(),
            on_battery: self.is_on_battery(),
            charging: self.is_charging(),
            current_profile: self.current_profile,
        }
    }
}

impl Default for DesktopPowerManager {
    fn default() -> Self {
        Self::new()
    }
}

/// PowerManagerStatistics
#[derive(Debug, Clone, Copy)]
pub struct PowerManagerStatistics {
    pub total_batteries: usize,
    pub average_capacity: f32,
    pub on_battery: bool,
    pub charging: bool,
    pub current_profile: PowerProfile,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_manager_state() {
        let manager = DesktopPowerManager::new();
        let stats = manager.get_statistics();

        assert_eq!(stats.total_batteries, 1);
        assert_eq!(stats.current_profile, PowerProfile::Balanced);
        assert_eq!(manager.get_lid_close_action(), PowerAction::Suspend);
        assert_eq!(manager.get_power_button_action(), PowerAction::ScreenOff);
        assert_eq!(manager.get_auto_suspend_timeout(), 30);
        assert_eq!(manager.get_screen_off_timeout(), 10);
    }

    #[test]
    fn test_add_battery() {
        let mut manager = DesktopPowerManager::new();
        let initial_count = manager.get_batteries().len();

        let battery = BatteryDevice::new(
            "custom".to_string(),
            "Custom Battery".to_string(),
        );

        let id = manager.add_battery(battery);
        assert!(manager.get_battery(&id).is_some());
        assert_eq!(manager.get_batteries().len(), initial_count + 1);
    }

    #[test]
    fn test_remove_battery() {
        let mut manager = DesktopPowerManager::new();

        let battery = BatteryDevice::new(
            "custom".to_string(),
            "Custom Battery".to_string(),
        );

        let id = manager.add_battery(battery);
        assert!(manager.remove_battery(&id));
        assert!(manager.get_battery(&id).is_none());
    }

    #[test]
    fn test_update_battery_capacity() {
        let mut manager = DesktopPowerManager::new();
        let battery_id = "battery_0";

        assert!(manager.update_battery_capacity(battery_id, 75));
        let battery = manager.get_battery(battery_id).unwrap();
        assert_eq!(battery.capacity, 75);
    }

    #[test]
    fn test_update_battery_status() {
        let mut manager = DesktopPowerManager::new();
        let battery_id = "battery_0";

        assert!(manager.update_battery_status(battery_id, BatteryStatus::Charging));
        let battery = manager.get_battery(battery_id).unwrap();
        assert_eq!(battery.status, BatteryStatus::Charging);
    }

    #[test]
    fn test_set_power_profile() {
        let mut manager = DesktopPowerManager::new();

        manager.set_power_profile(PowerProfile::Performance);
        assert_eq!(manager.get_power_profile(), PowerProfile::Performance);

        manager.set_power_profile(PowerProfile::PowerSaver);
        assert_eq!(manager.get_power_profile(), PowerProfile::PowerSaver);
    }

    #[test]
    fn test_set_lid_close_action() {
        let mut manager = DesktopPowerManager::new();

        manager.set_lid_close_action(PowerAction::Hibernate);
        assert_eq!(manager.get_lid_close_action(), PowerAction::Hibernate);
    }

    #[test]
    fn test_set_power_button_action() {
        let mut manager = DesktopPowerManager::new();

        manager.set_power_button_action(PowerAction::Shutdown);
        assert_eq!(manager.get_power_button_action(), PowerAction::Shutdown);
    }

    #[test]
    fn test_set_timeouts() {
        let mut manager = DesktopPowerManager::new();

        manager.set_auto_suspend_timeout(60);
        assert_eq!(manager.get_auto_suspend_timeout(), 60);

        manager.set_screen_off_timeout(5);
        assert_eq!(manager.get_screen_off_timeout(), 5);
    }

    #[test]
    fn test_is_on_battery() {
        let mut manager = DesktopPowerManager::new();
        let battery_id = "battery_0";

        manager.update_battery_status(battery_id, BatteryStatus::Discharging);
        assert!(manager.is_on_battery());

        manager.update_battery_status(battery_id, BatteryStatus::Charging);
        assert!(!manager.is_on_battery());
    }

    #[test]
    fn test_is_charging() {
        let mut manager = DesktopPowerManager::new();
        let battery_id = "battery_0";

        manager.update_battery_status(battery_id, BatteryStatus::Charging);
        assert!(manager.is_charging());

        manager.update_battery_status(battery_id, BatteryStatus::Discharging);
        assert!(!manager.is_charging());
    }

    #[test]
    fn test_battery_critical() {
        let mut manager = DesktopPowerManager::new();
        let battery_id = "battery_0";

        manager.update_battery_capacity(battery_id, 50);
        manager.update_battery_status(battery_id, BatteryStatus::Discharging);
        assert!(!manager.get_battery_critical(20));

        manager.update_battery_capacity(battery_id, 15);
        assert!(manager.get_battery_critical(20));
    }

    #[test]
    fn test_average_capacity() {
        let mut manager = DesktopPowerManager::new();

        manager.update_battery_capacity("battery_0", 80);

        let battery2 = BatteryDevice::new(
            "custom".to_string(),
            "Battery 2".to_string(),
        );
        let id2 = manager.add_battery(battery2);
        manager.update_battery_capacity(&id2, 60);

        let avg = manager.get_average_capacity();
        assert!((avg - 70.0).abs() < 0.1);
    }
}
