// SPDX-License-Identifier: MIT
// SigmaOS - GPU Frequency Scaling
// Inspired by Linux GPU frequency scaling (sysfs: /sys/class/drm/card0/device/)
// Provides GPU frequency control for AMDGPU, NVIDIA, and Intel Xe

#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, AtomicBool, Ordering};

/// GPU vendor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuVendor {
    AMD,
    NVIDIA,
    Intel,
    Unknown,
}

/// GPU frequency scaling mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuFrequencyMode {
    Auto,
    Performance,
    PowerSaving,
    Manual,
}

/// GPU power profile
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuPowerProfile {
    Default,
    Performance,
    PerformancePerWatt,
    Balanced,
    PowerSaving,
}

/// GPU frequency scaling controller
pub struct GpuFrequencyController {
    pub vendor: GpuVendor,
    pub device_id: u32,
    pub current_clock: AtomicU32, // MHz
    pub max_clock: AtomicU32, // MHz
    pub min_clock: AtomicU32, // MHz
    pub power_limit: AtomicU32, // milliwatts
    pub max_power_limit: AtomicU32, // milliwatts
    pub temperature: AtomicU32, // Celsius
    pub frequency_mode: GpuFrequencyMode,
    pub power_profile: GpuPowerProfile,
    pub overdrive_enabled: AtomicBool,
    pub overdrive_percentage: AtomicU32, // percentage over stock
}

impl GpuFrequencyController {
    pub fn new(vendor: GpuVendor, device_id: u32) -> Self {
        Self {
            vendor,
            device_id,
            current_clock: AtomicU32::new(1000), // Default 1GHz
            max_clock: AtomicU32::new(2000), // Default 2GHz max
            min_clock: AtomicU32::new(300), // Default 300MHz min
            power_limit: AtomicU32::new(150000), // Default 150W
            max_power_limit: AtomicU32::new(300000), // Default 300W max
            temperature: AtomicU32::new(40), // Default 40°C
            frequency_mode: GpuFrequencyMode::Auto,
            power_profile: GpuPowerProfile::Default,
            overdrive_enabled: AtomicBool::new(false),
            overdrive_percentage: AtomicU32::new(0),
        }
    }
    
    /// Set frequency mode
    pub fn set_frequency_mode(&mut self, mode: GpuFrequencyMode) {
        self.frequency_mode = mode;
        
        match mode {
            GpuFrequencyMode::Performance => {
                self.set_clock(self.max_clock.load(Ordering::Relaxed));
            }
            GpuFrequencyMode::PowerSaving => {
                self.set_clock(self.min_clock.load(Ordering::Relaxed));
            }
            GpuFrequencyMode::Auto => {
                // In real implementation, would be dynamic based on load
                self.set_clock(1000);
            }
            GpuFrequencyMode::Manual => {
                // No change
            }
        }
    }
    
    /// Set GPU clock (MHz)
    pub fn set_clock(&self, clock: u32) {
        let max = self.max_clock.load(Ordering::Relaxed);
        let min = self.min_clock.load(Ordering::Relaxed);
        let clock = clock.max(min).min(max);
        self.current_clock.store(clock, Ordering::Relaxed);
        
        // In real implementation, would write to sysfs:
        // AMDGPU: /sys/class/drm/card0/device/pp_dpm_sclk
        // NVIDIA: nvidia-smi -pl
        // Intel Xe: /sys/class/drm/card0/gt_cur_freq_mhz
    }
    
    /// Get current clock (MHz)
    pub fn get_clock(&self) -> u32 {
        self.current_clock.load(Ordering::Relaxed)
    }
    
    /// Set power limit (milliwatts)
    pub fn set_power_limit(&self, limit: u32) {
        let max = self.max_power_limit.load(Ordering::Relaxed);
        let limit = limit.min(max);
        self.power_limit.store(limit, Ordering::Relaxed);
        
        // In real implementation, would write to sysfs:
        // AMDGPU: /sys/class/drm/card0/device/hwmon/hwmonX/power1_cap
        // NVIDIA: nvidia-smi -pl
    }
    
    /// Get power limit (milliwatts)
    pub fn get_power_limit(&self) -> u32 {
        self.power_limit.load(Ordering::Relaxed)
    }
    
