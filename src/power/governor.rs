// SigmaOS CPU Frequency Scaling & Advanced Power Governors (Linux Inspired)
// Implements cpufreq-compatible CpuGovernors, active frequency scaling cores,
// TLP/powertop-compatible PCIe Active State Power Management (ASPM), and Energy-Aware Thread Balancers.

extern crate alloc;

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

// 1. CPU FREQUENCY GOVERNORS
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

    pub fn scale_frequency(&self, cpu_utilization: usize) -> usize {
        let utilization = cpu_utilization.min(100);

        let target = match self.active_governor {
            CpuGovernor::Performance => self.max_frequency_mhz,
            CpuGovernor::Powersave => self.min_frequency_mhz,
            CpuGovernor::Ondemand => {
                if utilization > 80 {
                    self.max_frequency_mhz
                } else {
                    let range = self.max_frequency_mhz - self.min_frequency_mhz;
                    self.min_frequency_mhz + (range * utilization / 100)
                }
            }
            CpuGovernor::Conservative => {
                let current = self.current_frequency_mhz.load(Ordering::SeqCst);
                if utilization > 60 {
                    let step = (self.max_frequency_mhz - self.min_frequency_mhz) / 10;
                    (current + step).min(self.max_frequency_mhz)
                } else if utilization < 20 {
                    let step = (self.max_frequency_mhz - self.min_frequency_mhz) / 10;
                    current.saturating_sub(step).max(self.min_frequency_mhz)
                } else {
                    current
                }
            }
            CpuGovernor::Schedutil => {
                let calculated = (self.max_frequency_mhz * utilization * 125) / 10000;
                calculated.clamp(self.min_frequency_mhz, self.max_frequency_mhz)
            }
        };

        self.current_frequency_mhz.store(target, Ordering::SeqCst);
        target
    }
}

// 2. TLP/POWERTOP POWER MANAGEMENT
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AspmLevel {
    L0s,
    L1,
    L1_1,
    L1_2,
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
            dirty_writeback_centisecs: AtomicUsize::new(500),
            runtime_pm: true,
        }
    }

    pub fn set_pcie_aspm(&mut self, level: AspmLevel) {
        self.pcie_aspm = level;
    }

    pub fn set_dirty_writeback_duration(&self, duration_secs: usize) {
        self.dirty_writeback_centisecs.store(duration_secs * 100, Ordering::SeqCst);
    }

    pub fn get_writeback_expiry_secs(&self) -> f64 {
        self.dirty_writeback_centisecs.load(Ordering::SeqCst) as f64 / 100.0
    }
}

// 3. ENERGY-AWARE Schedutil Balancers
pub struct EnergyAwareThreadBalancer {
    pub scale_ratio: f64,
}

impl EnergyAwareThreadBalancer {
    pub const fn new(ratio: f64) -> Self {
        Self { scale_ratio: ratio }
    }

    pub fn boost_interactive_threads(&self, is_interactive: bool, count: usize) -> usize {
        if is_interactive {
            (count as f64 * 1.4) as usize
        } else {
            (count as f64 * 0.8) as usize
        }
    }
}

pub struct MemoryPageBlock {
    pub page_id: u64,
    pub is_fragmented: bool,
    pub data_size: usize,
}

pub struct SigmaSupportResourceOptimizer {
    pub managed_pages: Vec<MemoryPageBlock>,
    pub total_defragmentations_completed: u64,
}

impl SigmaSupportResourceOptimizer {
    pub fn new() -> Self {
        SigmaSupportResourceOptimizer {
            managed_pages: Vec::new(),
            total_defragmentations_completed: 0,
        }
    }

    pub fn register_page_block(&mut self, id: u64, fragmented: bool, size: usize) {
        self.managed_pages.push(MemoryPageBlock {
            page_id: id,
            is_fragmented: fragmented,
            data_size: size,
        });
    }

    pub fn execute_ram_defragmentation(&mut self) -> usize {
        let mut defragged = 0;
        for page in &mut self.managed_pages {
            if page.is_fragmented {
                page.is_fragmented = false;
                defragged += 1;
            }
        }
        self.total_defragmentations_completed += 1;
        defragged
    }
}

pub struct RunningProcessTask {
    pub process_id: u32,
    pub process_name: String,
    pub priority_niceness: i32,
    pub current_cpu_usage: f64,
}

pub struct SigmaSupportPriorityOptimizer {
    pub running_processes: Vec<RunningProcessTask>,
}

impl SigmaSupportPriorityOptimizer {
    pub fn new() -> Self {
        SigmaSupportPriorityOptimizer {
            running_processes: Vec::new(),
        }
    }

