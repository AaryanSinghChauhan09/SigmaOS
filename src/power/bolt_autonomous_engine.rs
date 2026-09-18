// Bolt ⚡ Autonomous Power & Thermal Optimization Engine
// Location: src/power/bolt_autonomous_engine.rs
//
// Real-time CPU frequency scaling via cpufreq interface, thermal governors,
// and zero-manual-intervention battery profile automation.

use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpufreqGovernor {
    Performance,
    Powersave,
    Schedutil,
    Ondemand,
    Conservative,
}

impl CpufreqGovernor {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Performance => "performance",
            Self::Powersave => "powersave",
            Self::Schedutil => "schedutil",
            Self::Ondemand => "ondemand",
            Self::Conservative => "conservative",
        }
    }
}

/// Linux /sys/devices/system/cpu/cpufreq interface emulation and hardware control shim.
#[derive(Debug, Clone)]
pub struct CpufreqInterface {
    pub core_id: u32,
    pub cur_freq_mhz: u32,
    pub min_freq_mhz: u32,
    pub max_freq_mhz: u32,
    pub active_governor: CpufreqGovernor,
}

impl CpufreqInterface {
    pub fn new(core_id: u32, min_mhz: u32, max_mhz: u32) -> Self {
        Self {
            core_id,
            cur_freq_mhz: max_mhz,
            min_freq_mhz: min_mhz,
            max_freq_mhz: max_mhz,
            active_governor: CpufreqGovernor::Schedutil,
        }
    }

    pub fn set_governor(&mut self, governor: CpufreqGovernor) {
        self.active_governor = governor;
    }

    /// Real-time CPU frequency scaling based on CPU utilization metrics and governor rules.
    pub fn scale_for_utilization(&mut self, utilization_pct: f32) -> u32 {
        let load = utilization_pct.clamp(0.0, 100.0);
        let freq = match self.active_governor {
            CpufreqGovernor::Performance => self.max_freq_mhz,
            CpufreqGovernor::Powersave => self.min_freq_mhz,
            CpufreqGovernor::Schedutil => {
                let range = (self.max_freq_mhz - self.min_freq_mhz) as f32;
                let calculated = self.min_freq_mhz + (range * (load / 100.0) * 1.25) as u32;
                calculated.clamp(self.min_freq_mhz, self.max_freq_mhz)
            }
            CpufreqGovernor::Ondemand => {
                if load > 75.0 {
                    self.max_freq_mhz
                } else {
                    let range = (self.max_freq_mhz - self.min_freq_mhz) as f32;
                    self.min_freq_mhz + (range * (load / 100.0)) as u32
                }
            }
            CpufreqGovernor::Conservative => {
                if load > 60.0 {
                    (self.cur_freq_mhz + 200).min(self.max_freq_mhz)
                } else if load < 20.0 {
                    self.cur_freq_mhz.saturating_sub(200).max(self.min_freq_mhz)
                } else {
                    self.cur_freq_mhz
                }
            }
        };

        self.cur_freq_mhz = freq;
        freq
    }
}

/// Multi-tier thermal zone trip points and RAPL power capping.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThermalState {
    Normal,
    PassiveThrottling,
    ActiveThrottling,
    HotThrottling,
    CriticalShutdown,
}

pub struct ThermalGovernor {
    pub current_temp_celsius: f32,
    pub passive_trip_celsius: f32,
    pub active_trip_celsius: f32,
    pub hot_trip_celsius: f32,
    pub critical_trip_celsius: f32,
    pub rapl_package_limit_watts: f32,
    pub active_state: ThermalState,
}

impl ThermalGovernor {
    pub fn new() -> Self {
        Self {
            current_temp_celsius: 40.0,
            passive_trip_celsius: 75.0,
            active_trip_celsius: 85.0,
            hot_trip_celsius: 92.0,
            critical_trip_celsius: 100.0,
            rapl_package_limit_watts: 45.0,
            active_state: ThermalState::Normal,
        }
    }

