// SigmaOS ACPI Comprehensive Power & Thermal Management Engine
// Provides ACPI Sleep States (S0..S5), Thermal Management, and Device Power Transitions (D0..D3)

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcpiSleepState {
    S0Working,
    S1CpuStop,
    S2CpuPowerOff,
    S3SuspendToRam,
    S4Hibernation,
    S5SoftOff,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcpiDevicePowerState {
    D0FullyOn,
    D1Intermediate,
    D2Intermediate,
    D3HotOffAuxPower,
    D3ColdCompletelyOff,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThermalCoolingPolicy {
    ActiveFanControl,
    PassiveCpuThrottling,
    CriticalShutdownThreshold,
}

#[derive(Debug, Clone)]
pub struct AcpiThermalZone {
    pub zone_id: u32,
    pub current_temp_celsius: f32,
    pub active_trip_temp_celsius: f32,
    pub passive_trip_temp_celsius: f32,
    pub critical_trip_temp_celsius: f32,
    pub fan_speed_percent: u8,
    pub is_cpu_throttled: bool,
}

#[derive(Debug, Clone)]
pub struct AcpiDevicePowerDescriptor {
    pub device_id: u32,
    pub name: String,
    pub current_power_state: AcpiDevicePowerState,
    pub supports_wake: bool,
}

pub struct AcpiPowerThermalManagerEngine {
    current_sleep_state: AcpiSleepState,
    thermal_zones: BTreeMap<u32, AcpiThermalZone>,
    devices: BTreeMap<u32, AcpiDevicePowerDescriptor>,
}

impl AcpiPowerThermalManagerEngine {
    pub fn new() -> Self {
        let mut thermal_zones = BTreeMap::new();
        thermal_zones.insert(
            0,
            AcpiThermalZone {
                zone_id: 0,
                current_temp_celsius: 45.0,
                active_trip_temp_celsius: 60.0,
                passive_trip_temp_celsius: 80.0,
                critical_trip_temp_celsius: 95.0,
                fan_speed_percent: 30,
                is_cpu_throttled: false,
            },
        );

        let mut devices = BTreeMap::new();
        devices.insert(
            1,
            AcpiDevicePowerDescriptor {
                device_id: 1,
                name: "PCI Express Root Complex".to_string(),
                current_power_state: AcpiDevicePowerState::D0FullyOn,
                supports_wake: true,
            },
        );

        Self {
            current_sleep_state: AcpiSleepState::S0Working,
            thermal_zones,
            devices,
        }
    }

    pub fn current_sleep_state(&self) -> AcpiSleepState {
        self.current_sleep_state
    }

    pub fn transition_sleep_state(&mut self, target_state: AcpiSleepState) -> Result<(), &'static str> {
        self.current_sleep_state = target_state;
        match target_state {
            AcpiSleepState::S3SuspendToRam | AcpiSleepState::S4Hibernation => {
                for dev in self.devices.values_mut() {
                    dev.current_power_state = AcpiDevicePowerState::D3HotOffAuxPower;
                }
            }
            AcpiSleepState::S0Working => {
                for dev in self.devices.values_mut() {
                    dev.current_power_state = AcpiDevicePowerState::D0FullyOn;
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn update_thermal_zone_temp(&mut self, zone_id: u32, new_temp_celsius: f32) -> Result<ThermalCoolingPolicy, &'static str> {
        let zone = self.thermal_zones.get_mut(&zone_id).ok_or("Thermal zone not found")?;
        zone.current_temp_celsius = new_temp_celsius;

        if new_temp_celsius >= zone.critical_trip_temp_celsius {
            return Ok(ThermalCoolingPolicy::CriticalShutdownThreshold);
        }

        if new_temp_celsius >= zone.passive_trip_temp_celsius {
            zone.fan_speed_percent = 100;
            zone.is_cpu_throttled = true;
            return Ok(ThermalCoolingPolicy::PassiveCpuThrottling);
        }

        if new_temp_celsius >= zone.active_trip_temp_celsius {
            zone.fan_speed_percent = 75;
            zone.is_cpu_throttled = false;
            return Ok(ThermalCoolingPolicy::ActiveFanControl);
        }

        zone.fan_speed_percent = 30;
        zone.is_cpu_throttled = false;
        Ok(ThermalCoolingPolicy::ActiveFanControl)
    }

    pub fn set_device_power_state(&mut self, device_id: u32, power_state: AcpiDevicePowerState) -> Result<(), &'static str> {
        let dev = self.devices.get_mut(&device_id).ok_or("Device not found")?;
        dev.current_power_state = power_state;
        Ok(())
    }

    pub fn get_thermal_zone(&self, zone_id: u32) -> Option<&AcpiThermalZone> {
        self.thermal_zones.get(&zone_id)
    }

    pub fn get_device_power_state(&self, device_id: u32) -> Option<AcpiDevicePowerState> {
        self.devices.get(&device_id).map(|d| d.current_power_state)
    }
}

#[cfg(test)]
mod acpi_power_tests {
    use super::*;

    #[test]
    fn test_acpi_sleep_state_transition() {
        let mut engine = AcpiPowerThermalManagerEngine::new();
        assert_eq!(engine.current_sleep_state(), AcpiSleepState::S0Working);

        assert!(engine.transition_sleep_state(AcpiSleepState::S3SuspendToRam).is_ok());
        assert_eq!(engine.current_sleep_state(), AcpiSleepState::S3SuspendToRam);
        assert_eq!(engine.get_device_power_state(1).unwrap(), AcpiDevicePowerState::D3HotOffAuxPower);

        assert!(engine.transition_sleep_state(AcpiSleepState::S0Working).is_ok());
        assert_eq!(engine.get_device_power_state(1).unwrap(), AcpiDevicePowerState::D0FullyOn);
    }

    #[test]
    fn test_acpi_thermal_policy_triggers() {
        let mut engine = AcpiPowerThermalManagerEngine::new();

        let normal = engine.update_thermal_zone_temp(0, 50.0).unwrap();
        assert_eq!(normal, ThermalCoolingPolicy::ActiveFanControl);
        assert_eq!(engine.get_thermal_zone(0).unwrap().fan_speed_percent, 30);

        let active = engine.update_thermal_zone_temp(0, 65.0).unwrap();
        assert_eq!(active, ThermalCoolingPolicy::ActiveFanControl);
        assert_eq!(engine.get_thermal_zone(0).unwrap().fan_speed_percent, 75);

        let passive = engine.update_thermal_zone_temp(0, 85.0).unwrap();
        assert_eq!(passive, ThermalCoolingPolicy::PassiveCpuThrottling);
        assert!(engine.get_thermal_zone(0).unwrap().is_cpu_throttled);

        let critical = engine.update_thermal_zone_temp(0, 98.0).unwrap();
        assert_eq!(critical, ThermalCoolingPolicy::CriticalShutdownThreshold);
    }
}
