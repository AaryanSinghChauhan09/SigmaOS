//! SigmaOS Battery Manager
//!
//! Inspired by: Omarchy battery controls (omancy-battery-low, omancy-battery-present, omancy-battery-status)
//! Following principles: SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY,
//! LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING,
//! SECURE_DEFAULTS, POWER_MANAGEMENT, LOW_LEVEL_CONTROL, DIRECT_DEVICE_CONTROL,
/// USER_NOTIFICATION, MEMORY_SAFETY.

/// Battery status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BatteryStatus {
    Charging,
    Discharging,
    Full,
    Empty,
    Unknown,
}

/// Battery device
#[derive(Debug, Clone)]
pub struct BatteryDevice {
    pub id: String,
    pub name: String,
    pub present: bool,
    pub charge_percent: f32,
    pub status: BatteryStatus,
    pub capacity_mah: Option<u32>,
    pub current_mah: Option<i32>,
    pub voltage_mv: Option<u32>,
    pub temperature_c: Option<f32>,
    pub cycle_count: Option<u32>,
}

/// Battery manager
pub struct BatteryManager {
    pub batteries: HashMap<String, BatteryDevice>,
    pub low_threshold: f32,
    pub critical_threshold: f32,
}

impl BatteryManager {
    pub fn new() -> Self {
        Self {
            batteries: HashMap::new(),
            low_threshold: 20.0,
            critical_threshold: 10.0,
        }
    }

    /// Add a battery device
    pub fn add_battery(&mut self, battery: BatteryDevice) {
        self.batteries.insert(battery.id.clone(), battery);
    }

    /// Get battery by ID
    pub fn get_battery(&self, id: &str) -> Option<&BatteryDevice> {
        self.batteries.get(id)
    }

    /// List all batteries
    pub fn list_batteries(&self) -> Vec<&BatteryDevice> {
        self.batteries.values().collect()
    }

    /// Update battery charge
    pub fn update_charge(&mut self, id: &str, percent: f32) -> Result<(), String> {
        let battery = self
            .batteries
            .get_mut(id)
            .ok_or_else(|| format!("Battery {} not found", id))?;
        if percent < 0.0 || percent > 100.0 {
            return Err("Charge percent must be between 0 and 100".to_string());
        }
        battery.charge_percent = percent;
        Ok(())
    }

    /// Update battery status
    pub fn update_status(&mut self, id: &str, status: BatteryStatus) -> Result<(), String> {
        let battery = self
            .batteries
            .get_mut(id)
            .ok_or_else(|| format!("Battery {} not found", id))?;
        battery.status = status;
        Ok(())
    }

    /// Get total charge percent (average across batteries)
    pub fn total_charge_percent(&self) -> f32 {
        if self.batteries.is_empty() {
            return 0.0;
        }
        self.batteries
            .values()
            .map(|b| b.charge_percent)
            .sum::<f32>()
            / self.batteries.len() as f32
    }

    /// Is any battery low
    pub fn is_any_low(&self) -> bool {
        self.batteries
            .values()
            .any(|b| b.charge_percent <= self.low_threshold)
    }

    /// Is any battery critical
    pub fn is_any_critical(&self) -> bool {
        self.batteries
            .values()
            .any(|b| b.charge_percent <= self.critical_threshold)
    }

    /// Is any battery charging
    pub fn is_any_charging(&self) -> bool {
        self.batteries
            .values()
            .any(|b| b.status == BatteryStatus::Charging)
    }

    /// Set low threshold
    pub fn set_low_threshold(&mut self, percent: f32) -> Result<(), String> {
        if percent < 0.0 || percent > 100.0 {
            return Err("Threshold must be between 0 and 100".to_string());
        }
        self.low_threshold = percent;
        Ok(())
    }

    /// Set critical threshold
    pub fn set_critical_threshold(&mut self, percent: f32) -> Result<(), String> {
        if percent < 0.0 || percent > 100.0 {
            return Err("Threshold must be between 0 and 100".to_string());
        }
        self.critical_threshold = percent;
        Ok(())
    }

    /// Battery count
    pub fn battery_count(&self) -> usize {
        self.batteries.len()
    }

    /// Is any battery present
    pub fn is_any_present(&self) -> bool {
        self.batteries.values().any(|b| b.present)
    }
}

