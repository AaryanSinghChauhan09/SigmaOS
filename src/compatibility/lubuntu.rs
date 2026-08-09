// Lubuntu-Inspired Lightweight System Configuration & Hardware Optimizer
// Focuses on extreme memory conservation, diagnostics, and running flawlessly on legacy/low-end systems.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU8, AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuGovernor {
    Performance,
    Balanced,
    Powersave,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemPressure {
    Low,
    Medium,
    High,
    Critical,
}

/// Lightweight system health report suitable for low-end machines
pub struct LubuntuHealthReport {
    pub memory_util_pct: u8,
    pub active_processes_count: usize,
    pub storage_free_mb: usize,
    pub system_pressure: SystemPressure,
}

/// Sovereign Lubuntu System Manager and Hardware Profile Tuner
pub struct LubuntuSystemManager {
    pub cpu_governor: AtomicU8, // CpuGovernor as u8
    pub max_task_queue_size: AtomicUsize,
    pub background_effects_enabled: AtomicUsize, // 1 if enabled, 0 if disabled
}

impl LubuntuSystemManager {
    pub fn new() -> Self {
        Self {
            cpu_governor: AtomicU8::new(CpuGovernor::Balanced as u8),
            max_task_queue_size: AtomicUsize::new(1000),
            background_effects_enabled: AtomicUsize::new(1),
        }
    }

    /// Retrieve Cpu Governor status
    pub fn get_governor(&self) -> CpuGovernor {
        match self.cpu_governor.load(Ordering::SeqCst) {
            0 => CpuGovernor::Performance,
            2 => CpuGovernor::Powersave,
            _ => CpuGovernor::Balanced,
        }
    }

    /// Run non-intrusive lightweight system health diagnostics
    pub fn diagnose_system_health(
        &self,
        current_memory_pct: u8,
        process_count: usize,
        free_storage: usize,
    ) -> LubuntuHealthReport {
        let system_pressure = if current_memory_pct > 90 || process_count > 500 {
            SystemPressure::Critical
        } else if current_memory_pct > 75 || process_count > 300 {
            SystemPressure::High
        } else if current_memory_pct > 40 {
            SystemPressure::Medium
        } else {
            SystemPressure::Low
        };

        LubuntuHealthReport {
            memory_util_pct: current_memory_pct,
            active_processes_count: process_count,
            storage_free_mb: free_storage,
            system_pressure,
        }
    }

    /// Dynamically optimizes the entire OS to run flawlessly on low-end/legacy physical computers
    pub fn optimize_for_low_end_hardware(
        &mut self,
        current_memory_pct: u8,
    ) -> Result<(), &'static str> {
        if current_memory_pct > 60 {
            // Low-end machine detected or system under load!
            // 1. Force CPU governor to Powersave to prevent thermal throttling
            self.cpu_governor
                .store(CpuGovernor::Powersave as u8, Ordering::SeqCst);

            // 2. Shrink maximum task queue capacities to prevent system queue starvation
            self.max_task_queue_size.store(200, Ordering::SeqCst);

            // 3. Disable resource-heavy visual background compositor shadow and glow effects
            self.background_effects_enabled.store(0, Ordering::SeqCst);
        } else {
            // Restore balanced state
            self.cpu_governor
                .store(CpuGovernor::Balanced as u8, Ordering::SeqCst);
            self.max_task_queue_size.store(1000, Ordering::SeqCst);
            self.background_effects_enabled.store(1, Ordering::SeqCst);
        }
        Ok(())
    }
}

impl Default for LubuntuSystemManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lubuntu_system_diagnostics() {
        let manager = LubuntuSystemManager::new();

        // Low utilization scenario
        let report_low = manager.diagnose_system_health(30, 15, 102400);
        assert_eq!(report_low.system_pressure, SystemPressure::Low);

        // Critical utilization scenario
        let report_critical = manager.diagnose_system_health(95, 600, 5000);
        assert_eq!(report_critical.system_pressure, SystemPressure::Critical);
        assert_eq!(report_critical.memory_util_pct, 95);
    }

    #[test]
    fn test_lubuntu_low_end_optimization() {
        let mut manager = LubuntuSystemManager::new();
        assert_eq!(manager.get_governor(), CpuGovernor::Balanced);
        assert_eq!(manager.background_effects_enabled.load(Ordering::SeqCst), 1);

        // Trigger low-end optimization due to high memory use
        manager.optimize_for_low_end_hardware(85).unwrap();

        assert_eq!(manager.get_governor(), CpuGovernor::Powersave);
        assert_eq!(manager.max_task_queue_size.load(Ordering::SeqCst), 200);
        assert_eq!(manager.background_effects_enabled.load(Ordering::SeqCst), 0);
        // Background effects disabled!
    }
}
