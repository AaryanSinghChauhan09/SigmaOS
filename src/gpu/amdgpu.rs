// SPDX-License-Identifier: MIT
// SigmaOS - AMDGPU Driver with Sysfs Integration
// Inspired by Linux AMDGPU driver (drivers/gpu/drm/amd/amdgpu/)
// Provides AMD GPU control via sysfs for frequency scaling, overdrive, and monitoring

#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};

/// AMDGPU power profile mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AmdgpuPowerProfile {
    BootupDefault,
    Performance,
    Balanced,
    PowerSaving,
    Compute,
    Custom,
}

/// AMDGPU performance level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AmdgpuPerfLevel {
    Auto,
    Low,
    Mid,
    High,
    Manual,
}

/// AMDGPU sclk (system clock) frequency
#[derive(Debug, Clone, Copy)]
pub struct AmdgpuSclk {
    pub min: u32,     // MHz
    pub max: u32,     // MHz
    pub current: u32, // MHz
}

/// AMDGPU mclk (memory clock) frequency
#[derive(Debug, Clone, Copy)]
pub struct AmdgpuMclk {
    pub min: u32,     // MHz
    pub max: u32,     // MHz
    pub current: u32, // MHz
}

/// AMDGPU voltage
#[derive(Debug, Clone, Copy)]
pub struct AmdgpuVoltage {
    pub min: u32,     // mV
    pub max: u32,     // mV
    pub current: u32, // mV
}

/// AMDGPU power state
#[derive(Debug, Clone, Copy)]
pub struct AmdgpuPowerState {
    pub sclk: AmdgpuSclk,
    pub mclk: AmdgpuMclk,
    pub voltage: AmdgpuVoltage,
    pub power_limit: u32,   // mW
    pub current_power: u32, // mW
}

/// AMDGPU overdrive settings
#[derive(Debug, Clone, Copy)]
pub struct AmdgpuOverdrive {
    pub sclk_percent: u32,        // percentage over stock
    pub mclk_percent: u32,        // percentage over stock
    pub voltage_offset: i32,      // mV offset
    pub power_limit_percent: u32, // percentage over stock
    pub enabled: bool,
}

/// AMDGPU device
pub struct AmdgpuDevice {
    pub device_id: u32,
    pub vendor_id: u32,
    pub pci_bus: u8,
    pub pci_slot: u8,
    pub pci_function: u8,
    pub power_profile: AmdgpuPowerProfile,
    pub perf_level: AmdgpuPerfLevel,
    pub power_state: AmdgpuPowerState,
    pub overdrive: AmdgpuOverdrive,
    pub temperature: AtomicU32,  // Celsius
    pub fan_speed: AtomicU32,    // percentage
    pub core_usage: AtomicU32,   // percentage
    pub memory_usage: AtomicU32, // percentage
    pub active: AtomicBool,
}

impl AmdgpuDevice {
    pub fn new(
        device_id: u32,
        vendor_id: u32,
        pci_bus: u8,
        pci_slot: u8,
        pci_function: u8,
    ) -> Self {
        Self {
            device_id,
            vendor_id,
            pci_bus,
            pci_slot,
            pci_function,
            power_profile: AmdgpuPowerProfile::Balanced,
            perf_level: AmdgpuPerfLevel::Auto,
            power_state: AmdgpuPowerState {
                sclk: AmdgpuSclk {
                    min: 300,
                    max: 2000,
                    current: 1000,
                },
                mclk: AmdgpuMclk {
                    min: 100,
                    max: 1000,
                    current: 500,
                },
                voltage: AmdgpuVoltage {
                    min: 800,
                    max: 1200,
                    current: 1000,
                },
                power_limit: 150000,  // 150W
                current_power: 75000, // 75W
            },
            overdrive: AmdgpuOverdrive {
                sclk_percent: 0,
                mclk_percent: 0,
                voltage_offset: 0,
                power_limit_percent: 0,
                enabled: false,
            },
            temperature: AtomicU32::new(40),
            fan_speed: AtomicU32::new(30),
            core_usage: AtomicU32::new(0),
            memory_usage: AtomicU32::new(0),
            active: AtomicBool::new(false),
        }
    }