    /// Set max clock (MHz)
    pub fn set_max_clock(&self, clock: u32) {
        self.max_clock.store(clock, Ordering::Relaxed);
    }
    
    /// Set min clock (MHz)
    pub fn set_min_clock(&self, clock: u32) {
        self.min_clock.store(clock, Ordering::Relaxed);
    }
    
    /// Set power profile
    pub fn set_power_profile(&mut self, profile: GpuPowerProfile) {
        self.power_profile = profile;
        
        // In real implementation, would write to sysfs:
        // AMDGPU: /sys/class/drm/card0/device/pp_power_profile_mode
    }
    
    /// Enable overdrive (AMDGPU)
    pub fn enable_overdrive(&self, enabled: bool) {
        self.overdrive_enabled.store(enabled, Ordering::Relaxed);
        
        if enabled {
            let overdrive = self.overdrive_percentage.load(Ordering::Relaxed);
            let boost = (self.max_clock.load(Ordering::Relaxed) as u64 * (100 + overdrive) as u64) / 100;
            self.set_clock(boost as u32);
        }
    }
    
    /// Set overdrive percentage (AMDGPU)
    pub fn set_overdrive_percentage(&self, percentage: u32) {
        self.overdrive_percentage.store(percentage, Ordering::Relaxed);
        
        if self.overdrive_enabled.load(Ordering::Relaxed) {
            self.enable_overdrive(true);
        }
    }
    
    /// Update temperature (called by hardware monitor)
    pub fn update_temperature(&self, temp: u32) {
        self.temperature.store(temp, Ordering::Relaxed);
    }
    
    /// Get temperature (Celsius)
    pub fn get_temperature(&self) -> u32 {
        self.temperature.load(Ordering::Relaxed)
    }
    
    /// Read from sysfs (in real implementation)
    pub fn read_sysfs(&self, path: &str) -> Result<String, &'static str> {
        // In real implementation, would read from actual sysfs
        // For now, return simulated values
        Ok("0".to_string())
    }
    
    /// Write to sysfs (in real implementation)
    pub fn write_sysfs(&self, path: &str, value: &str) -> Result<(), &'static str> {
        // In real implementation, would write to actual sysfs
        // AMDGPU example:
        // echo "profile_performance" > /sys/class/drm/card0/device/pp_power_profile_mode
        // NVIDIA example:
        // nvidia-smi -pl 200
        Ok(())
    }
}

impl Default for GpuFrequencyController {
    fn default() -> Self {
        Self::new(GpuVendor::Unknown, 0)
    }
}

/// GPU manager for multiple GPUs
pub struct GpuManager {
    pub gpus: Vec<GpuFrequencyController>,
    pub default_gpu: Option<usize>,
}

impl GpuManager {
    pub fn new() -> Self {
        Self {
            gpus: Vec::new(),
            default_gpu: None,
        }
    }
    
    /// Add GPU
    pub fn add_gpu(&mut self, vendor: GpuVendor, device_id: u32) {
        let gpu = GpuFrequencyController::new(vendor, device_id);
        self.gpus.push(gpu);
    }
    
    /// Get GPU by index
    pub fn get_gpu(&self, index: usize) -> Option<&GpuFrequencyController> {
        self.gpus.get(index)
    }
    
    /// Get mutable GPU by index
    pub fn get_gpu_mut(&mut self, index: usize) -> Option<&mut GpuFrequencyController> {
        self.gpus.get_mut(index)
    }
    
    /// Set default GPU
    pub fn set_default_gpu(&mut self, index: usize) {
        if index < self.gpus.len() {
            self.default_gpu = Some(index);
        }
    }
    
    /// Get default GPU
    pub fn get_default_gpu(&self) -> Option<&GpuFrequencyController> {
        self.default_gpu.and_then(|i| self.gpus.get(i))
    }
    
    /// Get mutable default GPU
    pub fn get_default_gpu_mut(&mut self) -> Option<&mut GpuFrequencyController> {
        self.default_gpu.and_then(|i| self.gpus.get_mut(i))
    }
    
    /// Set all GPUs to performance mode
    pub fn set_all_performance(&mut self) {
        for gpu in &mut self.gpus {
            gpu.set_frequency_mode(GpuFrequencyMode::Performance);
        }
    }
    