    /// Evaluates thermal zone temperatures and adjusts RAPL limits and throttling curves.
    pub fn evaluate_thermal_state(&mut self, temp_celsius: f32) -> ThermalState {
        self.current_temp_celsius = temp_celsius;

        if temp_celsius >= self.critical_trip_celsius {
            self.active_state = ThermalState::CriticalShutdown;
            self.rapl_package_limit_watts = 5.0;
        } else if temp_celsius >= self.hot_trip_celsius {
            self.active_state = ThermalState::HotThrottling;
            self.rapl_package_limit_watts = 15.0;
        } else if temp_celsius >= self.active_trip_celsius {
            self.active_state = ThermalState::ActiveThrottling;
            self.rapl_package_limit_watts = 25.0;
        } else if temp_celsius >= self.passive_trip_celsius {
            self.active_state = ThermalState::PassiveThrottling;
            self.rapl_package_limit_watts = 35.0;
        } else {
            self.active_state = ThermalState::Normal;
            self.rapl_package_limit_watts = 45.0;
        }

        self.active_state
    }
}

impl Default for ThermalGovernor {
    fn default() -> Self {
        Self::new()
    }
}

/// Battery power source and automation profiles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerSource {
    AcPower,
    Battery,
    LowBatteryCrisis,
}

#[derive(Debug, Clone)]
pub struct PowerProfileAutomationEngine {
    pub active_source: PowerSource,
    pub battery_percentage: u8,
    pub dirty_writeback_secs: u32,
    pub pcie_aspm_l1_enabled: bool,
    pub nvme_apst_enabled: bool,
    pub display_refresh_hz: u32,
    pub background_throttled: bool,
}

impl PowerProfileAutomationEngine {
    pub fn new() -> Self {
        Self {
            active_source: PowerSource::AcPower,
            battery_percentage: 100,
            dirty_writeback_secs: 5,
            pcie_aspm_l1_enabled: false,
            nvme_apst_enabled: true,
            display_refresh_hz: 144,
            background_throttled: false,
        }
    }

    /// Zero-manual-intervention auto-tuning of power, storage, and PCIe parameters based on power state.
    pub fn auto_tune_profile(&mut self, is_charging: bool, battery_pct: u8) -> PowerSource {
        self.battery_percentage = battery_pct;

        if is_charging {
            self.active_source = PowerSource::AcPower;
            self.dirty_writeback_secs = 5;
            self.pcie_aspm_l1_enabled = false;
            self.display_refresh_hz = 144;
            self.background_throttled = false;
        } else if battery_pct <= 15 {
            self.active_source = PowerSource::LowBatteryCrisis;
            self.dirty_writeback_secs = 30; // Defer disk writes to save maximum energy
            self.pcie_aspm_l1_enabled = true;
            self.display_refresh_hz = 60;
            self.background_throttled = true;
        } else {
            self.active_source = PowerSource::Battery;
            self.dirty_writeback_secs = 15;
            self.pcie_aspm_l1_enabled = true;
            self.display_refresh_hz = 90;
            self.background_throttled = false;
        }

        self.active_source
    }
}

impl Default for PowerProfileAutomationEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Optimization report generated by Bolt ⚡ Autonomous System.
#[derive(Debug, Clone)]
pub struct BoltOptimizationReport {
    pub active_power_source: PowerSource,
    pub thermal_state: ThermalState,
    pub target_cpu_freq_mhz: u32,
    pub active_governor: CpufreqGovernor,
    pub rapl_power_limit_watts: f32,
    pub display_refresh_hz: u32,
    pub dirty_writeback_secs: u32,
    pub pcie_aspm_l1_enabled: bool,
}

/// Bolt ⚡ Autonomous System Agent: Real-time CPU frequency scaling, thermal regulation, and power profile automation.
pub struct BoltAutonomousAgent {
    pub cpufreq_cores: Vec<CpufreqInterface>,
    pub thermal_governor: ThermalGovernor,
    pub power_automation: PowerProfileAutomationEngine,
    pub cycle_count: u64,
}

impl BoltAutonomousAgent {
    pub fn new(core_count: usize) -> Self {
        let mut cpufreq_cores = Vec::new();
        for i in 0..core_count {
            cpufreq_cores.push(CpufreqInterface::new(i as u32, 800, 4200));
        }

        Self {
            cpufreq_cores,
            thermal_governor: ThermalGovernor::new(),
            power_automation: PowerProfileAutomationEngine::new(),
            cycle_count: 0,
        }
    }

