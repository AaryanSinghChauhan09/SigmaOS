// SPDX-License-Identifier: MIT
// SigmaOS - CPU Frequency Scaling
// Inspired by Linux CPU frequency scaling (sysfs: /sys/devices/system/cpu/cpu0/cpufreq/)
// Provides CPU frequency control with governors (performance, powersave, ondemand, schedutil)

#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, AtomicBool, Ordering};

/// CPU frequency governor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuGovernor {
    Performance,
    Powersave,
    OnDemand,
    Conservative,
    Schedutil,
}

impl CpuGovernor {
    pub fn as_str(&self) -> &str {
        match self {
            CpuGovernor::Performance => "performance",
            CpuGovernor::Powersave => "powersave",
            CpuGovernor::OnDemand => "ondemand",
            CpuGovernor::Conservative => "conservative",
            CpuGovernor::Schedutil => "schedutil",
        }
    }
    
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "performance" => Some(CpuGovernor::Performance),
            "powersave" => Some(CpuGovernor::Powersave),
            "ondemand" => Some(CpuGovernor::OnDemand),
            "conservative" => Some(CpuGovernor::Conservative),
            "schedutil" => Some(CpuGovernor::Schedutil),
            _ => None,
        }
    }
}

/// CPU frequency scaling controller
pub struct CpuFrequencyController {
    pub cpu_id: u32,
    pub current_frequency: AtomicU32, // MHz
    pub max_frequency: AtomicU32, // MHz
    pub min_frequency: AtomicU32, // MHz
    pub governor: CpuGovernor,
    pub previous_governor: Option<CpuGovernor>,
    pub turbo_enabled: AtomicBool,
    pub temperature: AtomicU32, // Celsius
}

impl CpuFrequencyController {
    pub fn new(cpu_id: u32) -> Self {
        Self {
            cpu_id,
            current_frequency: AtomicU32::new(1000), // Default 1GHz
            max_frequency: AtomicU32::new(4000), // Default 4GHz max
            min_frequency: AtomicU32::new(800), // Default 800MHz min
            governor: CpuGovernor::Powersave,
            previous_governor: None,
            turbo_enabled: AtomicBool::new(true),
            temperature: AtomicU32::new(40), // Default 40°C
        }
    }
    
    /// Set governor
    pub fn set_governor(&mut self, governor: CpuGovernor) {
        self.previous_governor = Some(self.governor);
        self.governor = governor;
        
        match governor {
            CpuGovernor::Performance => {
                self.set_frequency(self.max_frequency.load(Ordering::Relaxed));
            }
            CpuGovernor::Powersave => {
                self.set_frequency(self.min_frequency.load(Ordering::Relaxed));
            }
            CpuGovernor::OnDemand => {
                // In real implementation, would be dynamic based on load
                self.set_frequency(1500);
            }
            CpuGovernor::Conservative => {
                // Gradual frequency changes
                self.set_frequency(1200);
            }
            CpuGovernor::Schedutil => {
                // Scheduler-based frequency selection
                self.set_frequency(1400);
            }
        }
        
        // In real implementation, would write to sysfs:
        // echo "performance" > /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor
    }
    
    /// Restore previous governor
    pub fn restore_governor(&mut self) {
        if let Some(prev) = self.previous_governor {
            self.set_governor(prev);
        }
    }
    
    /// Set frequency (MHz)
    pub fn set_frequency(&self, frequency: u32) {
        let max = self.max_frequency.load(Ordering::Relaxed);
        let min = self.min_frequency.load(Ordering::Relaxed);
        let frequency = frequency.max(min).min(max);
        self.current_frequency.store(frequency, Ordering::Relaxed);
        
        // In real implementation, would write to sysfs:
        // echo 4000000 > /sys/devices/system/cpu/cpu0/cpufreq/scaling_sets_freq
    }
    
    /// Get current frequency (MHz)
    pub fn get_frequency(&self) -> u32 {
        self.current_frequency.load(Ordering::Relaxed)
    }
    
    /// Set max frequency (MHz)
    pub fn set_max_frequency(&self, frequency: u32) {
        self.max_frequency.store(frequency, Ordering::Relaxed);
    }
    
