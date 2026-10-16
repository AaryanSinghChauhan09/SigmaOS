// SPDX-License-Identifier: MIT
// SigmaOS - Intel Xe/ARC Driver Skeleton
// Inspired by Linux Intel Xe driver (drivers/gpu/drm/xe/)
// Provides Intel Xe/ARC GPU control framework with frequency scaling and monitoring

#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// Intel GPU architecture
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntelGpuArch {
    Gen9,  // Skylake
    Gen11, // Ice Lake
    Gen12, // Tiger Lake, Alder Lake
    Xe,    // Intel Xe
    Arc,   // Intel Arc Alchemist
}

/// Intel performance level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntelPerfLevel {
    Auto,
    Balanced,
    Performance,
    PowerSaving,
}

/// Intel power state
#[derive(Debug, Clone, Copy)]
pub struct IntelPowerState {
    pub gt_frequency: u32,     // MHz
    pub render_frequency: u32, // MHz
    pub voltage: u32,          // mV
    pub power_limit: u32,      // mW
    pub current_power: u32,    // mW
}

/// Intel Xe device
pub struct IntelXeDevice {
    pub device_id: u32,
    pub vendor_id: u32,
    pub pci_bus: u8,
    pub pci_slot: u8,
    pub pci_function: u8,
    pub arch: IntelGpuArch,
    pub perf_level: IntelPerfLevel,
    pub power_state: IntelPowerState,
    pub temperature: AtomicU32,  // Celsius
    pub fan_speed: AtomicU32,    // percentage
    pub gt_usage: AtomicU32,     // percentage
    pub render_usage: AtomicU32, // percentage
    pub active: AtomicBool,
}

impl IntelXeDevice {
    pub fn new(
        device_id: u32,
        vendor_id: u32,
        pci_bus: u8,
        pci_slot: u8,
        pci_function: u8,
        arch: IntelGpuArch,
    ) -> Self {
        Self {
            device_id,
            vendor_id,
            pci_bus,
            pci_slot,
            pci_function,
            arch,
            perf_level: IntelPerfLevel::Balanced,
            power_state: IntelPowerState {
                gt_frequency: 1000,
                render_frequency: 1000,
                voltage: 1000,
                power_limit: 75000,   // 75W
                current_power: 37500, // 37.5W
            },
            temperature: AtomicU32::new(40),
            fan_speed: AtomicU32::new(30),
            gt_usage: AtomicU32::new(0),
            render_usage: AtomicU32::new(0),
            active: AtomicBool::new(false),
        }
    }

    /// Initialize Intel Xe device
    pub fn init(&mut self) -> Result<(), &'static str> {
        self.active.store(true, Ordering::Relaxed);
        Ok(())
    }

    /// Set performance level
    pub fn set_perf_level(&mut self, level: IntelPerfLevel) {
        self.perf_level = level;

        match level {
            IntelPerfLevel::Performance => {
                self.set_gt_frequency(1500); // Max
                self.set_render_frequency(1500); // Max
            }
            IntelPerfLevel::PowerSaving => {
                self.set_gt_frequency(300); // Min
                self.set_render_frequency(300); // Min
            }
            IntelPerfLevel::Balanced => {
                self.set_gt_frequency(1000); // Mid
                self.set_render_frequency(1000); // Mid
            }
            IntelPerfLevel::Auto => {
                // No change
            }
        }

        // Write to sysfs: /sys/class/drm/card0/gt_cur_freq_mhz
        self.write_sysfs(
            "gt_cur_freq_mhz",
            &self.power_state.gt_frequency.to_string(),
        )
        .ok();
    }

    /// Set GT frequency
    pub fn set_gt_frequency(&mut self, freq: u32) {
        self.power_state.gt_frequency = freq;

        // Write to sysfs: /sys/class/drm/card0/gt_cur_freq_mhz
        self.write_sysfs("gt_cur_freq_mhz", &freq.to_string()).ok();
    }

    /// Set render frequency
    pub fn set_render_frequency(&mut self, freq: u32) {
        self.power_state.render_frequency = freq;
    }

    /// Set power limit
    pub fn set_power_limit(&mut self, limit: u32) {
        self.power_state.power_limit = limit;

        // Write to sysfs: /sys/class/drm/card0/gt_RP1_freq_mhz (power limit)
        self.write_sysfs("gt_RP1_freq_mhz", &limit.to_string()).ok();
    }

    /// Update temperature
    pub fn update_temperature(&mut self, temp: u32) {
        self.temperature.store(temp, Ordering::Relaxed);

        // Thermal throttling
        if temp > 90 {
            let current = self.power_state.gt_frequency;
            self.power_state.gt_frequency = current * 80 / 100;
        }
    }

    /// Update fan speed
    pub fn update_fan_speed(&self, speed: u32) {
        self.fan_speed.store(speed, Ordering::Relaxed);
    }

    /// Update GT usage
    pub fn update_gt_usage(&self, usage: u32) {
        self.gt_usage.store(usage, Ordering::Relaxed);
    }

    /// Update render usage
    pub fn update_render_usage(&self, usage: u32) {
        self.render_usage.store(usage, Ordering::Relaxed);
    }

    /// Get temperature
    pub fn get_temperature(&self) -> u32 {
        self.temperature.load(Ordering::Relaxed)
    }

    /// Get fan speed
    pub fn get_fan_speed(&self) -> u32 {
        self.fan_speed.load(Ordering::Relaxed)
    }

    /// Get GT usage
    pub fn get_gt_usage(&self) -> u32 {
        self.gt_usage.load(Ordering::Relaxed)
    }

    /// Get render usage
    pub fn get_render_usage(&self) -> u32 {
        self.render_usage.load(Ordering::Relaxed)
    }

    /// Read from sysfs
    pub fn read_sysfs(&self, path: &str) -> Result<String, &'static str> {
        // In real implementation, would read from actual sysfs
        // Example paths:
        // /sys/class/drm/card0/gt_cur_freq_mhz
        // /sys/class/drm/card0/gt_boost_freq_mhz
        // /sys/class/drm/card0/gt_RP0_freq_mhz
        // /sys/class/drm/card0/gt_RP1_freq_mhz
        Ok("0".to_string())
    }

    /// Write to sysfs
    pub fn write_sysfs(&self, path: &str, value: &str) -> Result<(), &'static str> {
        // In real implementation, would write to actual sysfs
        Ok(())
    }
}