impl Default for BatteryManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_battery() {
        let mut mgr = BatteryManager::new();
        let battery = BatteryDevice {
            id: "bat0".to_string(),
            name: "Battery 0".to_string(),
            present: true,
            charge_percent: 85.0,
            status: BatteryStatus::Discharging,
            capacity_mah: Some(5000),
            current_mah: None,
            voltage_mv: None,
            temperature_c: None,
            cycle_count: None,
        };
        mgr.add_battery(battery);
        assert_eq!(mgr.battery_count(), 1);
        assert!(mgr.is_any_present());
    }

    #[test]
    fn test_update_charge() {
        let mut mgr = BatteryManager::new();
        mgr.add_battery(BatteryDevice {
            id: "bat0".to_string(),
            name: "Battery 0".to_string(),
            present: true,
            charge_percent: 50.0,
            status: BatteryStatus::Discharging,
            capacity_mah: None,
            current_mah: None,
            voltage_mv: None,
            temperature_c: None,
            cycle_count: None,
        });
        assert!(mgr.update_charge("bat0", 75.0).is_ok());
        assert_eq!(mgr.get_battery("bat0").unwrap().charge_percent, 75.0);
    }

    #[test]
    fn test_update_charge_invalid() {
        let mut mgr = BatteryManager::new();
        mgr.add_battery(BatteryDevice {
            id: "bat0".to_string(),
            name: "Battery 0".to_string(),
            present: true,
            charge_percent: 50.0,
            status: BatteryStatus::Discharging,
            capacity_mah: None,
            current_mah: None,
            voltage_mv: None,
            temperature_c: None,
            cycle_count: None,
        });
        assert!(mgr.update_charge("bat0", 150.0).is_err());
    }

    #[test]
    fn test_low_threshold() {
        let mut mgr = BatteryManager::new();
        mgr.add_battery(BatteryDevice {
            id: "bat0".to_string(),
            name: "Battery 0".to_string(),
            present: true,
            charge_percent: 15.0,
            status: BatteryStatus::Discharging,
            capacity_mah: None,
            current_mah: None,
            voltage_mv: None,
            temperature_c: None,
            cycle_count: None,
        });
        assert!(mgr.is_any_low());
        assert!(!mgr.is_any_critical());
    }

    #[test]
    fn test_critical_threshold() {
        let mut mgr = BatteryManager::new();
        mgr.add_battery(BatteryDevice {
            id: "bat0".to_string(),
            name: "Battery 0".to_string(),
            present: true,
            charge_percent: 5.0,
            status: BatteryStatus::Discharging,
            capacity_mah: None,
            current_mah: None,
            voltage_mv: None,
            temperature_c: None,
            cycle_count: None,
        });
        assert!(mgr.is_any_low());
        assert!(mgr.is_any_critical());
    }

    #[test]
    fn test_is_charging() {
        let mut mgr = BatteryManager::new();
        mgr.add_battery(BatteryDevice {
            id: "bat0".to_string(),
            name: "Battery 0".to_string(),
            present: true,
            charge_percent: 50.0,
            status: BatteryStatus::Charging,
            capacity_mah: None,
            current_mah: None,
            voltage_mv: None,
            temperature_c: None,
            cycle_count: None,
        });
        assert!(mgr.is_any_charging());
    }

    #[test]
    fn test_total_charge() {
        let mut mgr = BatteryManager::new();
        mgr.add_battery(BatteryDevice {
            id: "bat0".to_string(),
            name: "Battery 0".to_string(),
            present: true,
            charge_percent: 80.0,
            status: BatteryStatus::Discharging,
            capacity_mah: None,
            current_mah: None,
            voltage_mv: None,
            temperature_c: None,
            cycle_count: None,
        });
        mgr.add_battery(BatteryDevice {
            id: "bat1".to_string(),
            name: "Battery 1".to_string(),
            present: true,
            charge_percent: 40.0,
            status: BatteryStatus::Discharging,
            capacity_mah: None,
            current_mah: None,
            voltage_mv: None,
            temperature_c: None,
            cycle_count: None,
        });
        assert!((mgr.total_charge_percent() - 60.0).abs() < 0.01);
    }

    #[test]
    fn test_set_thresholds() {
        let mut mgr = BatteryManager::new();
        assert!(mgr.set_low_threshold(15.0).is_ok());
        assert_eq!(mgr.low_threshold, 15.0);
        assert!(mgr.set_low_threshold(101.0).is_err());
    }
}