    /// Set min frequency (MHz)
    pub fn set_min_frequency(&self, frequency: u32) {
        self.min_frequency.store(frequency, Ordering::Relaxed);
    }
    
    /// Enable turbo boost
    pub fn enable_turbo(&self, enabled: bool) {
        self.turbo_enabled.store(enabled, Ordering::Relaxed);
        
        if enabled {
            // Allow frequency above max
            let turbo_max = self.max_frequency.load(Ordering::Relaxed) * 11 / 10; // 10% boost
            self.set_frequency(turbo_max);
        }
    }
    
    /// Update temperature (called by hardware monitor)
    pub fn update_temperature(&self, temp: u32) {
        self.temperature.store(temp, Ordering::Relaxed);
        
        // Thermal throttling
        if temp > 90 {
            // Throttle back
            let current = self.current_frequency.load(Ordering::Relaxed);
            self.set_frequency(current * 80 / 100);
        }
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
        // Example:
        // echo "performance" > /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor
        Ok(())
    }
}

impl Default for CpuFrequencyController {
    fn default() -> Self {
        Self::new(0)
    }
}

/// CPU frequency manager for multiple CPUs
pub struct CpuFrequencyManager {
    pub cpus: Vec<CpuFrequencyController>,
    pub global_governor: CpuGovernor,
}

impl CpuFrequencyManager {
    pub fn new() -> Self {
        Self {
            cpus: Vec::new(),
            global_governor: CpuGovernor::Powersave,
        }
    }
    
    /// Add CPU
    pub fn add_cpu(&mut self, cpu_id: u32) {
        let cpu = CpuFrequencyController::new(cpu_id);
        self.cpus.push(cpu);
    }
    
    /// Get CPU by index
    pub fn get_cpu(&self, index: usize) -> Option<&CpuFrequencyController> {
        self.cpus.get(index)
    }
    
    /// Get mutable CPU by index
    pub fn get_cpu_mut(&mut self, index: usize) -> Option<&mut CpuFrequencyController> {
        self.cpus.get_mut(index)
    }
    
    /// Set global governor for all CPUs
    pub fn set_global_governor(&mut self, governor: CpuGovernor) {
        self.global_governor = governor;
        for cpu in &mut self.cpus {
            cpu.set_governor(governor);
        }
    }
    
    /// Restore all CPUs to previous governor
    pub fn restore_all_governors(&mut self) {
        for cpu in &mut self.cpus {
            cpu.restore_governor();
        }
    }
    
    /// Set all CPUs to performance mode
    pub fn set_all_performance(&mut self) {
        self.set_global_governor(CpuGovernor::Performance);
    }
    
    /// Set all CPUs to power saving mode
    pub fn set_all_powersave(&mut self) {
        self.set_global_governor(CpuGovernor::Powersave);
    }
    
    /// Enable turbo boost on all CPUs
    pub fn enable_turbo_all(&self, enabled: bool) {
        for cpu in &self.cpus {
            cpu.enable_turbo(enabled);
        }
    }
    
    /// Get average frequency
    pub fn get_average_frequency(&self) -> u32 {
        if self.cpus.is_empty() {
            return 0;
        }
        
        let sum: u32 = self.cpus.iter().map(|cpu| cpu.get_frequency()).sum();
        sum / self.cpus.len() as u32
    }
    
    /// Get average temperature
    pub fn get_average_temperature(&self) -> u32 {
        if self.cpus.is_empty() {
            return 0;
        }
        
        let sum: u32 = self.cpus.iter().map(|cpu| cpu.get_temperature()).sum();
        sum / self.cpus.len() as u32
    }
}

impl Default for CpuFrequencyManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global CPU frequency manager
static mut CPU_FREQ_MANAGER: Option<CpuFrequencyManager> = None;

/// Initialize CPU frequency manager
pub fn init_cpu_frequency_manager() {
    unsafe {
        CPU_FREQ_MANAGER = Some(CpuFrequencyManager::new());
    }
}