/// Intel Xe manager for multiple devices
pub struct IntelXeManager {
    pub devices: Vec<IntelXeDevice>,
    pub default_device: Option<usize>,
}

impl IntelXeManager {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
            default_device: None,
        }
    }

    /// Add Intel Xe device
    pub fn add_device(&mut self, device: IntelXeDevice) {
        self.devices.push(device);
    }

    /// Get device by index
    pub fn get_device(&self, index: usize) -> Option<&IntelXeDevice> {
        self.devices.get(index)
    }

    /// Get mutable device by index
    pub fn get_device_mut(&mut self, index: usize) -> Option<&mut IntelXeDevice> {
        self.devices.get_mut(index)
    }

    /// Set default device
    pub fn set_default_device(&mut self, index: usize) {
        if index < self.devices.len() {
            self.default_device = Some(index);
        }
    }

    /// Set all devices to performance
    pub fn set_all_performance(&mut self) {
        for device in &mut self.devices {
            device.set_perf_level(IntelPerfLevel::Performance);
        }
    }
}

impl Default for IntelXeManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global Intel Xe manager
static mut INTEL_XE_MANAGER: Option<IntelXeManager> = None;

/// Initialize Intel Xe manager
pub fn init_intel_xe_manager() {
    unsafe {
        INTEL_XE_MANAGER = Some(IntelXeManager::new());
    }
}

/// Get global Intel Xe manager
pub fn get_intel_xe_manager() -> Option<&'static mut IntelXeManager> {
    unsafe { INTEL_XE_MANAGER.as_mut() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intel_xe_device_init() {
        let device = IntelXeDevice::new(0x8086, 0x56A0, 0, 0, 0, IntelGpuArch::Xe);
        assert_eq!(device.device_id, 0x8086);
        assert_eq!(device.arch, IntelGpuArch::Xe);
    }

    #[test]
    fn test_set_perf_level() {
        let mut device = IntelXeDevice::new(0x8086, 0x56A0, 0, 0, 0, IntelGpuArch::Xe);
        device.set_perf_level(IntelPerfLevel::Performance);
        assert_eq!(device.perf_level, IntelPerfLevel::Performance);
        assert_eq!(device.power_state.gt_frequency, 1500);
    }

    #[test]
    fn test_set_gt_frequency() {
        let mut device = IntelXeDevice::new(0x8086, 0x56A0, 0, 0, 0, IntelGpuArch::Xe);
        device.set_gt_frequency(1200);
        assert_eq!(device.power_state.gt_frequency, 1200);
    }

    #[test]
    fn test_thermal_throttling() {
        let mut device = IntelXeDevice::new(0x8086, 0x56A0, 0, 0, 0, IntelGpuArch::Xe);
        device.set_gt_frequency(1500);
        device.update_temperature(95);
        assert_eq!(device.power_state.gt_frequency, 1200); // Throttled
    }

    #[test]
    fn test_intel_xe_manager() {
        let mut manager = IntelXeManager::new();
        let device = IntelXeDevice::new(0x8086, 0x56A0, 0, 0, 0, IntelGpuArch::Xe);
        manager.add_device(device);
        assert_eq!(manager.devices.len(), 1);
    }

    #[test]
    fn test_set_all_performance() {
        let mut manager = IntelXeManager::new();
        manager.add_device(IntelXeDevice::new(
            0x8086,
            0x56A0,
            0,
            0,
            0,
            IntelGpuArch::Xe,
        ));
        manager.add_device(IntelXeDevice::new(
            0x8086,
            0x56A0,
            0,
            1,
            0,
            IntelGpuArch::Arc,
        ));
        manager.set_all_performance();
        assert_eq!(
            manager.get_device(0).unwrap().perf_level,
            IntelPerfLevel::Performance
        );
    }
}