    /// Set all GPUs to power saving mode
    pub fn set_all_powersave(&mut self) {
        for gpu in &mut self.gpus {
            gpu.set_frequency_mode(GpuFrequencyMode::PowerSaving);
        }
    }
}

impl Default for GpuManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global GPU manager
static mut GPU_MANAGER: Option<GpuManager> = None;

/// Initialize GPU manager
pub fn init_gpu_manager() {
    unsafe {
        GPU_MANAGER = Some(GpuManager::new());
    }
}

/// Get global GPU manager
pub fn get_gpu_manager() -> Option<&'static mut GpuManager> {
    unsafe {
        GPU_MANAGER.as_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_gpu_frequency_controller_init() {
        let gpu = GpuFrequencyController::new(GpuVendor::AMD, 0x1002);
        assert_eq!(gpu.vendor, GpuVendor::AMD);
        assert_eq!(gpu.device_id, 0x1002);
    }
    
    #[test]
    fn test_set_clock() {
        let gpu = GpuFrequencyController::new(GpuVendor::AMD, 0x1002);
        gpu.set_clock(1500);
        assert_eq!(gpu.get_clock(), 1500);
    }
    
    #[test]
    fn test_clock_clamping() {
        let gpu = GpuFrequencyController::new(GpuVendor::AMD, 0x1002);
        gpu.set_clock(5000); // Above max
        assert_eq!(gpu.get_clock(), 2000); // Clamped to max
        gpu.set_clock(100); // Below min
        assert_eq!(gpu.get_clock(), 300); // Clamped to min
    }
    
    #[test]
    fn test_set_power_limit() {
        let gpu = GpuFrequencyController::new(GpuVendor::AMD, 0x1002);
        gpu.set_power_limit(200000);
        assert_eq!(gpu.get_power_limit(), 200000);
    }
    
    #[test]
    fn test_power_limit_clamping() {
        let gpu = GpuFrequencyController::new(GpuVendor::AMD, 0x1002);
        gpu.set_power_limit(500000); // Above max
        assert_eq!(gpu.get_power_limit(), 300000); // Clamped to max
    }
    
    #[test]
    fn test_frequency_mode_performance() {
        let mut gpu = GpuFrequencyController::new(GpuVendor::AMD, 0x1002);
        gpu.set_frequency_mode(GpuFrequencyMode::Performance);
        assert_eq!(gpu.get_clock(), 2000); // Max clock
    }
    
    #[test]
    fn test_frequency_mode_powersave() {
        let mut gpu = GpuFrequencyController::new(GpuVendor::AMD, 0x1002);
        gpu.set_frequency_mode(GpuFrequencyMode::PowerSaving);
        assert_eq!(gpu.get_clock(), 300); // Min clock
    }
    
    #[test]
    fn test_overdrive() {
        let gpu = GpuFrequencyController::new(GpuVendor::AMD, 0x1002);
        gpu.set_overdrive_percentage(20);
        gpu.enable_overdrive(true);
        assert!(gpu.overdrive_enabled.load(Ordering::Relaxed));
    }
    
    #[test]
    fn test_temperature() {
        let gpu = GpuFrequencyController::new(GpuVendor::AMD, 0x1002);
        gpu.update_temperature(75);
        assert_eq!(gpu.get_temperature(), 75);
    }
    
    #[test]
    fn test_gpu_manager() {
        let mut manager = GpuManager::new();
        manager.add_gpu(GpuVendor::AMD, 0x1002);
        manager.add_gpu(GpuVendor::NVIDIA, 0x10DE);
        assert_eq!(manager.gpus.len(), 2);
    }
    
    #[test]
    fn test_default_gpu() {
        let mut manager = GpuManager::new();
        manager.add_gpu(GpuVendor::AMD, 0x1002);
        manager.set_default_gpu(0);
        assert!(manager.get_default_gpu().is_some());
    }
    
    #[test]
    fn test_set_all_performance() {
        let mut manager = GpuManager::new();
        manager.add_gpu(GpuVendor::AMD, 0x1002);
        manager.add_gpu(GpuVendor::NVIDIA, 0x10DE);
        manager.set_all_performance();
        assert_eq!(manager.get_gpu(0).unwrap().get_clock(), 2000);
        assert_eq!(manager.get_gpu(1).unwrap().get_clock(), 2000);
    }
}