/// Get global CPU frequency manager
pub fn get_cpu_frequency_manager() -> Option<&'static mut CpuFrequencyManager> {
    unsafe {
        CPU_FREQ_MANAGER.as_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cpu_frequency_controller_init() {
        let cpu = CpuFrequencyController::new(0);
        assert_eq!(cpu.cpu_id, 0);
        assert_eq!(cpu.governor, CpuGovernor::Powersave);
    }
    
    #[test]
    fn test_set_governor_performance() {
        let mut cpu = CpuFrequencyController::new(0);
        cpu.set_governor(CpuGovernor::Performance);
        assert_eq!(cpu.governor, CpuGovernor::Performance);
        assert_eq!(cpu.get_frequency(), 4000); // Max frequency
    }
    
    #[test]
    fn test_set_governor_powersave() {
        let mut cpu = CpuFrequencyController::new(0);
        cpu.set_governor(CpuGovernor::Powersave);
        assert_eq!(cpu.governor, CpuGovernor::Powersave);
        assert_eq!(cpu.get_frequency(), 800); // Min frequency
    }
    
    #[test]
    fn test_set_frequency() {
        let cpu = CpuFrequencyController::new(0);
        cpu.set_frequency(2000);
        assert_eq!(cpu.get_frequency(), 2000);
    }
    
    #[test]
    fn test_frequency_clamping() {
        let cpu = CpuFrequencyController::new(0);
        cpu.set_frequency(5000); // Above max
        assert_eq!(cpu.get_frequency(), 4000); // Clamped to max
        cpu.set_frequency(500); // Below min
        assert_eq!(cpu.get_frequency(), 800); // Clamped to min
    }
    
    #[test]
    fn test_turbo_boost() {
        let cpu = CpuFrequencyController::new(0);
        cpu.enable_turbo(true);
        assert!(cpu.turbo_enabled.load(Ordering::Relaxed));
    }
    
    #[test]
    fn test_thermal_throttling() {
        let cpu = CpuFrequencyController::new(0);
        cpu.set_frequency(4000);
        cpu.update_temperature(95); // Above 90°C
        assert_eq!(cpu.get_frequency(), 3200); // Throttled to 80%
    }
    
    #[test]
    fn test_restore_governor() {
        let mut cpu = CpuFrequencyController::new(0);
        cpu.set_governor(CpuGovernor::Performance);
        cpu.restore_governor();
        assert_eq!(cpu.governor, CpuGovernor::Powersave); // Restored to original
    }
    
    #[test]
    fn test_cpu_frequency_manager() {
        let mut manager = CpuFrequencyManager::new();
        manager.add_cpu(0);
        manager.add_cpu(1);
        assert_eq!(manager.cpus.len(), 2);
    }
    
    #[test]
    fn test_set_global_governor() {
        let mut manager = CpuFrequencyManager::new();
        manager.add_cpu(0);
        manager.add_cpu(1);
        manager.set_global_governor(CpuGovernor::Performance);
        assert_eq!(manager.get_cpu(0).unwrap().governor, CpuGovernor::Performance);
        assert_eq!(manager.get_cpu(1).unwrap().governor, CpuGovernor::Performance);
    }
    
    #[test]
    fn test_set_all_performance() {
        let mut manager = CpuFrequencyManager::new();
        manager.add_cpu(0);
        manager.add_cpu(1);
        manager.set_all_performance();
        assert_eq!(manager.get_cpu(0).unwrap().get_frequency(), 4000);
        assert_eq!(manager.get_cpu(1).unwrap().get_frequency(), 4000);
    }
    
    #[test]
    fn test_average_frequency() {
        let mut manager = CpuFrequencyManager::new();
        manager.add_cpu(0);
        manager.add_cpu(1);
        manager.get_cpu_mut(0).unwrap().set_frequency(2000);
        manager.get_cpu_mut(1).unwrap().set_frequency(3000);
        assert_eq!(manager.get_average_frequency(), 2500);
    }
    
    #[test]
    fn test_governor_from_str() {
        assert_eq!(CpuGovernor::from_str("performance"), Some(CpuGovernor::Performance));
        assert_eq!(CpuGovernor::from_str("powersave"), Some(CpuGovernor::Powersave));
        assert_eq!(CpuGovernor::from_str("invalid"), None);
    }
    
    #[test]
    fn test_governor_as_str() {
        assert_eq!(CpuGovernor::Performance.as_str(), "performance");
        assert_eq!(CpuGovernor::Powersave.as_str(), "powersave");
    }
}
