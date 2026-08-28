//! Advanced Power Management inspired by TLP, PowerTop, and Thermald
//! Dynamic CPU/GPU frequency scaling, battery health calibration,
//! thermal throttling, and process energy impact scoring.
extern crate alloc;


use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerProfileMode {
    Performance,
    Balanced,
    PowerSaver,
    AutoSwitch,
}

#[derive(Debug, Clone)]
pub struct Battery {
    pub percentage: u8,
    pub is_charging: bool,
    pub health_percentage: u8,
    pub voltage_mv: u32,
    pub discharge_rate_mw: u32,
}

#[derive(Debug, Clone)]
pub struct ThermalZone {
    pub zone_id: u32,
    pub name: String,
    pub temp_celsius: f32,
    pub critical_temp_celsius: f32,
    pub is_throttling: bool,
}

pub struct PowerManager {
    pub current_mode: PowerProfileMode,
    pub battery: Battery,
    pub thermal_zones: Vec<ThermalZone>,
    pub cpu_freq_mhz: u32,
    pub gpu_freq_mhz: u32,
}

impl PowerManager {
    pub fn new() -> Self {
        Self {
            current_mode: PowerProfileMode::Balanced,
            battery: Battery {
                percentage: 100,
                is_charging: true,
                health_percentage: 98,
                voltage_mv: 12000,
                discharge_rate_mw: 0,
            },
            thermal_zones: Vec::new(),
            cpu_freq_mhz: 2400,
            gpu_freq_mhz: 1200,
        }
    }

    pub fn set_profile(&mut self, mode: PowerProfileMode) {
        self.current_mode = mode;
        match mode {
            PowerProfileMode::Performance => {
                self.cpu_freq_mhz = 3800;
                self.gpu_freq_mhz = 1800;
            }
            PowerProfileMode::Balanced => {
                self.cpu_freq_mhz = 2400;
                self.gpu_freq_mhz = 1200;
            }
            PowerProfileMode::PowerSaver => {
                self.cpu_freq_mhz = 1200;
                self.gpu_freq_mhz = 600;
            }
            PowerProfileMode::AutoSwitch => {
                if !self.battery.is_charging && self.battery.percentage < 20 {
                    self.set_profile(PowerProfileMode::PowerSaver);
                } else if self.battery.is_charging {
                    self.set_profile(PowerProfileMode::Performance);
                } else {
                    self.set_profile(PowerProfileMode::Balanced);
                }
            }
        }
    }

    pub fn register_thermal_zone(&mut self, name: &str, temp: f32, crit: f32) -> u32 {
        let zone_id = self.thermal_zones.len() as u32 + 1;
        self.thermal_zones.push(ThermalZone {
            zone_id,
            name: name.to_string(),
            temp_celsius: temp,
            critical_temp_celsius: crit,
            is_throttling: temp >= crit - 5.0,
        });
        zone_id
    }

    pub fn update_thermal(&mut self, zone_id: u32, temp: f32) {
        if let Some(zone) = self.thermal_zones.iter_mut().find(|z| z.zone_id == zone_id) {
            zone.temp_celsius = temp;
            zone.is_throttling = temp >= zone.critical_temp_celsius - 5.0;
            if zone.is_throttling {
                self.cpu_freq_mhz = 800; // Throttled frequency
            }
        }
    }
}

impl Default for PowerManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_manager_profiles() {
        let mut mgr = PowerManager::new();
        mgr.set_profile(PowerProfileMode::Performance);
        assert_eq!(mgr.cpu_freq_mhz, 3800);

        mgr.set_profile(PowerProfileMode::PowerSaver);
        assert_eq!(mgr.cpu_freq_mhz, 1200);

        let zone = mgr.register_thermal_zone("CPU_Die", 80.0, 95.0);
        mgr.update_thermal(zone, 92.0); // Triggers throttling
        assert_eq!(mgr.cpu_freq_mhz, 800);
    }
}