    /// Autonomous loop execution: samples metrics, auto-scales frequencies, regulates thermals, and applies power profiles.
    pub fn run_autonomous_cycle(
        &mut self,
        cpu_utilization_pct: f32,
        temp_celsius: f32,
        is_charging: bool,
        battery_pct: u8,
    ) -> BoltOptimizationReport {
        self.cycle_count += 1;

        // Step 1: Auto-tune power profile
        let power_source = self.power_automation.auto_tune_profile(is_charging, battery_pct);

        // Step 2: Evaluate thermal state
        let thermal_state = self.thermal_governor.evaluate_thermal_state(temp_celsius);

        // Step 3: Determine target governor based on thermal and power constraints
        let target_governor = if thermal_state == ThermalState::HotThrottling
            || thermal_state == ThermalState::CriticalShutdown
        {
            CpufreqGovernor::Powersave
        } else if power_source == PowerSource::LowBatteryCrisis {
            CpufreqGovernor::Powersave
        } else if power_source == PowerSource::Battery {
            CpufreqGovernor::Schedutil
        } else {
            CpufreqGovernor::Performance
        };

        // Step 4: Scale CPU core frequencies
        let mut scaled_freqs = Vec::new();
        for core in &mut self.cpufreq_cores {
            core.set_governor(target_governor);
            let freq = core.scale_for_utilization(cpu_utilization_pct);
            scaled_freqs.push(freq);
        }

        let avg_freq = if scaled_freqs.is_empty() {
            800
        } else {
            scaled_freqs.iter().sum::<u32>() / scaled_freqs.len() as u32
        };

        BoltOptimizationReport {
            active_power_source: power_source,
            thermal_state,
            target_cpu_freq_mhz: avg_freq,
            active_governor: target_governor,
            rapl_power_limit_watts: self.thermal_governor.rapl_package_limit_watts,
            display_refresh_hz: self.power_automation.display_refresh_hz,
            dirty_writeback_secs: self.power_automation.dirty_writeback_secs,
            pcie_aspm_l1_enabled: self.power_automation.pcie_aspm_l1_enabled,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpufreq_scaling() {
        let mut core = CpufreqInterface::new(0, 800, 4000);
        core.set_governor(CpufreqGovernor::Performance);
        assert_eq!(core.scale_for_utilization(10.0), 4000);

        core.set_governor(CpufreqGovernor::Powersave);
        assert_eq!(core.scale_for_utilization(90.0), 800);

        core.set_governor(CpufreqGovernor::Schedutil);
        let freq = core.scale_for_utilization(50.0);
        assert!(freq > 800 && freq <= 4000);
    }

    #[test]
    fn test_thermal_governor_trips() {
        let mut thermal = ThermalGovernor::new();
        assert_eq!(thermal.evaluate_thermal_state(45.0), ThermalState::Normal);
        assert_eq!(thermal.rapl_package_limit_watts, 45.0);

        assert_eq!(thermal.evaluate_thermal_state(88.0), ThermalState::ActiveThrottling);
        assert_eq!(thermal.rapl_package_limit_watts, 25.0);

        assert_eq!(thermal.evaluate_thermal_state(95.0), ThermalState::HotThrottling);
        assert_eq!(thermal.rapl_package_limit_watts, 15.0);
    }

    #[test]
    fn test_power_profile_auto_tuning() {
        let mut power = PowerProfileAutomationEngine::new();
        assert_eq!(power.auto_tune_profile(true, 100), PowerSource::AcPower);
        assert_eq!(power.display_refresh_hz, 144);

        assert_eq!(power.auto_tune_profile(false, 50), PowerSource::Battery);
        assert_eq!(power.display_refresh_hz, 90);
        assert!(power.pcie_aspm_l1_enabled);

        assert_eq!(power.auto_tune_profile(false, 10), PowerSource::LowBatteryCrisis);
        assert_eq!(power.display_refresh_hz, 60);
        assert_eq!(power.dirty_writeback_secs, 30);
    }

    #[test]
    fn test_bolt_autonomous_agent() {
        let mut agent = BoltAutonomousAgent::new(4);

        // AC charging mode
        let report = agent.run_autonomous_cycle(80.0, 50.0, true, 90);
        assert_eq!(report.active_power_source, PowerSource::AcPower);
        assert_eq!(report.active_governor, CpufreqGovernor::Performance);
        assert_eq!(report.target_cpu_freq_mhz, 4200);

        // Thermal surge mode
        let report = agent.run_autonomous_cycle(80.0, 95.0, true, 90);
        assert_eq!(report.thermal_state, ThermalState::HotThrottling);
        assert_eq!(report.active_governor, CpufreqGovernor::Powersave);
        assert_eq!(report.target_cpu_freq_mhz, 800);
    }
}
