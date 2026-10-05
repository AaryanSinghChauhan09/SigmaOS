//! Power Manager
//!
//! Power management system inspired by Linux Mint's power settings and Omarchy's
//! power utilities, supporting battery monitoring, power profiles, and sleep/hibernate.

use std::time::{SystemTime, UNIX_EPOCH};

/// Power source type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerSource {
    Battery,
    AC,
    UPS,
}

impl PowerSource {
    pub fn as_str(&self) -> &str {
        match self {
            PowerSource::Battery => "Battery",
            PowerSource::AC => "AC",
            PowerSource::UPS => "UPS",
        }
    }
}

/// Battery status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerBatteryStatus {
    Charging,
    Discharging,
    Full,
    NotCharging,
    Unknown,
}

impl PowerBatteryStatus {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "charging" => Some(PowerBatteryStatus::Charging),
            "discharging" => Some(PowerBatteryStatus::Discharging),
            "full" => Some(PowerBatteryStatus::Full),
            "not charging" => Some(PowerBatteryStatus::NotCharging),
            _ => Some(PowerBatteryStatus::Unknown),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            PowerBatteryStatus::Charging => "Charging",
            PowerBatteryStatus::Discharging => "Discharging",
            PowerBatteryStatus::Full => "Full",
            PowerBatteryStatus::NotCharging => "Not Charging",
            PowerBatteryStatus::Unknown => "Unknown",
        }
    }
}

/// Power profile
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerProfile {
    Performance,
    Balanced,
    PowerSaver,
}

impl PowerProfile {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "performance" => Some(PowerProfile::Performance),
            "balanced" => Some(PowerProfile::Balanced),
            "power saver" | "powersaver" => Some(PowerProfile::PowerSaver),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            PowerProfile::Performance => "Performance",
            PowerProfile::Balanced => "Balanced",
            PowerProfile::PowerSaver => "Power Saver",
        }
    }
}

/// Sleep action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SleepAction {
    Suspend,
    Hibernate,
    HybridSleep,
    Shutdown,
}

impl SleepAction {
    pub fn as_str(&self) -> &str {
        match self {
            SleepAction::Suspend => "Suspend",
            SleepAction::Hibernate => "Hibernate",
            SleepAction::HybridSleep => "Hybrid Sleep",
            SleepAction::Shutdown => "Shutdown",
        }
    }
}

/// Battery information
#[derive(Debug, Clone)]
pub struct BatteryInfo {
    pub percentage: f64,
    pub status: PowerBatteryStatus,
    pub capacity_wh: f64,
    pub energy_wh: f64,
    pub power_w: f64,
    pub voltage_v: f64,
    pub time_to_empty_minutes: Option<u32>,
    pub time_to_full_minutes: Option<u32>,
}

impl BatteryInfo {
    pub fn new() -> Self {
        Self {
            percentage: 100.0,
            status: PowerBatteryStatus::Full,
            capacity_wh: 50.0,
            energy_wh: 50.0,
            power_w: 0.0,
            voltage_v: 12.0,
            time_to_empty_minutes: None,
            time_to_full_minutes: None,
        }
    }

    pub fn update(&mut self, percentage: f64, status: PowerBatteryStatus) {
        self.percentage = percentage.clamp(0.0, 100.0);
        self.status = status;
        self.energy_wh = (self.capacity_wh * percentage) / 100.0;
    }

    pub fn is_low(&self, threshold: f64) -> bool {
        self.percentage < threshold && self.status == PowerBatteryStatus::Discharging
    }

    pub fn is_critical(&self, threshold: f64) -> bool {
        self.percentage < threshold && self.status == PowerBatteryStatus::Discharging
    }
}

impl Default for BatteryInfo {
    fn default() -> Self {
        Self::new()
    }
}

/// Power configuration
#[derive(Debug, Clone)]
pub struct PowerConfig {
    pub power_source: PowerSource,
    pub profile: PowerProfile,
    pub auto_sleep_enabled: bool,
    pub auto_sleep_timeout_minutes: u32,
    pub screen_dim_timeout_minutes: u32,
    pub screen_off_timeout_minutes: u32,
    pub low_battery_threshold: f64,
    pub critical_battery_threshold: f64,
    pub suspend_on_low_battery: bool,
    pub suspend_on_lid_close: bool,
}

impl Default for PowerConfig {
    fn default() -> Self {
        Self {
            power_source: PowerSource::AC,
            profile: PowerProfile::Balanced,
            auto_sleep_enabled: true,
            auto_sleep_timeout_minutes: 30,
            screen_dim_timeout_minutes: 5,
            screen_off_timeout_minutes: 10,
            low_battery_threshold: 20.0,
            critical_battery_threshold: 5.0,
            suspend_on_low_battery: true,
            suspend_on_lid_close: true,
        }
    }
}

/// Power manager
#[derive(Debug)]
pub struct PowerManager {
    battery: BatteryInfo,
    config: PowerConfig,
    last_update: u64,
}

