// SigmaOS CPU Frequency Scaling & Advanced Power Governors (Linux Inspired)
// Implements cpufreq-compatible CpuGovernors, active frequency scaling cores,
// TLP/powertop-compatible PCIe Active State Power Management (ASPM), and Energy-Aware Thread Balancers.

extern crate alloc;

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

// =========================================================================
// 1. CPU FREQUENCY GOVERNORS
// =========================================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuGovernor {
    Performance,  // Always maximum frequency limit
    Powersave,    // Always minimum frequency limit
    Ondemand,     // Rapidly jumps to max on load spike, decays slowly
    Conservative, // Step-by-step gradual frequency adjustments
    Schedutil,    // Uses scheduler task utilization to map frequency
}

pub struct CpuFreqCore {
    pub current_frequency_mhz: AtomicUsize,
    pub min_frequency_mhz: usize,
    pub max_frequency_mhz: usize,
    pub active_governor: CpuGovernor,
}

impl CpuFreqCore {
    pub const fn new(min_mhz: usize, max_mhz: usize, governor: CpuGovernor) -> Self {
        Self {
            current_frequency_mhz: AtomicUsize::new(max_mhz),
            min_frequency_mhz: min_mhz,
            max_frequency_mhz: max_mhz,
            active_governor: governor,
        }
    }

    /// Linux cpufreq-style dynamic frequency scaling calculation.
    /// Maps CPU load/utilization metric to target frequency based on the active governor.
    pub fn scale_frequency(&self, cpu_utilization: usize) -> usize {
        let utilization = cpu_utilization.min(100);

        let target = match self.active_governor {
            CpuGovernor::Performance => self.max_frequency_mhz,
            CpuGovernor::Powersave => self.min_frequency_mhz,
            CpuGovernor::Ondemand => {
                if utilization > 80 {
                    self.max_frequency_mhz // Jump directly to max on spike
                } else {
                    // Decays gradually relative to load
                    let range = self.max_frequency_mhz - self.min_frequency_mhz;
                    self.min_frequency_mhz + (range * utilization / 100)
                }
            }
            CpuGovernor::Conservative => {
                let current = self.current_frequency_mhz.load(Ordering::SeqCst);
                if utilization > 60 {
                    // Step up gradually by 10%
                    let step = (self.max_frequency_mhz - self.min_frequency_mhz) / 10;
                    (current + step).min(self.max_frequency_mhz)
                } else if utilization < 20 {
                    // Step down gradually by 10%
                    let step = (self.max_frequency_mhz - self.min_frequency_mhz) / 10;
                    current.saturating_sub(step).max(self.min_frequency_mhz)
                } else {
                    current
                }
            }
            CpuGovernor::Schedutil => {
                // Schedutil: Frequency = 1.25 * MaxFrequency * (Utilization / 100)
                let calculated = (self.max_frequency_mhz * utilization * 125) / 10000;
                calculated.clamp(self.min_frequency_mhz, self.max_frequency_mhz)
            }
        };

        self.current_frequency_mhz.store(target, Ordering::SeqCst);
        target
    }
}

// =========================================================================
// 2. TLP/POWERTOP POWER MANAGEMENT
// =========================================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AspmLevel {
    L0s,  // Lowest latency, minimal power savings
    L1,   // Medium latency, moderate power savings
    L1_1, // Deeper substate, higher power savings
    L1_2, // Deepest substate, maximum power savings
}

pub struct TlpPowerManager {
    pub pcie_aspm: AspmLevel,
    pub dirty_writeback_centisecs: AtomicUsize,
    pub runtime_pm: bool,
}

impl TlpPowerManager {
    pub const fn new() -> Self {
        Self {
            pcie_aspm: AspmLevel::L0s,
            dirty_writeback_centisecs: AtomicUsize::new(500), // Default 5 seconds writeback
            runtime_pm: false,
        }
    }

    /// Emulates TLP profile application (AC vs Battery mode)
    pub fn apply_power_profile(&mut self, profile_name: &str) {
        if profile_name == "battery" {
            self.pcie_aspm = AspmLevel::L1_2;
            self.dirty_writeback_centisecs.store(1500, Ordering::SeqCst); // 15 seconds deferred writeback (saves disk spins)
            self.runtime_pm = true;
        } else {
            // AC mode
            self.pcie_aspm = AspmLevel::L0s;
            self.dirty_writeback_centisecs.store(500, Ordering::SeqCst);
            self.runtime_pm = false;
        }
    }

    pub fn get_writeback_expiry_secs(&self) -> f64 {
        (self.dirty_writeback_centisecs.load(Ordering::SeqCst) as f64) / 100.0
    }
}

// =========================================================================
// 3. ENERGY-AWARE THREAD BALANCER
// =========================================================================
pub struct EnergyAwareThreadBalancer {
    pub scale_factor: f64,
}

impl EnergyAwareThreadBalancer {
    pub const fn new(scale: f64) -> Self {
        Self { scale_factor: scale }
    }

    /// Linux EAS (Energy Aware Scheduling) parity:
    /// Dynamically shifts scheduler priorities to boost interactive/foreground tasks
    /// and throttle background batch tasks to maintain optimal performance-per-watt curves.
    pub fn boost_interactive_threads(&self, is_interactive: bool, base_priority: u8) -> u8 {
        if is_interactive {
            // Priority boost for UI/interactive threads
            base_priority.saturating_add(4)
        } else {
            // Shift background batch threads to energy-saving priority levels
            base_priority.saturating_sub(2).max(1)
        }
    }
}

// =========================================================================
// UNIT TESTS
// =========================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpufreq_governors_transitions() {
        // Core with frequency limits 800MHz to 4000MHz
        let mut core = CpuFreqCore::new(800, 4000, CpuGovernor::Performance);
        assert_eq!(core.scale_frequency(50), 4000);

        core.active_governor = CpuGovernor::Powersave;
        assert_eq!(core.scale_frequency(50), 800);

        core.active_governor = CpuGovernor::Ondemand;
        assert_eq!(core.scale_frequency(90), 4000); // load spike triggers max freq
        assert_eq!(core.scale_frequency(50), 800 + (3200 * 50 / 100)); // standard scale

        core.active_governor = CpuGovernor::Schedutil;
        // 1.25 * 4000 * 0.5 = 2500
        assert_eq!(core.scale_frequency(50), 2500);
    }

    #[test]
    fn test_tlp_power_management() {
        let mut tlp = TlpPowerManager::new();
        assert_eq!(tlp.pcie_aspm, AspmLevel::L0s);
        assert_eq!(tlp.get_writeback_expiry_secs(), 5.0);

        // Apply battery profile
        tlp.apply_power_profile("battery");
        assert_eq!(tlp.pcie_aspm, AspmLevel::L1_2);
        assert_eq!(tlp.get_writeback_expiry_secs(), 15.0);
        assert!(tlp.runtime_pm);
    }

    #[test]
    fn test_energy_aware_thread_balancer() {
        let balancer = EnergyAwareThreadBalancer::new(1.0);
        // Interactive task gets boosted
        assert_eq!(balancer.boost_interactive_threads(true, 10), 14);
        // Batch background task gets throttled to save power
        assert_eq!(balancer.boost_interactive_threads(false, 10), 8);
    }
}