    /// Initialize AMDGPU device
    pub fn init(&mut self) -> Result<(), &'static str> {
        self.active.store(true, Ordering::Relaxed);
        Ok(())
    }

    /// Set power profile
    pub fn set_power_profile(&mut self, profile: AmdgpuPowerProfile) {
        self.power_profile = profile;

        // Apply profile settings
        match profile {
            AmdgpuPowerProfile::Performance => {
                self.set_sclk(self.power_state.sclk.max);
                self.set_mclk(self.power_state.mclk.max);
            }
            AmdgpuPowerProfile::PowerSaving => {
                self.set_sclk(self.power_state.sclk.min);
                self.set_mclk(self.power_state.mclk.min);
            }
            AmdgpuPowerProfile::Balanced => {
                self.set_sclk((self.power_state.sclk.min + self.power_state.sclk.max) / 2);
                self.set_mclk((self.power_state.mclk.min + self.power_state.mclk.max) / 2);
            }
            _ => {}
        }

        // Write to sysfs: /sys/class/drm/card0/device/pp_power_profile_mode
        self.write_sysfs("pp_power_profile_mode", &self.get_power_profile_string())
            .ok();
    }

    /// Get power profile string for sysfs
    fn get_power_profile_string(&self) -> String {
        match self.power_profile {
            AmdgpuPowerProfile::BootupDefault => "BOOTUP_DEFAULT".to_string(),
            AmdgpuPowerProfile::Performance => "PERFORMANCE".to_string(),
            AmdgpuPowerProfile::Balanced => "BALANCED".to_string(),
            AmdgpuPowerProfile::PowerSaving => "POWER_SAVING".to_string(),
            AmdgpuPowerProfile::Compute => "COMPUTE".to_string(),
            AmdgpuPowerProfile::Custom => "CUSTOM".to_string(),
        }
    }

    /// Set performance level
    pub fn set_perf_level(&mut self, level: AmdgpuPerfLevel) {
        self.perf_level = level;

        match level {
            AmdgpuPerfLevel::High => {
                self.set_sclk(self.power_state.sclk.max);
            }
            AmdgpuPerfLevel::Low => {
                self.set_sclk(self.power_state.sclk.min);
            }
            AmdgpuPerfLevel::Mid => {
                self.set_sclk((self.power_state.sclk.min + self.power_state.sclk.max) / 2);
            }
            _ => {}
        }

        // Write to sysfs: /sys/class/drm/card0/device/pp_dpm_sclk
        self.write_sysfs("pp_dpm_sclk", &self.get_perf_level_string())
            .ok();
    }

    /// Get performance level string for sysfs
    fn get_perf_level_string(&self) -> String {
        match self.perf_level {
            AmdgpuPerfLevel::Auto => "auto".to_string(),
            AmdgpuPerfLevel::Low => "low".to_string(),
            AmdgpuPerfLevel::Mid => "mid".to_string(),
            AmdgpuPerfLevel::High => "high".to_string(),
            AmdgpuPerfLevel::Manual => "manual".to_string(),
        }
    }

    /// Set sclk (system clock)
    pub fn set_sclk(&mut self, frequency: u32) {
        let max = self.power_state.sclk.max;
        let min = self.power_state.sclk.min;
        let frequency = frequency.max(min).min(max);
        self.power_state.sclk.current = frequency;

        // Write to sysfs: /sys/class/drm/card0/device/pp_dpm_sclk
        self.write_sysfs("pp_dpm_sclk", &frequency.to_string()).ok();
    }

    /// Set mclk (memory clock)
    pub fn set_mclk(&mut self, frequency: u32) {
        let max = self.power_state.mclk.max;
        let min = self.power_state.mclk.min;
        let frequency = frequency.max(min).min(max);
        self.power_state.mclk.current = frequency;

        // Write to sysfs: /sys/class/drm/card0/device/pp_dpm_mclk
        self.write_sysfs("pp_dpm_mclk", &frequency.to_string()).ok();
    }

    /// Enable overdrive
    pub fn enable_overdrive(&mut self, enabled: bool) {
        self.overdrive.enabled = enabled;

        if enabled {
            // Apply overdrive percentages
            let sclk_boost = (self.power_state.sclk.max as u64
                * (100 + self.overdrive.sclk_percent) as u64)
                / 100;
            self.set_sclk(sclk_boost as u32);

            let mclk_boost = (self.power_state.mclk.max as u64
                * (100 + self.overdrive.mclk_percent) as u64)
                / 100;
            self.set_mclk(mclk_boost as u32);

            let power_boost = (self.power_state.power_limit as u64
                * (100 + self.overdrive.power_limit_percent) as u64)
                / 100;
            self.set_power_limit(power_boost as u32);
        }

        // Write to sysfs: /sys/class/drm/card0/device/pp_od_clk_voltage
        self.write_sysfs("pp_od_clk_voltage", if enabled { "1" } else { "0" })
            .ok();
    }

    /// Set overdrive sclk percentage
    pub fn set_overdrive_sclk(&mut self, percent: u32) {
        self.overdrive.sclk_percent = percent;
        if self.overdrive.enabled {
            self.enable_overdrive(true);
        }
    }

    /// Set overdrive mclk percentage
    pub fn set_overdrive_mclk(&mut self, percent: u32) {
        self.overdrive.mclk_percent = percent;
        if self.overdrive.enabled {
            self.enable_overdrive(true);
        }
    }

    /// Set power limit
    pub fn set_power_limit(&mut self, limit: u32) {
        self.power_state.power_limit = limit;

        // Write to sysfs: /sys/class/drm/card0/device/hwmon/hwmonX/power1_cap
        self.write_sysfs("power1_cap", &limit.to_string()).ok();
    }

    /// Update temperature (called by hardware monitor)
    pub fn update_temperature(&mut self, temp: u32) {
        self.temperature.store(temp, Ordering::Relaxed);

        // Thermal throttling
        if temp > 90 {
            // Throttle back
            let current = self.power_state.sclk.current;
            self.set_sclk(current * 80 / 100);
        }
    }

    /// Update fan speed
    pub fn update_fan_speed(&self, speed: u32) {
        self.fan_speed.store(speed, Ordering::Relaxed);

        // Write to sysfs: /sys/class/drm/card0/device/hwmon/hwmonX/pwm1
        self.write_sysfs("pwm1", &speed.to_string()).ok();
    }

    /// Update core usage
    pub fn update_core_usage(&self, usage: u32) {
        self.core_usage.store(usage, Ordering::Relaxed);
    }

    /// Update memory usage
    pub fn update_memory_usage(&self, usage: u32) {
        self.memory_usage.store(usage, Ordering::Relaxed);
    }

    /// Get temperature
    pub fn get_temperature(&self) -> u32 {
        self.temperature.load(Ordering::Relaxed)
    }

    /// Get fan speed
    pub fn get_fan_speed(&self) -> u32 {
        self.fan_speed.load(Ordering::Relaxed)
    }

    /// Get core usage
    pub fn get_core_usage(&self) -> u32 {
        self.core_usage.load(Ordering::Relaxed)
    }

    /// Get memory usage
    pub fn get_memory_usage(&self) -> u32 {
        self.memory_usage.load(Ordering::Relaxed)
    }

    /// Get current power consumption
    pub fn get_current_power(&self) -> u32 {
        self.power_state.current_power
    }

    /// Read from sysfs
    pub fn read_sysfs(&self, path: &str) -> Result<String, &'static str> {
        // In real implementation, would read from actual sysfs
        // For now, return simulated values
        Ok("0".to_string())
    }

    /// Write to sysfs
    pub fn write_sysfs(&self, path: &str, value: &str) -> Result<(), &'static str> {
        // In real implementation, would write to actual sysfs
        // Example paths:
        // /sys/class/drm/card0/device/pp_power_profile_mode
        // /sys/class/drm/card0/device/pp_dpm_sclk
        // /sys/class/drm/card0/device/pp_dpm_mclk
        // /sys/class/drm/card0/device/pp_od_clk_voltage
        // /sys/class/drm/card0/device/hwmon/hwmonX/power1_cap
        // /sys/class/drm/card0/device/hwmon/hwmonX/pwm1
        Ok(())
    }

    /// Get sysfs path for this device
    pub fn get_sysfs_path(&self) -> String {
        format!("/sys/class/drm/card{}/device", self.device_id)
    }
}

