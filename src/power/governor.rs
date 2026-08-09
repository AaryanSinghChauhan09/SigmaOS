// SigmaOS Dynamic CPU Performance & Power Governor (SigmaGovernor)
// Designed for real-time task scaling, thermal bursts, and CPU cycle optimization

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GovernorMode {
    Performance,     // Keep high frequency for gaming/computation
    Powersave,       // Minimize frequency for battery saving
    Schedutil,       // Dynamic frequency scaling based on scheduler utilization
}

pub struct CPUState {
    pub cpu_id: u32,
    pub current_frequency_mhz: u32,
    pub max_frequency_mhz: u32,
    pub min_frequency_mhz: u32,
    pub core_utilization: f32, // 0.0 to 1.0
}

pub struct SigmaGovernor {
    pub mode: GovernorMode,
    pub cores: Vec<CPUState>,
    pub thermal_throttle_threshold_celsius: f32,
}

impl SigmaGovernor {
    pub fn new(mode: GovernorMode) -> Self {
        let mut governor = SigmaGovernor {
            mode,
            cores: Vec::new(),
            thermal_throttle_threshold_celsius: 80.0,
        };
        // Seed default cores (Quad-core system)
        for i in 0..4 {
            governor.cores.push(CPUState {
                cpu_id: i,
                current_frequency_mhz: 2400,
                max_frequency_mhz: 4200,
                min_frequency_mhz: 800,
                core_utilization: 0.0,
            });
        }
        governor.adjust_frequencies();
        governor
    }

    pub fn set_mode(&mut self, mode: GovernorMode) {
        self.mode = mode;
        self.adjust_frequencies();
    }

    pub fn record_utilization(&mut self, cpu_id: u32, utilization: f32) -> Result<(), ()> {
        if let Some(core) = self.cores.iter_mut().find(|c| c.cpu_id == cpu_id) {
            core.core_utilization = utilization.clamp(0.0, 1.0);
            self.adjust_core_frequency(cpu_id);
            Ok(())
        } else {
            Err(())
        }
    }

    fn adjust_frequencies(&mut self) {
        let ids: Vec<u32> = self.cores.iter().map(|c| c.cpu_id).collect();
        for id in ids {
            self.adjust_core_frequency(id);
        }
    }

    fn adjust_core_frequency(&mut self, cpu_id: u32) {
        if let Some(core) = self.cores.iter_mut().find(|c| c.cpu_id == cpu_id) {
            match self.mode {
                GovernorMode::Performance => {
                    core.current_frequency_mhz = core.max_frequency_mhz;
                }
                GovernorMode::Powersave => {
                    core.current_frequency_mhz = core.min_frequency_mhz;
                }
                GovernorMode::Schedutil => {
                    // Dynamically scale between min and max based on utilization
                    let delta = (core.max_frequency_mhz - core.min_frequency_mhz) as f32;
                    let calculated = core.min_frequency_mhz + (delta * core.core_utilization) as u32;
                    core.current_frequency_mhz = calculated.clamp(core.min_frequency_mhz, core.max_frequency_mhz);
                }
            }
        }
    }
}

// =========================================================================
// 1. SigmaSupportResourceOptimizer (Glary/Advanced SystemCare RAM Defrag Parity)
// =========================================================================

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

    /// Emulates Glary Utilities RAM defragger: compacts page frames to reclaim system RAM
    pub fn execute_ram_defragmentation(&mut self) -> usize {
        let mut pages_compacted = 0;
        for page in &mut self.managed_pages {
            if page.is_fragmented {
                page.is_fragmented = false; // compacted
                pages_compacted += 1;
            }
        }
        if pages_compacted > 0 {
            self.total_defragmentations_completed += 1;
        }
        pages_compacted
    }
}

// =========================================================================
// 2. SigmaSupportPriorityOptimizer (Glary/Advanced SystemCare CPU Priority Parity)
// =========================================================================

pub struct RunningProcessTask {
    pub process_id: u32,
    pub process_name: String,
    pub priority_niceness: i32, // standard niceness (-20 to 19)
    pub current_cpu_usage: f32,
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

    /// Dynamically optimizes CPU priority by renicing low-priority apps when critical apps spike
    pub fn optimize_cpu_priorities(&mut self, critical_app_pid: u32) -> usize {
        let mut reniced_count = 0;

        let critical_spiking = self.running_processes
            .iter()
            .any(|p| p.process_id == critical_app_pid && p.current_cpu_usage >= 0.80);

        if critical_spiking {
            for proc in &mut self.running_processes {
                if proc.process_id != critical_app_pid && proc.priority_niceness < 10 {
                    proc.priority_niceness = 15; // lower priority (higher niceness)
                    reniced_count += 1;
                }
            }
        }

        reniced_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_governor_modes() {
        let mut governor = SigmaGovernor::new(GovernorMode::Performance);
        assert_eq!(governor.cores[0].current_frequency_mhz, 4200);

        governor.set_mode(GovernorMode::Powersave);
        assert_eq!(governor.cores[0].current_frequency_mhz, 800);
    }

    #[test]
    fn test_governor_dynamic_scaling() {
        let mut governor = SigmaGovernor::new(GovernorMode::Schedutil);
        // Standard utilization at 50%
        governor.record_utilization(0, 0.5).unwrap();
        // Delta = 4200 - 800 = 3400. 3400 * 0.5 = 1700. 800 + 1700 = 2500MHz.
        assert_eq!(governor.cores[0].current_frequency_mhz, 2500);
    }

    #[test]
    fn test_sigma_support_resource_optimizer() {
        let mut opt = SigmaSupportResourceOptimizer::new();
        opt.register_page_block(1001, true, 4096);
        opt.register_page_block(1002, false, 4096);

        let compacted = opt.execute_ram_defragmentation();
        assert_eq!(compacted, 1);
        assert_eq!(opt.total_defragmentations_completed, 1);
        assert!(!opt.managed_pages[0].is_fragmented);
    }

    #[test]
    fn test_sigma_support_priority_optimizer() {
        let mut opt = SigmaSupportPriorityOptimizer::new();
        opt.register_running_process(101, "zenith_desktop", -5);
        opt.register_running_process(102, "background_indexer", 0);

        // Simulate desktop application CPU spike (85% usage)
        opt.running_processes[0].current_cpu_usage = 0.85;

        let reniced = opt.optimize_cpu_priorities(101);
        assert_eq!(reniced, 1);
        assert_eq!(opt.running_processes[1].priority_niceness, 15); // background_indexer reniced to lower priority
    }
}
