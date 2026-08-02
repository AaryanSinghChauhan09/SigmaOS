#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS Dynamic CPU Performance & Power Governor (SigmaGovernor)
// Designed for real-time task scaling, thermal bursts, and CPU cycle optimization

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GovernorMode {
    Performance, // Keep high frequency for gaming/computation
    Powersave,   // Minimize frequency for battery saving
    Schedutil,   // Dynamic frequency scaling based on scheduler utilization
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
                    let calculated =
                        core.min_frequency_mhz + (delta * core.core_utilization) as u32;
                    core.current_frequency_mhz =
                        calculated.clamp(core.min_frequency_mhz, core.max_frequency_mhz);
                }
            }
        }
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
}