/// AMDGPU manager for multiple devices
pub struct AmdgpuManager {
    pub devices: Vec<AmdgpuDevice>,
    pub default_device: Option<usize>,
}

impl AmdgpuManager {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
            default_device: None,
        }
    }

    /// Add AMDGPU device
    pub fn add_device(&mut self, device: AmdgpuDevice) {
        self.devices.push(device);
    }

    /// Get device by index
    pub fn get_device(&self, index: usize) -> Option<&AmdgpuDevice> {
        self.devices.get(index)
    }

    /// Get mutable device by index
    pub fn get_device_mut(&mut self, index: usize) -> Option<&mut AmdgpuDevice> {
        self.devices.get_mut(index)
    }

    /// Set default device
    pub fn set_default_device(&mut self, index: usize) {
        if index < self.devices.len() {
            self.default_device = Some(index);
        }
    }

    /// Get default device
    pub fn get_default_device(&self) -> Option<&AmdgpuDevice> {
        self.default_device.and_then(|i| self.devices.get(i))
    }

    /// Get mutable default device
    pub fn get_default_device_mut(&mut self) -> Option<&mut AmdgpuDevice> {
        self.default_device.and_then(|i| self.devices.get_mut(i))
    }

    /// Set all devices to performance profile
    pub fn set_all_performance(&mut self) {
        for device in &mut self.devices {
            device.set_power_profile(AmdgpuPowerProfile::Performance);
        }
    }

    /// Set all devices to power saving profile
    pub fn set_all_powersave(&mut self) {
        for device in &mut self.devices {
            device.set_power_profile(AmdgpuPowerProfile::PowerSaving);
        }
    }
}