impl PowerManager {
    pub fn new() -> Self {
        let last_update = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            battery: BatteryInfo::new(),
            config: PowerConfig::default(),
            last_update,
        }
    }

    /// Get battery information
    pub fn get_battery(&self) -> &BatteryInfo {
        &self.battery
    }

    /// Get battery mutably
    pub fn get_battery_mut(&mut self) -> &mut BatteryInfo {
        &mut self.battery
    }

    /// Get power configuration
    pub fn get_config(&self) -> &PowerConfig {
        &self.config
    }

    /// Set power configuration
    pub fn set_config(&mut self, config: PowerConfig) {
        self.config = config;
    }

    /// Set power profile
    pub fn set_profile(&mut self, profile: PowerProfile) {
        self.config.profile = profile;
    }

    /// Get current power profile
    pub fn get_profile(&self) -> PowerProfile {
        self.config.profile
    }

    /// Update battery status
    pub fn update_battery(&mut self, percentage: f64, status: PowerBatteryStatus) {
        self.battery.update(percentage, status);
        self.last_update = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    /// Check if battery is low
    pub fn is_battery_low(&self) -> bool {
        self.battery.is_low(self.config.low_battery_threshold)
    }

    /// Check if battery is critical
    pub fn is_battery_critical(&self) -> bool {
        self.battery.is_critical(self.config.critical_battery_threshold)
    }

    /// Suggest sleep action based on battery level
    pub fn suggest_sleep_action(&self) -> Option<SleepAction> {
        if self.is_battery_critical() && self.config.suspend_on_low_battery {
            Some(SleepAction::Suspend)
        } else {
            None
        }
    }

    /// Calculate estimated time remaining
    pub fn get_time_remaining(&self) -> Option<String> {
        if let Some(minutes) = self.battery.time_to_empty_minutes {
            if minutes >= 60 {
                let hours = minutes / 60;
                let mins = minutes % 60;
                Some(format!("{}h {}m", hours, mins))
            } else {
                Some(format!("{}m", minutes))
            }
        } else {
            None
        }
    }

    /// Get power statistics
    pub fn get_statistics(&self) -> PowerStatistics {
        PowerStatistics {
            battery_percentage: self.battery.percentage,
            battery_status: self.battery.status,
            power_source: self.config.power_source,
            power_profile: self.config.profile,
            is_low: self.is_battery_low(),
            is_critical: self.is_battery_critical(),
            last_update: self.last_update,
        }
    }
}

impl Default for PowerManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Power statistics
#[derive(Debug, Clone)]
pub struct PowerStatistics {
    pub battery_percentage: f64,
    pub battery_status: PowerBatteryStatus,
    pub power_source: PowerSource,
    pub power_profile: PowerProfile,
    pub is_low: bool,
    pub is_critical: bool,
    pub last_update: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_battery_status_from_str() {
        assert_eq!(PowerBatteryStatus::from_str("charging"), Some(PowerBatteryStatus::Charging));
        assert_eq!(PowerBatteryStatus::from_str("discharging"), Some(PowerBatteryStatus::Discharging));
    }

    #[test]
    fn test_power_profile_from_str() {
        assert_eq!(PowerProfile::from_str("performance"), Some(PowerProfile::Performance));
        assert_eq!(PowerProfile::from_str("balanced"), Some(PowerProfile::Balanced));
    }

    #[test]
    fn test_battery_info_creation() {
        let battery = BatteryInfo::new();
        assert_eq!(battery.percentage, 100.0);
        assert_eq!(battery.status, PowerBatteryStatus::Full);
    }

    #[test]
    fn test_battery_update() {
        let mut battery = BatteryInfo::new();
        battery.update(75.0, PowerBatteryStatus::Discharging);
        assert_eq!(battery.percentage, 75.0);
        assert_eq!(battery.status, PowerBatteryStatus::Discharging);
    }

    #[test]
    fn test_battery_low() {
        let mut battery = BatteryInfo::new();
        battery.update(15.0, PowerBatteryStatus::Discharging);
        assert!(battery.is_low(20.0));
    }

    #[test]
    fn test_power_manager_creation() {
        let manager = PowerManager::new();
        assert_eq!(manager.get_profile(), PowerProfile::Balanced);
    }

    #[test]
    fn test_set_profile() {
        let mut manager = PowerManager::new();
        manager.set_profile(PowerProfile::Performance);
        assert_eq!(manager.get_profile(), PowerProfile::Performance);
    }

    #[test]
    fn test_update_battery() {
        let mut manager = PowerManager::new();
        manager.update_battery(50.0, PowerBatteryStatus::Discharging);
        assert_eq!(manager.get_battery().percentage, 50.0);
    }

    #[test]
    fn test_battery_low_detection() {
        let mut manager = PowerManager::new();
        manager.update_battery(15.0, PowerBatteryStatus::Discharging);
        assert!(manager.is_battery_low());
    }

    #[test]
    fn test_battery_critical_detection() {
        let mut manager = PowerManager::new();
        manager.update_battery(3.0, PowerBatteryStatus::Discharging);
        assert!(manager.is_battery_critical());
    }

    #[test]
    fn test_statistics() {
        let manager = PowerManager::new();
        let stats = manager.get_statistics();
        assert_eq!(stats.battery_percentage, 100.0);
    }
}