    pub fn register_running_process(&mut self, pid: u32, name: &str, priority: i32) {
        self.running_processes.push(RunningProcessTask {
            process_id: pid,
            process_name: name.to_string(),
            priority_niceness: priority,
            current_cpu_usage: 0.0,
        });
    }

    pub fn optimize_cpu_priorities(&mut self, critical_app_pid: u32) -> usize {
        let mut reniced_count = 0;

        let critical_spiking = self.running_processes
            .iter()
            .any(|p| p.process_id == critical_app_pid && p.current_cpu_usage >= 0.80);

        if critical_spiking {
            for proc in &mut self.running_processes {
                if proc.process_id != critical_app_pid && proc.priority_niceness < 10 {
                    proc.priority_niceness = 15;
                    reniced_count += 1;
                }
            }
        }

        reniced_count
    }
}

pub type GovernorMode = CpuGovernor;

pub struct SigmaGovernor {
    pub cores: Vec<CpuFreqCore>,
    pub mode: CpuGovernor,
}

impl SigmaGovernor {
    pub fn new(mode: CpuGovernor) -> Self {
        Self {
            cores: vec![CpuFreqCore::new(800, 4200, mode)],
            mode,
        }
    }

    pub fn set_mode(&mut self, mode: CpuGovernor) {
        self.mode = mode;
        self.cores[0] = CpuFreqCore::new(800, 4200, mode);
        self.cores[0].scale_frequency(100);
    }

    pub fn record_utilization(&mut self, core_idx: usize, utilization_ratio: f64) -> Result<(), &'static str> {
        if core_idx >= self.cores.len() {
            return Err("Invalid core index");
        }
        let util_pct = (utilization_ratio * 100.0) as usize;
        self.cores[core_idx].scale_frequency(util_pct);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpufreq_governors_transitions() {
        let core = CpuFreqCore::new(800, 4200, CpuGovernor::Performance);
        assert_eq!(core.scale_frequency(10), 4200);

        let core_saving = CpuFreqCore::new(800, 4200, CpuGovernor::Powersave);
        assert_eq!(core_saving.scale_frequency(95), 800);
    }

    #[test]
    fn test_tlp_power_management_modes() {
        let mut tlp = TlpPowerManager::new();
        tlp.set_pcie_aspm(AspmLevel::L1_2);
        tlp.set_dirty_writeback_duration(15);

        assert_eq!(tlp.pcie_aspm, AspmLevel::L1_2);
        assert_eq!(tlp.get_writeback_expiry_secs(), 15.0);
        assert!(tlp.runtime_pm);
    }

    #[test]
    fn test_energy_aware_thread_balancer() {
        let balancer = EnergyAwareThreadBalancer::new(1.0);
        assert_eq!(balancer.boost_interactive_threads(true, 10), 14);
        assert_eq!(balancer.boost_interactive_threads(false, 10), 8);
    }

    #[test]
    fn test_governor_modes() {
        let mut governor = SigmaGovernor::new(GovernorMode::Performance);
        assert_eq!(governor.cores[0].current_frequency_mhz.load(Ordering::SeqCst), 4200);

        governor.set_mode(GovernorMode::Powersave);
        assert_eq!(governor.cores[0].current_frequency_mhz.load(Ordering::SeqCst), 800);
    }

    #[test]
    fn test_governor_dynamic_scaling() {
        let mut governor = SigmaGovernor::new(GovernorMode::Schedutil);
        governor.record_utilization(0, 0.5).unwrap();
        assert_eq!(governor.cores[0].current_frequency_mhz.load(Ordering::SeqCst), 2625);
    }

    #[test]
    fn test_sigma_support_resource_optimizer() {
        let mut opt = SigmaSupportResourceOptimizer::new();
        opt.register_page_block(1001, true, 4096);
        opt.register_page_block(1002, false, 4096);

        let defragged = opt.execute_ram_defragmentation();
        assert_eq!(defragged, 1);
        assert_eq!(opt.total_defragmentations_completed, 1);
        assert!(!opt.managed_pages[0].is_fragmented);
    }

    #[test]
    fn test_sigma_support_priority_optimizer() {
        let mut opt = SigmaSupportPriorityOptimizer::new();
        opt.register_running_process(101, "zenith_desktop", -5);
        opt.register_running_process(102, "background_indexer", 0);

        opt.running_processes[0].current_cpu_usage = 0.85;

        let reniced = opt.optimize_cpu_priorities(101);
        assert_eq!(reniced, 1);
        assert_eq!(opt.running_processes[1].priority_niceness, 15);
    }
}