impl Default for AmdgpuManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global AMDGPU manager
static mut AMDGPU_MANAGER: Option<AmdgpuManager> = None;

/// Initialize AMDGPU manager
pub fn init_amdgpu_manager() {
    unsafe {
        AMDGPU_MANAGER = Some(AmdgpuManager::new());
    }
}

/// Get global AMDGPU manager
pub fn get_amdgpu_manager() -> Option<&'static mut AmdgpuManager> {
    unsafe { AMDGPU_MANAGER.as_mut() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amdgpu_device_init() {
        let device = AmdgpuDevice::new(0x1002, 0x15DD, 0, 0, 0);
        assert_eq!(device.device_id, 0x1002);
        assert_eq!(device.vendor_id, 0x15DD);
    }

    #[test]
    fn test_set_power_profile() {
        let mut device = AmdgpuDevice::new(0x1002, 0x15DD, 0, 0, 0);
        device.set_power_profile(AmdgpuPowerProfile::Performance);
        assert_eq!(device.power_profile, AmdgpuPowerProfile::Performance);
        assert_eq!(device.power_state.sclk.current, 2000); // Max
    }

    #[test]
    fn test_set_sclk() {
        let mut device = AmdgpuDevice::new(0x1002, 0x15DD, 0, 0, 0);
        device.set_sclk(1500);
        assert_eq!(device.power_state.sclk.current, 1500);
    }

    #[test]
    fn test_sclk_clamping() {
        let mut device = AmdgpuDevice::new(0x1002, 0x15DD, 0, 0, 0);
        device.set_sclk(5000); // Above max
        assert_eq!(device.power_state.sclk.current, 2000); // Clamped
        device.set_sclk(100); // Below min
        assert_eq!(device.power_state.sclk.current, 300); // Clamped
    }

    #[test]
    fn test_enable_overdrive() {
        let mut device = AmdgpuDevice::new(0x1002, 0x15DD, 0, 0, 0);
        device.set_overdrive_sclk(20);
        device.enable_overdrive(true);
        assert!(device.overdrive.enabled);
        assert_eq!(device.overdrive.sclk_percent, 20);
    }

    #[test]
    fn test_thermal_throttling() {
        let mut device = AmdgpuDevice::new(0x1002, 0x15DD, 0, 0, 0);
        device.set_sclk(2000);
        device.update_temperature(95); // Above 90°C
        assert_eq!(device.power_state.sclk.current, 1600); // Throttled to 80%
    }

    #[test]
    fn test_amdgpu_manager() {
        let mut manager = AmdgpuManager::new();
        let device = AmdgpuDevice::new(0x1002, 0x15DD, 0, 0, 0);
        manager.add_device(device);
        assert_eq!(manager.devices.len(), 1);
    }

    #[test]
    fn test_set_all_performance() {
        let mut manager = AmdgpuManager::new();
        manager.add_device(AmdgpuDevice::new(0x1002, 0x15DD, 0, 0, 0));
        manager.add_device(AmdgpuDevice::new(0x1002, 0x15DD, 0, 1, 0));
        manager.set_all_performance();
        assert_eq!(
            manager.get_device(0).unwrap().power_profile,
            AmdgpuPowerProfile::Performance
        );
        assert_eq!(
            manager.get_device(1).unwrap().power_profile,
            AmdgpuPowerProfile::Performance
        );
    }
}
